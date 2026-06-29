use model::state_machine::{FountainEvent, FountainState, FountainStateMachine};

#[test]
fn test_fountain_transitions() {
    let mut fsm = FountainStateMachine::new();
    assert_eq!(fsm.state(), FountainState::Idle);

    fsm.transition(FountainEvent::PowerOn);
    assert_eq!(fsm.state(), FountainState::Pumping);

    fsm.transition(FountainEvent::WaterMissing);
    assert_eq!(fsm.state(), FountainState::LowWaterWarning);

    fsm.transition(FountainEvent::WaterDetected);
    assert_eq!(fsm.state(), FountainState::Pumping);

    fsm.transition(FountainEvent::PowerOff);
    assert_eq!(fsm.state(), FountainState::Idle);
}
