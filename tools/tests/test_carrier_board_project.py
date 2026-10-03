"""Reproduction and regression unit test for WORM-005 (BUG-005) and CR-2a0d53ac.

Verifies that:
1. The carrier board 2.0 app is integrated into the app crate alongside cat_detector.
2. carrier_board is not a separate standalone workspace member in Cargo.toml.
3. The carrier board firmware design doc resides under app/carrier_board.md with required design evaluations.
4. app/carrier_board_bringup.yaml exists as the source of truth for bringup verification.
"""

from pathlib import Path
import tomllib
import yaml


WORKSPACE_ROOT = Path(__file__).resolve().parent.parent.parent


def test_carrier_board_project_structure() -> None:
    """Verify carrier_board is integrated in app crate and standalone crate is removed."""
    # 1. Root Cargo.toml workspace membership does not have carrier_board as standalone crate
    root_cargo_path = WORKSPACE_ROOT / "Cargo.toml"
    assert root_cargo_path.exists(), "Root Cargo.toml must exist"
    root_cargo = tomllib.loads(root_cargo_path.read_text(encoding="utf-8"))
    members = root_cargo.get("workspace", {}).get("members", [])
    assert "carrier_board" not in members, "carrier_board must NOT be a separate standalone member"
    assert "app" in members, "app crate must be in workspace.members"

    # 2. Standalone carrier_board directory must not exist
    cb_dir = WORKSPACE_ROOT / "carrier_board"
    assert not cb_dir.exists(), "Standalone carrier_board/ directory must not exist"

    # 3. app crate contains carrier_board modules and binary
    app_dir = WORKSPACE_ROOT / "app"
    app_cargo_path = app_dir / "Cargo.toml"
    assert app_cargo_path.exists(), "app/Cargo.toml must exist"
    app_cargo = tomllib.loads(app_cargo_path.read_text(encoding="utf-8"))

    # Assert carrier_board_shell is registered as a binary in app crate
    bins = [b.get("name") for b in app_cargo.get("bin", [])]
    assert "carrier_board_shell" in bins, "carrier_board_shell binary must be in app/Cargo.toml"

    # Assert source files exist under app/src/
    assert (app_dir / "src" / "carrier_board.rs").exists(), "app/src/carrier_board.rs must exist"
    assert (app_dir / "src" / "carrier_board_shell.rs").exists(), (
        "app/src/carrier_board_shell.rs must exist"
    )
    assert (app_dir / "tests" / "carrier_board_tests.rs").exists(), (
        "app/tests/carrier_board_tests.rs must exist"
    )


def test_carrier_board_firmware_document() -> None:
    """Verify carrier board firmware documentation exists under app/carrier_board.md with all required evaluations."""
    doc_path = WORKSPACE_ROOT / "app" / "carrier_board.md"
    assert doc_path.exists(), "app/carrier_board.md must exist"

    content = doc_path.read_text(encoding="utf-8")

    # Architecture & Key Components
    assert "Carrier Board 2.0" in content
    assert "MCX N947" in content or "MCXN947" in content
    assert "W25N01GV" in content
    assert "BQ24074" in content
    assert "MAX17048" in content
    assert "LP5009" in content
    assert "IQS7222A" in content
    assert "NINA-B312" in content

    # Cost breakdown for .text and .data
    assert ".text" in content
    assert ".data" in content
    assert "Neutron NPU" in content or "eIQ" in content
    assert "DSP" in content or "PowerQuad" in content
    assert "gesture" in content.lower() or "camera" in content.lower()
    assert "location" in content.lower()

    # Single-slot vs dual-slot OTA evaluation (download to NAND)
    assert "NAND" in content
    assert "Slot B" in content or "single-slot" in content.lower() or "Single-Slot" in content

    # SRAM constraints & dual-core Rust architecture
    assert "SRAM" in content
    assert "AMP" in content or "dual-core" in content.lower() or "Dual-Core" in content

    # UART vs RTT evaluation
    assert "UART" in content
    assert "RTT" in content

    # Power breakdown with all cores and accelerators
    assert "PowerQuad" in content or "Neutron" in content
    assert "mA" in content

    # Custom Embassy HAL & validation plan
    assert "Embassy" in content
    assert "Validation" in content or "validation" in content


def test_carrier_board_bringup_yaml() -> None:
    """Verify app/carrier_board_bringup.yaml exists as source of truth for bringup verification."""
    yaml_path = WORKSPACE_ROOT / "app" / "carrier_board_bringup.yaml"
    assert yaml_path.exists(), "app/carrier_board_bringup.yaml must exist"

    data = yaml.safe_load(yaml_path.read_text(encoding="utf-8"))
    assert data["project_name"] == "Carrier Board 2.0"
    assert "MCXN947" in data["device_chip"]
    assert "carrier_board_shell" in data["cargo_target"]

    steps = data.get("steps", [])
    assert len(steps) >= 5, "carrier_board_bringup.yaml must define comprehensive bringup steps"
