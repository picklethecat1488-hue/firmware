//! Generalized motor controller that orchestrates motor driver outputs and current sensor monitoring.

#![deny(missing_docs)]

use crate::telemetry_controller::MotorTelemetryClient;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use model::interfaces::{Motor, PowerMeasurementMode, PowerSensor};
use model::telemetry::TelemetryClient;
use model::types::PeripheralError;
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

/// A generalized motor controller that orchestrates motor driver outputs and current sensor monitoring.
pub struct MotorController<M, C> {
    state: MotorState,
    /// The physical or mock motor peripheral.
    pub motor: M,
    /// The physical or mock current sensor peripheral.
    pub current_sensor: C,
    /// Telemetry: last measured current in mA.
    last_current_ma: i32,
    speed: u8,
    min_current_ma: i32,
    max_current_ma: i32,
    calibration_present: bool,
}

impl<M: Motor, C: PowerSensor> MotorController<M, C>
where
    <C as PowerSensor>::Error: ToPeripheralError,
    <M as Motor>::Error: ToPeripheralError,
{
    /// Creates a new motor controller managing the specified motor and current sensor.
    pub const fn new(motor: M, current_sensor: C) -> Self {
        Self {
            state: MotorState::Off,
            motor,
            current_sensor,
            last_current_ma: 0,
            speed: 0,
            min_current_ma: 15,
            max_current_ma: 800,
            calibration_present: false,
        }
    }

    /// Gets the current operating state of the motor.
    pub fn state(&self) -> MotorState {
        self.state
    }

    /// Gets the minimum current limit in mA.
    pub fn min_current_ma(&self) -> i32 {
        self.min_current_ma
    }

    /// Gets the maximum current limit in mA.
    pub fn max_current_ma(&self) -> i32 {
        self.max_current_ma
    }

    /// Sets the minimum and maximum current limits for load/stall safety checks.
    pub fn set_current_limits(&mut self, min_ma: i32, max_ma: i32) {
        self.min_current_ma = min_ma;
        self.max_current_ma = max_ma;
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
        telemetry_client: Option<
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

        // If the motor is running, verify load torque
        if self.state == MotorState::On {
            // Check for dry running (current is unusually low when running)
            if current < self.min_current_ma {
                self.state = MotorState::Off;
                self.speed = 0;
                self.motor.stop().map_err(|e| e.to_peripheral_error())?;
                self.current_sensor
                    .set_measurement_mode(PowerMeasurementMode::PowerDown)
                    .map_err(|e| e.to_peripheral_error())?;
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::warn!(
                    "Motor Controller: Low load / dry detected (current: {} mA). Stopped motor.",
                    current
                );
            } else if current > self.max_current_ma {
                // Check for motor stall (current is too high)
                self.state = MotorState::Off;
                self.speed = 0;
                self.motor.stop().map_err(|e| e.to_peripheral_error())?;
                let _ = self
                    .current_sensor
                    .set_measurement_mode(PowerMeasurementMode::PowerDown);
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::error!(
                    "Motor Controller: Motor stall detected (current: {} mA). Stopped motor.",
                    current
                );
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
        telemetry_client: Option<
            &mut MotorTelemetryClient<
                '_,
                CriticalSectionRawMutex,
                { crate::telemetry_controller::CHANNEL_CAPACITY },
            >,
        >,
    ) {
        match cmd {
            MotorCommand::SetSpeed(speed) => {
                if speed > 0 {
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
                    if let Err(e) = self.motor.set_speed(speed) {
                        if let Some(ref client) = telemetry_client {
                            client.report_error(e.to_peripheral_error());
                        }
                    }
                } else {
                    self.state = MotorState::Off;
                    self.speed = 0;
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
            MotorCommand::Stop => {
                self.state = MotorState::Off;
                self.speed = 0;
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

        loop {
            match embassy_time::with_timeout(
                embassy_time::Duration::from_millis(1000),
                command_rx.receive(),
            )
            .await
            {
                Ok(cmd) => {
                    self.handle_command(cmd, Some(&mut telemetry_client));
                }
                Err(_timeout) => {
                    if let Err(e) = self.update(Some(&mut telemetry_client)) {
                        telemetry_client.report_error(e);
                    }
                }
            }
        }
    }
}

/// One-way commands sent to the Motor Controller from the shell or app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorCommand {
    /// Set the motor speed (0-100)
    SetSpeed(u8),
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

impl<M: Motor, C: PowerSensor> model::calibration::Calibration for MotorController<M, C> {
    #[allow(clippy::single_match)]
    fn set_calibration(&mut self, calibration: model::calibration::CalibrationType) {
        match calibration {
            model::calibration::CalibrationType::MotorCal(min, max) => {
                self.calibration_present = true;
                self.min_current_ma = min;
                self.max_current_ma = max;
            }
            _ => {}
        }
    }
}

impl<M: Motor, C: PowerSensor> crate::BlockingMotorReader for MotorController<M, C>
where
    <C as PowerSensor>::Error: ToPeripheralError,
    <M as Motor>::Error: ToPeripheralError,
{
    fn read_current_ma_blocking(&mut self) -> Result<i32, PeripheralError> {
        self.read_torque_ma()
    }
}

impl<M: Motor, C: PowerSensor> crate::BlockingMotorWriter for MotorController<M, C>
where
    <C as PowerSensor>::Error: ToPeripheralError,
    <M as Motor>::Error: ToPeripheralError,
{
    fn set_motor_speed(&mut self, speed: u8) -> Result<(), PeripheralError> {
        let _ = self.motor.set_speed(speed);
        Ok(())
    }
}
