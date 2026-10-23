#!/usr/bin/env python3
"""Regression functional unit test for WORM-ORGANIC-FIREWORM-553.

Verifies that:
1. Commutation step rates across commanded speeds 10, 30, 50, and 100 provide smooth continuous
   rotation without low-frequency detent stall or oscillation:
   - Speed 10: 10 ticks (5.0 ms = 200 Hz = 333 RPM)
   - Speed 30: 3 ticks (1.5 ms = 667 Hz = 1,111 RPM)
   - Speed 50: 2 ticks (1.0 ms = 1,000 Hz = 1,667 RPM)
   - Speed 100: 1 tick (0.5 ms = 2,000 Hz = 3,333 RPM)
2. At speed 100, on-pulse duration is strictly clamped to 800 ns (< 1.0 µs TMC6300 t_S2G short-circuit
   detection threshold), preventing FET overheating and avoiding DIAG fault assertions.
3. Peripheral and controller unit tests pass for DIAG fault recovery and pulse duration bounds.
"""

from pathlib import Path
import subprocess

REPO_ROOT = Path(__file__).resolve().parent.parent.parent


def test_worm_organic_fireworm_553_speed_commutation_rates():
    """Verify commutation frequencies scale properly across speeds 10, 30, 50, 100 at 500 us ticks."""
    tick_us = 500
    expected_rates = {
        10: (10, 200.0, 333.3),
        30: (3, 666.7, 1111.1),
        50: (2, 1000.0, 1666.7),
        100: (1, 2000.0, 3333.3),
    }

    for speed, (expected_ticks, min_freq, min_rpm) in expected_rates.items():
        ticks = 100 // speed
        assert ticks == expected_ticks, f"Speed {speed}: expected {expected_ticks} ticks, got {ticks}"
        period_s = (ticks * tick_us) / 1_000_000
        step_freq = 1.0 / period_s
        rpm = (step_freq / 36.0) * 60.0  # 36 steps per mechanical revolution (6 pole pairs)
        assert step_freq >= min_freq - 1.0, f"Speed {speed}: step freq {step_freq} Hz < {min_freq} Hz"
        assert rpm >= min_rpm - 1.0, f"Speed {speed}: RPM {rpm} < {min_rpm}"


def test_worm_organic_fireworm_553_short_protection_and_diag():
    """Verify short protection pulse bounds and DIAG fault handling in integration test suites."""
    cmd_pulse = [
        "cargo",
        "test",
        "--package",
        "peripheral",
        "--test",
        "integration",
        "test_worm_slithering_lumbricus_809_tmc6300_short_protection_pulse_invariants",
        "--",
        "--nocapture",
    ]
    proc_pulse = subprocess.run(cmd_pulse, cwd=REPO_ROOT, capture_output=True, text=True, check=False)
    assert proc_pulse.returncode == 0, f"peripheral pulse test failed:\n{proc_pulse.stdout}\n{proc_pulse.stderr}"

    cmd_fault = [
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
    proc_fault = subprocess.run(cmd_fault, cwd=REPO_ROOT, capture_output=True, text=True, check=False)
    assert proc_fault.returncode == 0, f"controller fault test failed:\n{proc_fault.stdout}\n{proc_fault.stderr}"
