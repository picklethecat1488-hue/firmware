//! Sensor controller for the Time-of-Flight (ToF) proximity sensors.

#![deny(missing_docs)]

use crate::tracing;
use crate::types::{SensorDirection, SensorMetadata};
use crate::BlockingProximityReader;
use crate::Sender;
use core::fmt::Write as _;
use embassy_sync::blocking_mutex::raw::RawMutex;
use firmware_lib::{select_branch_with_timeout, subcommand_enum, BlockingAsyncFlash};
use model::interfaces::ProximitySensor;
use model::types::{Direction, PeripheralError};
use peripherals::ToPeripheralError;

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

/// One-way commands sent to the Sensor Controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorCommand {
    /// Force proximity sensor check and print telemetry logs
    ReadSensors,
    /// Enable periodic automatic readings
    EnablePeriodic,
    /// Disable periodic automatic readings (runs only via manual commands)
    DisablePeriodic,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        link_section = ".data.ram_func"
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
        link_section = ".data.ram_func"
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
> {
    metadata: SensorMetadata,
    sensor: S,
    periodic_enabled: bool,
    upstream_tx: Option<Sender<'a, M, Cmd, 4>>,
    interrupt_pin: Option<Pin>,
    _marker: core::marker::PhantomData<Data>,
}

impl<'a, S, Data, M: embassy_sync::blocking_mutex::raw::RawMutex, Pin, Cmd>
    SensorStateManager<'a, S, Data, M, Pin, Cmd>
{
    /// Creates a new SensorStateManager.
    pub const fn new(
        metadata: SensorMetadata,
        sensor: S,
        upstream_tx: Option<Sender<'a, M, Cmd, 4>>,
        interrupt_pin: Option<Pin>,
    ) -> Self {
        Self {
            metadata,
            sensor,
            periodic_enabled: true,
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
        self.periodic_enabled
    }

    /// Sets whether periodic monitoring is enabled.
    pub fn set_periodic_enabled(&mut self, enabled: bool) {
        self.periodic_enabled = enabled;
    }
}

impl<
        'a,
        S,
        Data: Copy + Into<u16>,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin,
        Cmd: FromProximityUpdate + Clone + core::fmt::Debug,
    > SensorStateManager<'a, S, Data, M, Pin, Cmd>
{
    /// Sends a command upstream if configured.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.ram_func"
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

impl<'a, S, Data, M: embassy_sync::blocking_mutex::raw::RawMutex, Pin: DataReadyPin, Cmd>
    SensorStateManager<'a, S, Data, M, Pin, Cmd>
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
pub struct SensorController<
    'a,
    S,
    M: embassy_sync::blocking_mutex::raw::RawMutex = embassy_sync::blocking_mutex::raw::NoopRawMutex,
    Pin = DummyDataReadyPin,
    Cmd = (),
    Reader: SensorReader<S> = ProximityReader,
> {
    state_manager: SensorStateManager<'a, S, Reader::Data, M, Pin, Cmd>,
    latest_data: Reader::Data,
    context: Reader::Context,
}

impl<'a, S, M: embassy_sync::blocking_mutex::raw::RawMutex, Pin, Cmd, Reader: SensorReader<S>>
    core::ops::Deref for SensorController<'a, S, M, Pin, Cmd, Reader>
{
    type Target = SensorStateManager<'a, S, Reader::Data, M, Pin, Cmd>;

    fn deref(&self) -> &Self::Target {
        &self.state_manager
    }
}

impl<'a, S, M: embassy_sync::blocking_mutex::raw::RawMutex, Pin, Cmd, Reader: SensorReader<S>>
    core::ops::DerefMut for SensorController<'a, S, M, Pin, Cmd, Reader>
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
    >
{
    /// Creates a new SensorController managing a single proximity sensor.
    pub const fn new(metadata: SensorMetadata, sensor: S, wake_threshold_mm: u16) -> Self {
        Self {
            state_manager: SensorStateManager::new(metadata, sensor, None, None),
            latest_data: 1000,
            context: ProximityReaderContext { wake_threshold_mm },
        }
    }
}

impl<
        'a,
        S: ProximitySensor,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Cmd: FromProximityUpdate + Clone + core::fmt::Debug,
    > SensorController<'a, S, M, DummyDataReadyPin, Cmd, ProximityReader>
{
    /// Creates a new SensorController with upstream system notification.
    pub fn new_with_fusion(
        metadata: SensorMetadata,
        sensor: S,
        upstream_tx: Sender<'a, M, Cmd, 4>,
        wake_threshold_mm: u16,
    ) -> Self {
        Self {
            state_manager: SensorStateManager::new(metadata, sensor, Some(upstream_tx), None),
            latest_data: 1000,
            context: ProximityReaderContext { wake_threshold_mm },
        }
    }
}

impl<
        'a,
        S,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin: DataReadyPin,
        Cmd: FromProximityUpdate + Clone + core::fmt::Debug,
        Reader: SensorReader<S>,
    > SensorController<'a, S, M, Pin, Cmd, Reader>
where
    Reader::Data: Copy + Into<u16>,
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
        }
    }

    /// Creates a generic SensorController with upstream system notification.
    pub fn new_generic_with_fusion(
        metadata: SensorMetadata,
        sensor: S,
        latest_data: Reader::Data,
        upstream_tx: Sender<'a, M, Cmd, 4>,
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
        }
    }

    /// Gets a mutable reference to the underlying sensor.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.ram_func"
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
        link_section = ".data.ram_func"
    )]
    pub fn is_periodic_enabled(&self) -> bool {
        self.state_manager.is_periodic_enabled()
    }

    /// Ticks the sensor control loop, updating proximity distance.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.ram_func"
    )]
    #[tracing::instrument(name = "sensor_controller::update", level = "info")]
    pub fn update(&mut self) -> Result<Reader::Data, Reader::Error> {
        let data = Reader::read_data(self.state_manager.sensor_mut(), &self.context)?;

        self.latest_data = data;

        self.notify_upstream(data);

        Ok(data)
    }

    /// Handles a SensorCommand.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.ram_func"
    )]
    #[tracing::instrument(name = "sensor_controller::handle_command", level = "info", skip(cmd))]
    pub fn handle_command(&mut self, cmd: SensorCommand) {
        match cmd {
            SensorCommand::ReadSensors => {
                let _ = self.update();
            }
            SensorCommand::EnablePeriodic => {
                self.set_periodic_enabled(true);
            }
            SensorCommand::DisablePeriodic => {
                self.set_periodic_enabled(false);
            }
        }
    }

    /// Runs the controller's main run loop, executing periodic telemetry updates.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.ram_func"
    )]
    pub async fn run(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, M, SensorCommand, 4>,
    ) -> ! {
        loop {
            let timeout_dur = if self.is_periodic_enabled() {
                embassy_time::Duration::from_millis(1000)
            } else {
                embassy_time::Duration::MAX
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

            if res.is_none() {
                let _ = self.update();
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
    > SensorController<'a, S, M, Pin, Cmd, ProximityReader>
{
    /// Creates a new SensorController with upstream system notification and interrupt pin support.
    pub fn new_with_fusion_and_interrupt(
        metadata: SensorMetadata,
        sensor: S,
        upstream_tx: Sender<'a, M, Cmd, 4>,
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

impl<'a, S: ProximitySensor, M: embassy_sync::blocking_mutex::raw::RawMutex, Pin, Cmd>
    crate::BlockingProximityReader for SensorController<'a, S, M, Pin, Cmd, ProximityReader>
where
    S::Error: ToPeripheralError,
{
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.ram_func"
    )]
    fn read_distance_blocking(&mut self) -> Result<u16, PeripheralError> {
        self.sensor_mut()
            .read_distance_mm()
            .map_err(|e| e.to_peripheral_error())
    }
}

impl<
        'a,
        S: ProximitySensor + model::calibration::Calibration,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin,
        Cmd,
    > model::calibration::Calibration for SensorController<'a, S, M, Pin, Cmd, ProximityReader>
{
    fn set_calibration(&mut self, calibration: model::calibration::CalibrationType) {
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
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let mut fs_buf = resolver.lock_fs_buffer()?;
    let fs_buf_static = unsafe { fs_buf.as_static_mut() };

    let cmd = subcommand.ok_or("Missing sensor subcommand")?;

    match cmd {
        SensorSubcommand::Status => {
            let dn = resolver
                .resolve_sensor(Some("north"))?
                .read_distance_blocking()
                .map_err(|_| "Proximity sensor north failed to read")?;
            let de = resolver
                .resolve_sensor(Some("east"))?
                .read_distance_blocking()
                .map_err(|_| "Proximity sensor east failed to read")?;
            let dw = resolver
                .resolve_sensor(Some("west"))?
                .read_distance_blocking()
                .map_err(|_| "Proximity sensor west failed to read")?;
            let _ = core::writeln!(
                writer,
                "\r\nDirect proximity readings: North = {} mm, East = {} mm, West = {} mm",
                dn,
                de,
                dw
            );
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

            let i2c = resolver.resolve_i2c(None)?;
            let (addr, name) = match direction {
                SensorDirection::North => (0x30, "North"),
                SensorDirection::East => (0x31, "East"),
                SensorDirection::West => (0x32, "West"),
            };

            let d_raw = {
                let mut sensor = peripherals::vl53l0x::Vl53l0x::new(i2c, addr);
                sensor.read_distance_mm().unwrap_or(1000)
            };

            if d_raw >= 900 {
                return Err("Sensor disconnected or target out of range");
            }

            let _ = core::writeln!(
                writer,
                "\r\nCalibrating cover (near) for {} sensor: Raw distance = {} mm",
                name,
                d_raw
            );

            let partition = resolver.resolve_partition(None)?;
            let flash_ref = unsafe { &mut *partition.flash_ptr };
            let async_flash = BlockingAsyncFlash(flash_ref);
            let mut fs = crate::filesystem_controller::FilesystemController::new(
                async_flash,
                partition.start_address..partition.end_address,
                fs_buf_static,
            );

            let mut buf = [0u8; 128];
            let mut proximity_cal =
                embassy_futures::block_on(fs.read_file("vl53l0x_cal.cbor", &mut buf))
                    .ok()
                    .flatten()
                    .and_then(|bytes| {
                        minicbor::decode::<model::calibration::Vl53l0xCalibration>(bytes).ok()
                    })
                    .unwrap_or_default();

            let dir = model::types::Direction::from(direction);
            proximity_cal[dir].low = d_raw;

            let mut write_buf = [0u8; 128];
            let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
            let mut encoder = minicbor::Encoder::new(cursor);
            encoder.encode(proximity_cal).unwrap();
            let len = encoder.into_writer().position();

            embassy_futures::block_on(fs.write_file("vl53l0x_cal.cbor", &write_buf[..len]))
                .map(|_| {
                    let _ =
                        core::writeln!(writer, "Saved cover calibration for {} to flash.", name);
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

            let i2c = resolver.resolve_i2c(None)?;
            let (addr, name) = match direction {
                SensorDirection::North => (0x30, "North"),
                SensorDirection::East => (0x31, "East"),
                SensorDirection::West => (0x32, "West"),
            };

            let d_raw = {
                let mut sensor = peripherals::vl53l0x::Vl53l0x::new(i2c, addr);
                sensor.read_distance_mm().unwrap_or(1000)
            };

            if d_raw >= 900 {
                return Err("Sensor disconnected or target out of range");
            }

            let _ = core::writeln!(
                writer,
                "\r\nCalibrating 100mm (far) for {} sensor: Raw distance = {} mm",
                name,
                d_raw
            );

            let partition = resolver.resolve_partition(None)?;
            let flash_ref = unsafe { &mut *partition.flash_ptr };
            let async_flash = BlockingAsyncFlash(flash_ref);
            let mut fs = crate::filesystem_controller::FilesystemController::new(
                async_flash,
                partition.start_address..partition.end_address,
                fs_buf_static,
            );

            let mut buf = [0u8; 128];
            let mut proximity_cal =
                embassy_futures::block_on(fs.read_file("vl53l0x_cal.cbor", &mut buf))
                    .ok()
                    .flatten()
                    .and_then(|bytes| {
                        minicbor::decode::<model::calibration::Vl53l0xCalibration>(bytes).ok()
                    })
                    .unwrap_or_default();

            let dir = model::types::Direction::from(direction);
            proximity_cal[dir].high = d_raw;

            let mut write_buf = [0u8; 128];
            let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
            let mut encoder = minicbor::Encoder::new(cursor);
            encoder.encode(proximity_cal).unwrap();
            let len = encoder.into_writer().position();

            embassy_futures::block_on(fs.write_file("vl53l0x_cal.cbor", &write_buf[..len]))
                .map(|_| {
                    let _ =
                        core::writeln!(writer, "Saved 100mm calibration for {} to flash.", name);
                })
                .map_err(|_| "Error saving calibration to flash")
        }
    }
}

/// Standard config implementation for ProximityFeature.
pub struct ProximityFeatureConfig<
    MutexRaw: RawMutex + 'static,
    const N: usize,
    const S_CAP: usize = 3,
    const T_CAP: usize = { crate::telemetry_controller::CHANNEL_CAPACITY },
> {
    /// Sensor channel senders
    pub sensor_txs: heapless::Vec<crate::SensorSender<MutexRaw, N>, S_CAP>,
    /// Proximity gesture detector state
    pub gesture_detector:
        core::cell::RefCell<firmware_lib::gesture_detector::ProximityGestureDetector>,
    /// Proximity telemetry client
    pub telemetry_client:
        core::cell::RefCell<crate::telemetry_controller::ProximityTelemetryClient<MutexRaw, T_CAP>>,
    /// Active proximity detection state
    pub proximity_active: core::cell::Cell<bool>,
    /// Proximity detection threshold
    pub wake_threshold_mm: u16,
    /// Last seen distances indexed by Direction (0 = North, 1 = East, 2 = West)
    pub distances: [core::cell::Cell<u16>; 3],
    /// Mapped action for DualLongPress gesture
    pub dual_long_press_action: crate::GestureAction,
}

impl<MutexRaw: RawMutex + 'static, const N: usize, const S_CAP: usize, const T_CAP: usize>
    ProximityFeatureConfig<MutexRaw, N, S_CAP, T_CAP>
{
    /// Creates a new `ProximityFeatureConfig` with the given list of sensor senders (up to S_CAP).
    pub fn new(
        sensor_senders: &[crate::SensorSender<MutexRaw, N>],
        press_threshold_mm: u16,
        wake_threshold_mm: u16,
        dual_long_press_action: crate::GestureAction,
        telemetry_tx: Option<crate::TelemetrySender<MutexRaw, T_CAP>>,
    ) -> Self {
        let mut sensor_txs = heapless::Vec::new();
        for sender in sensor_senders {
            let _ = sensor_txs.push(*sender);
        }
        Self {
            sensor_txs,
            gesture_detector: core::cell::RefCell::new(
                firmware_lib::gesture_detector::ProximityGestureDetector::new(press_threshold_mm),
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

impl<MutexRaw: RawMutex + 'static, const N: usize, const S_CAP: usize, const T_CAP: usize>
    crate::SystemFeature<MutexRaw, N> for ProximityFeatureConfig<MutexRaw, N, S_CAP, T_CAP>
{
    fn on_proximity_update(
        &self,
        direction: model::types::Direction,
        distance_mm: u16,
        status: model::types::SystemStatus,
    ) -> (Option<model::types::Gesture>, crate::ProximityAction) {
        use firmware_lib::gesture_detector::GestureDetector as _;
        use model::telemetry::TelemetryClient as _;
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

    fn on_tick(
        &self,
        _elapsed_ms: u32,
        _crossed_tick: bool,
        _status: model::types::SystemStatus,
        support: crate::DeviceSupport,
        _wake_locks: u32,
    ) {
        if support.proximity {
            for sensor_tx in &self.sensor_txs {
                let _ = sensor_tx.try_send(crate::SensorCommand::ReadSensors);
            }
        }
    }
}
