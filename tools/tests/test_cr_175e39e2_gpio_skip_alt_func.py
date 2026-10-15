#!/usr/bin/env python3
"""Regression test for code review comment CR-175e39e2.

Guards against reintroducing `rp-pac` into `platform` crate dependencies or
calling into `rp_pac` in `platform/src/io/gpio.rs`. Verifies that GPIO diagnostic
handlers detect and skip pins configured for alternate (non-GPIO) peripheral functions
(e.g., UART0, PIO0, I2C0) to avoid pulling in vendor PAC crates.
"""

from pathlib import Path


def test_cr_175e39e2_no_rp_pac_in_platform():
    """Verify platform/Cargo.toml and platform/src/io/gpio.rs have no rp-pac dependency."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cargo_toml = workspace_root / "platform" / "Cargo.toml"
    gpio_rs = workspace_root / "platform" / "src" / "io" / "gpio.rs"

    assert cargo_toml.exists(), f"File not found: {cargo_toml}"
    assert gpio_rs.exists(), f"File not found: {gpio_rs}"

    cargo_content = cargo_toml.read_text(encoding="utf-8")
    assert "rp-pac" not in cargo_content, "platform/Cargo.toml must not depend directly on rp-pac! (CR-175e39e2)"

    gpio_content = gpio_rs.read_text(encoding="utf-8")
    assert "rp_pac" not in gpio_content, "platform/src/io/gpio.rs must not import or reference rp_pac! (CR-175e39e2)"


def test_cr_175e39e2_skips_alternate_function_gpio():
    """Verify handle_gpio_cli detects and skips pins configured for alternate functions."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    gpio_rs = workspace_root / "platform" / "src" / "io" / "gpio.rs"

    content = gpio_rs.read_text(encoding="utf-8")

    assert "is_bus_assigned(p)" in content, (
        "platform/src/io/gpio.rs must inspect is_bus_assigned before reading logic levels."
    )
    assert 'let _ = writeln!(writer, "GP{:<2}  Alt", p);' in content, (
        "platform/src/io/gpio.rs must label alternate function pins as 'Alt' in status output."
    )
    assert "is configured for alternate function (not GPIO)" in content, (
        "platform/src/io/gpio.rs must inform user when attempting to read an alternate function pin."
    )


def test_cr_175e39e2_no_raw_memory_map_in_gpio_rs():
    """Verify platform/src/io/gpio.rs does not assume the RP2040 memory map."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    gpio_rs = workspace_root / "platform" / "src" / "io" / "gpio.rs"

    content = gpio_rs.read_text(encoding="utf-8")

    assert "0x4001" not in content, "platform/src/io/gpio.rs must not hardcode RP2040 IO_BANK0 memory map addresses."
    assert "0xd000" not in content, "platform/src/io/gpio.rs must not hardcode RP2040 SIO memory map addresses."
    assert "read_volatile" not in content, (
        "platform/src/io/gpio.rs must rely on Embassy driver abstractions rather than raw volatile reads."
    )
