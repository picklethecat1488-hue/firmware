//! Sensor controller for the Time-of-Flight (ToF) proximity sensors.

#![deny(missing_docs)]

use core::fmt::Write as _;
use model::interfaces::ProximitySensor;
use model::types::PeripheralError;
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

    fn read_data(sensor: &mut S, _ctx: &Self::Context) -> Result<Self::Data, Self::Error> {
        sensor.read_distance_mm()
    }
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
    sensor_id: u8,
    sensor: S,
    periodic_enabled: bool,
    upstream_tx: Option<embassy_sync::channel::Sender<'a, M, Cmd, 4>>,
    make_cmd: Option<fn(u8, Data) -> Cmd>,
    interrupt_pin: Option<Pin>,
}

impl<'a, S, Data, M: embassy_sync::blocking_mutex::raw::RawMutex, Pin, Cmd>
    SensorStateManager<'a, S, Data, M, Pin, Cmd>
{
    /// Creates a new SensorStateManager.
    pub const fn new(
        sensor_id: u8,
        sensor: S,
        upstream_tx: Option<embassy_sync::channel::Sender<'a, M, Cmd, 4>>,
        make_cmd: Option<fn(u8, Data) -> Cmd>,
        interrupt_pin: Option<Pin>,
    ) -> Self {
        Self {
            sensor_id,
            sensor,
            periodic_enabled: true,
            upstream_tx,
            make_cmd,
            interrupt_pin,
        }
    }

    /// Gets the sensor ID.
    pub fn sensor_id(&self) -> u8 {
        self.sensor_id
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
        Data: Copy,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin,
        Cmd: Clone + core::fmt::Debug,
    > SensorStateManager<'a, S, Data, M, Pin, Cmd>
{
    /// Sends a command upstream if configured.
    pub fn notify_upstream(&self, data: Data) {
        if let (Some(tx), Some(make_cmd)) = (&self.upstream_tx, &self.make_cmd) {
            let cmd = make_cmd(self.sensor_id, data);
            tx.try_send(cmd).unwrap();
        }
    }
}

impl<'a, S, Data, M: embassy_sync::blocking_mutex::raw::RawMutex, Pin: DataReadyPin, Cmd>
    SensorStateManager<'a, S, Data, M, Pin, Cmd>
{
    /// Waits for the data ready interrupt to trigger if the interrupt pin is configured.
    pub async fn wait_for_interrupt(&mut self) {
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
    pub const fn new(sensor_id: u8, sensor: S, wake_threshold_mm: u16) -> Self {
        Self {
            state_manager: SensorStateManager::new(sensor_id, sensor, None, None, None),
            latest_data: 1000,
            context: ProximityReaderContext { wake_threshold_mm },
        }
    }
}

impl<
        'a,
        S: ProximitySensor,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Cmd: Clone + core::fmt::Debug,
    > SensorController<'a, S, M, DummyDataReadyPin, Cmd, ProximityReader>
{
    /// Creates a new SensorController with upstream system notification.
    pub fn new_with_fusion(
        sensor_id: u8,
        sensor: S,
        upstream_tx: embassy_sync::channel::Sender<'a, M, Cmd, 4>,
        make_cmd: fn(u8, u16) -> Cmd,
        wake_threshold_mm: u16,
    ) -> Self {
        Self {
            state_manager: SensorStateManager::new(
                sensor_id,
                sensor,
                Some(upstream_tx),
                Some(make_cmd),
                None,
            ),
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
        Cmd: Clone + core::fmt::Debug,
        Reader: SensorReader<S>,
    > SensorController<'a, S, M, Pin, Cmd, Reader>
{
    /// Creates a generic SensorController.
    pub fn new_generic(
        sensor_id: u8,
        sensor: S,
        latest_data: Reader::Data,
        interrupt_pin: Option<Pin>,
        context: Reader::Context,
    ) -> Self {
        Self {
            state_manager: SensorStateManager::new(sensor_id, sensor, None, None, interrupt_pin),
            latest_data,
            context,
        }
    }

    /// Creates a generic SensorController with upstream system notification.
    pub fn new_generic_with_fusion(
        sensor_id: u8,
        sensor: S,
        latest_data: Reader::Data,
        upstream_tx: embassy_sync::channel::Sender<'a, M, Cmd, 4>,
        make_cmd: fn(u8, Reader::Data) -> Cmd,
        interrupt_pin: Option<Pin>,
        context: Reader::Context,
    ) -> Self {
        Self {
            state_manager: SensorStateManager::new(
                sensor_id,
                sensor,
                Some(upstream_tx),
                Some(make_cmd),
                interrupt_pin,
            ),
            latest_data,
            context,
        }
    }

    /// Gets a mutable reference to the underlying sensor.
    pub fn sensor_mut(&mut self) -> &mut S {
        self.state_manager.sensor_mut()
    }

    /// Gets the latest read sensor data.
    pub fn latest_data(&self) -> Reader::Data {
        self.latest_data
    }

    /// Gets the sensor ID.
    pub fn sensor_id(&self) -> u8 {
        self.state_manager.sensor_id()
    }

    /// Gets whether periodic monitoring is enabled.
    pub fn is_periodic_enabled(&self) -> bool {
        self.state_manager.is_periodic_enabled()
    }

    /// Ticks the sensor control loop, updating proximity distance.
    pub fn update(&mut self) -> Result<Reader::Data, Reader::Error> {
        let data = Reader::read_data(self.state_manager.sensor_mut(), &self.context)?;

        self.latest_data = data;

        self.notify_upstream(data);

        Ok(data)
    }

    /// Handles a SensorCommand.
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
    pub async fn run(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, M, SensorCommand, 4>,
    ) -> ! {
        loop {
            let rx_fut = command_rx.receive();
            let interrupt_fut = self.wait_for_interrupt();
            let timeout_fut = embassy_time::Timer::after(embassy_time::Duration::from_millis(1000));

            match embassy_futures::select::select3(rx_fut, interrupt_fut, timeout_fut).await {
                // Command received from system shell/console
                embassy_futures::select::Either3::First(cmd) => {
                    self.handle_command(cmd);
                }
                // Proximity interrupt triggered (GPIO1 output from ToF went low)
                embassy_futures::select::Either3::Second(_) => {
                    if self.is_periodic_enabled() {
                        let _ = self.update();
                    }
                }
                // Periodic update interval elapsed
                embassy_futures::select::Either3::Third(_) => {
                    if self.is_periodic_enabled() {
                        let _ = self.update();
                    }
                }
            }
        }
    }
}

impl<
        'a,
        S: ProximitySensor,
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        Pin: DataReadyPin,
        Cmd: Clone + core::fmt::Debug,
    > SensorController<'a, S, M, Pin, Cmd, ProximityReader>
{
    /// Creates a new SensorController with upstream system notification and interrupt pin support.
    pub fn new_with_fusion_and_interrupt(
        sensor_id: u8,
        sensor: S,
        upstream_tx: embassy_sync::channel::Sender<'a, M, Cmd, 4>,
        make_cmd: fn(u8, u16) -> Cmd,
        interrupt_pin: Pin,
        wake_threshold_mm: u16,
    ) -> Self {
        Self {
            state_manager: SensorStateManager::new(
                sensor_id,
                sensor,
                Some(upstream_tx),
                Some(make_cmd),
                Some(interrupt_pin),
            ),
            latest_data: 1000,
            context: ProximityReaderContext { wake_threshold_mm },
        }
    }

    /// Gets the current proximity telemetry reading.
    pub fn telemetry(&self) -> model::types::ProximityTelemetry {
        let dir = model::types::Direction::try_from(self.sensor_id())
            .unwrap_or(model::types::Direction::North);
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

/// Represents the physical directions of ToF proximity sensors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorDirection {
    /// North sensor
    North,
    /// East sensor
    East,
    /// West sensor
    West,
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

/// Sensor-specific CLI commands
#[derive(Debug, embedded_cli::Command, Clone, Copy, PartialEq, Eq)]
pub enum SensorCliCommand {
    /// Read proximity sensors
    Status,
    /// Calibrate cover (near)
    #[command(name = "cal_near")]
    CalNear {
        /// Sensor direction ('north', 'east', or 'west')
        direction: SensorDirection,
    },
    /// Calibrate 100mm (far)
    #[command(name = "cal_far")]
    CalFar {
        /// Sensor direction ('north', 'east', or 'west')
        direction: SensorDirection,
    },
}

/// Processes sensor-specific CLI commands
#[allow(clippy::too_many_arguments)]
pub fn process_sensor_command<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    I2c: embedded_hal::i2c::I2c + 'static,
    Flash: embedded_storage::nor_flash::NorFlash + 'static,
    S: crate::BlockingProximityReader,
>(
    sensor_north: Option<*mut S>,
    sensor_east: Option<*mut S>,
    sensor_west: Option<*mut S>,
    i2c_ptr: Option<*mut I2c>,
    flash_ptr: Option<*mut Flash>,
    storage_start: u32,
    storage_end: u32,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
    cmd: SensorCliCommand,
) -> Result<(), &'static str> {
    match cmd {
        SensorCliCommand::Status => {
            let read_sensor = |ptr_opt: Option<*mut S>| {
                ptr_opt
                    .ok_or("Proximity sensor pointer not available")
                    .and_then(|p| {
                        unsafe { &mut *p }
                            .read_distance_blocking()
                            .map_err(|_| "Proximity sensor failed to read")
                    })
            };
            let dn = read_sensor(sensor_north)?;
            let de = read_sensor(sensor_east)?;
            let dw = read_sensor(sensor_west)?;
            let _ = core::writeln!(
                writer,
                "\r\nDirect proximity readings: North = {} mm, East = {} mm, West = {} mm",
                dn,
                de,
                dw
            );
            Ok(())
        }
        SensorCliCommand::CalNear { direction } => {
            let i2c_raw = i2c_ptr.ok_or("I2C controller not available")?;
            let i2c = unsafe { &mut *i2c_raw };
            let (addr, name) = match direction {
                SensorDirection::North => (0x30, "North"),
                SensorDirection::East => (0x31, "East"),
                SensorDirection::West => (0x32, "West"),
            };

            let d_raw = {
                let mut sensor = peripherals::vl53l0x::Vl53l0x::new(i2c, addr);
                sensor.read_distance_mm().unwrap_or(1000)
            };

            let _ = core::writeln!(
                writer,
                "\r\nCalibrating cover (near) for {} sensor: Raw distance = {} mm",
                name,
                d_raw
            );

            let flash_raw = flash_ptr.ok_or("Flash controller not available")?;
            static mut SHELL_FS_BUF_1: [u8; 2048] = [0u8; 2048];
            let flash_ref = unsafe { &mut *flash_raw };
            let async_flash = firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
            let fs_buf = unsafe { &mut *core::ptr::addr_of_mut!(SHELL_FS_BUF_1) };
            let mut fs = crate::filesystem_controller::FilesystemController::new(
                async_flash,
                storage_start..storage_end,
                fs_buf,
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
        SensorCliCommand::CalFar { direction } => {
            let i2c_raw = i2c_ptr.ok_or("I2C controller not available")?;
            let i2c = unsafe { &mut *i2c_raw };
            let (addr, name) = match direction {
                SensorDirection::North => (0x30, "North"),
                SensorDirection::East => (0x31, "East"),
                SensorDirection::West => (0x32, "West"),
            };

            let d_raw = {
                let mut sensor = peripherals::vl53l0x::Vl53l0x::new(i2c, addr);
                sensor.read_distance_mm().unwrap_or(1000)
            };

            let _ = core::writeln!(
                writer,
                "\r\nCalibrating 100mm (far) for {} sensor: Raw distance = {} mm",
                name,
                d_raw
            );

            let flash_raw = flash_ptr.ok_or("Flash controller not available")?;
            static mut SHELL_FS_BUF_2: [u8; 2048] = [0u8; 2048];
            let flash_ref = unsafe { &mut *flash_raw };
            let async_flash = firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
            let fs_buf = unsafe { &mut *core::ptr::addr_of_mut!(SHELL_FS_BUF_2) };
            let mut fs = crate::filesystem_controller::FilesystemController::new(
                async_flash,
                storage_start..storage_end,
                fs_buf,
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
