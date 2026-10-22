#!/usr/bin/env python3
"""Regression functional unit test for WORM-SLIMY-PLANARIAN-761.

Verifies that:
1. When calling host.py with no arguments, the runner enters package selection mode
   and does not execute sync/flash until the user confirms configuration.
2. Target packages and application binaries are discovered from workspace Cargo.toml files.
3. Target ARM architecture (e.g. thumbv6m-none-eabi for RP2040 Cortex-M0+) is automatically inferred.
4. Target selection (package, binary, profile, architecture, speed) is remembered across connected sessions
   via target/host_session.json.
5. Host daemon REST API (/api/targets and /api/target/configure) provides package selection and configuration.
6. host_daemon.html.j2 template includes the target configuration modal (#targetConfigModal), package/binary selectors,
   release profile checkbox, inferred architecture display, and auto-opens when in package selection mode.
"""

import json
from pathlib import Path
import sys
from unittest.mock import MagicMock

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))
_tools_dir = REPO_ROOT / "tools"
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

from host_daemon.provider.daemon.config import (  # noqa: E402
    discover_target_packages,
    has_explicit_target_args,
    infer_arm_target,
    load_last_session,
    save_last_session,
)
from host_daemon.provider.daemon.runner import DaemonRunner  # noqa: E402
from host_daemon.provider.daemon.server import HostDaemonRequestHandler, HostDaemonServer  # noqa: E402


def test_package_discovery_and_target_inference():
    """Verify discovery of workspace packages, binaries, and ARM architecture inference."""
    packages = discover_target_packages(REPO_ROOT)
    assert len(packages) > 0, "Expected at least one target package discovered in workspace."

    cat_pkg = next((p for p in packages if p["package"] == "cat_detector"), None)
    assert cat_pkg is not None, "cat_detector package not discovered."
    assert "cat_detector_shell" in cat_pkg["binaries"]
    assert "cat_detector_app" in cat_pkg["binaries"]
    assert "carrier_board_shell" in cat_pkg["binaries"]
    assert cat_pkg["inferred_target"] == "thumbv6m-none-eabi"

    # Test automatic target ARM architecture inference
    assert infer_arm_target("cat_detector", chip="rp2040", repo_root=REPO_ROOT) == "thumbv6m-none-eabi"
    assert infer_arm_target("generic_m0", chip="cortex-m0plus", repo_root=REPO_ROOT) == "thumbv6m-none-eabi"
    assert infer_arm_target("nxp_app", chip="mcx_n947", repo_root=REPO_ROOT) == "thumbv8m.main-none-eabihf"
    assert infer_arm_target("stm_app", chip="stm32f407", repo_root=REPO_ROOT) == "thumbv7em-none-eabihf"


def test_session_persistence_roundtrip(tmp_path: Path):
    """Verify target selection is saved and remembered from the last connected session."""
    session_data = {
        "package": "cat_detector",
        "bin": "carrier_board_shell",
        "release": True,
        "target": "thumbv6m-none-eabi",
        "speed": 4000,
        "chip": "rp2040",
    }
    save_last_session(tmp_path, session_data)

    loaded = load_last_session(tmp_path)
    assert loaded is not None
    assert loaded["package"] == "cat_detector"
    assert loaded["bin"] == "carrier_board_shell"
    assert loaded["release"] is True
    assert loaded["target"] == "thumbv6m-none-eabi"
    assert loaded["speed"] == 4000


def test_explicit_args_detection():
    """Verify detection of whether host.py was called with no arguments or explicit target flags."""
    # No arguments (or only transport/window flags)
    assert has_explicit_target_args([]) is False
    assert has_explicit_target_args(["--no-detach"]) is False
    assert has_explicit_target_args(["--port", "8899", "--no-browser"]) is False

    # Explicit target arguments
    assert has_explicit_target_args(["--package", "cat_detector"]) is True
    assert has_explicit_target_args(["--bin", "cat_detector_shell"]) is True
    assert has_explicit_target_args(["--release"]) is True
    assert has_explicit_target_args(["--target", "thumbv6m-none-eabi"]) is True
    assert has_explicit_target_args(["cli", "motor", "speed", "100"]) is True


def test_runner_package_selection_mode():
    """Verify DaemonRunner halts auto-sync when initialized in package selection mode."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        package_selection_mode=True,
    )
    assert runner.package_selection_mode is True
    assert runner.status_message == "Awaiting Target Configuration"

    # When package selection mode is active, sync_status is idle
    assert runner.sync_status["active"] is False
    assert runner.sync_status["step"] == "idle"


def test_api_targets_and_configure_endpoints():
    """Verify /api/targets GET and /api/target/configure POST endpoints."""
    runner = DaemonRunner(repo_root=REPO_ROOT, package_selection_mode=True)
    server = HostDaemonServer("127.0.0.1", 0, runner)

    try:
        # Test GET /api/targets
        handler = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler.server = server
        handler.path = "/api/targets"
        handler.headers = {}
        handler._send_json = MagicMock()

        handler.do_GET()
        handler._send_json.assert_called_once()
        targets_payload = handler._send_json.call_args[0][0]
        assert "packages" in targets_payload
        assert targets_payload["package_selection_mode"] is True
        assert any(p["package"] == "cat_detector" for p in targets_payload["packages"])

        # Test POST /api/target/configure
        config_body = json.dumps(
            {
                "package": "cat_detector",
                "bin": "carrier_board_shell",
                "release": True,
                "speed": 4000,
            }
        ).encode("utf-8")

        handler.path = "/api/target/configure"
        handler.headers = {"Content-Length": str(len(config_body))}
        handler.rfile = MagicMock()
        handler.rfile.read.return_value = config_body
        handler._send_json = MagicMock()

        handler.do_POST()
        handler._send_json.assert_called_once()
        res = handler._send_json.call_args[0][0]
        assert res.get("success") is True
        assert runner.package == "cat_detector"
        assert runner.bin_name == "carrier_board_shell"
        assert runner.release is True
        assert runner.target == "thumbv6m-none-eabi"
        assert runner.package_selection_mode is False
    finally:
        server.server_close()


def test_template_target_configuration_modal():
    """Verify host_daemon.html.j2 has targetConfigModal, package/bin inputs, and auto-open logic."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")

    assert 'id="targetConfigModal"' in content
    assert 'id="cfgPackageSelect"' in content
    assert 'id="cfgBinSelect"' in content
    assert 'id="cfgReleaseCheckbox"' in content
    assert 'id="cfgInferredArchDisplay"' in content
    assert 'id="btnConfirmTargetConfig"' in content
    assert 'id="btnTargetConfig"' in content
    assert "submitTargetConfig" in content
    assert "openTargetConfigModal" in content
    assert "package_selection_mode" in content
