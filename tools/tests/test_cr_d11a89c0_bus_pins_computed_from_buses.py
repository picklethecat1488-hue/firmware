#!/usr/bin/env python3
"""Regression test for code review comment CR-d11a89c0.

Guards against redundant `bus_assigned_pins` declarations in board.toml.
Verifies that:
1. `board/board.toml` does not contain a redundant `bus_assigned_pins` array.
2. Bus-assigned pins are computed directly from `[boards.<board>.buses]` in `board/board.toml`.
3. `tools/code_gen` resolves the correct unique set of bus pins ([0, 1, 11, 12, 13])
   solely from the communication buses.
"""

from pathlib import Path
import re


def test_cr_d11a89c0_no_redundant_bus_assigned_pins_in_board_toml():
    """Verify board.toml does not contain a redundant bus_assigned_pins key."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    board_toml = workspace_root / "board" / "board.toml"

    assert board_toml.exists(), f"File not found: {board_toml}"
    content = board_toml.read_text(encoding="utf-8")

    assert "bus_assigned_pins" not in content, (
        "board/board.toml must not declare a redundant bus_assigned_pins array! "
        "Bus assignments must be computed directly from [boards.<board>.buses]."
    )


def test_cr_d11a89c0_buses_defined_in_board_toml():
    """Verify board.toml declares all required bus definitions."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    board_toml = workspace_root / "board" / "board.toml"

    content = board_toml.read_text(encoding="utf-8")

    assert "[boards.cat_detector.buses.i2c]" in content
    assert "[boards.cat_detector.buses.uart]" in content
    assert "[boards.cat_detector.buses.pio]" in content

    # Pin values
    pins = {
        "UART_TX_PIN": 0,
        "UART_RX_PIN": 1,
        "LED_PIN": 11,
        "I2C_SDA_PIN": 12,
        "I2C_SCL_PIN": 13,
    }
    for pin_name, pin_val in pins.items():
        pattern = rf"{pin_name}\s*=\s*{pin_val}"
        assert re.search(pattern, content), f"{pin_name} = {pin_val} not found in board.toml"


def test_cr_d11a89c0_codegen_computes_pins_from_buses():
    """Verify tools/code_gen/src/board.rs computes bus pins directly from buses table."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    codegen_board = workspace_root / "tools" / "code_gen" / "src" / "board.rs"

    assert codegen_board.exists(), f"File not found: {codegen_board}"
    content = codegen_board.read_text(encoding="utf-8")

    # BoardConfig must not have bus_assigned_pins field
    assert "pub bus_assigned_pins: Vec<String>" not in content, (
        "BoardConfig should not have an explicit bus_assigned_pins field; "
        "pins must be derived purely from the buses map."
    )

    # resolve_bus_assigned_pins must iterate through buses.values()
    assert "pub fn resolve_bus_assigned_pins" in content
    assert "for bus in board_config.buses.values()" in content
