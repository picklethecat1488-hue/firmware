//! Deterministic state machine for managing motor states.

#![deny(missing_docs)]

/// The operating states of the motor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MotorState {
    /// The motor is powered off.
    #[default]
    Off,
    /// The motor is starting up and ramping up its duty cycle.
    RampUp,
    /// The motor is running continuously at target speed.
    On,
    /// The motor is shutting down and ramping down its duty cycle to 0.
    RampDown,
}

/// External events that drive motor state transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorEvent {
    /// Turn the motor on.
    PowerOn,
    /// Turn the motor off.
    PowerOff,
    /// Motor has finished ramping to target speed or completely stopped.
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
            // PowerOff transitions to RampDown unless we are already Off
            (MotorState::Off, MotorEvent::PowerOff) => MotorState::Off,
            (MotorState::RampDown, MotorEvent::PowerOff) => MotorState::RampDown,
            (_, MotorEvent::PowerOff) => MotorState::RampDown,

            // PowerOn transitions to RampUp unless we are already On
            (MotorState::On, MotorEvent::PowerOn) => MotorState::On,
            (MotorState::RampUp, MotorEvent::PowerOn) => MotorState::RampUp,
            (_, MotorEvent::PowerOn) => MotorState::RampUp,

            // RampComplete transitions: RampUp -> On, RampDown -> Off
            (MotorState::RampUp, MotorEvent::RampComplete) => MotorState::On,
            (MotorState::RampDown, MotorEvent::RampComplete) => MotorState::Off,

            _ => self.state,
        };
    }
}
