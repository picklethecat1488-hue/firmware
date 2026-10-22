#!/usr/bin/env python3
"""Regression functional test for code review comment CR-7f321327.

Asserts that:
1. The motor controller's tick interval is kept in sync with the motor driver's tick interval.
2. The motor driver (TMC6300) defines its required tick interval (500µs / 2kHz).
3. The Tickable trait provides `tick_interval_us(&self) -> u32` enabling drivers to dictate timing.
4. MotorController dynamically queries the driver's tick interval for its ticker.
"""

from pathlib import Path
import subprocess


def test_cr_7f321327_motor_driver_tick_sync():
    """Verify motor driver tick interval synchronization across peripheral and controller."""
    workspace_root = Path(__file__).resolve().parent.parent.parent

    # 1. Run peripheral driver tick interval unit tests
    cmd_peripheral = [
        "cargo",
        "test",
        "--package",
        "peripheral",
        "--test",
        "integration",
        "test_cr_7f321327",
        "--",
        "--nocapture",
    ]
    proc_periph = subprocess.run(
        cmd_peripheral,
        cwd=workspace_root,
        capture_output=True,
        text=True,
        check=False,
    )
    assert proc_periph.returncode == 0, (
        f"cargo test peripheral failed with return code {proc_periph.returncode}:\n"
        f"STDOUT:\n{proc_periph.stdout}\nSTDERR:\n{proc_periph.stderr}"
    )

    # 2. Run controller tick interval synchronization unit tests
    cmd_controller = [
        "cargo",
        "test",
        "--package",
        "controller",
        "--test",
        "controller_tests",
        "test_cr_7f321327",
        "--",
        "--nocapture",
    ]
    proc_ctrl = subprocess.run(
        cmd_controller,
        cwd=workspace_root,
        capture_output=True,
        text=True,
        check=False,
    )
    assert proc_ctrl.returncode == 0, (
        f"cargo test controller failed with return code {proc_ctrl.returncode}:\n"
        f"STDOUT:\n{proc_ctrl.stdout}\nSTDERR:\n{proc_ctrl.stderr}"
    )
