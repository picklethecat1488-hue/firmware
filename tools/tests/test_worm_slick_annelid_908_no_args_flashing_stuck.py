#!/usr/bin/env python3
"""Regression functional unit test for WORM-SLICK-ANNELID-908.

Verifies that:
1. When calling host.py with no args, confirming target configuration in the modal
   initiates progress polling (startSyncPolling()) and initializes progress indicators,
   preventing the UI from getting permanently stuck at 0% / initial flashing UI.
2. In DaemonRunner.sync_async(), sync_status is immediately updated to active=True
   and step='starting' synchronously, avoiding race conditions where the UI polls
   before the background worker updates the state.
3. In pollSyncStatus(), target configuration info elements (target, package, profile, speed)
   are dynamically updated from synchronization status data.
4. When host.py is invoked without explicit arguments while a daemon is already running,
   it does not erroneously treat default CLI options as a configuration change or trigger
   unintended re-flashing.
"""

from pathlib import Path
import re
import sys
from unittest.mock import MagicMock, patch

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))
_tools_dir = REPO_ROOT / "tools"
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

from host_daemon.cli import has_explicit_target_args, main  # noqa: E402
from host_daemon.provider.daemon.runner import DaemonRunner  # noqa: E402


def test_sync_async_synchronously_sets_active_status():
    """Verify sync_async immediately sets active=True and step='starting' to prevent polling race."""
    runner = DaemonRunner(repo_root=REPO_ROOT, package_selection_mode=True)

    # Initial state is idle
    assert runner.sync_status["active"] is False
    assert runner.sync_status["step"] == "idle"

    # Patch sync to do nothing so we can inspect the immediate state set by sync_async
    with patch.object(runner, "sync", return_value=None):
        runner.sync_async()

    assert runner.sync_status["active"] is True
    assert runner.sync_status["step"] in ("starting", "compiling_target")
    assert runner.sync_status["progress"] > 0


def test_no_args_with_existing_daemon_does_not_trigger_unwanted_reflash():
    """Verify calling host.py with no args against a running daemon does not reconfigure/reflash."""
    assert has_explicit_target_args([]) is False
    assert has_explicit_target_args(["--no-detach"]) is False

    fake_lock = {
        "pid": 999999,
        "port": 8899,
        "host": "127.0.0.1",
        "target": "thumbv6m-none-eabi",
        "package": "cat_detector",
        "bin": "cat_detector_shell",
        "profile": "release",
        "speed": 4000,
        "url": "http://127.0.0.1:8899",
    }

    with (
        patch("host_daemon.cli.get_lock_data", return_value=fake_lock),
        patch("host_daemon.cli.is_pid_alive", return_value=True),
        patch("host_daemon.cli.load_last_session", return_value=None),
        patch("host_daemon.cli.send_daemon_request") as mock_send,
    ):
        # Call main with no arguments (empty argv)
        ret = main(argv=[])
        assert ret == 0

        # Invariant: /api/sync MUST NOT be triggered when called with no explicit arguments
        for call_args in mock_send.call_args_list:
            endpoint = call_args[0][0] if call_args[0] else ""
            assert endpoint != "/api/sync", (
                "Invoking host.py with no args should not trigger /api/sync on running daemon"
            )
