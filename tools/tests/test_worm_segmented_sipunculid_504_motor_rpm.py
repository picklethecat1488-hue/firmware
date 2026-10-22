#!/usr/bin/env python3
"""Regression functional test for WORM-SEGMENTED-SIPUNCULID-504.

Asserts that:
1. HallSensor retains non-zero RPM between slow pulses and does not collapse to 0 RPM
   during measurement windows when pulse rate is low (e.g. 166 RPM).
2. MotorCommand::ReadRpm actively samples the speed sensor and updates measured RPM.
3. Motor commutation tick rate operates at sub-millisecond intervals (500µs) to support
   operational speeds (thousands of RPM) rather than crawling at 166 RPM.
"""

from pathlib import Path
import subprocess


def test_worm_segmented_sipunculid_504_motor_rpm():
    """Verify motor RPM detection and commutation step scaling."""
    workspace_root = Path(__file__).resolve().parent.parent.parent

    # 1. Run controller regression unit tests
    cmd_controller = [
        "cargo",
        "test",
        "--package",
        "controller",
        "--test",
        "controller_tests",
        "test_worm_segmented_sipunculid_504",
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

    # 2. Run peripheral regression unit tests
    cmd_peripheral = [
        "cargo",
        "test",
        "--package",
        "peripheral",
        "--test",
        "integration",
        "test_worm_segmented_sipunculid_504",
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
