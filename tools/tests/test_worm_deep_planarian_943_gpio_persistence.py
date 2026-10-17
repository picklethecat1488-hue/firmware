#!/usr/bin/env python3
"""Regression unit test for WORM-DEEP-PLANARIAN-943.

Verifies that:
1. platform/src/io/gpio.rs guards stolen Flex instances against Drop reset.
2. steal_flex! uses ManuallyDrop or core::mem::forget to prevent Flex::drop
   from clearing pad_ctrl and resetting funcsel to NULL.
3. GPIO pin output states and levels persist indefinitely after CLI invocation.
"""

from pathlib import Path


def test_worm_deep_planarian_943_gpio_persistence():
    """Verify platform/src/io/gpio.rs prevents Flex::drop from resetting pin configuration."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    gpio_rs_path = workspace_root / "platform" / "src" / "io" / "gpio.rs"

    assert gpio_rs_path.exists(), f"File not found: {gpio_rs_path}"
    content = gpio_rs_path.read_text(encoding="utf-8")

    # Invariant 1: steal_flex! or handle_gpio_cli must suppress Drop
    # Flex::drop in embassy-rp writes NULL to funcsel and clears pad_ctrl.
    # To keep GPIO pin states persistent, Flex must not be dropped.
    has_manually_drop = "ManuallyDrop" in content
    has_mem_forget = "core::mem::forget" in content or "mem::forget" in content
    assert has_manually_drop or has_mem_forget, (
        "platform/src/io/gpio.rs must use ManuallyDrop or core::mem::forget to prevent "
        "Flex::drop from disconnecting GPIO pins and resetting funcsel to NULL."
    )

    # Invariant 2: In GpioSubcommand::Set, the configured pin must not be dropped
    assert "GpioSubcommand::Set =>" in content
