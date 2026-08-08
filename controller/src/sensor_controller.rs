//! Sensor controller for the Time-of-Flight (ToF) proximity sensors.

#![deny(missing_docs)]

use crate::tracing::{self, controller_context};
use crate::types::{SensorDirection, SensorMetadata};
use crate::BlockingProximityReader;
use crate::Sender;
use core::fmt::Write as _;
use embassy_sync::blocking_mutex::raw::RawMutex;
use model::calibration::{Calibration, CalibrationType, Vl53l0xCalibration};
use model::interfaces::ProximitySensor;
use model::types::{Direction, PeriodicInterval, PeripheralError};
use peripherals::ToPeripheralError;
use platform::{select_branch_with_timeout, subcommand_enum, BlockingAsyncFlash};

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

/// Wrapper for the synchronous/blocking CLI signal pointer to implement Send and Sync.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CliSignalPtr(pub *const ::platform::OnceLock<Result<u16, PeripheralError>>);

unsafe impl Send for CliSignalPtr {}
unsafe impl Sync for CliSignalPtr {}

impl core::fmt::Debug for CliSignalPtr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CliSignalPtr({:p})", self.0)
    }
}

/// Wrapper for the diagnostic CLI signal pointer.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DiagnosticSignalPtr(
    pub *const ::platform::OnceLock<Result<(u16, u8, u16), PeripheralError>>,
);

unsafe impl Send for DiagnosticSignalPtr {}
unsafe impl Sync for DiagnosticSignalPtr {}

impl core::fmt::Debug for DiagnosticSignalPtr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DiagnosticSignalPtr({:p})", self.0)
    }
}

/// Type alias for the sensor command sender.
pub type SensorSender<M> = embassy_sync::channel::Sender<'static, M, SensorCommand, 4>;

/// One-way commands sent to the Sensor Controller.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum SensorCommand {
    /// Force proximity sensor check and print telemetry logs
    ReadSensors,
    /// Force proximity sensor check and signal completion via OnceLock
    ReadSensorsWithSignal(CliSignalPtr),
    /// Force raw proximity sensor check and signal completion via OnceLock
    ReadRawSensorsWithSignal(CliSignalPtr),
    /// Force diagnostic proximity sensor check and signal completion via OnceLock
    ReadDiagnosticsWithSignal(DiagnosticSignalPtr),
    /// Set periodic automatic reading interval
    SetInterval(PeriodicInterval),
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
    fn read_data(sensor: &mut S, ctx: &Self::Context) -> Result<Self::Data, Self::Error>;
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
    type Data = u16;
    type Error = S::Error;

    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    fn read_data(sensor: &mut S, _ctx: &Self::Context) -> Result<Self::Data, Self::Error> {
        sensor.read_distance_mm()
    }
}

/// A trait to convert proximity sensor reading updates to a system command.
pub trait FromProximityUpdate {
    /// Constructs a command from sensor metadata and distance in mm.
    fn from_proximity_update(metadata: SensorMetadata, distance_mm: u16) -> Self;
}

impl FromProximityUpdate for () {
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    fn from_proximity_update(_metadata: SensorMetadata, _distance_mm: u16) -> Self {}
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
        Data: Copy + Into<u16>,
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
                defmt::warn!(
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
            latest_data: 1000,
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
            latest_data: 1000,
            context: ProximityReaderContext { wake_threshold_mm },
            command_tx: None,
        }
    }
}

impl<
        'a,
        S: ProximitySensor,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin: DataReadyPin,
        Cmd: FromProximityUpdate + Clone + core::fmt::Debug,
        Reader: SensorReader<S>,
        const SYS_CAP: usize,
    > SensorController<'a, S, M, Pin, Cmd, Reader, SYS_CAP>
where
    Reader::Data: Copy + Into<u16>,
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
    #[tracing::instrument(core1 = "core1", name = "sensor_controller::update", level = "info")]
    pub fn update(&mut self) -> Result<Reader::Data, Reader::Error> {
        let data = Reader::read_data(self.state_manager.sensor_mut(), &self.context)?;

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
    pub fn handle_command(&mut self, cmd: SensorCommand) {
        match cmd {
            SensorCommand::ReadSensors => {
                let _ = self.update();
            }
            SensorCommand::ReadSensorsWithSignal(signal_ptr) => {
                let res = Reader::read_data(self.state_manager.sensor_mut(), &self.context)
                    .map(|d| {
                        self.latest_data = d;
                        d.into()
                    })
                    .map_err(|_| PeripheralError::DeviceNotAvailable);
                unsafe {
                    let lock = &*signal_ptr.0;
                    let _ = lock.set(res);
                }
            }
            SensorCommand::ReadRawSensorsWithSignal(signal_ptr) => {
                let res = self
                    .state_manager
                    .sensor_mut()
                    .read_distance_raw()
                    .map_err(|_| PeripheralError::DeviceNotAvailable);
                unsafe {
                    let lock = &*signal_ptr.0;
                    let _ = lock.set(res);
                }
            }
            SensorCommand::ReadDiagnosticsWithSignal(signal_ptr) => {
                let res = self
                    .state_manager
                    .sensor_mut()
                    .read_diagnostics()
                    .map_err(|_| PeripheralError::DeviceNotAvailable);
                unsafe {
                    let lock = &*signal_ptr.0;
                    let _ = lock.set(res);
                }
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
        loop {
            let timeout_dur = match self.state_manager.periodic_interval() {
                PeriodicInterval::None => crate::OVERFLOW_SAFE_MAX_DURATION,
                PeriodicInterval::UpdateMs(ms) => embassy_time::Duration::from_millis(ms as u64),
            };
            let res = select_branch_with_timeout!(
                timeout_dur,
                command_rx.receive() => |cmd| {
                    self.handle_command(cmd);
                    Some(())
                },
                self.wait_for_data_ready() => || {
                    None
                },
            );

            if res.is_none() && self.update().is_err() {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::warn!("SensorController: Periodic read failed; disabling periodic updates.");
                self.state_manager
                    .set_periodic_interval(PeriodicInterval::None);
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
            latest_data: 1000,
            context: ProximityReaderContext { wake_threshold_mm },
            command_tx: None,
        }
    }

    /// Gets the current proximity telemetry reading.
    pub fn telemetry(&self) -> model::types::ProximityTelemetry {
        let dir = self.direction();
        if self.latest_data < self.context.wake_threshold_mm {
            model::types::ProximityTelemetry::InRange(dir, self.latest_data)
        } else {
            model::types::ProximityTelemetry::OutRange(dir, self.latest_data)
        }
    }

    /// Gets the latest read proximity telemetry distance.
    pub fn latest_distance(&self) -> u16 {
        self.latest_data
    }
}

impl<
        'a,
        S: ProximitySensor,
        M: embassy_sync::blocking_mutex::raw::RawMutex + 'static,
        Pin,
        Cmd,
    > crate::BlockingProximityReader for SensorController<'a, S, M, Pin, Cmd, ProximityReader>
where
    S::Error: ToPeripheralError,
{
    fn read_distance_blocking(&mut self) -> Result<u16, PeripheralError> {
        let lock = ::platform::OnceLock::new();
        let lock_ptr = CliSignalPtr(&lock as *const _);
        self.send_command(SensorCommand::ReadSensorsWithSignal(lock_ptr))?;
        *lock.wait()
    }

    fn read_raw_distance_blocking(&mut self) -> Result<u16, PeripheralError> {
        let lock = ::platform::OnceLock::new();
        let lock_ptr = CliSignalPtr(&lock as *const _);
        self.send_command(SensorCommand::ReadRawSensorsWithSignal(lock_ptr))?;
        *lock.wait()
    }

    fn read_diagnostics_blocking(&mut self) -> Result<(u16, u8, u16), PeripheralError> {
        let lock = ::platform::OnceLock::new();
        let lock_ptr = DiagnosticSignalPtr(&lock as *const _);
        self.send_command(SensorCommand::ReadDiagnosticsWithSignal(lock_ptr))?;
        *lock.wait()
    }

    fn latest_distance(&self) -> u16 {
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
}

impl<
        'a,
        S: ProximitySensor + Calibration,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin,
        Cmd,
    > Calibration for SensorController<'a, S, M, Pin, Cmd, ProximityReader>
{
    fn set_calibration(&mut self, calibration: CalibrationType) {
        self.sensor_mut().set_calibration(calibration);
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
    }
    "Invalid sensor subcommand. Expected: status, cal_near, cal_far"
}

/// Processes sensor-specific CLI subcommands by validating and delegating.
pub fn handle_sensor_cli<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    C: crate::ShellConfig,
>(
    resolver: &impl crate::ShellDeviceResolver<C>,
    subcommand: Option<SensorSubcommand>,
    arg1: Option<&str>,
    partition_name: Option<&str>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let mut fs_buf = resolver.lock_fs_buffer()?;
    let fs_buf_static = unsafe { fs_buf.as_static_mut() };

    let cmd = subcommand.ok_or("Missing sensor subcommand")?;

    match cmd {
        SensorSubcommand::Status => {
            let _ = core::writeln!(writer, "\r\nDirect proximity readings:");
            for named in resolver.sensors() {
                let sensor = unsafe { &mut *named.device };
                let lock = ::platform::OnceLock::new();
                let lock_ptr = CliSignalPtr(&lock as *const _);
                let dist = if sensor
                    .send_command(SensorCommand::ReadSensorsWithSignal(lock_ptr))
                    .is_ok()
                {
                    *lock.wait()
                } else {
                    Err(PeripheralError::DeviceNotAvailable)
                };
                match dist {
                    Ok(d) => {
                        let _ = core::writeln!(writer, "  {} = {} mm", named.name, d);
                    }
                    Err(_) => {
                        let _ = core::writeln!(writer, "  {} = FAILED to read", named.name);
                    }
                }
            }
            Ok(())
        }
        SensorSubcommand::CalNear => {
            let dir_str = arg1.ok_or("Missing direction parameter")?;
            let direction = match dir_str {
                "north" => SensorDirection::North,
                "east" => SensorDirection::East,
                "west" => SensorDirection::West,
                _ => return Err("Invalid direction. Expected: north, east, west"),
            };

            let name = match direction {
                SensorDirection::North => "North",
                SensorDirection::East => "East",
                SensorDirection::West => "West",
            };

            let sensor_ctrl = resolver.resolve_sensor(Some(dir_str))?;
            let mut d_raw = 8190;
            for _ in 0..10 {
                if let Ok(d) = sensor_ctrl.read_raw_distance_blocking() {
                    if d < 900 {
                        d_raw = d;
                        break;
                    }
                }
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                embassy_time::block_for(embassy_time::Duration::from_millis(50));
            }

            if d_raw >= 900 {
                return Err("Sensor disconnected or target out of range");
            }

            let _ = core::writeln!(
                writer,
                "\r\nCalibrating cover (near) for {} sensor: Raw distance = {} mm",
                name,
                d_raw
            );

            let (map_fs, flash_ptr) = match resolver.resolve_partition(partition_name)? {
                crate::ResolvedPartition::Map(fs, ptr) => (fs, ptr),
                _ => return Err("Requested partition is not a map filesystem"),
            };
            let flash_ref = unsafe { &mut *flash_ptr };
            let mut async_flash = BlockingAsyncFlash(flash_ref);

            let mut buf = [0u8; 128];
            let mut proximity_cal = embassy_futures::block_on(platform::flash::read_file_direct(
                &mut async_flash,
                map_fs.clone(),
                fs_buf_static,
                "vl53l0x_cal.cbor",
                &mut buf,
            ))
            .ok()
            .flatten()
            .and_then(|len| minicbor::decode::<Vl53l0xCalibration>(&buf[..len]).ok())
            .unwrap_or_default();

            let dir = model::types::Direction::from(direction);
            proximity_cal[dir].low = d_raw;

            let mut write_buf = [0u8; 128];
            let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
            let mut encoder = minicbor::Encoder::new(cursor);
            encoder.encode(proximity_cal).unwrap();
            let len = encoder.into_writer().position();

            embassy_futures::block_on(platform::flash::write_file_direct(
                &mut async_flash,
                map_fs.clone(),
                fs_buf_static,
                "vl53l0x_cal.cbor",
                &write_buf[..len],
            ))
            .map(|_| {
                let _ = core::writeln!(writer, "Saved cover calibration for {} to flash.", name);
            })
            .map_err(|_| "Error saving calibration to flash")
        }
        SensorSubcommand::CalFar => {
            let dir_str = arg1.ok_or("Missing direction parameter")?;
            let direction = match dir_str {
                "north" => SensorDirection::North,
                "east" => SensorDirection::East,
                "west" => SensorDirection::West,
                _ => return Err("Invalid direction. Expected: north, east, west"),
            };

            let name = match direction {
                SensorDirection::North => "North",
                SensorDirection::East => "East",
                SensorDirection::West => "West",
            };

            let sensor_ctrl = resolver.resolve_sensor(Some(dir_str))?;
            let mut d_raw = 8190;
            for _ in 0..10 {
                if let Ok(d) = sensor_ctrl.read_raw_distance_blocking() {
                    if d < 900 {
                        d_raw = d;
                        break;
                    }
                }
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                embassy_time::block_for(embassy_time::Duration::from_millis(50));
            }

            if d_raw >= 900 {
                return Err("Sensor disconnected or target out of range");
            }

            let _ = core::writeln!(
                writer,
                "\r\nCalibrating 100mm (far) for {} sensor: Raw distance = {} mm",
                name,
                d_raw
            );

            let (map_fs, flash_ptr) = match resolver.resolve_partition(partition_name)? {
                crate::ResolvedPartition::Map(fs, ptr) => (fs, ptr),
                _ => return Err("Requested partition is not a map filesystem"),
            };
            let flash_ref = unsafe { &mut *flash_ptr };
            let mut async_flash = BlockingAsyncFlash(flash_ref);

            let mut buf = [0u8; 128];
            let mut proximity_cal = embassy_futures::block_on(platform::flash::read_file_direct(
                &mut async_flash,
                map_fs.clone(),
                fs_buf_static,
                "vl53l0x_cal.cbor",
                &mut buf,
            ))
            .ok()
            .flatten()
            .and_then(|len| minicbor::decode::<Vl53l0xCalibration>(&buf[..len]).ok())
            .unwrap_or_default();

            let dir = model::types::Direction::from(direction);
            proximity_cal[dir].high = d_raw;

            let mut write_buf = [0u8; 128];
            let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
            let mut encoder = minicbor::Encoder::new(cursor);
            encoder.encode(proximity_cal).unwrap();
            let len = encoder.into_writer().position();

            embassy_futures::block_on(platform::flash::write_file_direct(
                &mut async_flash,
                map_fs,
                fs_buf_static,
                "vl53l0x_cal.cbor",
                &write_buf[..len],
            ))
            .map(|_| {
                let _ = core::writeln!(writer, "Saved 100mm calibration for {} to flash.", name);
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
        core::cell::RefCell<crate::telemetry_controller::ProximityTelemetryClient<MutexRaw>>,
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
                platform::gesture_detector::ProximityGestureDetector::new(press_threshold_mm),
            ),
            telemetry_client: core::cell::RefCell::new(
                crate::telemetry_controller::ProximityTelemetryClient::new(
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
        distance_mm: u16,
        status: model::types::SystemStatus,
    ) -> (Option<model::types::Gesture>, crate::ProximityAction) {
        use model::telemetry::TelemetryClient as _;
        use platform::gesture_detector::GestureDetector as _;
        self.telemetry_client
            .borrow_mut()
            .report((direction, distance_mm));

        let now_us = embassy_time::Instant::now().as_micros();
        let gesture = self
            .gesture_detector
            .borrow_mut()
            .update((direction, distance_mm), now_us);

        // Register distance locally in the feature using direction map index
        let idx = match direction {
            model::types::Direction::North => 0,
            model::types::Direction::East => 1,
            model::types::Direction::West => 2,
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
                if status == model::types::SystemStatus::Active {
                    action = crate::ProximityAction::AcquireWakeLock;
                } else if status == model::types::SystemStatus::Sleep {
                    action = crate::ProximityAction::WakeSystem;
                }
            } else if status == model::types::SystemStatus::Active {
                action = crate::ProximityAction::ReleaseWakeLock;
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
                self.set_interval(PeriodicInterval::None);
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
