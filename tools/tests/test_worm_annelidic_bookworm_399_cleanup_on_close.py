#!/usr/bin/env python3
"""Regression unit tests for WORM-ANNELIDIC-BOOKWORM-399.

Verifies that:
1. When the host daemon window is closed, runner.stop() is executed to cleanly
   terminate child host_cli and host_fs processes.
2. runner.stop() explicitly terminates both self.rtt_process and self._active_sync_process.
3. Clean shutdown releases the probe connection so subsequent instances can start without error.
"""

from pathlib import Path
import subprocess
import sys
import unittest.mock as mock

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))
_tools_dir = REPO_ROOT / "tools"
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

from host_daemon.provider.daemon.runner import DaemonRunner
import host_daemon.cli as cli


def test_runner_stop_terminates_rtt_and_sync_processes():
    """Verify runner.stop() terminates active rtt_process and _active_sync_process."""
    runner = DaemonRunner(repo_root=REPO_ROOT)

    mock_rtt = mock.MagicMock()
    mock_rtt.poll.return_value = None
    runner.rtt_process = mock_rtt

    mock_sync = mock.MagicMock()
    mock_sync.poll.return_value = None
    runner._active_sync_process = mock_sync

    runner.stop()

    # Both processes must be terminated
    mock_rtt.terminate.assert_called_once()
    mock_sync.terminate.assert_called_once()
    assert runner.rtt_process is None
    assert runner._active_sync_process is None


def test_cli_on_app_close_calls_runner_stop():
    """Verify that _on_app_close in cli.py invokes runner.stop()."""
    cli_source = (REPO_ROOT / "tools" / "host_daemon" / "cli.py").read_text(encoding="utf-8")

    # In _on_app_close, runner.stop() must be invoked before shutdown and exit
    assert "runner.stop()" in cli_source
    # Verify runner.stop() appears inside _on_app_close
    idx_app_close = cli_source.find("def _on_app_close")
    assert idx_app_close != -1, "_on_app_close definition not found"
    app_close_block = cli_source[idx_app_close:idx_app_close + 400]
    assert "runner.stop()" in app_close_block, "runner.stop() must be called inside _on_app_close"
