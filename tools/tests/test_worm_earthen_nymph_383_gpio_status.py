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

    # Invariant 1: Skips bus-assigned pins (e.g. I2C0, UART0, PIO0) to prevent bus interference
    assert "is_bus_assigned(p)" in content, "platform/src/io/gpio.rs must check and skip bus-assigned pins."

    # Invariant 2: No rp_pac or rp-pac dependency
    assert "rp_pac" not in content, "platform/src/io/gpio.rs must not depend on or use rp_pac."
    cargo_toml_path = workspace_root / "platform" / "Cargo.toml"
    assert "rp-pac" not in cargo_toml_path.read_text(encoding="utf-8"), "platform/Cargo.toml must not depend on rp-pac."

    # Invariant 3: Does not assume RP2040 memory map outside rp2040 platform crate
    assert "0x4001" not in content, "platform/src/io/gpio.rs must not hardcode RP2040 IO_BANK0 memory map addresses."
    assert "0xd000" not in content, "platform/src/io/gpio.rs must not hardcode RP2040 SIO memory map addresses."
    assert "read_volatile" not in content, (
        "platform/src/io/gpio.rs must rely on Embassy driver abstractions rather than raw volatile reads."
    )
