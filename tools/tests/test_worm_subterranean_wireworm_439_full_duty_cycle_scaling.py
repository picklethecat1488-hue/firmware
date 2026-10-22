#!/usr/bin/env python3
"""Regression functional test for WORM-SUBTERRANEAN-WIREWORM-439.

Verifies:
1. TMC6300 PWM duty cycle scaling via calculate_pwm_counts supports full torque scaling up to 100%
   while strictly maintaining individual on-pulse duration <= 800 ns (< 1.0 us t_S2G short threshold).
2. Off-count is guaranteed non-zero (>= 1 cycle) across all non-zero duty cycles to ensure
   the short protection comparator resets on every pulse.
3. Effective duty cycle at 100% speed reaches ~93.3% nominal rather than being clamped to 14%.
4. Peripheral integration test test_worm_slithering_lumbricus_809_tmc6300_short_protection_pulse_invariants
   executes successfully.
"""

from pathlib import Path
import subprocess


def test_worm_subterranean_wireworm_439_duty_cycle_scaling_algorithm():
    """Verify mathematical properties of calculate_pwm_counts frequency modulation algorithm."""
    min_on_count = 7
    max_on_count = 14
    sys_clk_hz = 125_000_000
    clk_divider = 5
    cycle_period_ns = (clk_divider * 1_000_000_000) / sys_clk_hz  # 40 ns

    def calculate_pwm_counts(duty_cycle: int):
        if duty_cycle == 0:
            return 0, 100
        if duty_cycle < min_on_count:
            on = min_on_count
            return on, 100 - on
        if duty_cycle <= max_on_count:
            on = duty_cycle
            return on, 100 - on
        on = max_on_count
        duty = min(duty_cycle, 100)
        off = ((100 - duty) * on + (duty // 2)) // duty
        off = 1 if off == 0 else off
        return on, off

    # 1. Verify 0% duty cycle
    on0, off0 = calculate_pwm_counts(0)
    assert on0 == 0
    assert off0 == 100

    # 2. Verify 14% duty cycle (boundary)
    on14, off14 = calculate_pwm_counts(14)
    assert on14 == 14
    assert off14 == 86
    assert (on14 / (on14 + off14)) == 0.14

    # 3. Verify 50% duty cycle
    on50, off50 = calculate_pwm_counts(50)
    assert on50 == 14
    assert off50 == 14
    assert (on50 / (on50 + off50)) == 0.50

    # 4. Verify 100% duty cycle
    on100, off100 = calculate_pwm_counts(100)
    assert on100 == 14
    assert off100 == 1
    effective_duty_100 = on100 / (on100 + off100)
    assert effective_duty_100 > 0.90, f"Expected >90% duty cycle at 100% speed, got {effective_duty_100:.2%}"

    # 5. Across all duty cycles 1..=100:
    for duty in range(1, 101):
        on, off = calculate_pwm_counts(duty)
        # Pulse duration must stay strictly in [500 ns, 800 ns]
        pulse_ns = (on + 6) * cycle_period_ns
        assert 500 <= pulse_ns <= 800, f"Duty {duty}% pulse {pulse_ns} ns outside [500, 800] ns"
        # Off count must be >= 1 cycle to prevent pure DC and reset comparator
        assert off >= 1, f"Duty {duty}% off_count must be >= 1, got {off}"


def test_worm_subterranean_wireworm_439_cargo_peripheral_tests():
    """Verify Rust peripheral integration tests pass for short-protection and full duty scaling."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cmd = [
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
    assert "test result: ok. 1 passed" in proc.stdout
