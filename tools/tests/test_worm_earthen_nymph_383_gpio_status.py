#!/usr/bin/env python3
"""Regression test for WORM-EARTHEN-NYMPH-383.

Verifies that GPIO diagnostic CLI handlers in platform/src/io/gpio.rs read GPIO
states non-destructively through the RP2040 SIO block rather than stealing pins
via AnyPin::steal / Flex::new, which previously clobbered peripheral function
selection (FUNCSEL) on active buses such as I2C0 (GP12/13), breaking SensorController.
"""

from pathlib import Path


def test_worm_earthen_nymph_383_no_pin_stealing_in_gpio_cli():
    """Verify that handle_gpio_cli does not steal pins or reconfigure FUNCSEL."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    gpio_rs_path = workspace_root / "platform" / "src" / "io" / "gpio.rs"

    assert gpio_rs_path.exists(), f"File not found: {gpio_rs_path}"
    content = gpio_rs_path.read_text(encoding="utf-8")

    # Invariant 1: No raw AnyPin::steal or Flex::new in GPIO CLI handler
    assert "AnyPin::steal" not in content, (
        "platform/src/io/gpio.rs must not use AnyPin::steal! "
        "Stealing pins clobbers FUNCSEL and breaks active peripherals like I2C."
    )
    assert "steal_flex" not in content, (
        "platform/src/io/gpio.rs must not define or use steal_flex macro."
    )
    assert "Flex::new" not in content, (
        "platform/src/io/gpio.rs must not wrap pins in Flex::new."
    )

    # Invariant 2: Direct non-destructive register read via rp_pac SIO
    assert "rp_pac::SIO.gpio_in" in content, (
        "platform/src/io/gpio.rs must read GPIO levels non-destructively via rp_pac::SIO.gpio_in."
    )

    # Invariant 3: Helper is_pin_high function is defined and exported
    assert "pub fn is_pin_high" in content, (
        "platform/src/io/gpio.rs must expose is_pin_high for logic level testing."
    )
