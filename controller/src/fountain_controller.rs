use model::state_machine::{FountainEvent, FountainState, FountainStateMachine};
use peripherals::pump::Pump;
use peripherals::water_sensor::WaterSensor;

/// A project-agnostic coordinator connecting water fountain models and peripherals.
pub struct FountainController<P, S> {
    fsm: FountainStateMachine,
    /// The physical or mock pump peripheral.
    pub pump: P,
    /// The physical or mock water presence sensor.
    pub sensor: S,
}

impl<P: Pump, S: WaterSensor> FountainController<P, S> {
    /// Creates a new fountain controller managing the specified pump and water sensor.
    pub const fn new(pump: P, sensor: S) -> Self {
        Self {
            fsm: FountainStateMachine::new(),
            pump,
            sensor,
        }
    }

    /// Gets the current operating state of the fountain.
    pub fn state(&self) -> FountainState {
        self.fsm.state()
    }

    /// Ticks the control loop, reading sensor inputs and updating the pump speed accordingly.
    pub fn update(&mut self) -> Result<(), FountainError<P::Error, S::Error>> {
        let water = self
            .sensor
            .is_water_detected()
            .map_err(FountainError::Sensor)?;

        if water {
            if self.fsm.state() == FountainState::LowWaterWarning {
                self.fsm.transition(FountainEvent::WaterDetected);
            } else if self.fsm.state() == FountainState::Idle {
                self.fsm.transition(FountainEvent::PowerOn);
            }
            self.pump.set_speed(100).map_err(FountainError::Pump)?;
        } else {
            self.fsm.transition(FountainEvent::WaterMissing);
            self.pump.stop().map_err(FountainError::Pump)?;
        }

        Ok(())
    }

    /// Runs the controller's control loop infinitely, reading from the command channel.
    pub async fn run<M: embassy_sync::blocking_mutex::raw::RawMutex, const N: usize>(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, M, FountainCommand, N>,
    ) -> ! {
        loop {
            match embassy_time::with_timeout(
                embassy_time::Duration::from_millis(1000),
                command_rx.receive(),
            )
            .await
            {
                Ok(cmd) => match cmd {
                    FountainCommand::SetSpeed(speed) => {
                        let _ = self.pump.set_speed(speed);
                    }
                    FountainCommand::Stop => {
                        let _ = self.pump.stop();
                    }
                },
                Err(_timeout) => {
                    let _ = self.update();
                }
            }
        }
    }
}

/// One-way commands sent to the Fountain Controller from the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FountainCommand {
    /// Set the pump speed (0-100)
    SetSpeed(u8),
    /// Stop the pump
    Stop,
}

/// Errors returned by the fountain controller loop.
#[derive(Debug)]
pub enum FountainError<PE, SE> {
    /// Error originating from the pump driver.
    Pump(PE),
    /// Error originating from the water sensor driver.
    Sensor(SE),
}
