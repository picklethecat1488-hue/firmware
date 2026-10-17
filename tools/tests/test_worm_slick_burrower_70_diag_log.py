#!/usr/bin/env python3
"""Reproduction test for WORM-SLICK-BURROWER-70:
We should log when DIAG pin gets asserted in the TMC6300 driver tick call.
"""

from pathlib import Path


def test_worm_slick_burrower_70_diag_assertion_logged_in_tick():
    """Verify that DIAG assertion in tick() emits a structured error log."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    tmc6300_rs = workspace_root / "peripheral" / "src" / "tmc6300.rs"
    content = tmc6300_rs.read_text(encoding="utf-8")

    # In Tmc6300::tick
    assert "TMC6300 DIAG pin asserted" in content, (
        "peripheral/src/tmc6300.rs must log an error when DIAG pin is asserted in Tmc6300::tick."
    )

    # In Tmc6300Pio::tick
    assert "TMC6300 PIO DIAG pin asserted" in content, (
        "peripheral/src/tmc6300.rs must log an error when DIAG pin is asserted in Tmc6300Pio::tick."
    )
