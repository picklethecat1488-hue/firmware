#!/usr/bin/env python3
"""Reproduction and regression functional test for WORM-PROBOSCID-ROUNDWORM-8.

Asserts that:
1. The motor driver's default tick interval (DEFAULT_TICK_INTERVAL_US) is 10,000 µs (10 ms),
   ensuring the open-loop commutation step timing (10 ticks = 100 ms) allows the physical
   BLDC motor to lock into synchronism from standstill without stalling at 0 RPM.
2. Tmc6300 and Tmc6300Pio drivers execute Step 0 upon the first tick after stop(), rather
   than pre-incrementing past Step 0 and skipping it.
3. MotorController initializes step_tick_counter to 0 upon start, holding Step 0 for the
   full startup commutation interval to permit magnetic rotor alignment.
4. startup_blanking_ticks is configured for 300 ticks (3.0 seconds at 100 Hz).
"""

from pathlib import Path
import subprocess


def test_worm_proboscid_roundworm_8_motor_spin():
    """Verify motor commutation startup timing, Step 0 execution, and rotor alignment."""
    workspace_root = Path(__file__).resolve().parent.parent.parent

    # 1. Run controller regression unit tests for startup alignment and commutation timing
    cmd_controller = [
        "cargo",
        "test",
        "--package",
        "controller",
        "--test",
        "controller_tests",
        "test_worm_proboscid_roundworm_8",
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
        "test test_worm_proboscid_roundworm_8_startup_commutation_and_alignment ... ok"
        in proc_ctrl.stdout
    )

    # 2. Run peripheral regression unit tests for driver Step 0 execution on first tick
    cmd_peripheral = [
        "cargo",
        "test",
        "--package",
        "peripheral",
        "--test",
        "integration",
        "test_worm_proboscid_roundworm_8",
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
    assert (
        "test test_worm_proboscid_roundworm_8_driver_first_tick_executes_step_0 ... ok"
        in proc_periph.stdout
    )
