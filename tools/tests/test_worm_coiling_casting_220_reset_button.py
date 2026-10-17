#!/usr/bin/env python3
"""Regression unit test for WORM-COILING-CASTING-220: Add RESET button to host daemon.

Verifies that:
1. DaemonRunner has a reset_target() method that stops RTT streaming, runs probe-rs reset,
   and restarts RTT streaming.
2. HostDaemonServer exposes /api/reset endpoint handling POST requests.
3. The host daemon template contains the Reset button and triggerReset client script.
4. Host daemon CLI supports 'reset' subcommand.
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

from host_daemon.cli import parse_arguments
from host_daemon.runner import DaemonRunner
from host_daemon.server import HostDaemonRequestHandler, HostDaemonServer


def test_daemon_runner_reset_target():
    """Verify DaemonRunner.reset_target stops RTT, invokes probe-rs reset, and starts RTT."""
    runner = DaemonRunner(repo_root=REPO_ROOT, chip="rp2040", speed=4000)
    runner.stop_rtt_streaming = MagicMock()
    runner.start_rtt_streaming = MagicMock()
    runner.run_probe_rs = MagicMock(return_value=(0, "Target reset successfully"))

    res = runner.reset_target()
    assert res["success"] is True
    assert res["probe_reset"] is True
    runner.stop_rtt_streaming.assert_called_once()
    runner.start_rtt_streaming.assert_called_once()
    runner.run_probe_rs.assert_called_once()
    args, _ = runner.run_probe_rs.call_args
    assert "reset" in args[0]
    assert "--chip" in args[0]
    assert "rp2040" in args[0]


def test_api_reset_endpoint():
    """Verify /api/reset POST endpoint calls runner.reset_target and returns JSON."""
    runner = MagicMock()
    runner.reset_target.return_value = {"success": True, "message": "Target reset completed"}

    server = HostDaemonServer("127.0.0.1", 0, runner)
    try:
        handler = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler.server = server
        handler.path = "/api/reset"
        handler.headers = {"Content-Length": "0"}
        handler.rfile = MagicMock()
        handler.rfile.read.return_value = b""
        handler._send_json = MagicMock()

        handler.do_POST()
        handler._send_json.assert_called_once_with({"success": True, "message": "Target reset completed"})
        runner.reset_target.assert_called_once()
    finally:
        server.server_close()


def test_template_contains_reset_button():
    """Verify host_daemon.html.j2 contains Reset button and triggerReset script."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")

    assert 'id="btnReset"' in content
    assert "triggerReset" in content
    assert "btn-amber" in content
    assert "/api/reset" in content


def test_cli_supports_reset_subcommand():
    """Verify CLI parser accepts 'reset' subcommand."""
    args = parse_arguments(["reset"])
    assert args.subcommand == "reset"
