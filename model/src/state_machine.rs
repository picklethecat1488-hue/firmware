/// The operating states of the cat water fountain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FountainState {
    /// The fountain is powered down or idle.
    #[default]
    Idle,
    /// The impeller is running and water is pumping.
    Pumping,
    /// Alert state indicating the water level is too low.
    LowWaterWarning,
}

/// External events that trigger state machine transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FountainEvent {
    /// Power on button pressed or supply connected.
    PowerOn,
    /// Water sensor detects that water is present.
    WaterDetected,
    /// Water sensor detects that water is missing.
    WaterMissing,
    /// Timer for pumping duration has expired.
    TimerExpired,
    /// Power off button pressed or supply disconnected.
    PowerOff,
}

/// A target-agnostic state machine for managing cat water fountain state.
#[derive(Default)]
pub struct FountainStateMachine {
    state: FountainState,
}

impl FountainStateMachine {
    /// Creates a new state machine initialized to the Idle state.
    pub const fn new() -> Self {
        Self {
            state: FountainState::Idle,
        }
    }

    /// Gets the current state of the fountain.
    pub fn state(&self) -> FountainState {
        self.state
    }

    /// Transition to a new state based on an external event.
    pub fn transition(&mut self, event: FountainEvent) {
        self.state = match (self.state, event) {
            (_, FountainEvent::PowerOff) => FountainState::Idle,
            (FountainState::Idle, FountainEvent::PowerOn) => FountainState::Pumping,
            (FountainState::Pumping, FountainEvent::WaterMissing) => FountainState::LowWaterWarning,
            (FountainState::LowWaterWarning, FountainEvent::WaterDetected) => {
                FountainState::Pumping
            }
            (FountainState::Pumping, FountainEvent::TimerExpired) => FountainState::Idle,
            _ => self.state,
        };
    }
}
