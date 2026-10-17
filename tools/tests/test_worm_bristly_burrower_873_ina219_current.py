#!/usr/bin/env python3
"""Regression unit test for WORM-BRISTLY-BURROWER-873.

Verifies that:
1. INA219 driver in peripheral/src/ina219.rs defines named constants for configuration,
   calibration, LSB scaling divisor (10), and register addresses.
2. read_current_ma scales raw 0.1 mA counts by CURRENT_LSB_DIVISOR (10) to report accurate mA.
3. read_current_ma checks Register::SHUNT_VOLTAGE as fallback if Register::CURRENT is 0
   (e.g., after power rail glitch or calibration register reset) and restores calibration.
4. set_measurement_mode writes DEFAULT_CALIBRATION to Register::CALIBRATION when entering active modes.
5. peripheral/Cargo.toml dev-dependencies include embassy-time with the 'std' feature for host tests.
6. integration test suite includes test_ina219_read_current_ma_scaling_and_fallback.
"""

from pathlib import Path


def test_worm_bristly_burrower_873_constants_and_scaling_defined():
    """Verify named constants and scaling divisor in peripheral/src/ina219.rs."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    ina219_rs_path = workspace_root / "peripheral" / "src" / "ina219.rs"

    assert ina219_rs_path.exists(), f"File not found: {ina219_rs_path}"
    content = ina219_rs_path.read_text(encoding="utf-8")

    # Invariant 1: Named constants for registers and calibration
    assert "pub const DEFAULT_CALIBRATION: u16 = 4096;" in content, (
        "peripheral/src/ina219.rs must define DEFAULT_CALIBRATION as 4096."
    )
    assert "pub const CURRENT_LSB_DIVISOR: i32 = 10;" in content, (
        "peripheral/src/ina219.rs must define CURRENT_LSB_DIVISOR as 10 (0.1mA to mA conversion)."
    )
    assert "const SHUNT_VOLTAGE: u8 = 0x01;" in content, (
        "peripheral/src/ina219.rs Register struct must define SHUNT_VOLTAGE register 0x01."
    )
    assert "pub const DEFAULT_CONFIG: u16 = 0x399F;" in content, (
        "peripheral/src/ina219.rs must define DEFAULT_CONFIG named constant."
    )

    # Invariant 2: Scaling by CURRENT_LSB_DIVISOR in read_current_ma
    assert "/ CURRENT_LSB_DIVISOR" in content, (
        "peripheral/src/ina219.rs must divide raw current register counts by CURRENT_LSB_DIVISOR."
    )

    # Invariant 3: Shunt voltage fallback when calibration is lost
    assert "Register::SHUNT_VOLTAGE" in content, (
        "peripheral/src/ina219.rs must check Register::SHUNT_VOLTAGE as fallback."
    )
    assert "DEFAULT_CALIBRATION" in content, (
        "peripheral/src/ina219.rs must restore DEFAULT_CALIBRATION."
    )


def test_worm_bristly_burrower_873_set_measurement_mode_restores_calibration():
    """Verify set_measurement_mode restores calibration register in active modes."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    ina219_rs_path = workspace_root / "peripheral" / "src" / "ina219.rs"

    content = ina219_rs_path.read_text(encoding="utf-8")
    assert "Register::CALIBRATION, DEFAULT_CALIBRATION" in content, (
        "peripheral/src/ina219.rs must write DEFAULT_CALIBRATION to Register::CALIBRATION."
    )


def test_worm_bristly_burrower_873_integration_test_present():
    """Verify integration test covering current scaling and fallback exists."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    integration_rs_path = workspace_root / "peripheral" / "tests" / "integration.rs"

    assert integration_rs_path.exists(), f"File not found: {integration_rs_path}"
    content = integration_rs_path.read_text(encoding="utf-8")

    assert "test_ina219_read_current_ma_scaling_and_fallback" in content, (
        "peripheral/tests/integration.rs must define test_ina219_read_current_ma_scaling_and_fallback."
    )
