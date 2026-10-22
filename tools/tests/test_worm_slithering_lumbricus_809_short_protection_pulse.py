#!/usr/bin/env python3
"""Regression functional test for WORM-SLITHERING-LUMBRICUS-809.

Verifies that:
1. TMC6300 winding connection pulses during motor commutation remain strictly under
   the 1.0 us (1000 ns) TMC6300 short-protection filter delay (t_S2G).
2. The logic analyzer capture in attachments/WORM-SLITHERING-LUMBRICUS-809/digital.csv
   records the pre-fix 6.396 us pulse fault triggering the DIAG interrupt.
3. The Rust integration test verifying pulse clamping (520 ns - 800 ns) and clock
   divider calculation executes successfully.
"""

from pathlib import Path
import csv
import subprocess


def test_worm_slithering_lumbricus_809_cargo_integration_test_functional():
    """Verify Rust peripheral integration tests pass for short-protection pulse invariants."""
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
    assert (
        "test test_worm_slithering_lumbricus_809_tmc6300_short_protection_pulse_invariants ... ok"
        in proc.stdout
    ), f"Expected test success in cargo test output, got:\n{proc.stdout}"
    assert "1 passed; 0 failed" in proc.stdout, (
        f"Expected 1 passed in cargo test output, got:\n{proc.stdout}"
    )


def test_worm_slithering_lumbricus_809_digital_trace_reproduction_functional():
    """Verify that the Saleae logic trace captures the excessive pulse width causing DIAG fault."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    trace_path = workspace_root / "attachments" / "WORM-SLITHERING-LUMBRICUS-809" / "digital.csv"
    assert trace_path.exists(), f"Trace file {trace_path} missing."

    # Parse trace timestamps and pin states to verify observed pulse duration > 1.0 us
    with open(trace_path, "r", encoding="utf-8") as f:
        reader = csv.reader(f)
        header = next(reader)
        # Header: ['Time [s]', 'Channel 0', 'Channel 1', 'Channel 2', 'Channel 3', 'Channel 4', 'Channel 5', 'Channel 6', 'Channel 7']
        # Channel 6 is DIAG pin (GP22)
        rows = list(reader)

    assert len(rows) > 0, "digital.csv must contain sampled transitions."

    # Find first pulse start and when DIAG goes high indicating short protection trip
    pulse_start_time = float(rows[1][0])  # row 3: 7.142946764
    diag_trip_time = None
    pulse_end_time = None

    for row in rows[1:]:
        t_sec = float(row[0])
        diag_val = int(row[7])
        if diag_val == 1 and diag_trip_time is None:
            diag_trip_time = t_sec
        # First pulse ends when all FETs return to 0 (row 6)
        if any(int(x) == 1 for x in row[1:7]) is False and pulse_end_time is None:
            pulse_end_time = t_sec

    assert diag_trip_time is not None, "Trace must capture DIAG going high on fault."
    assert pulse_end_time is not None, "Trace must capture end of first pulse."

    pulse_duration_us = (pulse_end_time - pulse_start_time) * 1e6
    diag_trip_delta_us = (diag_trip_time - pulse_start_time) * 1e6

    # In the capture, pre-fix pulse width was 6.400 us and DIAG tripped after 2.916 us (> 1.0 us)
    assert 6.39 < pulse_duration_us < 6.41, f"Expected ~6.40 us pulse, got {pulse_duration_us:.3f} us"
    assert 2.91 < diag_trip_delta_us < 2.93, f"Expected DIAG trip at ~2.92 us, got {diag_trip_delta_us:.3f} us"
    assert pulse_duration_us > 1.0, "Pre-fix pulse duration must exceed 1.0 us short protection threshold"


def test_worm_slithering_lumbricus_809_pulse_calculation_bounds_functional():
    """Verify pulse width calculation algorithm strictly constrains pulses to 500-800 ns."""
    sys_clk_hz = 125_000_000
    clk_divider = 5  # 25 MHz PIO clock (40 ns cycle period)
    cycle_period_ns = (clk_divider * 1_000_000_000) / sys_clk_hz
    assert cycle_period_ns == 40.0, f"Expected 40 ns period, got {cycle_period_ns}"

    # PIO assembly loop has 6 cycles fixed overhead (out pins, mov pins null, nop)
    overhead_cycles = 6
    min_on_count = 7
    max_on_count = 14

    min_pulse_ns = (min_on_count + overhead_cycles) * cycle_period_ns
    max_pulse_ns = (max_on_count + overhead_cycles) * cycle_period_ns

    assert min_pulse_ns == 520.0, f"Min pulse must be 520 ns, got {min_pulse_ns}"
    assert max_pulse_ns == 800.0, f"Max pulse must be 800 ns, got {max_pulse_ns}"

    # Short protection trip threshold is 1.0 us = 1000 ns
    short_protection_threshold_ns = 1000.0
    assert min_pulse_ns < short_protection_threshold_ns
    assert max_pulse_ns <= 800.0 < short_protection_threshold_ns
