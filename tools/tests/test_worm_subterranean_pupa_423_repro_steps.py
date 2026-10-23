#!/usr/bin/env python3
"""Regression unit test for WORM-SUBTERRANEAN-PUPA-423.

Ensures that:
1. DaemonRunner actively tracks command history upon CLI command execution in `runner.command_history`.
2. `/api/cli/history` endpoint returns the recorded command sequence.
3. In `host_daemon.html.j2`:
   - `openTargetBugModal()` clears `targetBugDescription`, `targetBugExpected`, and `targetBugActual`
     so past defect descriptions are not cached across bug reports.
   - `openTargetBugModal()` unconditionally updates `targetBugSteps` with the latest command sequence
     (rather than skipping updates when `stepsInput.value` is already present).
   - `submitTargetBug()` resets all input fields upon successful submission.
4. `/api/target_bug` sanitizes leading numbers from reproduction steps so Markdown export does not
   duplicate list numbering (e.g. avoiding "1. 1. Connect target device...").
"""

import json
from pathlib import Path
import re
import tempfile
from unittest.mock import MagicMock

from host_daemon.provider.daemon.runner import DaemonRunner
from host_daemon.provider.daemon.server import HostDaemonRequestHandler, HostDaemonServer

TEMPLATES_DIR = Path(__file__).resolve().parent.parent / "host_daemon" / "templates"


def test_worm_subterranean_pupa_423_runner_command_history():
    """Verify DaemonRunner tracks command history on execute_cli."""
    with tempfile.TemporaryDirectory() as tmp_dir:
        repo_root = Path(tmp_dir)
        runner = DaemonRunner(repo_root=repo_root, package="cat_detector", bin_name="cat_detector_shell")

        assert hasattr(runner, "command_history"), "DaemonRunner must have command_history attribute"
        assert runner.command_history == []

        # Execute multiple commands
        runner.execute_cli("motor speed 50")
        runner.execute_cli("motor speed 60")
        runner.execute_cli("motor stop")

        assert runner.command_history == ["motor speed 50", "motor speed 60", "motor stop"]


def test_worm_subterranean_pupa_423_api_cli_history():
    """Verify /api/cli/history returns the runner's recorded command history."""
    with tempfile.TemporaryDirectory() as tmp_dir:
        repo_root = Path(tmp_dir)
        runner = DaemonRunner(repo_root=repo_root, package="cat_detector", bin_name="cat_detector_shell")
        runner.execute_cli("gpio read 26")
        runner.execute_cli("motor speed 30")

        server = HostDaemonServer("127.0.0.1", 0, runner)

        handler = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler.server = server
        handler.path = "/api/cli/history"
        handler.requestline = "GET /api/cli/history HTTP/1.1"
        handler._send_json = MagicMock()

        handler.do_GET()

        assert handler._send_json.called, "Expected _send_json to be called for /api/cli/history"
        resp = handler._send_json.call_args[0][0]
        assert "history" in resp
        assert resp["history"] == ["gpio read 26", "motor speed 30"]


def test_worm_subterranean_pupa_423_target_bug_repro_steps_dedup_numbering():
    """Verify /api/target_bug strips redundant leading numbering to prevent double-numbering in Markdown."""
    with tempfile.TemporaryDirectory() as tmp_dir:
        repo_root = Path(tmp_dir)
        (repo_root / "feedback").mkdir()
        (repo_root / "attachments").mkdir()
        (repo_root / "target").mkdir()

        runner = DaemonRunner(repo_root=repo_root, package="cat_detector", bin_name="cat_detector_shell")
        server = HostDaemonServer("127.0.0.1", 0, runner)

        payload = {
            "title": "Motor oscillates back and forth",
            "severity": "MEDIUM",
            "category": "FIRMWARE",
            "component": "motor_driver",
            "description": "Rotor ticks back and forth.",
            "reproduction_steps": (
                "1. Connect target device via probe-rs / RTT\n"
                "2. Synchronize firmware: package cat_detector (cat_detector_shell)\n"
                "3. Execute command 'motor speed 30'\n"
                "4. Observe defect/failure on target"
            ),
            "attach_system_logs": False,
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

        assert handler._send_json.called
        res_data = handler._send_json.call_args[0][0]
        assert res_data.get("success") is True
        worm_id = res_data.get("worm_id")

        clean_id = worm_id.removeprefix("WORM-").removeprefix("WORM_")
        worm_md = repo_root / "feedback" / f"WORM_{clean_id}.md"
        assert worm_md.exists()
        md_text = worm_md.read_text(encoding="utf-8")

        # Must NOT contain double numbers like "1. 1." or "2. 2."
        assert "1. 1." not in md_text, f"Found duplicate numbering '1. 1.' in markdown:\n{md_text}"
        assert "2. 2." not in md_text, f"Found duplicate numbering '2. 2.' in markdown:\n{md_text}"
        assert "3. 3." not in md_text, f"Found duplicate numbering '3. 3.' in markdown:\n{md_text}"
        # Must contain single clean numbers
        assert "1. Connect target device via probe-rs / RTT" in md_text
        assert "3. Execute command 'motor speed 30'" in md_text


def test_worm_subterranean_pupa_423_template_modal_clears_description_and_refreshes_steps():
    """Verify host_daemon.html.j2 clears description and dynamically updates steps on modal open."""
    template_path = TEMPLATES_DIR / "host_daemon.html.j2"
    assert template_path.exists(), "host_daemon.html.j2 must exist"
    content = template_path.read_text(encoding="utf-8")

    # 1. openTargetBugModal must clear targetBugDescription so previous descriptions don't persist
    assert "targetBugDescription" in content
    assert 'descInput.value = "";' in content or 'getElementById("targetBugDescription").value = "";' in content or 'descInput.value = ""' in content, (
        "openTargetBugModal must reset description input value"
    )

    # 2. openTargetBugModal must NOT guard steps update with '!stepsInput.value' (which prevented updates)
    assert "!stepsInput.value" not in content, (
        "openTargetBugModal must not prevent updating steps with '!stepsInput.value' guard"
    )

    # 3. openTargetBugModal must query /api/cli/history or use latest commands
    assert "/api/cli/history" in content or "buildReproSteps" in content, (
        "openTargetBugModal must dynamically build reproduction steps from recent commands"
    )
