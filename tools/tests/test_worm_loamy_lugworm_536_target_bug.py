"""Reproduction and regression test for WORM-LOAMY-LUGWORM-536:
Add Target Bug button and automated bug reporting to host tool.
"""

from datetime import datetime, timezone
import json
from pathlib import Path
import tempfile
from unittest.mock import MagicMock, patch
import pytest

from host_daemon.runner import DaemonRunner
from host_daemon.server import HostDaemonRequestHandler, HostDaemonServer
from model.worm_report import WormCategory, WormSeverity, WormStatus


TEMPLATES_DIR = Path(__file__).resolve().parent.parent / "host_daemon" / "templates"


def test_target_bug_ui_elements_in_template() -> None:
    """Verify Target Bug button, modal, form fields, and attachment options exist in host_daemon.html.j2."""
    template_path = TEMPLATES_DIR / "host_daemon.html.j2"
    assert template_path.exists(), "host_daemon.html.j2 not found"
    content = template_path.read_text(encoding="utf-8")

    # 1. Target Bug button in header
    assert 'id="btnTargetBug"' in content, "Missing #btnTargetBug button in header"
    assert "openTargetBugModal()" in content, "Missing openTargetBugModal() onclick handler"

    # 2. Target Bug modal and form inputs
    assert 'id="targetBugModal"' in content, "Missing #targetBugModal container"
    assert 'id="targetBugTitle"' in content, "Missing #targetBugTitle input"
    assert 'id="targetBugSeverity"' in content, "Missing #targetBugSeverity select"
    assert 'id="targetBugCategory"' in content, "Missing #targetBugCategory select"
    assert 'id="targetBugComponent"' in content, "Missing #targetBugComponent input"
    assert 'id="targetBugDescription"' in content, "Missing #targetBugDescription textarea"
    assert 'id="targetBugSteps"' in content, "Missing #targetBugSteps textarea"
    assert 'id="targetBugExpected"' in content, "Missing #targetBugExpected textarea"
    assert 'id="targetBugActual"' in content, "Missing #targetBugActual textarea"

    # 3. Attachment checkboxes
    assert 'id="attachSystemLogs"' in content, "Missing #attachSystemLogs checkbox"
    assert 'id="attachCrashLog"' in content, "Missing #attachCrashLog checkbox"
    assert 'id="attachTelemetry"' in content, "Missing #attachTelemetry checkbox"
    assert 'id="attachMetadata"' in content, "Missing #attachMetadata checkbox"

    # 4. Submit button and handler
    assert 'id="btnSubmitTargetBug"' in content, "Missing #btnSubmitTargetBug submit button"
    assert "submitTargetBug()" in content, "Missing submitTargetBug() JavaScript function"


def test_target_bug_server_api_creates_worm_with_attachments() -> None:
    """Verify /api/target_bug creates a worm report with system logs and target fs attachments."""
    with tempfile.TemporaryDirectory() as tmp_dir:
        repo_root = Path(tmp_dir)
        (repo_root / "feedback").mkdir()
        (repo_root / "attachments").mkdir()
        (repo_root / "target").mkdir()

        runner = DaemonRunner(repo_root=repo_root, package="cat_detector", bin_name="cat_detector_shell")
        # Populate dummy logs in runner
        runner.log("Target initialized and running", level="INFO", channel="daemon")
        runner.log("Motor fault detected on phase A", level="ERROR", channel="rtt")
        runner.log_cli("motor speed 100", is_input=True)

        # Mock execute_fs to return dummy crash-log and telemetry
        def mock_execute_fs(cmd: str, args=None) -> str:
            if "crash-log" in cmd:
                return "CRASH LOG: HardFault at 0x08001234, HFSR: 0x40000000"
            elif "export-telemetry" in cmd:
                return '{"temp_c": 42.5, "motor_current_ma": 7.0, "vbus_mv": 12050}'
            elif "cat metadata" in cmd:
                return "FS_VERSION=1.0\nBLOCKS=512\nERASE_CYCLES=12"
            return "OK"

        runner.execute_fs = MagicMock(side_effect=mock_execute_fs)

        server = HostDaemonServer("127.0.0.1", 0, runner)

        # Create mock request handler to test do_POST on /api/target_bug
        payload = {
            "title": "Motor fails to spin at speed 100",
            "severity": "HIGH",
            "category": "FIRMWARE",
            "component": "motor_driver",
            "description": "Motor enters HardFault when commanded to 100% duty cycle.",
            "reproduction_steps": [
                "1. Synchronize target with cat_detector_shell",
                "2. Send 'motor speed 100' CLI command",
                "3. Observe motor current remains constant at 7 mA",
            ],
            "expected_behavior": "Motor accelerates smoothly to 100% duty cycle.",
            "actual_behavior": "Motor stalls and HardFault is logged.",
            "attach_system_logs": True,
            "attach_crash_log": True,
            "attach_telemetry": True,
            "attach_metadata": True,
        }

        handler = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler.server = server
        handler.path = "/api/target_bug"
        handler.headers = {"Content-Length": str(len(json.dumps(payload)))}
        handler.rfile = MagicMock()
        handler.rfile.read.return_value = json.dumps(payload).encode("utf-8")
        handler._send_json = MagicMock()
        handler.send_error = MagicMock()

        handler.do_POST()

        assert handler._send_json.called, f"Expected _send_json to be called, got send_error: {handler.send_error.call_args}"
        res_data = handler._send_json.call_args[0][0]
        assert res_data.get("success") is True
        worm_id = res_data.get("worm_id")
        assert worm_id and (worm_id.startswith("WORM-") or worm_id.startswith("WORM_"))
        assert res_data.get("attachments_count", 0) >= 3

        # Verify exported markdown exists
        clean_id = worm_id.removeprefix("WORM-").removeprefix("WORM_")
        worm_md = repo_root / "feedback" / f"WORM_{clean_id}.md"
        assert worm_md.exists(), f"Expected {worm_md} to exist"
        md_text = worm_md.read_text(encoding="utf-8")
        assert "Motor fails to spin at speed 100" in md_text
        assert "HardFault" in md_text

        # Verify attachments were created on disk
        worm_att_dir = repo_root / "attachments" / worm_id
        if not worm_att_dir.exists():
            worm_att_dir = repo_root / "feedback" / "attachments" / worm_id
        assert worm_att_dir.exists(), f"Attachment directory {worm_att_dir} does not exist"
        att_files = list(worm_att_dir.glob("*"))
        assert len(att_files) >= 3, f"Expected at least 3 attachment files, found: {att_files}"
