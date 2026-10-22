#!/usr/bin/env python3
"""Regression test for WORM-MOIST-MILLIPEDE-488: Host flashes cat_detector_app instead of shell.

Verifies:
1. Target package discovery sets default_bin to 'cat_detector_shell' and puts it first in binaries list.
2. Cargo.toml for app package specifies default-run = 'cat_detector_shell'.
3. Session persistence loads cat_detector_shell and test suites do not overwrite it.
"""

from pathlib import Path
import sys

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))
_tools_dir = REPO_ROOT / "tools"
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore

from host_daemon.provider.daemon.config import discover_target_packages, load_last_session


def test_cat_detector_defaults_to_shell():
    """Verify cat_detector target package discovers cat_detector_shell as default binary."""
    packages = discover_target_packages(REPO_ROOT)
    cat_pkg = next((p for p in packages if p["package"] == "cat_detector"), None)
    assert cat_pkg is not None, "cat_detector package not discovered."
    assert cat_pkg["default_bin"] == "cat_detector_shell", (
        f"Expected default_bin to be 'cat_detector_shell', got '{cat_pkg['default_bin']}'"
    )
    assert cat_pkg["binaries"][0] == "cat_detector_shell", (
        f"Expected 'cat_detector_shell' to be first in binaries list, got {cat_pkg['binaries']}"
    )


def test_app_cargo_toml_specifies_default_run():
    """Verify app/Cargo.toml contains default-run = 'cat_detector_shell'."""
    cargo_toml = REPO_ROOT / "app" / "Cargo.toml"
    assert cargo_toml.exists(), "app/Cargo.toml not found"
    data = tomllib.loads(cargo_toml.read_text(encoding="utf-8"))
    assert data.get("package", {}).get("default-run") == "cat_detector_shell"


def test_session_remembers_shell_binary():
    """Verify load_last_session returns cat_detector_shell."""
    session = load_last_session(REPO_ROOT)
    assert session is not None, "Expected valid session file"
    assert session.get("bin") == "cat_detector_shell", (
        f"Expected session bin to be 'cat_detector_shell', got '{session.get('bin')}'"
    )
