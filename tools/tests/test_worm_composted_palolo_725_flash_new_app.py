#!/usr/bin/env python3
"""Regression unit test for WORM-COMPOSTED-PALOLO-725: Not able to flash a new app with host daemon.

Verifies that:
1. DaemonRunner.update_config() dynamically updates target, package, bin_name, profile, and speed.
2. HostDaemonServer /api/sync endpoint accepts JSON body with build parameters and updates runner.
3. CLI detects when target configuration differs from running daemon and forwards the reconfigure
   & flash request to /api/sync instead of silently failing to flash the new app.
4. CLI supports --restart flag to restart the daemon.
5. CLI supports 'sync' and 'flash' subcommands.
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

from host_daemon.cli import main, parse_arguments
from host_daemon.provider.daemon.runner import DaemonRunner
from host_daemon.provider.daemon.server import HostDaemonRequestHandler, HostDaemonServer


def test_daemon_runner_update_config():
    """Verify DaemonRunner.update_config updates runner fields and sync_status."""
    runner = DaemonRunner(repo_root=REPO_ROOT, package="cat_detector", bin_name="cat_detector_shell", release=False)
    assert runner.bin_name == "cat_detector_shell"
    assert runner.release is False

    runner.update_config(package="cat_detector", bin_name="cat_detector_app", release=True, speed=4000)
    assert runner.bin_name == "cat_detector_app"
    assert runner.release is True
    assert runner.speed == 4000
    assert runner.sync_status["bin"] == "cat_detector_app"
    assert runner.sync_status["profile"] == "release"
    assert runner.sync_status["speed"] == 4000


def test_api_sync_endpoint_updates_config():
    """Verify /api/sync endpoint handles configuration update payload."""
    runner = MagicMock()
    runner.sync_status = {"bin": "cat_detector_app"}
    server = HostDaemonServer("127.0.0.1", 0, runner)

    try:
        handler = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler.server = server
        handler.path = "/api/sync"
        handler.headers = {"Content-Length": "54"}
        handler.rfile = MagicMock()
        handler.rfile.read.return_value = b'{"package": "cat_detector", "bin": "cat_detector_app"}'
        handler._send_json = MagicMock()

        handler.do_POST()
        runner.update_config.assert_called_once_with(
            target=None,
            package="cat_detector",
            bin_name="cat_detector_app",
            release=None,
            speed=None,
            chip=None,
        )
        runner.sync_async.assert_called_once()
        handler._send_json.assert_called_once()
    finally:
        server.server_close()


def test_cli_forwards_new_app_when_daemon_running():
    """Verify running host_daemon with a new bin forwards sync request to running daemon."""
    mock_lock = {
        "pid": 999999,
        "port": 8899,
        "host": "127.0.0.1",
        "package": "cat_detector",
        "bin": "cat_detector_shell",
        "profile": "release",
        "speed": 1000,
        "target": "thumbv6m-none-eabi",
    }

    with patch("host_daemon.cli.get_lock_data", return_value=mock_lock), \
         patch("host_daemon.cli.is_pid_alive", return_value=True), \
         patch("host_daemon.cli.send_daemon_request") as mock_send:
        mock_send.return_value = {"success": True}
        ret = main(["--package", "cat_detector", "--bin", "cat_detector_app", "--release"])
        assert ret == 0
        mock_send.assert_called_once()
        call_path, payload = mock_send.call_args[0][:2]
        assert call_path == "/api/sync"
        assert payload["bin"] == "cat_detector_app"
        assert payload["release"] is True


def test_cli_supports_restart_flag_and_flash_subcommands():
    """Verify CLI argument parsing accepts --restart flag and sync/flash subcommands."""
    args_restart = parse_arguments(["--restart"])
    assert args_restart.restart is True

    args_sync = parse_arguments(["sync"])
    assert args_sync.subcommand == "sync"

    args_flash = parse_arguments(["flash"])
    assert args_flash.subcommand == "flash"
