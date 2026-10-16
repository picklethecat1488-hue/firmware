#!/usr/bin/env python3
"""Regression unit test for code review findings in CR_da4881c799958c39c88f8713a0967dd0b459a08a.

Verifies:
1. CR-29b62fd1: MotorController retains DEFAULT_MAX_CURRENT_MA (2500) and provides new_with_max_current.
   app.toml configures max_current_ma = 2000 under [[apps.cat_detector.controllers]].
   generated_app.rs.jinja generates MOTOR_MAX_CURRENT_MA constant.
   app/src/cat_detector.rs calls new_with_max_current passing MOTOR_MAX_CURRENT_MA.
2. CR-b5140851: peripheral/build.rs detects target and errors if non-RP2040 bare metal.
   peripheral/src/tmc6300.rs contains compile_error! guard when target_has_embassy_rp is missing.
3. CR-457ea4b3: peripheral/src/tmc6300.rs defines Tmc6300PhasePins struct, eliminating
   clippy::too_many_arguments from new_pio and new.
"""

from pathlib import Path


def test_cr_29b62fd1_decoupled_motor_current_limit_injection():
    """Verify MotorController decouples hardware current limit and receives it from app.toml."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    mc_rs = workspace_root / "controller" / "src" / "motor_controller.rs"
    app_toml = workspace_root / "app" / "app.toml"
    gen_app_jinja = workspace_root / "controller" / "templates" / "generated_app.rs.jinja"
    cat_detector_rs = workspace_root / "app" / "src" / "cat_detector.rs"

    mc_code = mc_rs.read_text(encoding="utf-8")
    assert "pub const DEFAULT_MAX_CURRENT_MA: i32 = 2500;" in mc_code
    assert "MAX_TMC6300_CURRENT_MA" not in mc_code
    assert "pub fn new_with_max_current(" in mc_code
    assert "max_hardware_current_ma" in mc_code

    app_toml_text = app_toml.read_text(encoding="utf-8")
    assert "max_current_ma = 2000" in app_toml_text

    jinja_text = gen_app_jinja.read_text(encoding="utf-8")
    assert "MOTOR_MAX_CURRENT_MA" in jinja_text

    cat_detector_code = cat_detector_rs.read_text(encoding="utf-8")
    assert "new_with_max_current(" in cat_detector_code
    assert "MOTOR_MAX_CURRENT_MA" in cat_detector_code


def test_cr_b5140851_target_compatibility_error_guard():
    """Verify build.rs and tmc6300.rs guard against non-RP2040 ARM targets."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    build_rs = workspace_root / "peripheral" / "build.rs"
    tmc6300_rs = workspace_root / "peripheral" / "src" / "tmc6300.rs"

    build_code = build_rs.read_text(encoding="utf-8")
    assert 'println!("cargo:rustc-cfg=target_has_embassy_rp");' in build_code
    assert "The TMC6300 PIO driver requires RP2040 hardware support" in build_code

    tmc_code = tmc6300_rs.read_text(encoding="utf-8")
    assert "compile_error!" in tmc_code
    assert "target_has_embassy_rp" in tmc_code


def test_cr_457ea4b3_tmc6300_phase_pins_bundle_no_clippy_allow():
    """Verify Tmc6300PhasePins bundles pins and removes clippy::too_many_arguments."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    tmc6300_rs = workspace_root / "peripheral" / "src" / "tmc6300.rs"
    board_cat_detector_rs = workspace_root / "board" / "src" / "cat_detector.rs"

    tmc_code = tmc6300_rs.read_text(encoding="utf-8")
    assert "pub struct Tmc6300PhasePins<PUH, PUL, PVH, PVL, PWH, PWL>" in tmc_code
    assert "clippy::too_many_arguments" not in tmc_code

    board_code = board_cat_detector_rs.read_text(encoding="utf-8")
    assert "peripheral::tmc6300::Tmc6300PhasePins" in board_code
