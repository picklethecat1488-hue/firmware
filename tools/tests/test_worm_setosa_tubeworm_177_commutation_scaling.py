#!/usr/bin/env python3
"""Regression functional test for WORM-SETOSE-TUBEWORM-177.

Ensures that:
1. Motor controller scales open-loop commutation step rate with commanded speed
   and executes a soft-start acceleration ramp from startup intervals to target intervals.
2. HallSensor periodically computes live RPM from accumulated pulse counts every 100ms.
"""

from pathlib import Path
import subprocess


def test_worm_setosa_tubeworm_177_commutation_scaling_and_hall_rpm():
    """Verify commutation scaling, soft-start ramp, and Hall sensor live RPM calculation."""
    workspace_root = Path(__file__).resolve().parent.parent.parent

    # 1. Run controller integration test for commutation scaling and soft-start ramp
    cmd_controller = [
        "cargo",
        "test",
        "--package",
        "controller",
        "--test",
        "controller_tests",
        "test_worm_setosa_tubeworm_177_commutation_step_scaling_and_soft_start_ramp",
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
    assert (
        "test test_worm_setosa_tubeworm_177_commutation_step_scaling_and_soft_start_ramp ... ok"
        in proc_ctrl.stdout
    )

    # 2. Run peripheral integration test for Hall sensor pulse counting and RPM calculation
    cmd_peripheral = [
        "cargo",
        "test",
        "--package",
        "peripheral",
        "--test",
        "integration",
        "test_hall_sensor_pulse_counting_and_rpm",
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
    assert "test test_hall_sensor_pulse_counting_and_rpm ... ok" in proc_periph.stdout
