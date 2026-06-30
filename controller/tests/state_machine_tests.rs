use controller::state_machine::{MotorEvent, MotorState, MotorStateMachine};

#[test]
fn test_motor_transitions() {
    let mut fsm = MotorStateMachine::new();
    assert_eq!(fsm.state(), MotorState::Off);

    fsm.transition(MotorEvent::PowerOn);
    assert_eq!(fsm.state(), MotorState::RampUp);

    fsm.transition(MotorEvent::RampComplete);
    assert_eq!(fsm.state(), MotorState::On);

    fsm.transition(MotorEvent::PowerOff);
    assert_eq!(fsm.state(), MotorState::RampDown);

    fsm.transition(MotorEvent::RampComplete);
    assert_eq!(fsm.state(), MotorState::Off);
}
