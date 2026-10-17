#!/usr/bin/env python3
"""Regression unit test for code review findings:

1. CR-766fd202: Decouple bare-metal non-RP2040 targets from RP2040-specific crates.
   peripheral/build.rs must not output errors for non-RP2040 baremetal targets.
   peripheral/src/tmc6300.rs must not contain unconditional compile_error! guards.
   peripheral/Cargo.toml must scope embassy-rp and pio under thumbv6m-none-eabi.
2. CR-2ffeff94: Commutation rewrite with two match statements and zero-speed error.
   apply_commutation must use separate match arms for positive and negative speed,
   and explicitly return Tmc6300Error::ZeroSpeed when speed == 0.
"""

from pathlib import Path


def test_cr_766fd202_baremetal_non_rp2040_target_decoupling():
    """Verify build.rs, Cargo.toml, and tmc6300.rs allow non-RP2040 baremetal targets to compile cleanly."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    build_rs = workspace_root / "peripheral" / "build.rs"
    cargo_toml = workspace_root / "peripheral" / "Cargo.toml"
    tmc6300_rs = workspace_root / "peripheral" / "src" / "tmc6300.rs"

    build_code = build_rs.read_text(encoding="utf-8")
    assert 'println!("cargo:rustc-cfg=target_has_embassy_rp");' in build_code
    assert 'eprintln!("Error:' not in build_code
    assert "target.contains(\"none\")" not in build_code

    cargo_text = cargo_toml.read_text(encoding="utf-8")
    assert "[target.thumbv6m-none-eabi.dependencies]" in cargo_text
    assert 'embassy-rp = { version = "0.3.0"' in cargo_text
    # embassy-rp and pio must NOT be in generic all(target_arch = "arm", target_os = "none") dependencies
    arm_none_section = cargo_text.split('[target.\'cfg(all(target_arch = "arm", target_os = "none"))\'.dependencies]')[1].split("[target.")[0]
    assert "embassy-rp" not in arm_none_section
    assert "pio" not in arm_none_section

    tmc_code = tmc6300_rs.read_text(encoding="utf-8")
    assert "compile_error!" not in tmc_code
    assert "target_has_embassy_rp" in tmc_code


def test_cr_2ffeff94_commutation_two_match_statements_and_zero_speed_error():
    """Verify apply_commutation uses 2 match statements (forward/reverse) and errors on speed == 0."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    tmc6300_rs = workspace_root / "peripheral" / "src" / "tmc6300.rs"

    tmc_code = tmc6300_rs.read_text(encoding="utf-8")
    assert "pub fn apply_commutation(" in tmc_code
    assert "if self.speed > 0 {" in tmc_code
    assert "else if self.speed < 0 {" in tmc_code
    assert "Err(Tmc6300Error::ZeroSpeed)" in tmc_code
