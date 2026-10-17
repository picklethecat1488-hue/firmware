#!/usr/bin/env python3
"""Regression test for code review comment CR-92c6ac4a.

Verifies that `is_alternate_function` has been completely deleted and
`is_bus_assigned` is called directly throughout `platform/src/io/gpio.rs`.
"""

from pathlib import Path


def test_cr_92c6ac4a_is_alternate_function_deleted():
    """Verify is_alternate_function is deleted and is_bus_assigned is used directly."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    gpio_rs = workspace_root / "platform" / "src" / "io" / "gpio.rs"

    assert gpio_rs.exists(), f"File not found: {gpio_rs}"
    content = gpio_rs.read_text(encoding="utf-8")

    # Invariant 1: No is_alternate_function definition or calls
    assert "is_alternate_function" not in content, (
        "is_alternate_function must be deleted per CR-92c6ac4a; call is_bus_assigned directly!"
    )

    # Invariant 2: Direct call to is_bus_assigned
    assert "is_bus_assigned(p)" in content, "platform/src/io/gpio.rs must call is_bus_assigned directly."
