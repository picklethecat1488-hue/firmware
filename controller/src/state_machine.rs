//! Deterministic state machine for managing motor states.

#![deny(missing_docs)]

/// The operating states of the motor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MotorState {
    /// The motor is powered off.
    #[default]
    Off,
    /// The motor is starting up and ramping its duty cycle.
    Ramping,
    /// The motor is running continuously at target speed.
    On,
}

/// External events that drive motor state transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorEvent {
    /// Turn the motor on.
    PowerOn,
    /// Turn the motor off.
    PowerOff,
    /// Motor has finished ramping to target speed.
    RampComplete,
}

/// A target-agnostic state machine for managing motor operating states.
#[derive(Default)]
pub struct MotorStateMachine {
    state: MotorState,
}

impl MotorStateMachine {
    /// Creates a new state machine initialized to the Off state.
    pub const fn new() -> Self {
        Self {
            state: MotorState::Off,
        }
    }

    /// Gets the current state of the motor.
    pub fn state(&self) -> MotorState {
        self.state
    }

    /// Transition to a new state based on an external event.
    pub fn transition(&mut self, event: MotorEvent) {
        self.state = match (self.state, event) {
            (_, MotorEvent::PowerOff) => MotorState::Off,
            (MotorState::Off, MotorEvent::PowerOn) => MotorState::Ramping,
            (MotorState::Ramping, MotorEvent::RampComplete) => MotorState::On,
            _ => self.state,
        };
    }
}
