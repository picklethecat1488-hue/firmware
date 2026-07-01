//! Generalized motor controller that orchestrates motor driver outputs and current sensor monitoring.

#![deny(missing_docs)]

use crate::state_machine::{MotorEvent, MotorState, MotorStateMachine};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use model::interfaces::{Motor, PowerMeasurementMode, PowerSensor};

/// A generalized motor controller that orchestrates motor driver outputs and current sensor monitoring.
pub struct MotorController<M, C> {
    fsm: MotorStateMachine,
    /// The physical or mock motor peripheral.
    pub motor: M,
    /// The physical or mock current sensor peripheral.
    pub current_sensor: C,
    /// Telemetry: last measured current in mA.
    last_current_ma: i32,
    speed: u8,
}

impl<M: Motor, C: PowerSensor> MotorController<M, C> {
    /// Creates a new motor controller managing the specified motor and current sensor.
    pub const fn new(motor: M, current_sensor: C) -> Self {
        Self {
            fsm: MotorStateMachine::new(),
            motor,
            current_sensor,
            last_current_ma: 0,
            speed: 0,
        }
    }

    /// Gets the current operating state of the motor.
    pub fn state(&self) -> MotorState {
        self.fsm.state()
    }

    /// Gets the last measured current in mA.
    pub fn last_current_ma(&self) -> i32 {
        self.last_current_ma
    }

    /// Directly reads the current draw (acting as a proxy for load torque) in mA from the sensor.
    pub fn read_torque_ma(&mut self) -> Result<i32, C::Error> {
        let current = self.current_sensor.read_current_ma()?;
        self.last_current_ma = current;
        Ok(current)
    }

    /// Ticks the control loop, reading current sensor input and updating safety states.
    pub fn update(
        &mut self,
        telemetry_tx: Option<
            &embassy_sync::channel::Sender<
                '_,
                CriticalSectionRawMutex,
                model::telemetry::TelemetryRecord,
                16,
            >,
        >,
    ) -> Result<(), MotorError<M::Error, C::Error>> {
        let is_running = self.fsm.state() == MotorState::On
            || self.fsm.state() == MotorState::RampUp
            || self.fsm.state() == MotorState::RampDown;

        let current = if is_running {
            // Read current sensor (torque proxy)
            self.read_torque_ma().map_err(MotorError::CurrentSensor)?
        } else {
            0
        };

        // Auto-transition ramping states
        if self.fsm.state() == MotorState::RampUp || self.fsm.state() == MotorState::RampDown {
            self.fsm.transition(MotorEvent::RampComplete);
        }

        // If the motor is running, verify load torque
        if self.fsm.state() == MotorState::On {
            // Check for dry running (current is unusually low when running, e.g. < 15mA)
            if current < 15 {
                self.fsm.transition(MotorEvent::PowerOff);
                self.speed = 0;
                self.motor.stop().map_err(MotorError::Motor)?;
                let _ = self
                    .current_sensor
                    .set_measurement_mode(PowerMeasurementMode::PowerDown);
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::warn!(
                    "Motor Controller: Low load / dry detected (current: {} mA). Stopped motor.",
                    current
                );
            } else if current > 800 {
                // Check for motor stall (current is too high, e.g. > 800mA)
                self.fsm.transition(MotorEvent::PowerOff);
                self.speed = 0;
                self.motor.stop().map_err(MotorError::Motor)?;
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

        if let Some(tx) = telemetry_tx {
            let running =
                self.fsm.state() == MotorState::On || self.fsm.state() == MotorState::RampUp;
            let status = if running {
                model::types::MotorStatus::Running(self.speed)
            } else {
                model::types::MotorStatus::Brake
            };
            let _ = tx.try_send(model::telemetry::TelemetryRecord::Motor(status));
        }

        Ok(())
    }

    /// Handles a received MotorCommand.
    pub fn handle_command(
        &mut self,
        cmd: MotorCommand,
        telemetry_tx: Option<
            &embassy_sync::channel::Sender<
                '_,
                CriticalSectionRawMutex,
                model::telemetry::TelemetryRecord,
                16,
            >,
        >,
    ) {
        match cmd {
            MotorCommand::SetSpeed(speed) => {
                if speed > 0 {
                    if self.fsm.state() == MotorState::Off {
                        self.fsm.transition(MotorEvent::PowerOn);
                        let _ = self
                            .current_sensor
                            .set_measurement_mode(PowerMeasurementMode::Continuous(true, true));
                    }
                    self.speed = speed;
                    let _ = self.motor.set_speed(speed);
                } else {
                    self.fsm.transition(MotorEvent::PowerOff);
                    self.speed = 0;
                    let _ = self.motor.stop();
                    let _ = self
                        .current_sensor
                        .set_measurement_mode(PowerMeasurementMode::PowerDown);
                }
            }
            MotorCommand::Stop => {
                self.fsm.transition(MotorEvent::PowerOff);
                self.speed = 0;
                let _ = self.motor.stop();
                let _ = self
                    .current_sensor
                    .set_measurement_mode(PowerMeasurementMode::PowerDown);
            }
        }

        if let Some(tx) = telemetry_tx {
            let running =
                self.fsm.state() == MotorState::On || self.fsm.state() == MotorState::RampUp;
            let status = if running {
                model::types::MotorStatus::Running(self.speed)
            } else {
                model::types::MotorStatus::Brake
            };
            let _ = tx.try_send(model::telemetry::TelemetryRecord::Motor(status));
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
            16,
        >,
    ) -> ! {
        // Put the current sensor into power-down mode on startup since the motor is off
        let _ = self
            .current_sensor
            .set_measurement_mode(PowerMeasurementMode::PowerDown);

        loop {
            match embassy_time::with_timeout(
                embassy_time::Duration::from_millis(1000),
                command_rx.receive(),
            )
            .await
            {
                Ok(cmd) => {
                    self.handle_command(cmd, Some(&telemetry_tx));
                }
                Err(_timeout) => {
                    let _ = self.update(Some(&telemetry_tx));
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
