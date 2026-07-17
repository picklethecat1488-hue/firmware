//! Generalized motor controller that orchestrates motor driver outputs and current sensor monitoring.

#![deny(missing_docs)]

use crate::telemetry_controller::MotorTelemetryClient;
use crate::{BlockingMotorReader, BlockingMotorWriter, MotorReceiver, TelemetrySender};
use core::fmt::Write as _;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::raw::RawMutex;
use model::interfaces::{Motor, PowerMeasurementMode, PowerSensor, Tickable};
use model::telemetry::TelemetryClient;
use model::types::{MotorSpeed, PeripheralError, SystemStatus};
use peripherals::ToPeripheralError;

use crate::tracing;
use crate::types::{MotorCalState, MotorSafetyStatus, MotorState};

/// The tick interval of the motor controller (10ms / 100Hz).
pub const MOTOR_TICK_INTERVAL: embassy_time::Duration = embassy_time::Duration::from_millis(10);

/// Safety limits for the motor controller.
pub struct MotorLimits {
    /// Minimum current threshold in mA for dry-run detection.
    pub min_current_ma: i32,
    /// Maximum current threshold in mA for stall detection.
    pub max_current_ma: i32,
    /// Physical maximum RPM at 100% duty cycle.
    pub max_rpm: u32,
    /// Maximum RPM limit for safety cut-off.
    pub rpm_limit: u32,
}

impl MotorLimits {
    /// Checks if the given RPM and current draw violate configured safety limits.
    pub fn check_limits(&self, rpm: u32, current_ma: i32) -> MotorSafetyStatus {
        if self.rpm_limit > 0 && rpm > self.rpm_limit {
            return MotorSafetyStatus::RpmExceeded(rpm);
        }
        if current_ma < self.min_current_ma {
            return MotorSafetyStatus::DryRun(current_ma);
        }
        if current_ma > self.max_current_ma {
            return MotorSafetyStatus::Stall(current_ma);
        }
        MotorSafetyStatus::Ok
    }
}

/// A generalized motor controller that orchestrates motor driver outputs and current sensor monitoring.
pub struct MotorController<M, C> {
    state: MotorState,
    /// The physical or mock motor peripheral.
    pub motor: M,
    /// The physical or mock current sensor peripheral.
    pub current_sensor: C,
    /// Telemetry: last measured current in mA.
    last_current_ma: i32,
    speed: MotorSpeed,
    active_speed: MotorSpeed,
    calibration_present: bool,
    limits: MotorLimits,
}

impl<M: Motor + Tickable, C: PowerSensor> MotorController<M, C>
where
    <M as Motor>::Error: ToPeripheralError,
    <M as Tickable>::Error: ToPeripheralError,
    <C as PowerSensor>::Error: ToPeripheralError,
{
    /// Creates a new motor controller managing the specified motor and current sensor.
    pub const fn new(motor: M, current_sensor: C) -> Self {
        Self {
            state: MotorState::Off,
            motor,
            current_sensor,
            last_current_ma: 0,
            speed: MotorSpeed::ZERO,
            active_speed: MotorSpeed::ZERO,
            calibration_present: false,
            limits: MotorLimits {
                min_current_ma: 15,
                max_current_ma: 800,
                max_rpm: 0,
                rpm_limit: 0,
            },
        }
    }

    /// Gets the current operating state of the motor.
    pub fn state(&self) -> MotorState {
        self.state
    }

    /// Gets the minimum current limit in mA.
    pub fn min_current_ma(&self) -> i32 {
        self.limits.min_current_ma
    }

    /// Gets the maximum current limit in mA.
    pub fn max_current_ma(&self) -> i32 {
        self.limits.max_current_ma
    }

    /// Sets the minimum and maximum current limits for load/stall safety checks.
    pub fn set_current_limits(&mut self, min_ma: i32, max_ma: i32) {
        self.limits.min_current_ma = min_ma;
        self.limits.max_current_ma = max_ma;
    }

    /// Returns the current estimated RPM based on the active speed and max_rpm calibration.
    pub fn current_rpm(&self) -> u32 {
        if self.limits.max_rpm == 0 {
            0
        } else {
            (self.active_speed.get() as u32) * self.limits.max_rpm / 100
        }
    }

    /// Gets the last measured current in mA.
    pub fn last_current_ma(&self) -> i32 {
        self.last_current_ma
    }

    /// Directly reads the current draw (acting as a proxy for load torque) in mA from the sensor.
    pub fn read_torque_ma(&mut self) -> Result<i32, PeripheralError> {
        let current = self
            .current_sensor
            .read_current_ma()
            .map_err(|e| e.to_peripheral_error())?;
        self.last_current_ma = current;
        Ok(current)
    }

    /// Ticks the control loop, reading current sensor input and updating safety states.
    #[tracing::instrument(
        name = "motor_controller::update",
        level = "debug",
        skip(telemetry_client)
    )]
    pub fn update(
        &mut self,
        mut telemetry_client: Option<
            &mut MotorTelemetryClient<
                CriticalSectionRawMutex,
                { crate::telemetry_controller::CHANNEL_CAPACITY },
            >,
        >,
    ) -> Result<(), PeripheralError> {
        let is_running = self.state == MotorState::On;

        let current = if is_running {
            // Read current sensor (torque proxy)
            self.read_torque_ma()?
        } else {
            0
        };

        // If the motor is running, verify safety limits (RPM and load torque)
        if self.state == MotorState::On {
            let rpm = self.current_rpm();
            match self.limits.check_limits(rpm, current) {
                MotorSafetyStatus::RpmExceeded(_rpm_val) => {
                    self.handle_command(MotorCommand::Stop, telemetry_client.as_deref_mut());
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    defmt::error!(
                        "Motor Controller: RPM safety limit exceeded ({} RPM). Stopped motor.",
                        _rpm_val
                    );
                }
                MotorSafetyStatus::DryRun(_current_val) => {
                    self.handle_command(MotorCommand::Stop, telemetry_client.as_deref_mut());
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    defmt::warn!(
                        "Motor Controller: Low load / dry detected (current: {} mA). Stopped motor.",
                        _current_val
                    );
                }
                MotorSafetyStatus::Stall(_current_val) => {
                    self.handle_command(MotorCommand::Stop, telemetry_client.as_deref_mut());
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    defmt::error!(
                        "Motor Controller: Motor stall detected (current: {} mA). Stopped motor.",
                        _current_val
                    );
                }
                MotorSafetyStatus::Ok => {}
            }
        }

        if let Some(client) = telemetry_client {
            let running = self.state == MotorState::On;
            let status = if running {
                model::types::MotorStatus::Running(self.speed)
            } else {
                model::types::MotorStatus::Brake
            };
            client.report(status);
        }

        Ok(())
    }

    /// Handles a received MotorCommand.
    #[tracing::instrument(
        name = "motor_controller::handle_command",
        level = "debug",
        skip(cmd, telemetry_client)
    )]
    pub fn handle_command(
        &mut self,
        cmd: MotorCommand,
        mut telemetry_client: Option<
            &mut MotorTelemetryClient<
                CriticalSectionRawMutex,
                { crate::telemetry_controller::CHANNEL_CAPACITY },
            >,
        >,
    ) {
        match cmd {
            MotorCommand::SetSpeed(speed) => {
                if speed != MotorSpeed::ZERO {
                    if !self.calibration_present {
                        if self.speed != speed {
                            #[cfg(all(target_arch = "arm", target_os = "none"))]
                            defmt::error!(
                                "Motor Controller: Cannot start motor, calibration is not present!"
                            );
                            self.speed = speed;
                        }
                        return;
                    }
                    if self.state == MotorState::Off {
                        self.state = MotorState::On;
                        if let Err(e) = self
                            .current_sensor
                            .set_measurement_mode(PowerMeasurementMode::Continuous(true, true))
                        {
                            if let Some(ref client) = telemetry_client {
                                client.report_error(e.to_peripheral_error());
                            }
                        }
                    }
                    self.speed = speed;
                } else {
                    // Set target speed to 0 and let it ramp down
                    self.speed = MotorSpeed::ZERO;
                }
            }
            MotorCommand::SetSpeedRpm(rpm) => {
                let speed_val = if self.limits.max_rpm == 0 {
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    defmt::warn!(
                        "Motor Controller: Cannot set speed by RPM, max_rpm calibration is not set!"
                    );
                    0
                } else {
                    let val = (rpm * 100) / (self.limits.max_rpm as i32);
                    val.clamp(-100, 100)
                };
                let speed = MotorSpeed::new(speed_val as i8).unwrap_or(MotorSpeed::ZERO);
                self.handle_command(
                    MotorCommand::SetSpeed(speed),
                    telemetry_client.as_deref_mut(),
                );
            }
            MotorCommand::Stop => {
                // Immediate emergency stop
                self.state = MotorState::Off;
                self.speed = MotorSpeed::ZERO;
                self.active_speed = MotorSpeed::ZERO;
                if let Err(e) = self.motor.stop() {
                    if let Some(ref client) = telemetry_client {
                        client.report_error(e.to_peripheral_error());
                    }
                }
                if let Err(e) = self
                    .current_sensor
                    .set_measurement_mode(PowerMeasurementMode::PowerDown)
                {
                    if let Some(ref client) = telemetry_client {
                        client.report_error(e.to_peripheral_error());
                    }
                }
            }
        }

        if let Some(client) = telemetry_client {
            let running = self.state == MotorState::On;
            let status = if running {
                model::types::MotorStatus::Running(self.speed)
            } else {
                model::types::MotorStatus::Brake
            };
            client.report(status);
        }
    }

    /// Ticks the motor controller at a high frequency (e.g. 100Hz / every 10ms).
    /// This updates the ramping of the motor speed and runs the motor driver's duty cycle ticks.
    #[tracing::instrument(name = "motor_controller::tick_motor", level = "debug")]
    pub fn tick_motor(&mut self) -> Result<(), PeripheralError> {
        // 1. Ramping logic
        if self.state == MotorState::On {
            if self.active_speed < self.speed {
                // Ramp up
                let next = (self.active_speed.get() + 1).min(self.speed.get());
                self.active_speed = MotorSpeed::new(next).unwrap();
                if self.active_speed == MotorSpeed::ZERO && self.speed == MotorSpeed::ZERO {
                    self.state = MotorState::Off;
                    self.motor.stop().map_err(|e| e.to_peripheral_error())?;
                    let _ = self
                        .current_sensor
                        .set_measurement_mode(PowerMeasurementMode::PowerDown);
                } else {
                    self.motor
                        .set_speed(self.active_speed)
                        .map_err(|e| e.to_peripheral_error())?;
                }
            } else if self.active_speed > self.speed {
                // Ramp down
                let next = (self.active_speed.get() - 1).max(self.speed.get());
                self.active_speed = MotorSpeed::new(next).unwrap();
                if self.active_speed == MotorSpeed::ZERO && self.speed == MotorSpeed::ZERO {
                    self.state = MotorState::Off;
                    self.motor.stop().map_err(|e| e.to_peripheral_error())?;
                    let _ = self
                        .current_sensor
                        .set_measurement_mode(PowerMeasurementMode::PowerDown);
                } else {
                    self.motor
                        .set_speed(self.active_speed)
                        .map_err(|e| e.to_peripheral_error())?;
                }
            }
        } else if self.active_speed.get() != 0 {
            // Ramping down/up to 0 even if state is Off
            let current_raw = self.active_speed.get();
            let next = if current_raw > 0 {
                current_raw - 1
            } else {
                current_raw + 1
            };
            self.active_speed = MotorSpeed::new(next).unwrap();
            if self.active_speed == MotorSpeed::ZERO {
                self.motor.stop().map_err(|e| e.to_peripheral_error())?;
            } else {
                self.motor
                    .set_speed(self.active_speed)
                    .map_err(|e| e.to_peripheral_error())?;
            }
        }

        // 2. Call motor driver's tick() for software PWM toggling
        self.motor.tick().map_err(|e| e.to_peripheral_error())?;

        Ok(())
    }

    /// Runs the controller's control loop infinitely, reading from the command channel.
    pub async fn run<MutexRaw: embassy_sync::blocking_mutex::raw::RawMutex, const N: usize>(
        mut self,
        command_rx: MotorReceiver<MutexRaw, N>,
        telemetry_tx: TelemetrySender<
            CriticalSectionRawMutex,
            { crate::telemetry_controller::CHANNEL_CAPACITY },
        >,
    ) -> ! {
        let mut telemetry_client = MotorTelemetryClient::new(Some(telemetry_tx));
        // Put the current sensor into power-down mode on startup since the motor is off
        if let Err(e) = self
            .current_sensor
            .set_measurement_mode(PowerMeasurementMode::PowerDown)
        {
            telemetry_client.report_error(e.to_peripheral_error());
        }

        let mut ticker = embassy_time::Ticker::every(MOTOR_TICK_INTERVAL);
        let mut slow_tick_counter = 0;

        loop {
            // Process any available commands non-blockingly
            while let Ok(cmd) = command_rx.try_receive() {
                self.handle_command(cmd, Some(&mut telemetry_client));
            }

            // Tick ramping and duty cycle
            if let Err(e) = self.tick_motor() {
                telemetry_client.report_error(e);
            }

            // Run telemetry, safety, and current monitoring once per second
            slow_tick_counter += 1;
            if slow_tick_counter >= 100 {
                slow_tick_counter = 0;
                if let Err(e) = self.update(Some(&mut telemetry_client)) {
                    telemetry_client.report_error(e);
                }
            }

            ticker.next().await;
        }
    }
}

/// One-way commands sent to the Motor Controller from the shell or app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorCommand {
    /// Set the motor speed (0-100)
    SetSpeed(model::types::MotorSpeed),
    /// Set the motor speed using a target RPM (signed)
    SetSpeedRpm(i32),
    /// Stop the motor
    Stop,
}

impl<M: Motor + Tickable, C: PowerSensor> model::calibration::Calibration
    for MotorController<M, C>
{
    fn set_calibration(&mut self, calibration: model::calibration::CalibrationType) {
        if let model::calibration::CalibrationType::MotorCal {
            current_limits,
            max_rpm,
            rpm_limit,
        } = calibration
        {
            self.calibration_present = true;
            self.limits.min_current_ma = current_limits.low;
            self.limits.max_current_ma = current_limits.high;
            self.limits.max_rpm = max_rpm;
            self.limits.rpm_limit = rpm_limit;
        }
    }
}

impl<M: Motor + Tickable, C: PowerSensor> crate::BlockingMotorReader for MotorController<M, C>
where
    <C as PowerSensor>::Error: ToPeripheralError,
    <M as Motor>::Error: ToPeripheralError,
    <M as Tickable>::Error: ToPeripheralError,
{
    fn read_current_ma_blocking(&mut self) -> Result<i32, PeripheralError> {
        self.read_torque_ma()
    }
}

impl<M: Motor + Tickable, C: PowerSensor> crate::BlockingMotorWriter for MotorController<M, C>
where
    <C as PowerSensor>::Error: ToPeripheralError,
    <M as Motor>::Error: ToPeripheralError,
    <M as Tickable>::Error: ToPeripheralError,
{
    fn set_motor_speed(&mut self, speed: i8) -> Result<(), PeripheralError> {
        let motor_speed = MotorSpeed::new(speed).ok_or(PeripheralError::InvalidConfiguration)?;
        let _ = self.motor.set_speed(motor_speed);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), PeripheralError> {
        let _ = self.motor.stop();
        Ok(())
    }
}

impl<'a> embedded_cli::arguments::FromArgument<'a> for MotorCalState {
    fn from_arg(arg: &'a str) -> Result<Self, embedded_cli::arguments::FromArgumentError<'a>> {
        match arg {
            "empty" => Ok(MotorCalState::Empty),
            "low" => Ok(MotorCalState::Low),
            "high" => Ok(MotorCalState::High),
            "overload" => Ok(MotorCalState::Overload),
            _ => Err(embedded_cli::arguments::FromArgumentError {
                value: arg,
                expected: "one of 'empty', 'low', 'high', or 'overload'",
            }),
        }
    }
}

use firmware_lib::subcommand_enum;

subcommand_enum! {
    /// Motor subcommands for CLI processing.
    pub enum MotorSubcommand {
        /// Set motor speed
        Speed,
        /// Stop motor
        Stop,
        /// Calibrate motor
        Calibrate,
    }
    "Invalid motor subcommand. Expected: speed, stop, calibrate"
}

/// Processes motor-specific CLI subcommands.
pub fn handle_motor_cli<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    C: crate::ShellConfig,
>(
    resolver: &impl crate::ShellDeviceResolver<C>,
    subcommand: Option<MotorSubcommand>,
    arg1: Option<&str>,
    arg2: Option<&str>,
    arg3: Option<&str>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let motor_ctrl = resolver.resolve_motor_ctrl(None)?;
    let mut fs_buf = resolver.lock_fs_buffer()?;
    let fs_buf_static = unsafe { fs_buf.as_static_mut() };

    let cmd = subcommand.ok_or("Missing motor subcommand")?;

    match cmd {
        MotorSubcommand::Speed => {
            let speed_str = arg1.ok_or("Missing speed parameter")?;
            let speed = speed_str
                .parse::<i8>()
                .map_err(|_| "Invalid speed parameter")?;
            motor_ctrl
                .set_motor_speed(speed)
                .map_err(|_| "Failed to set motor speed")?;
            let current = motor_ctrl
                .read_current_ma_blocking()
                .map_err(|_| "Failed to read motor current")?;
            let _ = core::writeln!(writer, "\r\nMotor current: {} mA", current);
            Ok(())
        }
        MotorSubcommand::Stop => motor_ctrl.stop().map_err(|_| "Failed to stop motor"),
        MotorSubcommand::Calibrate => {
            let state_str = arg1.ok_or("Missing calibration state")?;
            let state = match state_str {
                "empty" => MotorCalState::Empty,
                "low" => MotorCalState::Low,
                "high" => MotorCalState::High,
                "overload" => MotorCalState::Overload,
                _ => return Err("Invalid calibration state. Expected: empty, low, high, overload"),
            };
            let max_rpm = arg2.and_then(|s| s.parse::<u32>().ok());
            let rpm_limit = arg3.and_then(|s| s.parse::<u32>().ok());

            let motor = resolver.resolve_motor(None)?;
            let _ = core::writeln!(writer, "\r\nStarting motor for calibration...");
            let _ = motor.set_speed(model::types::MotorSpeed::MAX);

            let _ = core::writeln!(writer, "Waiting 1 second for motor to ramp up...");
            embassy_time::block_for(embassy_time::Duration::from_millis(1000));

            let i2c = resolver.resolve_i2c(None)?;
            let mut current_sensor = peripherals::ina219::Ina219::new(i2c);
            current_sensor
                .init()
                .map_err(|_| "Failed to initialize INA219 current sensor")?;

            let mut sum = 0;
            for _ in 0..5 {
                let current_val = current_sensor
                    .read_current_ma()
                    .map_err(|_| "Failed to read current from INA219 current sensor")?;
                sum += current_val;
                embassy_time::block_for(embassy_time::Duration::from_millis(100));
            }
            let current = sum / 5;

            let name = match state {
                MotorCalState::Empty => "Empty",
                MotorCalState::Low => "Low",
                MotorCalState::High => "High",
                MotorCalState::Overload => "Overload",
            };

            let _ = core::writeln!(
                writer,
                "Stopping motor and recording measured current for {} state: {} mA",
                name,
                current
            );
            let _ = motor.stop();

            let partition = resolver.resolve_partition(None)?;
            let flash_ref = unsafe { &mut *partition.flash_ptr };
            let async_flash = firmware_lib::BlockingAsyncFlash(flash_ref);
            let mut fs = crate::filesystem_controller::FilesystemController::new(
                async_flash,
                partition.start_address..partition.end_address,
                fs_buf_static,
            );

            let mut buf = [0u8; 128];
            let cal = embassy_futures::block_on(fs.read_file("motor_cal.cbor", &mut buf))
                .ok()
                .flatten()
                .and_then(|bytes| {
                    minicbor::decode::<model::calibration::MotorCalibration>(bytes).ok()
                })
                .unwrap_or_default();

            let mut cal = cal;
            let ref_point = model::calibration::FourPointRef::from(state);
            cal.current_ma[ref_point] = current;
            if let Some(max_rpm_val) = max_rpm {
                cal.max_rpm = Some(max_rpm_val);
                let _ = core::writeln!(
                    writer,
                    "Configuring physical maximum RPM to {} RPM.",
                    max_rpm_val
                );
            }
            if let Some(limit_val) = rpm_limit {
                cal.rpm_limit = Some(limit_val);
                let _ = core::writeln!(
                    writer,
                    "Configuring maximum RPM safety limit to {} RPM.",
                    limit_val
                );
            }

            let mut write_buf = [0u8; 128];
            let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
            let mut encoder = minicbor::Encoder::new(cursor);
            encoder.encode(cal).unwrap();
            let len = encoder.into_writer().position();

            embassy_futures::block_on(fs.write_file("motor_cal.cbor", &write_buf[..len]))
                .map(|_| {
                    let _ = core::writeln!(writer, "Saved motor {} calibration to flash.", name);
                })
                .map_err(|_| "Error saving calibration to flash")
        }
    }
}

/// Standard config implementation for MotorFeature.
pub struct MotorFeatureConfig<MutexRaw: RawMutex + 'static, const N: usize> {
    /// Motor channel sender
    pub motor_tx: Option<crate::MotorSender<MutexRaw, N>>,
    /// Maximum motor speed
    pub max_speed: MotorSpeed,
}

impl<MutexRaw: RawMutex + 'static, const N: usize> MotorFeatureConfig<MutexRaw, N> {
    /// Creates a new `MotorFeatureConfig`.
    pub fn new(motor_tx: Option<crate::MotorSender<MutexRaw, N>>, max_speed: MotorSpeed) -> Self {
        Self {
            motor_tx,
            max_speed,
        }
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize> crate::SystemFeature<MutexRaw, N>
    for MotorFeatureConfig<MutexRaw, N>
{
    fn on_state_changed(
        &self,
        _from: SystemStatus,
        _to: SystemStatus,
        support: crate::DeviceSupport,
        _battery_status: Option<crate::BatteryStatus>,
        _thermal_critical: bool,
    ) {
        if let Some(ref motor_tx) = self.motor_tx {
            if support.motor {
                let _ = motor_tx.try_send(crate::MotorCommand::SetSpeed(self.max_speed));
            } else {
                let _ = motor_tx.try_send(crate::MotorCommand::Stop);
            }
        }
    }
}
