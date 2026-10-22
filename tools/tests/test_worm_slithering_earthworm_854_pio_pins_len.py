#!/usr/bin/env python3
"""Regression test for WORM-SLITHERING-EARTHWORM-854: Bootloop panic: assertion failed: pins.len() <= 5.

In embassy-rp, `Config::set_set_pins` enforces `assert!(pins.len() <= 5)` because
the RP2040 PIO `SET` instruction format only has a 5-bit immediate field (0..31),
meaning a state machine can only drive at most 5 SET pins.

The TMC6300 6-PWM driver controls 6 half-bridge pins (GP16..GP21).
The PIO program uses `out pins, 8` and `mov pins, null`, which operate exclusively on OUT pins.
Calling `cfg.set_set_pins(&pins)` with 6 pins trips the panic at startup, causing a bootloop.

This test asserts that `cfg.set_set_pins` is never called on the 6-pin bundle in `peripheral/src/tmc6300.rs`.
"""

from pathlib import Path


def test_worm_slithering_earthworm_854_no_set_set_pins_on_6_pins():
    """Verify peripheral/src/tmc6300.rs does not call set_set_pins with 6 pins."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    tmc6300_rs = workspace_root / "peripheral" / "src" / "tmc6300.rs"

    content = tmc6300_rs.read_text(encoding="utf-8")

    assert "cfg.set_set_pins" not in content, (
        "peripheral/src/tmc6300.rs must not call cfg.set_set_pins on 6 pins; "
        "RP2040 PIO SET pins are limited to at most 5 pins (pins.len() <= 5)."
    )
    assert "cfg.set_out_pins(&pins);" in content, (
        "peripheral/src/tmc6300.rs must configure OUT pins via cfg.set_out_pins(&pins)."
    )
