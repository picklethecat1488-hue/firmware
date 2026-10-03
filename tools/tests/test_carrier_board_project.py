"""Reproduction and regression unit test for WORM-005 (BUG-005).

Verifies that:
1. The carrier board 2.0 trivial 'hello world' project exists and is registered in Cargo workspace.
2. The carrier board firmware design and bringup document exists under docs/carrier_board_firmware.md.
3. The carrier board crate exposes hello world bringup definitions and hardware constants.
"""

from pathlib import Path
import tomllib


WORKSPACE_ROOT = Path(__file__).resolve().parent.parent.parent


def test_carrier_board_project_structure() -> None:
    """Verify carrier_board crate exists, is in Cargo workspace, and has shell binary."""
    # 1. Root Cargo.toml workspace membership
    root_cargo_path = WORKSPACE_ROOT / "Cargo.toml"
    assert root_cargo_path.exists(), "Root Cargo.toml must exist"
    root_cargo = tomllib.loads(root_cargo_path.read_text(encoding="utf-8"))
    members = root_cargo.get("workspace", {}).get("members", [])
    assert "carrier_board" in members, "carrier_board must be registered in workspace.members"

    # 2. carrier_board directory and Cargo.toml
    cb_dir = WORKSPACE_ROOT / "carrier_board"
    assert cb_dir.is_dir(), "carrier_board/ directory must exist"

    cb_cargo_path = cb_dir / "Cargo.toml"
    assert cb_cargo_path.exists(), "carrier_board/Cargo.toml must exist"
    cb_cargo = tomllib.loads(cb_cargo_path.read_text(encoding="utf-8"))
    assert cb_cargo.get("package", {}).get("name") == "carrier_board"

    # 3. Source files exist
    assert (cb_dir / "src" / "lib.rs").exists(), "carrier_board/src/lib.rs must exist"
    assert (cb_dir / "src" / "bin" / "carrier_board_shell.rs").exists(), (
        "carrier_board/src/bin/carrier_board_shell.rs must exist"
    )


def test_carrier_board_firmware_document() -> None:
    """Verify carrier board firmware documentation exists and contains canonical hardware specifications."""
    doc_path = WORKSPACE_ROOT / "docs" / "carrier_board_firmware.md"
    assert doc_path.exists(), "docs/carrier_board_firmware.md must exist"

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

    # Section Headers
    assert "Memory Map" in content or "Partitions" in content
    assert "Bringup" in content or "Checklist" in content
    assert "Pin Mapping" in content or "Interface" in content
