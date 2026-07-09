//! Generalized motor controller that orchestrates motor driver outputs and current sensor monitoring.

#![deny(missing_docs)]

use crate::telemetry_controller::MotorTelemetryClient;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use model::interfaces::{Motor, PowerMeasurementMode, PowerSensor, Tickable};
use model::telemetry::TelemetryClient;
use model::types::{MotorSpeed, PeripheralError};

/// The tick interval of the motor controller (10ms / 100Hz).
pub const MOTOR_TICK_INTERVAL: embassy_time::Duration = embassy_time::Duration::from_millis(10);
use peripherals::ToPeripheralError;

/// The operating states of the motor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MotorState {
    /// The motor is powered off.
    #[default]
    Off,
    /// The motor is running continuously at target speed.
    On,
}

/// Status representing safety check results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorSafetyStatus {
    /// All limits are within safe operating parameters.
    Ok,
    /// The motor RPM exceeded the safety limit.
    RpmExceeded(u32),
    /// Low load / dry run detected.
    DryRun(i32),
    /// Motor stall detected (high current).
    Stall(i32),
}

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
    pub fn update(
        &mut self,
        mut telemetry_client: Option<
            &mut MotorTelemetryClient<
                '_,
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
    pub fn handle_command(
        &mut self,
        cmd: MotorCommand,
        mut telemetry_client: Option<
            &mut MotorTelemetryClient<
                '_,
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
        command_rx: embassy_sync::channel::Receiver<'static, MutexRaw, MotorCommand, N>,
        telemetry_tx: embassy_sync::channel::Sender<
            'static,
            CriticalSectionRawMutex,
            model::telemetry::TelemetryRecord,
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

/// Errors returned by the motor controller loop.
#[derive(Debug)]
pub enum MotorError<ME, CE> {
    /// Error originating from the motor driver.
    Motor(ME),
    /// Error originating from the current sensor driver.
    CurrentSensor(CE),
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
}
