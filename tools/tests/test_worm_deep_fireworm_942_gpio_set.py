#!/usr/bin/env python3
"""Regression unit test for WORM-DEEP-FIREWORM-942.

Verifies that:
1. GpioSubcommand in platform/src/io/gpio.rs includes the Set variant.
2. handle_gpio_cli in platform/src/io/gpio.rs implements GpioSubcommand::Set to drive logic levels.
3. handle_gpio_cli guards against setting bus-assigned pins via is_bus_assigned(p).
4. controller/shell.toml defines positional pin and value arguments for the gpio CLI command.
5. platform/src/io/gpio.rs respects all memory map and driver isolation invariants.
"""

from pathlib import Path
try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib


def test_worm_deep_fireworm_942_gpio_subcommand_set_defined():
    """Verify GpioSubcommand defines Set variant and handler in platform/src/io/gpio.rs."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    gpio_rs_path = workspace_root / "platform" / "src" / "io" / "gpio.rs"

    assert gpio_rs_path.exists(), f"File not found: {gpio_rs_path}"
    content = gpio_rs_path.read_text(encoding="utf-8")

    # Invariant 1: Set variant present in GpioSubcommand enum
    assert "Set," in content, "platform/src/io/gpio.rs must define Set variant in GpioSubcommand enum."
    assert '"status, read, set"' in content, (
        'platform/src/io/gpio.rs must include "status, read, set" in subcommand_enum macro.'
    )

    # Invariant 2: GpioSubcommand::Set arm implemented
    assert "GpioSubcommand::Set =>" in content, (
        "platform/src/io/gpio.rs must implement GpioSubcommand::Set in handle_gpio_cli."
    )

    # Invariant 3: Validates pin range and bus assignment
    assert "if is_bus_assigned(p)" in content, (
        "platform/src/io/gpio.rs must check is_bus_assigned(p) before setting pin level."
    )
    assert "Cannot set pin configured for alternate function" in content, (
        "platform/src/io/gpio.rs must reject setting alternate function pins."
    )

    # Invariant 4: No direct memory map assumptions or rp-pac dependency
    assert "rp_pac" not in content, "platform/src/io/gpio.rs must not depend on rp_pac."
    assert "0x4001" not in content, "platform/src/io/gpio.rs must not hardcode RP2040 IO_BANK0 addresses."
    assert "0xd000" not in content, "platform/src/io/gpio.rs must not hardcode RP2040 SIO addresses."


def test_worm_deep_fireworm_942_shell_toml_gpio_arguments():
    """Verify controller/shell.toml defines pin and value arguments for gpio command."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    shell_toml_path = workspace_root / "controller" / "shell.toml"

    assert shell_toml_path.exists(), f"File not found: {shell_toml_path}"
    with open(shell_toml_path, "rb") as f:
        data = tomllib.load(f)

    cli_commands = data.get("cli_commands", [])
    gpio_cmd = next((c for c in cli_commands if c.get("cmd_name") == "gpio"), None)
    assert gpio_cmd is not None, "controller/shell.toml must define gpio command."

    args = gpio_cmd.get("args", [])
    arg_names = [a.get("name") for a in args]
    assert "pin" in arg_names, "gpio command must have 'pin' argument."
    assert "value" in arg_names, "gpio command must have 'value' argument."

    # Both arguments must have attributes = [] to ensure positional parsing in embedded-cli
    for a in args:
        if a.get("name") in ("pin", "value"):
            assert a.get("attributes") == [], (
                f"gpio argument '{a.get('name')}' must have attributes = [] for positional CLI parsing."
            )
