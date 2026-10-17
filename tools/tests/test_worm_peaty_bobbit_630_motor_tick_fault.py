#!/usr/bin/env python3
"""Regression functional test for WORM-PEATY-BOBBIT-630.

Ensures that when a driver fault (e.g. TMC6300 DIAG pin assertion) occurs during
motor ticking, the motor controller immediately transitions to MotorState::Off,
zeros out active speed and current/RPM telemetry, stops the motor, and halts active
ticking (eliminating repeated log spam).

Rather than checking for the existence of specific statements or source code strings,
this test executes the actual compiled functional integration test suite against the
motor controller implementation.
"""

from pathlib import Path
import subprocess


def test_worm_peaty_bobbit_630_motor_controller_stops_on_tick_error():
    """Verify motor controller halts ticking and transitions to Off on driver fault by executing functional test."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cmd = [
        "cargo",
        "test",
        "--package",
        "controller",
        "--test",
        "controller_tests",
        "test_worm_peaty_bobbit_630_motor_tick_driver_fault_stops_ticking",
        "--",
        "--nocapture",
    ]
    proc = subprocess.run(
        cmd,
        cwd=workspace_root,
        capture_output=True,
        text=True,
        check=False,
    )
    assert proc.returncode == 0, (
        f"cargo test failed with return code {proc.returncode}:\nSTDOUT:\n{proc.stdout}\nSTDERR:\n{proc.stderr}"
    )
    assert "test test_worm_peaty_bobbit_630_motor_tick_driver_fault_stops_ticking ... ok" in proc.stdout, (
        f"Expected test success in cargo test output, got:\n{proc.stdout}"
    )
    assert "1 passed; 0 failed" in proc.stdout, (
        f"Expected 1 passed in cargo test output, got:\n{proc.stdout}"
    )

