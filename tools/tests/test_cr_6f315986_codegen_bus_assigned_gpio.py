#!/usr/bin/env python3
"""Regression test for code review comment CR-6f315986.

Guards against hardcoding bus-assigned GPIO mappings inside platform code.
Verifies that:
1. `board/board.toml` declaratively marks GPIOs as bus-assigned.
2. `tools/code_gen` code-generates the bus-assigned GPIO mapping.
3. `platform/src/io/gpio.rs` includes the code-generated mapping rather than
   hardcoding matches!(pin, 0 | 1 | 11 | 12 | 13).
4. `Board` in the BSP exposes `BUS_ASSIGNED_PINS` and `is_bus_assigned`.
"""

from pathlib import Path


def test_cr_6f315986_no_hardcoded_gpio_matching_in_platform():
    """Verify platform/src/io/gpio.rs does not hardcode pin numbers."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    gpio_rs = workspace_root / "platform" / "src" / "io" / "gpio.rs"

    assert gpio_rs.exists(), f"File not found: {gpio_rs}"
    content = gpio_rs.read_text(encoding="utf-8")

    # Invariant 1: No hardcoded pin matching ladder
    assert "matches!(pin, 0 | 1" not in content, (
        "platform/src/io/gpio.rs must not hardcode bus-assigned GPIO pin patterns! "
        "These must be code-generated from board.toml."
    )

    # Invariant 2: Includes generated_gpio.rs
    assert 'include!(concat!(env!("OUT_DIR"), "/generated_gpio.rs"));' in content, (
        "platform/src/io/gpio.rs must include generated_gpio.rs generated from board.toml."
    )

    # Invariant 3: Calls is_bus_assigned directly; is_alternate_function is purged
    assert "is_alternate_function" not in content, (
        "is_alternate_function shim must be deleted; use is_bus_assigned directly."
    )
    assert "is_bus_assigned(p)" in content, "platform/src/io/gpio.rs must call is_bus_assigned directly."


def test_cr_6f315986_board_toml_marks_bus_assigned():
    """Verify board.toml marks GPIOs as bus-assigned via communication buses."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    board_toml = workspace_root / "board" / "board.toml"

    assert board_toml.exists(), f"File not found: {board_toml}"
    content = board_toml.read_text(encoding="utf-8")

    assert "[boards.cat_detector.buses.i2c]" in content, "board/board.toml must define i2c bus."
    assert "[boards.cat_detector.buses.uart]" in content, "board/board.toml must define uart bus."
    assert "[boards.cat_detector.buses.pio]" in content, "board/board.toml must define pio bus."
    assert "UART_TX_PIN" in content
    assert "UART_RX_PIN" in content
    assert "LED_PIN" in content
    assert "I2C_SDA_PIN" in content
    assert "I2C_SCL_PIN" in content


def test_cr_6f315986_codegen_generates_gpio_mapping():
    """Verify tools/code_gen provides GPIO generation utilities and templates."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    codegen_board = workspace_root / "tools" / "code_gen" / "src" / "board.rs"
    gpio_template = workspace_root / "platform" / "templates" / "generated_gpio.rs.jinja"
    board_template = workspace_root / "board" / "templates" / "generated_board.rs.jinja"

    assert codegen_board.exists(), f"File not found: {codegen_board}"
    assert gpio_template.exists(), f"File not found: {gpio_template}"
    assert board_template.exists(), f"File not found: {board_template}"

    content = codegen_board.read_text(encoding="utf-8")
    assert "pub fn generate_gpio_definitions" in content, (
        "tools/code_gen/src/board.rs must define generate_gpio_definitions."
    )
    assert "pub fn resolve_bus_assigned_pins" in content, (
        "tools/code_gen/src/board.rs must define resolve_bus_assigned_pins."
    )

    board_tmpl_content = board_template.read_text(encoding="utf-8")
    assert "BUS_ASSIGNED_PINS" in board_tmpl_content, (
        "board/templates/generated_board.rs.jinja must define BUS_ASSIGNED_PINS."
    )
    assert "pub fn is_bus_assigned" in board_tmpl_content, (
        "board/templates/generated_board.rs.jinja must define is_bus_assigned."
    )
