#!/usr/bin/env python3
"""Regression unit test for WORM-GLIDING-BRISTLEWORM-4: Sync modal console, auto-dismiss, and cancellation.

Verifies that:
1. DaemonRunner tracks sync console logs and includes them in sync_status.
2. DaemonRunner.cancel_sync() aborts running synchronization and marks step cancelled.
3. HostDaemonServer /api/sync/cancel endpoint handles POST and invokes cancel_sync().
4. host_daemon.html.j2 template includes #syncConsoleLog, #btnSyncCancel, auto-dismiss timeout,
   and CLI input disabling when sync is cancelled.
"""

from pathlib import Path
import sys
from unittest.mock import MagicMock, patch

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))
_tools_dir = REPO_ROOT / "tools"
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

from host_daemon.runner import DaemonRunner
from host_daemon.server import HostDaemonRequestHandler, HostDaemonServer


def test_runner_sync_console_and_cancellation():
    """Verify DaemonRunner sync logs capture and cancel_sync functionality."""
    runner = DaemonRunner(repo_root=REPO_ROOT)
    runner.log_sync_console("Compiling target thumbv6m-none-eabi...")
    assert "Compiling target thumbv6m-none-eabi..." in runner.sync_logs
    assert "console" in runner.sync_status
    assert any("Compiling target" in line for line in runner.sync_status["console"])

    # Test cancellation
    runner.cancel_sync()
    assert runner.sync_status["step"] == "cancelled"
    assert runner.sync_status["active"] is False
    assert runner.is_sync_cancelled is True


def test_api_sync_cancel_endpoint():
    """Verify /api/sync/cancel POST endpoint invokes runner.cancel_sync."""
    runner = MagicMock()
    runner.cancel_sync.return_value = {"success": True, "step": "cancelled"}
    server = HostDaemonServer("127.0.0.1", 0, runner)

    try:
        handler = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler.server = server
        handler.path = "/api/sync/cancel"
        handler.headers = {"Content-Length": "0"}
        handler.rfile = MagicMock()
        handler.rfile.read.return_value = b""
        handler._send_json = MagicMock()

        handler.do_POST()
        runner.cancel_sync.assert_called_once()
        handler._send_json.assert_called_once()
    finally:
        server.server_close()


def test_template_sync_modal_features():
    """Verify host_daemon.html.j2 has enlarged modal, console stream, cancel button, auto-dismiss and disabling."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")

    assert 'id="syncConsoleLog"' in content
    assert 'id="btnSyncCancel"' in content
    assert "cancelSync" in content
    assert "/api/sync/cancel" in content
    # Auto-dismiss on complete
    assert "closeSyncModal" in content
    # Workstation disabling on cancel
    assert "setWorkstationDisabled" in content or "cmdInput.disabled" in content
