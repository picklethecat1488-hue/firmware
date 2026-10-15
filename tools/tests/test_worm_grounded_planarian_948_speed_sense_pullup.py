#!/usr/bin/env python3
"""Regression test for WORM-GROUNDED-PLANARIAN-948.

Verifies that the digital latching Hall speed sense pin (GP26) is properly
configured with internal pull-up enabled, and that GPIO diagnostic CLI handlers
do not steal pins or wipe pad control registers (such as clearing pue/Pull::Up),
preventing open-drain sensor signals from floating or grounding to ~0.29V.
"""

from pathlib import Path


def test_worm_grounded_planarian_948_speed_sense_pin_pullup_configured():
    """Verify that SPEED_SENSE_PIN is configured as input with Pull::Up in Board::init."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    bsp_path = workspace_root / "board" / "src" / "cat_detector.rs"

    assert bsp_path.exists(), f"File not found: {bsp_path}"
    content = bsp_path.read_text(encoding="utf-8")

    # Invariant 1: Board::SPEED_SENSE_PIN mapped in gpio_pins
    assert "Board::SPEED_SENSE_PIN => PIN_26" in content, (
        "board/src/cat_detector.rs must map Board::SPEED_SENSE_PIN to PIN_26."
    )

    # Invariant 2: Configured as input with Pull::Up
    assert "speed_pin.set_as_input()" in content, (
        "board/src/cat_detector.rs must configure speed_pin as input."
    )
    assert "speed_pin.set_pull(Pull::Up)" in content, (
        "board/src/cat_detector.rs must enable Pull::Up on speed_pin for open-drain Hall sensor."
    )


def test_worm_grounded_planarian_948_no_pad_ctrl_clobbering_in_gpio_cli():
    """Verify that handle_gpio_cli does not modify pad control registers or wipe pull-ups."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    gpio_rs_path = workspace_root / "platform" / "src" / "io" / "gpio.rs"

    assert gpio_rs_path.exists(), f"File not found: {gpio_rs_path}"
    content = gpio_rs_path.read_text(encoding="utf-8")

    # Invariant 3: No pad_ctrl write in GPIO CLI status/read handlers
    assert "pad_ctrl" not in content, (
        "platform/src/io/gpio.rs must not write to pad_ctrl registers during diagnostic reads."
    )

    # Invariant 4: Non-invasive SIO register read
    assert "rp_pac::SIO.gpio_in" in content, (
        "platform/src/io/gpio.rs must perform non-destructive SIO register reads."
    )
