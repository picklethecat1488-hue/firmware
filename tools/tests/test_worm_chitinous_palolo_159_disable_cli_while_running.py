#!/usr/bin/env python3
"""Regression unit tests for WORM-CHITINOUS-PALOLO-159.

Verifies that the CLI and FS command input windows and buttons are disabled
while a command is executing, preventing duplicate executions (e.g. running 'fs ls'
twice) and serializing backend hardware probe access.
"""

import threading
import time
import sys
from pathlib import Path
import pytest

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))


def test_daemon_runner_guards_execute_fs_during_sync():
    """Verify DaemonRunner rejects execute_fs while target sync is active."""
    from tools.host_daemon.provider.daemon.runner import DaemonRunner

    runner = DaemonRunner(REPO_ROOT, "thumbv6m-none-eabi", "cat_detector", "cat_detector_shell", False, 4000)
    assert hasattr(runner, "_fs_lock"), "DaemonRunner must have _fs_lock"

    # Simulate an active sync thread
    dummy_thread = threading.Thread(target=lambda: time.sleep(0.5))
    dummy_thread.start()
    try:
        runner._sync_thread = dummy_thread
        res = runner.execute_fs("ls")
        assert "Target synchronization is currently in progress" in res
        assert "Error:" in res
    finally:
        dummy_thread.join()


def test_daemon_runner_serializes_execute_fs():
    """Verify DaemonRunner uses _fs_lock to serialize concurrent execute_fs calls."""
    from tools.host_daemon.provider.daemon.runner import DaemonRunner

    runner = DaemonRunner(REPO_ROOT, "thumbv6m-none-eabi", "cat_detector", "cat_detector_shell", False, 4000)

    order = []

    def mock_run_host_fs(fs_args):
        order.append("start")
        time.sleep(0.1)
        order.append("end")
        return 0, "mock fs output"

    runner.run_host_fs = mock_run_host_fs

    t1 = threading.Thread(target=runner.execute_fs, args=("ls",))
    t2 = threading.Thread(target=runner.execute_fs, args=("ls",))

    t1.start()
    t2.start()
    t1.join()
    t2.join()

    # If serialized, order should be start, end, start, end
    assert order == ["start", "end", "start", "end"], f"Calls must not interleave: {order}"
