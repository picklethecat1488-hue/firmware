#!/usr/bin/env python3
"""Regression unit test for WORM-TUNNELING-INCHWORM-510: FS workstation and probe exclusivity.

Verifies that:
1. runner.set_mode("FS") stops RTT streaming to release probe for host_fs exclusive access.
2. runner.set_mode("CLI") restarts RTT streaming.
3. runner.execute_fs() logs exclusively into runner.fs_outputs (not cli_outputs), preventing interleaving.
4. runner.execute_cli() logs exclusively into runner.cli_outputs (not fs_outputs).
5. /api/fs/output endpoint returns list of fs_outputs.
6. /api/mode endpoint invokes runner.set_mode().
7. host_daemon.html.j2 contains separate cliView and fsView panels.
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

from host_daemon.provider.daemon.runner import DaemonRunner
from host_daemon.provider.daemon.server import HostDaemonRequestHandler, HostDaemonServer


def test_runner_probe_exclusivity_on_mode_change():
    """Verify set_mode stops RTT streaming for FS and starts RTT streaming for CLI."""
    runner = DaemonRunner(repo_root=REPO_ROOT)
    runner.stop_rtt_streaming = MagicMock()
    runner.start_rtt_streaming = MagicMock()

    runner.set_mode("FS")
    assert runner.active_mode == "FS"
    runner.stop_rtt_streaming.assert_called_once()
    runner.start_rtt_streaming.assert_not_called()

    runner.stop_rtt_streaming.reset_mock()
    runner.start_rtt_streaming.reset_mock()

    runner.set_mode("CLI")
    assert runner.active_mode == "CLI"
    runner.start_rtt_streaming.assert_called_once()


def test_separate_command_workstations_output_isolation():
    """Verify CLI and FS output streams are strictly isolated and never interleaved."""
    runner = DaemonRunner(repo_root=REPO_ROOT)
    runner.run_host_fs = MagicMock(return_value=(0, "test_file.bin (256 bytes)"))

    # Execute a CLI command
    runner.execute_cli("help")
    assert len(runner.cli_outputs) > 0
    assert len(runner.fs_outputs) == 0

    # Execute an FS command
    runner.execute_fs("ls")
    assert len(runner.fs_outputs) > 0
    # Verify the FS command is recorded in fs_outputs and not newly added to cli_outputs
    cli_count_before = len(runner.cli_outputs)
    assert any("fs> ls" in entry["text"] for entry in runner.fs_outputs)
    assert not any("fs> ls" in entry["text"] for entry in runner.cli_outputs)


def test_api_fs_output_and_mode_endpoints():
    """Verify /api/fs/output GET endpoint and /api/mode POST endpoint."""
    runner = MagicMock()
    runner.fs_outputs = [{"text": "fs> ls"}]
    server = HostDaemonServer("127.0.0.1", 0, runner)

    try:
        # Test /api/fs/output GET
        handler_get = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler_get.server = server
        handler_get.path = "/api/fs/output"
        handler_get._send_json = MagicMock()
        handler_get.do_GET()
        handler_get._send_json.assert_called_once_with([{"text": "fs> ls"}])

        # Test /api/mode POST
        handler_post = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler_post.server = server
        handler_post.path = "/api/mode"
        handler_post.headers = {"Content-Length": "15"}
        handler_post.rfile = MagicMock()
        handler_post.rfile.read.return_value = b'{"mode": "FS"}'
        handler_post._send_json = MagicMock()
        handler_post.do_POST()
        runner.set_mode.assert_called_once_with("FS")
        handler_post._send_json.assert_called_once_with({"success": True, "mode": "FS"})
    finally:
        server.server_close()


def test_template_has_separate_views():
    """Verify template contains distinct cliView and fsView elements."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")

    assert 'id="cliView"' in content
    assert 'id="fsView"' in content
    assert "/api/fs/output" in content
