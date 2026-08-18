//! Sensor controller for the Time-of-Flight (ToF) proximity sensors.

#![deny(missing_docs)]

use crate::tracing::{self, controller_context};
use crate::types::{SensorDirection, SensorMetadata};
use crate::Sender;
use core::fmt::Write as _;
use embassy_sync::blocking_mutex::raw::RawMutex;
use model::calibration::{ApplyCalibration, Calibration};
use model::interfaces::ProximitySensor;
use model::types::{Direction, PeriodicInterval, PeripheralError, SensorReading};
use peripheral::ToPeripheralError;
use platform::{
    select_branch_with_timeout, subcommand_enum, BlockingAsyncFlash, CliSignal, OnceLock,
};

/// Trait for waiting on a data-ready interrupt pin.
#[allow(async_fn_in_trait)]
pub trait DataReadyPin {
    /// Wait for the data-ready pin to trigger (active state).
    async fn wait_for_data_ready(&mut self);
}

/// A dummy mock implementation of DataReadyPin that waits forever.
pub struct DummyDataReadyPin;

impl DataReadyPin for DummyDataReadyPin {
    async fn wait_for_data_ready(&mut self) {
        // Sleep forever to let the periodic timeout drive updates
        embassy_time::Timer::after_secs(3600 * 24).await;
    }
}

/// Maximum raw distance value in mm allowed during proximity sensor calibration.
const MAX_CALIBRATION_RAW_MM: u16 = 900;

/// Type alias for the sensor command sender.
pub type SensorSender<M> = embassy_sync::channel::Sender<'static, M, SensorCommand, 4>;

/// One-way commands sent to the Sensor Controller.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum SensorCommand {
    /// Force proximity sensor check and print telemetry logs
    ReadSensors,
    /// Force proximity sensor check and signal completion via CliSignal
    ReadSensorsWithSignal(CliSignal<Result<SensorReading, PeripheralError>>),
    /// Force raw proximity sensor check and signal completion via CliSignal
    ReadRawSensorsWithSignal(CliSignal<Result<SensorReading, PeripheralError>>),
    /// Force raw proximity sensor check with return rate and signal completion via CliSignal
    ReadXTalkRawWithSignal(CliSignal<Result<(SensorReading, u16), PeripheralError>>),
    /// Set periodic automatic reading interval
    SetInterval(PeriodicInterval),
}

/// Represents an async CLI command to be executed outside the synchronous CLI loop.
#[derive(Clone)]
pub enum PendingCommand {
    /// Sensor status command with optional number of readings.
    Status(usize),
    /// Calibrate near proximity.
    CalNear {
        /// Sensor direction
        direction: crate::types::SensorDirection,
        /// Calibration partition
        partition: heapless::String<16>,
    },
    /// Calibrate far proximity.
    CalFar {
        /// Sensor direction
        direction: crate::types::SensorDirection,
        /// Calibration partition
        partition: heapless::String<16>,
    },
    /// Calibrate crosstalk.
    CalXTalk {
        /// Sensor direction
        direction: crate::types::SensorDirection,
        /// Distance or partition parameter
        distance_or_partition: heapless::String<16>,
    },
}

impl PendingCommand {
    /// Parses a PendingCommand from raw CLI strings.
    pub fn parse(
        subcommand: SensorSubcommand,
        arg1: Option<&str>,
        partition: Option<&str>,
    ) -> Result<Self, &'static str> {
        use crate::types::SensorDirection;
        use heapless::String;

        let cmd = match subcommand {
            SensorSubcommand::Status => {
                let num_readings = if let Some(s) = arg1 {
                    s.parse::<usize>()
                        .map_err(|_| "Invalid number of readings")?
                } else {
                    1
                };
                PendingCommand::Status(num_readings)
            }
            SensorSubcommand::CalNear => {
                let dir_str = arg1.ok_or("Missing direction parameter")?;
                let direction = match dir_str {
                    "north" => SensorDirection::North,
                    "east" => SensorDirection::East,
                    "west" => SensorDirection::West,
                    _ => return Err("Invalid direction. Expected: north, east, west"),
                };
                let part_str = partition.unwrap_or("telemetry");
                let mut s = String::new();
                s.push_str(part_str)
                    .map_err(|_| "Partition name too long")?;
                PendingCommand::CalNear {
                    direction,
                    partition: s,
                }
            }
            SensorSubcommand::CalFar => {
                let dir_str = arg1.ok_or("Missing direction parameter")?;
                let direction = match dir_str {
                    "north" => SensorDirection::North,
                    "east" => SensorDirection::East,
                    "west" => SensorDirection::West,
                    _ => return Err("Invalid direction. Expected: north, east, west"),
                };
                let part_str = partition.unwrap_or("telemetry");
                let mut s = String::new();
                s.push_str(part_str)
                    .map_err(|_| "Partition name too long")?;
                PendingCommand::CalFar {
                    direction,
                    partition: s,
                }
            }
            SensorSubcommand::CalXTalk => {
                let dir_str = arg1.ok_or("Missing direction parameter")?;
                let direction = match dir_str {
                    "north" => SensorDirection::North,
                    "east" => SensorDirection::East,
                    "west" => SensorDirection::West,
                    _ => return Err("Invalid direction. Expected: north, east, west"),
                };
                let part_str = partition.unwrap_or("100");
                let mut s = String::new();
                s.push_str(part_str).map_err(|_| "Parameter too long")?;
                PendingCommand::CalXTalk {
                    direction,
                    distance_or_partition: s,
                }
            }
        };
        Ok(cmd)
    }
}

/// Trait for reading data from a generic sensor type.
pub trait SensorReader<S> {
    /// The trait-specific context block passed to the read_data method.
    type Context;
    /// The type of data returned by the read_data method.
    type Data: Copy;
    /// The error type returned by the read_data method.
    type Error;

    /// Reads data from the sensor using the provided context block.
    async fn read_data(sensor: &mut S, ctx: &Self::Context) -> Result<Self::Data, Self::Error>;
}

/// Context block for reading proximity sensors.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct ProximityReaderContext {
    /// The proximity threshold in millimeters under which target presence is detected.
    pub wake_threshold_mm: u16,
}

/// A reader adapter for proximity sensors.
pub struct ProximityReader;

impl<S: ProximitySensor> SensorReader<S> for ProximityReader {
    type Context = ProximityReaderContext;
    type Data = SensorReading;
    type Error = S::Error;

    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    async fn read_data(sensor: &mut S, _ctx: &Self::Context) -> Result<Self::Data, Self::Error> {
        sensor.read_distance_mm().await
    }
}

/// A trait to convert proximity sensor reading updates to a system command.
pub trait FromProximityUpdate {
    /// Constructs a command from sensor metadata and a typesafe reading.
    fn from_proximity_update(metadata: SensorMetadata, reading: SensorReading) -> Self;
}

impl FromProximityUpdate for () {
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    fn from_proximity_update(_metadata: SensorMetadata, _reading: SensorReading) -> Self {}
}

/// State manager for coordinating physical sensor access, interrupts, and notifications.
pub struct SensorStateManager<
    'a,
    S,
    Data,
    M: embassy_sync::blocking_mutex::raw::RawMutex = embassy_sync::blocking_mutex::raw::NoopRawMutex,
    Pin = DummyDataReadyPin,
    Cmd = (),
    const SYS_CAP: usize = 16,
> {
    metadata: SensorMetadata,
    sensor: S,
    periodic_interval: PeriodicInterval,
    upstream_tx: Option<Sender<'a, M, Cmd, SYS_CAP>>,
    interrupt_pin: Option<Pin>,
    _marker: core::marker::PhantomData<Data>,
}

impl<
        'a,
        S,
        Data,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin,
        Cmd,
        const SYS_CAP: usize,
    > SensorStateManager<'a, S, Data, M, Pin, Cmd, SYS_CAP>
{
    /// Creates a new SensorStateManager.
    pub const fn new(
        metadata: SensorMetadata,
        sensor: S,
        upstream_tx: Option<Sender<'a, M, Cmd, SYS_CAP>>,
        interrupt_pin: Option<Pin>,
    ) -> Self {
        Self {
            metadata,
            sensor,
            periodic_interval: PeriodicInterval::None,
            upstream_tx,
            interrupt_pin,
            _marker: core::marker::PhantomData,
        }
    }

    /// Gets the sensor metadata.
    pub fn metadata(&self) -> SensorMetadata {
        self.metadata
    }

    /// Gets the sensor direction.
    pub fn direction(&self) -> Direction {
        self.metadata.direction
    }

    /// Gets a mutable reference to the underlying sensor.
    pub fn sensor_mut(&mut self) -> &mut S {
        &mut self.sensor
    }

    /// Gets a reference to the underlying sensor.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    pub fn sensor(&self) -> &S {
        &self.sensor
    }

    /// Gets whether periodic monitoring is enabled.
    pub fn is_periodic_enabled(&self) -> bool {
        self.periodic_interval != PeriodicInterval::None
    }

    /// Gets the periodic monitoring interval.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    pub fn periodic_interval(&self) -> PeriodicInterval {
        self.periodic_interval
    }

    /// Sets whether periodic monitoring is enabled.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    pub fn set_periodic_enabled(&mut self, enabled: bool) {
        self.periodic_interval = if enabled {
            PeriodicInterval::UpdateMs(1000)
        } else {
            PeriodicInterval::None
        };
    }

    /// Sets the periodic monitoring interval.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    pub fn set_periodic_interval(&mut self, interval: PeriodicInterval) {
        self.periodic_interval = interval;
    }
}

impl<
        'a,
        S,
        Data: Copy + Into<SensorReading>,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin,
        Cmd: FromProximityUpdate + Clone + core::fmt::Debug,
        const SYS_CAP: usize,
    > SensorStateManager<'a, S, Data, M, Pin, Cmd, SYS_CAP>
{
    /// Sends a command upstream if configured.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    pub fn notify_upstream(&self, data: Data) {
        if let Some(tx) = &self.upstream_tx {
            let cmd = Cmd::from_proximity_update(self.metadata, data.into());
            if tx.try_send(cmd).is_err() {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::trace!(
                    "Sensor Controller: Upstream channel full, dropping proximity update."
                );
            }
        }
    }
}

impl<
        'a,
        S,
        Data,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin: DataReadyPin,
        Cmd,
        const SYS_CAP: usize,
    > SensorStateManager<'a, S, Data, M, Pin, Cmd, SYS_CAP>
{
    /// Waits for the data ready interrupt to trigger if the interrupt pin is configured.
    pub async fn wait_for_data_ready(&mut self) {
        if let Some(ref mut pin) = self.interrupt_pin {
            pin.wait_for_data_ready().await;
        } else {
            core::future::pending::<()>().await;
        }
    }
}

/// A controller that coordinates readings from a single proximity (ToF) sensor.
#[controller_context(core1_feature = "sensors-core")]
pub struct SensorController<
    'a,
    S,
    M: embassy_sync::blocking_mutex::raw::RawMutex + 'static = embassy_sync::blocking_mutex::raw::NoopRawMutex,
    Pin = DummyDataReadyPin,
    Cmd = (),
    Reader: SensorReader<S> = ProximityReader,
    const SYS_CAP: usize = 16,
> {
    state_manager: SensorStateManager<'a, S, Reader::Data, M, Pin, Cmd, SYS_CAP>,
    latest_data: Reader::Data,
    context: Reader::Context,
    command_tx: Option<SensorSender<M>>,
}

impl<
        'a,
        S,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin,
        Cmd,
        Reader: SensorReader<S>,
        const SYS_CAP: usize,
    > core::ops::Deref for SensorController<'a, S, M, Pin, Cmd, Reader, SYS_CAP>
{
    type Target = SensorStateManager<'a, S, Reader::Data, M, Pin, Cmd, SYS_CAP>;

    fn deref(&self) -> &Self::Target {
        &self.state_manager
    }
}

impl<
        'a,
        S,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin,
        Cmd,
        Reader: SensorReader<S>,
        const SYS_CAP: usize,
    > core::ops::DerefMut for SensorController<'a, S, M, Pin, Cmd, Reader, SYS_CAP>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state_manager
    }
}

impl<'a, S: ProximitySensor>
    SensorController<
        'a,
        S,
        embassy_sync::blocking_mutex::raw::NoopRawMutex,
        DummyDataReadyPin,
        (),
        ProximityReader,
        16,
    >
{
    /// Creates a new SensorController managing a single proximity sensor.
    pub const fn new(metadata: SensorMetadata, sensor: S, wake_threshold_mm: u16) -> Self {
        Self {
            state_manager: SensorStateManager::new(metadata, sensor, None, None),
            latest_data: SensorReading::Invalid,
            context: ProximityReaderContext { wake_threshold_mm },
            command_tx: None,
        }
    }
}

impl<
        'a,
        S: ProximitySensor,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Cmd: FromProximityUpdate + Clone + core::fmt::Debug,
        const SYS_CAP: usize,
    > SensorController<'a, S, M, DummyDataReadyPin, Cmd, ProximityReader, SYS_CAP>
{
    /// Creates a new SensorController with upstream system notification.
    pub fn new_with_fusion(
        metadata: SensorMetadata,
        sensor: S,
        upstream_tx: Sender<'a, M, Cmd, SYS_CAP>,
        wake_threshold_mm: u16,
    ) -> Self {
        Self {
            state_manager: SensorStateManager::new(metadata, sensor, Some(upstream_tx), None),
            latest_data: SensorReading::Invalid,
            context: ProximityReaderContext { wake_threshold_mm },
            command_tx: None,
        }
    }
}

impl<
        'a,
        S: ProximitySensor
            + ApplyCalibration<Input = SensorReading, Output = SensorReading, Error = &'static str>,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin: DataReadyPin,
        Cmd: FromProximityUpdate + Clone + core::fmt::Debug,
        Reader: SensorReader<S, Data = SensorReading>,
        const SYS_CAP: usize,
    > SensorController<'a, S, M, Pin, Cmd, Reader, SYS_CAP>
where
    Reader::Data: Copy + Into<SensorReading>,
    Reader::Error: core::fmt::Debug,
{
    /// Creates a generic SensorController.
    pub fn new_generic(
        metadata: SensorMetadata,
        sensor: S,
        latest_data: Reader::Data,
        interrupt_pin: Option<Pin>,
        context: Reader::Context,
    ) -> Self {
        Self {
            state_manager: SensorStateManager::new(metadata, sensor, None, interrupt_pin),
            latest_data,
            context,
            command_tx: None,
        }
    }

    /// Creates a generic SensorController with upstream system notification.
    pub fn new_generic_with_fusion(
        metadata: SensorMetadata,
        sensor: S,
        latest_data: Reader::Data,
        upstream_tx: Sender<'a, M, Cmd, SYS_CAP>,
        interrupt_pin: Option<Pin>,
        context: Reader::Context,
    ) -> Self {
        Self {
            state_manager: SensorStateManager::new(
                metadata,
                sensor,
                Some(upstream_tx),
                interrupt_pin,
            ),
            latest_data,
            context,
            command_tx: None,
        }
    }

    /// Binds a command sender channel to this controller.
    pub fn bind_command_tx(&mut self, tx: SensorSender<M>) {
        self.command_tx = Some(tx);
    }

    /// Gets a mutable reference to the underlying sensor.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    pub fn sensor_mut(&mut self) -> &mut S {
        self.state_manager.sensor_mut()
    }

    /// Gets the latest read sensor data.
    pub fn latest_data(&self) -> Reader::Data {
        self.latest_data
    }

    /// Gets the sensor direction.
    pub fn direction(&self) -> Direction {
        self.state_manager.direction()
    }

    /// Gets the sensor metadata.
    pub fn metadata(&self) -> SensorMetadata {
        self.state_manager.metadata()
    }

    /// Gets whether periodic monitoring is enabled.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    pub fn is_periodic_enabled(&self) -> bool {
        self.state_manager.is_periodic_enabled()
    }

    /// Ticks the sensor control loop, updating proximity distance.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", name = "sensor_controller::update", level = "trace")]
    pub async fn update(&mut self) -> Result<Reader::Data, Reader::Error> {
        let raw_data = Reader::read_data(self.state_manager.sensor_mut(), &self.context).await?;
        let data = self
            .state_manager
            .sensor()
            .apply_calibration(raw_data)
            .unwrap_or(raw_data);

        self.latest_data = data;

        self.notify_upstream(data);

        Ok(data)
    }

    /// Handles a SensorCommand.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(
        core1 = "core1",
        name = "sensor_controller::handle_command",
        level = "info",
        skip(cmd)
    )]
    pub async fn handle_command(&mut self, cmd: SensorCommand) {
        match cmd {
            SensorCommand::ReadSensors => {
                let _ = self.update().await;
            }
            SensorCommand::ReadSensorsWithSignal(signal_ptr) => {
                let res = Reader::read_data(self.state_manager.sensor_mut(), &self.context)
                    .await
                    .map(|raw_d| {
                        let d = self
                            .state_manager
                            .sensor()
                            .apply_calibration(raw_d)
                            .unwrap_or(raw_d);
                        self.latest_data = d;
                        d
                    })
                    .map_err(|_| PeripheralError::DeviceNotAvailable);
                let _ = unsafe { signal_ptr.set(res) };
            }
            SensorCommand::ReadRawSensorsWithSignal(signal_ptr) => {
                let res = Reader::read_data(self.state_manager.sensor_mut(), &self.context)
                    .await
                    .map_err(|_| PeripheralError::DeviceNotAvailable);
                let _ = unsafe { signal_ptr.set(res) };
            }
            SensorCommand::ReadXTalkRawWithSignal(signal_ptr) => {
                let res = self
                    .state_manager
                    .sensor_mut()
                    .read_raw_distance_and_rate()
                    .await
                    .map_err(|_| PeripheralError::DeviceNotAvailable);
                let _ = unsafe { signal_ptr.set(res) };
            }
            SensorCommand::SetInterval(interval) => {
                self.set_periodic_interval(interval);
            }
        }
    }

    /// Runs the controller's main run loop, executing periodic telemetry updates.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    pub async fn run(
        &mut self,
        command_rx: embassy_sync::channel::Receiver<'static, M, SensorCommand, 4>,
    ) -> ! {
        let mut last_read = embassy_time::Instant::now();
        loop {
            let timeout_dur = match self.state_manager.periodic_interval() {
                PeriodicInterval::None => crate::OVERFLOW_SAFE_MAX_DURATION,
                PeriodicInterval::UpdateMs(ms) => embassy_time::Duration::from_millis(ms as u64),
            };

            let is_periodic = !matches!(
                self.state_manager.periodic_interval(),
                PeriodicInterval::None
            );

            let res = if is_periodic {
                let next_read_time = last_read + timeout_dur;
                let now = embassy_time::Instant::now();
                let remaining = if next_read_time > now {
                    next_read_time - now
                } else {
                    embassy_time::Duration::from_millis(0)
                };

                // When actively polling periodically, ignore interrupt pin transitions to prevent unthrottled read storms
                match platform::with_timeout!(command_rx.receive(), remaining).await {
                    Some(cmd) => {
                        self.handle_command(cmd).await;
                        Some(())
                    }
                    None => {
                        last_read = embassy_time::Instant::now();
                        None
                    }
                }
            } else {
                // When deep sleeping (no periodic interval), wait for either a command or the interrupt pin to wake us up
                let res = select_branch_with_timeout!(
                    timeout_dur,
                    command_rx.receive() => |cmd| {
                        self.handle_command(cmd).await;
                        Some(())
                    },
                    self.wait_for_data_ready() => || {
                        None
                    },
                );
                if res.is_none() {
                    last_read = embassy_time::Instant::now();
                }
                res
            };

            if res.is_none() && self.update().await.is_err() {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::warn!("SensorController: Periodic read failed.");
            }
        }
    }
}

impl<
        'a,
        S: ProximitySensor,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin: DataReadyPin,
        Cmd: FromProximityUpdate + Clone + core::fmt::Debug,
        const SYS_CAP: usize,
    > SensorController<'a, S, M, Pin, Cmd, ProximityReader, SYS_CAP>
{
    /// Creates a new SensorController with upstream system notification and interrupt pin support.
    pub fn new_with_fusion_and_interrupt(
        metadata: SensorMetadata,
        sensor: S,
        upstream_tx: Sender<'a, M, Cmd, SYS_CAP>,
        interrupt_pin: Pin,
        wake_threshold_mm: u16,
    ) -> Self {
        Self {
            state_manager: SensorStateManager::new(
                metadata,
                sensor,
                Some(upstream_tx),
                Some(interrupt_pin),
            ),
            latest_data: SensorReading::Invalid,
            context: ProximityReaderContext { wake_threshold_mm },
            command_tx: None,
        }
    }

    /// Gets the current proximity telemetry reading.
    pub fn telemetry(&self) -> model::types::SensorTelemetry {
        model::types::SensorTelemetry::Status(self.direction(), self.latest_data)
    }

    /// Gets the latest read proximity telemetry distance.
    pub fn latest_distance(&self) -> SensorReading {
        self.latest_data
    }
}

impl<
        'a,
        S: ProximitySensor
            + Calibration<Store = model::calibration::Vl53l0xCalibration>
            + ApplyCalibration<Input = SensorReading, Output = SensorReading, Error = &'static str>,
        M: embassy_sync::blocking_mutex::raw::RawMutex + 'static,
        Pin,
        Cmd,
    > crate::ProximityReader for SensorController<'a, S, M, Pin, Cmd, ProximityReader>
where
    <S as ProximitySensor>::Error: ToPeripheralError,
{
    async fn read_distance(&mut self) -> Result<SensorReading, PeripheralError> {
        let lock = OnceLock::new();
        let lock_ptr = CliSignal::new(&lock);
        self.send_command(SensorCommand::ReadSensorsWithSignal(lock_ptr))?;
        *lock.wait().await
    }

    fn latest_distance(&self) -> SensorReading {
        self.latest_data
    }

    fn send_command(
        &mut self,
        cmd: crate::sensor_controller::SensorCommand,
    ) -> Result<(), PeripheralError> {
        if let Some(ref tx) = self.command_tx {
            tx.try_send(cmd)
                .map_err(|_| PeripheralError::DeviceNotAvailable)
        } else {
            Err(PeripheralError::DeviceNotAvailable)
        }
    }

    fn update_calibration(
        &mut self,
        cal: &model::calibration::Vl53l0xCalibration,
    ) -> Result<(), PeripheralError> {
        use model::calibration::Calibration as _;
        self.set_calibration(cal);
        Ok(())
    }
}

impl<
        'a,
        S: ProximitySensor
            + Calibration
            + ApplyCalibration<Input = SensorReading, Output = SensorReading>,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin,
        Cmd,
    > model::calibration::Calibration for SensorController<'a, S, M, Pin, Cmd, ProximityReader>
{
    const CALIBRATION_FILE_NAME: &'static str = S::CALIBRATION_FILE_NAME;
    type Store = S::Store;

    fn set_calibration(&mut self, store: &Self::Store) {
        self.sensor_mut().set_calibration(store);
    }

    fn get_calibration(&self) -> Self::Store {
        self.state_manager.sensor().get_calibration()
    }
}

impl<
        'a,
        S: ProximitySensor
            + Calibration
            + ApplyCalibration<Input = SensorReading, Output = SensorReading, Error = &'static str>,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin,
        Cmd,
    > model::calibration::ApplyCalibration
    for SensorController<'a, S, M, Pin, Cmd, ProximityReader>
{
    type Input = SensorReading;
    type Output = SensorReading;
    type Error = &'static str;

    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    fn apply_calibration(&self, reading: Self::Input) -> Result<Self::Output, Self::Error> {
        self.state_manager.sensor().apply_calibration(reading)
    }
}

impl<'a> embedded_cli::arguments::FromArgument<'a> for SensorDirection {
    fn from_arg(arg: &'a str) -> Result<Self, embedded_cli::arguments::FromArgumentError<'a>> {
        match arg {
            "north" => Ok(SensorDirection::North),
            "east" => Ok(SensorDirection::East),
            "west" => Ok(SensorDirection::West),
            _ => Err(embedded_cli::arguments::FromArgumentError {
                value: arg,
                expected: "one of 'north', 'east', or 'west'",
            }),
        }
    }
}

impl From<SensorDirection> for model::types::Direction {
    fn from(dir: SensorDirection) -> Self {
        match dir {
            SensorDirection::North => model::types::Direction::North,
            SensorDirection::East => model::types::Direction::East,
            SensorDirection::West => model::types::Direction::West,
        }
    }
}

subcommand_enum! {
    /// Sensor subcommands for CLI processing.
    pub enum SensorSubcommand {
        /// Read sensor values
        Status,
        /// Calibrate near proximity
        CalNear = "cal_near",
        /// Calibrate far proximity
        CalFar = "cal_far",
        /// Calibrate crosstalk
        CalXTalk = "cal_xtalk",
    }
    "Invalid sensor subcommand. Expected: status, cal_near, cal_far, cal_xtalk"
}

/// Processes sensor-specific CLI subcommands by validating and delegating.
pub async fn handle_sensor_cli<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    C: crate::ShellConfig,
>(
    resolver: &impl crate::ShellDeviceResolver<C>,
    subcommand: Option<SensorSubcommand>,
    arg1: Option<&str>,
    partition: Option<&str>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    use crate::ProximityReader as _;
    let command = PendingCommand::parse(
        subcommand.ok_or("Missing sensor subcommand")?,
        arg1,
        partition,
    )?;

    let mut fs_buf = resolver.lock_fs_buffer()?;
    let fs_buf_static = unsafe { fs_buf.as_static_mut() };

    match command {
        PendingCommand::Status(num_readings) => {
            struct WriteBuffer<'a> {
                buf: &'a mut [u8],
                len: usize,
            }

            impl<'a> WriteBuffer<'a> {
                fn new(buf: &'a mut [u8]) -> Self {
                    Self { buf, len: 0 }
                }

                fn as_str(&self) -> &str {
                    core::str::from_utf8(&self.buf[..self.len]).unwrap_or("")
                }
            }

            impl<'a> core::fmt::Write for WriteBuffer<'a> {
                fn write_str(&mut self, s: &str) -> core::fmt::Result {
                    let bytes = s.as_bytes();
                    let remaining = self.buf.len() - self.len;
                    let to_copy = core::cmp::min(bytes.len(), remaining);
                    self.buf[self.len..self.len + to_copy].copy_from_slice(&bytes[..to_copy]);
                    self.len += to_copy;
                    Ok(())
                }
            }

            let proximity_cal = (|| {
                let resolved = resolver.resolve_partition(None).ok()?;
                let (map_fs, flash_ptr) = match resolved {
                    crate::ResolvedPartition::Map(fs, ptr) => (fs, ptr),
                    _ => return None,
                };
                let flash_ref = unsafe { &mut *flash_ptr };
                let mut async_flash = BlockingAsyncFlash(flash_ref);

                let mut buf = [0u8; 128];
                platform::flash::read_calibration_direct_blocking::<
                    _,
                    <C::SensorCtrl as Calibration>::Store,
                >(
                    &mut async_flash,
                    map_fs,
                    fs_buf_static,
                    <C::SensorCtrl as Calibration>::CALIBRATION_FILE_NAME,
                    &mut buf,
                )
            })();

            let _ = core::writeln!(writer);
            // Print headers
            for named in resolver.sensors() {
                let _ = core::write!(writer, "{:<40}", named.name);
            }
            let _ = core::writeln!(writer);

            for _ in 0..num_readings {
                let sensors = resolver.sensors();
                let count = core::cmp::min(sensors.len(), 8);
                let mut readings = [Err(PeripheralError::DeviceNotAvailable); 8];

                for (i, named) in sensors.iter().take(count).enumerate() {
                    let sensor = unsafe { &mut *named.device };
                    let lock = OnceLock::new();
                    let lock_ptr = CliSignal::new(&lock);
                    readings[i] = if sensor
                        .send_command(SensorCommand::ReadSensorsWithSignal(lock_ptr))
                        .is_ok()
                    {
                        *lock.wait().await
                    } else {
                        Err(PeripheralError::DeviceNotAvailable)
                    };
                }

                for (i, named) in sensors.iter().take(count).enumerate() {
                    let dist = readings[i];
                    let direction = match named.name {
                        "north" => Some(model::types::Direction::North),
                        "east" => Some(model::types::Direction::East),
                        "west" => Some(model::types::Direction::West),
                        _ => None,
                    };

                    let mut val_buf = [0u8; 64];
                    let mut val_writer = WriteBuffer::new(&mut val_buf);
                    match dist {
                        Ok(reading) => match reading {
                            SensorReading::Proximity(d) => {
                                let cal_reading = if let (Some(dir), Some(cal)) =
                                    (direction, proximity_cal.as_ref())
                                {
                                    cal.map_calibrated(dir, reading).unwrap_or(reading)
                                } else {
                                    reading
                                };
                                let cal_d = match cal_reading {
                                    SensorReading::Proximity(val) => val,
                                    _ => d,
                                };
                                let xtalk = if let (Some(dir), Some(cal)) =
                                    (direction, proximity_cal.as_ref())
                                {
                                    cal.xtalk_m_mcps[dir as usize]
                                } else {
                                    0
                                };
                                if xtalk > 0 {
                                    let _ = core::write!(
                                        &mut val_writer,
                                        "{}mm (cal: {}mm, xtalk: {}mMCPS)",
                                        d,
                                        cal_d,
                                        xtalk
                                    );
                                } else if cal_d != d {
                                    let _ =
                                        core::write!(&mut val_writer, "{}mm (cal: {}mm)", d, cal_d);
                                } else {
                                    let _ = core::write!(&mut val_writer, "{}mm", d);
                                }
                            }
                            SensorReading::Invalid => {
                                let _ = core::write!(&mut val_writer, "INVALID");
                            }
                        },
                        Err(_) => {
                            let _ = core::write!(&mut val_writer, "FAILED");
                        }
                    }
                    let _ = core::write!(writer, "{:<40}", val_writer.as_str());
                }
                let _ = core::writeln!(writer);
            }
            Ok(())
        }
        PendingCommand::CalNear {
            direction,
            partition,
        } => {
            let dir_str = match direction {
                SensorDirection::North => "north",
                SensorDirection::East => "east",
                SensorDirection::West => "west",
            };

            let name = match direction {
                SensorDirection::North => "North",
                SensorDirection::East => "East",
                SensorDirection::West => "West",
            };

            let sensor_ctrl = resolver.resolve_sensor(Some(dir_str))?;
            let mut d_raw = SensorReading::Invalid;
            for _ in 0..10 {
                let lock = OnceLock::new();
                let lock_ptr = CliSignal::new(&lock);
                if sensor_ctrl
                    .send_command(SensorCommand::ReadRawSensorsWithSignal(lock_ptr))
                    .is_ok()
                {
                    if let Ok(reading) = *lock.wait().await {
                        d_raw = reading;
                        if let SensorReading::Proximity(d) = reading {
                            if d < MAX_CALIBRATION_RAW_MM {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                embassy_time::block_for(embassy_time::Duration::from_millis(50));
            }

            let d_val = match d_raw {
                SensorReading::Proximity(d) => {
                    if d < MAX_CALIBRATION_RAW_MM {
                        d
                    } else {
                        return Err("Target too far for cover calibration");
                    }
                }
                SensorReading::Invalid => return Err("Sensor disconnected or invalid reading"),
            };

            let _ = core::writeln!(
                writer,
                "\r\nCalibrating cover (near) for {} sensor: Raw distance = {} mm",
                name,
                d_val
            );

            let (map_fs, flash_ptr) = match resolver.resolve_partition(Some(partition.as_str()))? {
                crate::ResolvedPartition::Map(fs, ptr) => (fs, ptr),
                _ => return Err("Requested partition is not a map filesystem"),
            };
            let flash_ref = unsafe { &mut *flash_ptr };
            let mut async_flash = BlockingAsyncFlash(flash_ref);

            let mut buf = [0u8; 128];
            let mut proximity_cal = platform::flash::read_calibration_direct_blocking::<
                _,
                <C::SensorCtrl as Calibration>::Store,
            >(
                &mut async_flash,
                map_fs.clone(),
                fs_buf_static,
                <C::SensorCtrl as Calibration>::CALIBRATION_FILE_NAME,
                &mut buf,
            )
            .unwrap_or_default();

            let dir = model::types::Direction::from(direction);
            proximity_cal.sensors[dir as usize].low = d_val;

            let mut write_buf = [0u8; 128];
            platform::flash::write_calibration_direct_blocking(
                &mut async_flash,
                map_fs.clone(),
                fs_buf_static,
                <C::SensorCtrl as Calibration>::CALIBRATION_FILE_NAME,
                &proximity_cal,
                &mut write_buf,
            )
            .map(|_| {
                let _ = sensor_ctrl.update_calibration(&proximity_cal);
                let _ = core::writeln!(writer, "Saved cover calibration for {} to flash.", name);
            })
            .map_err(|_| "Error saving calibration to flash")
        }
        PendingCommand::CalFar {
            direction,
            partition,
        } => {
            let dir_str = match direction {
                SensorDirection::North => "north",
                SensorDirection::East => "east",
                SensorDirection::West => "west",
            };

            let name = match direction {
                SensorDirection::North => "North",
                SensorDirection::East => "East",
                SensorDirection::West => "West",
            };

            let sensor_ctrl = resolver.resolve_sensor(Some(dir_str))?;
            let mut d_raw = SensorReading::Invalid;
            for _ in 0..10 {
                let lock = OnceLock::new();
                let lock_ptr = CliSignal::new(&lock);
                if sensor_ctrl
                    .send_command(SensorCommand::ReadRawSensorsWithSignal(lock_ptr))
                    .is_ok()
                {
                    if let Ok(reading) = *lock.wait().await {
                        d_raw = reading;
                        if let SensorReading::Proximity(d) = reading {
                            if d < MAX_CALIBRATION_RAW_MM {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                embassy_time::block_for(embassy_time::Duration::from_millis(50));
            }

            let d_val = match d_raw {
                SensorReading::Proximity(d) => {
                    if d < MAX_CALIBRATION_RAW_MM {
                        d
                    } else {
                        return Err("Target too far for 100mm calibration");
                    }
                }
                SensorReading::Invalid => return Err("Sensor disconnected or invalid reading"),
            };

            let _ = core::writeln!(
                writer,
                "\r\nCalibrating 100mm (far) for {} sensor: Raw distance = {} mm",
                name,
                d_val
            );

            let (map_fs, flash_ptr) = match resolver.resolve_partition(Some(partition.as_str()))? {
                crate::ResolvedPartition::Map(fs, ptr) => (fs, ptr),
                _ => return Err("Requested partition is not a map filesystem"),
            };
            let flash_ref = unsafe { &mut *flash_ptr };
            let mut async_flash = BlockingAsyncFlash(flash_ref);

            let mut buf = [0u8; 128];
            let mut proximity_cal = platform::flash::read_calibration_direct_blocking::<
                _,
                <C::SensorCtrl as Calibration>::Store,
            >(
                &mut async_flash,
                map_fs.clone(),
                fs_buf_static,
                <C::SensorCtrl as Calibration>::CALIBRATION_FILE_NAME,
                &mut buf,
            )
            .unwrap_or_default();

            let dir = model::types::Direction::from(direction);
            proximity_cal.sensors[dir as usize].high = d_val;

            let mut write_buf = [0u8; 128];
            platform::flash::write_calibration_direct_blocking(
                &mut async_flash,
                map_fs,
                fs_buf_static,
                <C::SensorCtrl as Calibration>::CALIBRATION_FILE_NAME,
                &proximity_cal,
                &mut write_buf,
            )
            .map(|_| {
                let _ = sensor_ctrl.update_calibration(&proximity_cal);
                let _ = core::writeln!(writer, "Saved 100mm calibration for {} to flash.", name);
            })
            .map_err(|_| "Error saving calibration to flash")
        }
        PendingCommand::CalXTalk {
            direction,
            distance_or_partition,
        } => {
            let dir_str = match direction {
                SensorDirection::North => "north",
                SensorDirection::East => "east",
                SensorDirection::West => "west",
            };

            let name = match direction {
                SensorDirection::North => "North",
                SensorDirection::East => "East",
                SensorDirection::West => "West",
            };

            let cal_distance = distance_or_partition.parse::<u16>().unwrap_or(100);

            let sensor_ctrl = resolver.resolve_sensor(Some(dir_str))?;
            let mut d_raw = SensorReading::Invalid;
            let mut peak_rate_raw = 0u16;

            for _ in 0..10 {
                let lock = OnceLock::new();
                let lock_ptr = CliSignal::new(&lock);
                if sensor_ctrl
                    .send_command(SensorCommand::ReadXTalkRawWithSignal(lock_ptr))
                    .is_ok()
                {
                    if let Ok((reading, rate)) = *lock.wait().await {
                        d_raw = reading;
                        peak_rate_raw = rate;
                        if let SensorReading::Proximity(_) = reading {
                            break;
                        }
                    }
                }
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                embassy_time::block_for(embassy_time::Duration::from_millis(50));
            }

            let d_val = match d_raw {
                SensorReading::Proximity(d) => d,
                SensorReading::Invalid => return Err("Sensor disconnected or invalid reading"),
            };

            if d_val >= cal_distance {
                let _ = core::writeln!(
                    writer,
                    "Error: Measured distance ({} mm) must be less than calibration distance ({} mm) to calculate crosstalk.",
                    d_val,
                    cal_distance
                );
                return Err("Measured distance must be less than calibration distance");
            }

            // Calculate crosstalk rate in milli-MCPS:
            // R_xtalk_m = (PeakRate * 1000 * (CalDist - MeasuredDist)) / (128 * CalDist)
            let diff = cal_distance - d_val;
            let xtalk_m_mcps =
                ((peak_rate_raw as u32 * 1000 * diff as u32) / (128 * cal_distance as u32)) as u16;

            let _ = core::writeln!(
                writer,
                "\r\nCalibrating crosstalk for {} sensor: Raw distance = {} mm, Peak Rate = {}, Calculated Crosstalk = {} mMCPS",
                name,
                d_val,
                peak_rate_raw,
                xtalk_m_mcps
            );

            let (map_fs, flash_ptr) = match resolver
                .resolve_partition(Some(distance_or_partition.as_str()))
                .or_else(|_| resolver.resolve_partition(None))?
            {
                crate::ResolvedPartition::Map(fs, ptr) => (fs, ptr),
                _ => return Err("Requested partition is not a map filesystem"),
            };
            let flash_ref = unsafe { &mut *flash_ptr };
            let mut async_flash = BlockingAsyncFlash(flash_ref);

            let mut buf = [0u8; 128];
            let mut proximity_cal = platform::flash::read_calibration_direct_blocking::<
                _,
                <C::SensorCtrl as Calibration>::Store,
            >(
                &mut async_flash,
                map_fs.clone(),
                fs_buf_static,
                <C::SensorCtrl as Calibration>::CALIBRATION_FILE_NAME,
                &mut buf,
            )
            .unwrap_or_default();

            let dir = model::types::Direction::from(direction);
            proximity_cal.xtalk_m_mcps[dir as usize] = xtalk_m_mcps;

            let mut write_buf = [0u8; 128];
            platform::flash::write_calibration_direct_blocking(
                &mut async_flash,
                map_fs.clone(),
                fs_buf_static,
                <C::SensorCtrl as Calibration>::CALIBRATION_FILE_NAME,
                &proximity_cal,
                &mut write_buf,
            )
            .map(|_| {
                let _ = sensor_ctrl.update_calibration(&proximity_cal);
                let _ = core::writeln!(
                    writer,
                    "Saved crosstalk calibration for {} to flash ({} mMCPS).",
                    name,
                    xtalk_m_mcps
                );
            })
            .map_err(|_| "Error saving calibration to flash")
        }
    }
}

/// Standard config implementation for ProximityFeature.
pub struct ProximityFeatureConfig<MutexRaw: RawMutex + 'static, const S_CAP: usize = 3> {
    /// Sensor channel senders
    pub sensor_txs: heapless::Vec<crate::SensorSender<MutexRaw>, S_CAP>,
    /// Proximity gesture detector state
    pub gesture_detector: core::cell::RefCell<platform::gesture_detector::ProximityGestureDetector>,
    /// Proximity telemetry client
    pub telemetry_client:
        core::cell::RefCell<crate::telemetry_controller::SensorTelemetryClient<MutexRaw>>,
    /// Active proximity detection state
    pub proximity_active: core::cell::Cell<bool>,
    /// Proximity detection threshold
    pub wake_threshold_mm: u16,
    /// Last seen distances indexed by Direction (0 = North, 1 = East, 2 = West)
    pub distances: [core::cell::Cell<u16>; 3],
    /// Mapped action for DualLongPress gesture
    pub dual_long_press_action: crate::GestureAction,
}

impl<MutexRaw: RawMutex + 'static, const S_CAP: usize> ProximityFeatureConfig<MutexRaw, S_CAP> {
    /// Creates a new `ProximityFeatureConfig` with the given list of sensor senders (up to S_CAP).
    pub fn new(
        sensor_senders: &[crate::SensorSender<MutexRaw>],
        press_threshold_mm: u16,
        near_threshold_mm: u16,
        wake_threshold_mm: u16,
        dual_long_press_action: crate::GestureAction,
        telemetry_tx: Option<crate::TelemetrySender<MutexRaw>>,
    ) -> Self {
        let mut sensor_txs = heapless::Vec::new();
        for sender in sensor_senders {
            let _ = sensor_txs.push(*sender);
        }
        Self {
            sensor_txs,
            gesture_detector: core::cell::RefCell::new(
                platform::gesture_detector::ProximityGestureDetector::new(
                    press_threshold_mm,
                    near_threshold_mm,
                    wake_threshold_mm,
                ),
            ),
            telemetry_client: core::cell::RefCell::new(
                crate::telemetry_controller::SensorTelemetryClient::new(
                    telemetry_tx,
                    wake_threshold_mm,
                ),
            ),
            proximity_active: core::cell::Cell::new(false),
            wake_threshold_mm,
            distances: [
                core::cell::Cell::new(1000),
                core::cell::Cell::new(1000),
                core::cell::Cell::new(1000),
            ],
            dual_long_press_action,
        }
    }
}

impl<MutexRaw: RawMutex + 'static, const S_CAP: usize, const N: usize>
    crate::SystemFeature<MutexRaw, N> for ProximityFeatureConfig<MutexRaw, S_CAP>
{
    fn on_proximity_update(
        &self,
        direction: model::types::Direction,
        reading: SensorReading,
        status: model::types::SystemStatus,
    ) -> (Option<model::types::Gesture>, crate::ProximityAction) {
        use model::telemetry::TelemetryClient as _;
        use platform::gesture_detector::GestureDetector as _;

        let distance_mm = match reading {
            SensorReading::Proximity(d) => d,
            SensorReading::Invalid => u16::MAX,
        };

        self.telemetry_client
            .borrow_mut()
            .report((direction, reading));

        let idx = match direction {
            model::types::Direction::North => 0,
            model::types::Direction::East => 1,
            model::types::Direction::West => 2,
        };

        let prev_state = self.gesture_detector.borrow().trackers[idx].state;

        let now_us = embassy_time::Instant::now().as_micros();
        let detector_gesture = self
            .gesture_detector
            .borrow_mut()
            .update((direction, distance_mm), now_us);

        let new_state = self.gesture_detector.borrow().trackers[idx].state;

        if prev_state != new_state {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            {
                let dir_str = match direction {
                    model::types::Direction::North => "North",
                    model::types::Direction::East => "East",
                    model::types::Direction::West => "West",
                };
                use platform::gesture_detector::ProximityState;
                match new_state {
                    ProximityState::OutOfRange => {
                        defmt::info!(
                            "GestureDetector {} sensor: OUT OF RANGE ({} mm)",
                            dir_str,
                            distance_mm
                        );
                    }
                    ProximityState::InRange => {
                        defmt::info!(
                            "GestureDetector {} sensor: IN RANGE ({} mm)",
                            dir_str,
                            distance_mm
                        );
                    }
                    ProximityState::Near => {
                        defmt::info!(
                            "GestureDetector {} sensor: NEAR ({} mm)",
                            dir_str,
                            distance_mm
                        );
                    }
                    ProximityState::Down => {
                        defmt::info!(
                            "GestureDetector {} sensor: DOWN ({} mm)",
                            dir_str,
                            distance_mm
                        );
                    }
                }
            }
        }

        let gesture = if let SensorReading::Invalid = reading {
            None
        } else {
            detector_gesture
        };

        self.distances[idx].set(distance_mm);

        let in_range = self
            .distances
            .iter()
            .any(|d| d.get() < self.wake_threshold_mm);

        let mut action = crate::ProximityAction::None;
        if in_range != self.proximity_active.get() {
            self.proximity_active.set(in_range);
            if in_range {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::debug!("SensorController: Object in range (proximity active)");
                if status == model::types::SystemStatus::Active {
                    action = crate::ProximityAction::AcquireWakeLock;
                } else if status == model::types::SystemStatus::Sleep {
                    action = crate::ProximityAction::WakeSystem;
                }
            } else {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::debug!("SensorController: Object went out of range (proximity inactive)");
                if status == model::types::SystemStatus::Active {
                    action = crate::ProximityAction::ReleaseWakeLock;
                }
            }
        }

        (gesture, action)
    }

    fn map_gesture(
        &self,
        gesture: model::types::Gesture,
        _status: model::types::SystemStatus,
    ) -> crate::GestureAction {
        #[allow(unreachable_patterns)]
        match gesture {
            model::types::Gesture::DualLongPress => self.dual_long_press_action,
            _ => crate::GestureAction::None,
        }
    }

    fn on_state_changed(
        &self,
        _from: model::types::SystemStatus,
        to: model::types::SystemStatus,
        _support: crate::DeviceSupport,
        _battery_status: Option<crate::BatteryStatus>,
        _thermal_critical: bool,
    ) {
        use crate::Periodic;
        match to {
            model::types::SystemStatus::Active => {
                self.set_interval(PeriodicInterval::UpdateMs(200));
            }
            model::types::SystemStatus::Sleep => {
                self.set_interval(PeriodicInterval::UpdateMs(1000));
            }
            model::types::SystemStatus::PowerDown => {
                self.set_interval(PeriodicInterval::UpdateMs(1000));
            }
        }
    }
}

impl<MutexRaw: RawMutex + 'static, const S_CAP: usize> crate::Periodic
    for ProximityFeatureConfig<MutexRaw, S_CAP>
{
    fn set_interval(&self, interval: PeriodicInterval) {
        self.telemetry_client
            .borrow_mut()
            .report_interval(model::types::Device::Sensors, interval);
        for sensor_tx in &self.sensor_txs {
            if sensor_tx
                .try_send(SensorCommand::SetInterval(interval))
                .is_err()
            {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::error!(
                    "ProximityFeatureConfig: Failed to configure sensor periodic interval!"
                );
            }
        }
    }
}
