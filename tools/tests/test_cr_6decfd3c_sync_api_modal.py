#!/usr/bin/env python3
"""Regression unit test for Code Review CR-6decfd3c (CR_6decfd3cf857f7d8cb849df1bf4710a5567d3804.md).

Verifies that:
1. tools/host_daemon/runner.py provides sync(), sync_async(), and sync_status tracking.
2. tools/host_daemon/server.py exposes POST /api/sync and GET /api/sync/status endpoints.
3. tools/host_daemon/templates/host_daemon.html.j2 provides:
   - A Sync button in the header.
   - A modal flashing status window with progress bar and target info.
   - JavaScript functions to trigger and poll target synchronization.
4. HostDaemonServer correctly responds to /api/sync and /api/sync/status.
"""

import json
from pathlib import Path
import sys
import threading
import time
import urllib.request

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.host_daemon.runner import DaemonRunner
from tools.host_daemon.server import HostDaemonServer


def test_cr_6decfd3c_template_modal_and_sync_button():
    """Verify host_daemon template has Sync button and modal flashing status window."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    assert template_path.exists(), f"File not found: {template_path}"
    content = template_path.read_text(encoding="utf-8")

    # Invariant 1: Sync button
    assert "btnSync" in content or "triggerSync" in content
    assert "Sync" in content

    # Invariant 2: Modal status window with progress bar
    assert "syncModal" in content or "sync-modal" in content
    assert "syncProgressBar" in content or "progress-bar" in content
    assert "pulse" in content or "flashing" in content.lower()

    # Invariant 3: JS functions for sync
    assert "triggerSync" in content
    assert "/api/sync" in content


def test_cr_6decfd3c_runner_sync_api_and_status():
    """Verify DaemonRunner provides sync methods and status tracking."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
    )

    assert hasattr(runner, "sync_status"), "DaemonRunner must track sync_status"
    assert "progress" in runner.sync_status
    assert "step" in runner.sync_status
    assert hasattr(runner, "sync"), "DaemonRunner must implement sync()"
    assert hasattr(runner, "sync_async"), "DaemonRunner must implement sync_async()"


def test_cr_6decfd3c_server_sync_endpoint():
    """Verify HostDaemonServer responds to /api/sync and /api/sync/status."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
    )
    server = HostDaemonServer("127.0.0.1", 0, runner)
    port = server.server_address[1]

    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()

    try:
        # Check GET /api/sync/status
        req = urllib.request.Request(f"http://127.0.0.1:{port}/api/sync/status")
        with urllib.request.urlopen(req, timeout=5) as resp:
            assert resp.status == 200
            data = json.loads(resp.read().decode("utf-8"))
            assert "progress" in data
            assert "target" in data

        # Check POST /api/sync
        post_req = urllib.request.Request(
            f"http://127.0.0.1:{port}/api/sync",
            data=b"{}",
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urllib.request.urlopen(post_req, timeout=5) as resp:
            assert resp.status == 200
            data = json.loads(resp.read().decode("utf-8"))
            assert data.get("success") is True
            assert "sync" in data
    finally:
        server.shutdown()
