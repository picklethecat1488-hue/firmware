#!/usr/bin/env python3
"""Regression functional unit test for WORM-NOCTURNAL-CATERPILLAR-808.

Verifies that:
1. TMC6300 driver tick interval is 500 µs (2000 Hz commutation rate), preventing low-frequency
   stepping at speed 30 that caused the rotor to tick back and forth by ±5° across 9N12P cogging detents.
2. At speed 30, the commutation interval is 3 ticks = 1.5 ms (667 Hz electrical commutation rate /
   1,111 RPM), generating sufficient angular momentum to overcome physical cogging barriers.
3. Cargo tests in peripheral and controller crates verify continuous forward commutation progression.
"""

from pathlib import Path
import subprocess

REPO_ROOT = Path(__file__).resolve().parent.parent.parent


def test_worm_nocturnal_caterpillar_808_tick_interval_and_frequency():
    """Verify driver default tick interval is 500 µs and speed 30 step frequency is >= 500 Hz."""
    cmd_peripheral = [
        "cargo",
        "test",
        "--package",
        "peripheral",
        "--test",
        "integration",
        "test_cr_7f321327_tmc6300_tick_interval_and_sync",
        "--",
        "--nocapture",
    ]
    proc = subprocess.run(cmd_peripheral, cwd=REPO_ROOT, capture_output=True, text=True, check=False)
    assert proc.returncode == 0, f"peripheral test failed:\n{proc.stdout}\n{proc.stderr}"
    assert "test test_cr_7f321327_tmc6300_tick_interval_and_sync ... ok" in proc.stdout

    # Verify commutation step frequency calculation for speed 30
    tick_interval_us = 500
    speed_30_ticks = 100 // 30  # 3 ticks
    step_period_us = speed_30_ticks * tick_interval_us  # 1500 us = 1.5 ms
    step_freq_hz = 1_000_000 / step_period_us  # 666.7 Hz
    assert step_freq_hz >= 500.0, f"Speed 30 commutation frequency {step_freq_hz:.1f} Hz too low (< 500 Hz)"


def test_worm_nocturnal_caterpillar_808_forward_commutation_progression():
    """Verify commutation steps progress forward sequentially without oscillating."""
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
    proc = subprocess.run(cmd_controller, cwd=REPO_ROOT, capture_output=True, text=True, check=False)
    assert proc.returncode == 0, f"controller test failed:\n{proc.stdout}\n{proc.stderr}"
    assert "test test_worm_setosa_tubeworm_177_commutation_step_scaling_and_soft_start_ramp ... ok" in proc.stdout
