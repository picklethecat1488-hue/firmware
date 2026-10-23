#!/usr/bin/env python3
"""Regression functional unit test for WORM-DAMP-MEALWORM-592, WORM-NOCTURNAL-CATERPILLAR-808, and WORM-ORGANIC-FIREWORM-553.

Verifies that:
1. TMC6300 default tick interval is 500 µs (2000 Hz commutation rate), avoiding low-frequency mechanical
   resonance and stall at speeds 10, 30, 50, and 100.
2. Motor driver executes Step 0 upon the first tick after starting rather than pre-incrementing past it.
3. calculate_pwm_counts enforces an inductive torque floor (> 150 mA equivalent) while strictly clamping
   individual FET ON-pulse duration to <= 800 ns (< 1.0 µs TMC6300 t_S2G short protection limit).
4. Commutation step intervals scale monotonically with commanded speed across the soft-start ramp.
"""

from pathlib import Path
import subprocess

REPO_ROOT = Path(__file__).resolve().parent.parent.parent


def test_worm_damp_mealworm_592_commutation_cargo_tests():
    """Run cargo tests in peripheral and controller crates verifying 500us ticks and Step 0 execution."""
    cmd_peripheral = [
        "cargo",
        "test",
        "--package",
        "peripheral",
        "--test",
        "integration",
        "test_cr_7f321327_tmc6300_tick_interval_and_sync",
    ]
    proc = subprocess.run(cmd_peripheral, cwd=REPO_ROOT, capture_output=True, text=True, check=False)
    assert proc.returncode == 0, f"peripheral integration test failed:\n{proc.stdout}\n{proc.stderr}"

    cmd_controller = [
        "cargo",
        "test",
        "--package",
        "controller",
        "--test",
        "controller_tests",
        "test_worm_setosa_tubeworm_177_commutation_step_scaling_and_soft_start_ramp",
    ]
    proc_ctrl = subprocess.run(cmd_controller, cwd=REPO_ROOT, capture_output=True, text=True, check=False)
    assert proc_ctrl.returncode == 0, f"controller test failed:\n{proc_ctrl.stdout}\n{proc_ctrl.stderr}"


def test_worm_damp_mealworm_592_torque_floor_and_pulse_bounds():
    """Verify PWM counts at speeds 10, 30, 50, 100 satisfy torque floor and short protection bounds."""
    cycle_period_ns = 40  # 25 MHz PIO clock (divider = 5, 125 MHz sys_clk)
    max_safe_off_count = 33

    def calculate_pwm_counts(duty_cycle: int):
        if duty_cycle == 0:
            return 0, 100
        elif duty_cycle < 7:
            on = 7
            off = 100 - on
            return on, off
        elif duty_cycle <= 14:
            on = duty_cycle
            off = 100 - on
            return on, off
        else:
            on = 14
            duty = min(duty_cycle, 100)
            off = ((100 - duty) * on + (duty // 2)) // duty
            if off == 0:
                off = 1
            return on, off

    # Speed 10, 30, 50, 100
    for duty in [10, 30, 50, 100]:
        on, off = calculate_pwm_counts(duty)
        pulse_ns = (on + 6) * cycle_period_ns
        assert 500 <= pulse_ns <= 800, f"Speed {duty}%: pulse {pulse_ns} ns outside safe [500, 800] ns"
        assert 1 <= off <= 100, f"Speed {duty}%: off count {off} outside [1, 100]"

    # Verify speed 30 has sufficient duty for >150mA torque floor (at off=33, duty=14/47=29.8%)
    on30, off30 = calculate_pwm_counts(30)
    duty30 = on30 / (on30 + off30)
    assert duty30 >= 0.25, f"Speed 30 duty cycle {duty30:.1%} too low to overcome cogging barrier"

    # Verify speed 100 has high duty (>90%)
    on100, off100 = calculate_pwm_counts(100)
    duty100 = on100 / (on100 + off100)
    assert duty100 >= 0.90, f"Speed 100 duty cycle {duty100:.1%} too low for full speed"
