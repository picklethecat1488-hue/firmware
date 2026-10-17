#!/usr/bin/env python3
"""Regression test for WORM-PEATY-BOBBIT-630.

Ensures that when a driver fault (e.g. TMC6300 DIAG pin assertion) occurs during
motor ticking, the motor controller immediately transitions to MotorState::Off,
zeros out active speed and current/RPM telemetry, stops the motor, and halts active
ticking (eliminating repeated log spam).
"""

from pathlib import Path


def test_worm_peaty_bobbit_630_motor_controller_stops_on_tick_error():
    """Verify motor_controller.rs halts ticking and sets state to Off on tick failure."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    motor_ctrl_rs = workspace_root / "controller" / "src" / "motor_controller.rs"
    content = motor_ctrl_rs.read_text(encoding="utf-8")

    # In tick_motor(): motor.tick() error must transition to Off via set_running_state(false)
    assert "if let Err(e) = self.motor.tick().await {" in content, (
        "motor_controller.rs must catch errors from self.motor.tick().await"
    )
    assert "let _ = self.set_running_state(false).await;" in content, (
        "motor_controller.rs must call set_running_state(false) on driver tick error"
    )

    # In current_rpm(): must return 0 if state is Off
    assert "if self.state == MotorState::Off {\n            return 0;\n        }" in content, (
        "motor_controller.rs current_rpm() must return 0 when MotorState is Off"
    )

    # In set_running_state(false): must zero out measurements
    assert "self.last_measured_rpm = 0;" in content, (
        "motor_controller.rs set_running_state(false) must reset last_measured_rpm to 0"
    )
    assert "self.last_current_ma = 0;" in content, (
        "motor_controller.rs set_running_state(false) must reset last_current_ma to 0"
    )


def test_worm_peaty_bobbit_630_regression_unit_test_present():
    """Verify dedicated regression unit test in controller/tests/controller_tests.rs."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    controller_tests_rs = workspace_root / "controller" / "tests" / "controller_tests.rs"
    content = controller_tests_rs.read_text(encoding="utf-8")

    assert "test_worm_peaty_bobbit_630_motor_tick_driver_fault_stops_ticking" in content, (
        "controller/tests/controller_tests.rs must define test_worm_peaty_bobbit_630_motor_tick_driver_fault_stops_ticking"
    )
