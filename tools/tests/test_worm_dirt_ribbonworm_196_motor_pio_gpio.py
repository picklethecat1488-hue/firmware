#!/usr/bin/env python3
"""Regression unit test for WORM-DIRT-RIBBONWORM-196.

Verifies that:
1. In peripheral/src/tmc6300.rs, Tmc6300Pio::new calls sm.set_pin_dirs BEFORE
   sm.set_config(&cfg). Calling set_pin_dirs after set_config overwrites the
   pinctrl register in embassy-rp via pinctrl().write(), zeroing out out_base
   and out_count and preventing any GPIO activity on motor phase pins 16..21.
2. In board/src/cat_detector.rs, pump_diag is configured with Pull::Down or Pull::None,
   NOT Pull::Up (which triggers false active-high DIAG faults and stops the motor).
"""

from pathlib import Path


def test_worm_dirt_ribbonworm_196_pio_pin_dirs_before_config():
    """Verify sm.set_pin_dirs is called before sm.set_config in Tmc6300Pio::new."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    tmc6300_rs_path = workspace_root / "peripheral" / "src" / "tmc6300.rs"

    assert tmc6300_rs_path.exists(), f"File not found: {tmc6300_rs_path}"
    content = tmc6300_rs_path.read_text(encoding="utf-8")

    # Locate Tmc6300Pio::new bare-metal implementation
    assert "sm.set_pin_dirs(Direction::Out, &pins);" in content
    assert "sm.set_config(&cfg);" in content

    pin_dirs_pos = content.find("sm.set_pin_dirs(Direction::Out, &pins);")
    set_config_pos = content.find("sm.set_config(&cfg);")

    assert pin_dirs_pos != -1, "sm.set_pin_dirs not found in peripheral/src/tmc6300.rs"
    assert set_config_pos != -1, "sm.set_config not found in peripheral/src/tmc6300.rs"

    # Invariant: set_pin_dirs MUST come BEFORE set_config to prevent wiping out out_base/out_count
    assert pin_dirs_pos < set_config_pos, (
        "sm.set_pin_dirs must be called BEFORE sm.set_config(&cfg) in Tmc6300Pio::new. "
        "Calling set_pin_dirs after set_config overwrites pinctrl and zeroes out out_base/out_count, "
        "resulting in zero GPIO activity on motor phase pins."
    )


def test_worm_dirt_ribbonworm_196_pump_diag_not_pullup():
    """Verify pump_diag in board/src/cat_detector.rs is not configured with Pull::Up."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cat_detector_rs_path = workspace_root / "board" / "src" / "cat_detector.rs"

    assert cat_detector_rs_path.exists(), f"File not found: {cat_detector_rs_path}"
    content = cat_detector_rs_path.read_text(encoding="utf-8")

    # Invariant: pump_diag must not use Pull::Up (DIAG is active-high push-pull fault output)
    assert "pump_diag.set_pull(Pull::Up);" not in content, (
        "board/src/cat_detector.rs must not set Pull::Up on pump_diag (active-high fault indicator)."
    )
    assert (
        "pump_diag.set_pull(Pull::Down);" in content
        or "pump_diag.set_pull(Pull::None);" in content
    ), (
        "board/src/cat_detector.rs must set Pull::Down or Pull::None on pump_diag."
    )
