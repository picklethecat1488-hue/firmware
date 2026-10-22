#!/usr/bin/env python3
"""Regression unit test for WORM-SUPPLE-PUPA-464.

Verifies that:
1. tools/host.py exists and supports command line arguments:
   --target, --package, --bin, --release, --port, --speed, --no-browser, etc.
2. tools/host.py supports forwarding subcommands:
   'cli' and 'fs' to forward commands to an active running daemon.
3. Server implementation in tools/host_daemon/ exposes REST endpoints:
   /api/status, /api/logs, /api/cli/execute, /api/fs/execute, /api/mode.
4. Web UI template provides:
   - Dropdown selector for active mode (CLI or FS).
   - Separate logging view panel (defmt system logs).
   - Separate CLI output view panel.
5. REST API correctly handles command dispatching and status reporting.
"""

import json
from pathlib import Path
import subprocess
import sys
import threading
import time
import urllib.request

from host_daemon.provider.daemon.runner import DaemonRunner
from host_daemon.provider.daemon.server import HostDaemonServer


def test_worm_supple_pupa_464_host_daemon_exists():
    """Verify tools/host.py exists and is executable/importable."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    host_py = workspace_root / "tools" / "host.py"

    assert host_py.exists(), f"File not found: {host_py}"

    # Verify python tools/host.py --help succeeds
    proc = subprocess.run(
        [sys.executable, str(host_py), "--help"],
        capture_output=True,
        text=True,
    )
    assert proc.returncode == 0, f"host.py --help failed: {proc.stderr}"
    assert "--target" in proc.stdout
    assert "--package" in proc.stdout
    assert "--bin" in proc.stdout
    assert "cli" in proc.stdout
    assert "fs" in proc.stdout


def test_worm_supple_pupa_464_host_daemon_server_and_ui():
    """Verify host_daemon server and UI template meet specification."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    daemon_dir = workspace_root / "tools" / "host_daemon"
    template_path = daemon_dir / "templates" / "host_daemon.html.j2"

    assert daemon_dir.exists(), f"Directory not found: {daemon_dir}"
    assert template_path.exists(), f"File not found: {template_path}"

    content = template_path.read_text(encoding="utf-8")

    # Invariant: Dropdown selector for CLI and FS modes
    assert "modeSelect" in content or "mode-select" in content, (
        "UI template must contain a dropdown selector for active mode."
    )
    assert "CLI" in content and "FS" in content

    # Invariant: Separate logging and CLI output view panels
    assert "log-panel" in content or "logPanel" in content, "UI template must contain a logging view panel."
    assert "cli-panel" in content or "cliPanel" in content or "console-panel" in content, (
        "UI template must contain a CLI output view panel."
    )


def test_worm_supple_pupa_464_daemon_rest_endpoints():
    """Verify DaemonRunner and HostDaemonServer REST endpoints."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    runner = DaemonRunner(
        repo_root=workspace_root,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
        release=False,
    )

    # Use port 0 or high unused port for testing
    server = HostDaemonServer("127.0.0.1", 0, runner)
    host, port = server.server_address

    server_thread = threading.Thread(target=server.serve_forever, daemon=True)
    server_thread.start()

    try:
        base_url = f"http://{host}:{port}"

        # 1. Test GET / renders UI HTML
        with urllib.request.urlopen(f"{base_url}/") as resp:
            assert resp.status == 200
            html = resp.read().decode("utf-8")
            assert "XERXES HOST DAEMON" in html
            assert "modeSelect" in html

        # 2. Test GET /api/status
        with urllib.request.urlopen(f"{base_url}/api/status") as resp:
            assert resp.status == 200
            status_data = json.loads(resp.read().decode("utf-8"))
            assert status_data["target"] == "thumbv6m-none-eabi"
            assert status_data["package"] == "cat_detector"
            assert status_data["bin"] == "cat_detector_shell"
            assert status_data["mode"] == "CLI"

        # 3. Test POST /api/mode
        req = urllib.request.Request(
            f"{base_url}/api/mode",
            data=json.dumps({"mode": "FS"}).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urllib.request.urlopen(req) as resp:
            assert resp.status == 200
            data = json.loads(resp.read().decode("utf-8"))
            assert data["success"] is True
            assert data["mode"] == "FS"
            assert runner.active_mode == "FS"

        # 4. Test POST /api/cli/execute
        req = urllib.request.Request(
            f"{base_url}/api/cli/execute",
            data=json.dumps({"command": "gpio status"}).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urllib.request.urlopen(req) as resp:
            assert resp.status == 200
            data = json.loads(resp.read().decode("utf-8"))
            assert data["success"] is True
            assert "output" in data

        # 5. Test GET /api/cli/output
        with urllib.request.urlopen(f"{base_url}/api/cli/output") as resp:
            assert resp.status == 200
            outputs = json.loads(resp.read().decode("utf-8"))
            assert len(outputs) > 0
            assert any("gpio status" in o.get("text", "") for o in outputs)

        # 6. Test GET /api/logs
        runner.log("Testing defmt system log", level="INFO", channel="defmt")
        with urllib.request.urlopen(f"{base_url}/api/logs") as resp:
            assert resp.status == 200
            logs = json.loads(resp.read().decode("utf-8"))
            assert len(logs) > 0
            assert any("Testing defmt system log" in l.get("message", "") for l in logs)

    finally:
        server.shutdown()
        server_thread.join(timeout=2)


def test_worm_luminescent_palolo_130_host_py_rename():
    """Verify WORM-LUMINESCENT-PALOLO-130: tools/host.py canonical entrypoint exists and host_daemon.py is purged."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    host_py = workspace_root / "tools" / "host.py"
    host_daemon_py = workspace_root / "tools" / "host_daemon.py"

    assert host_py.exists(), f"Canonical entrypoint not found: {host_py}"
    assert not host_daemon_py.exists(), f"Obsolete host_daemon.py must be purged: {host_daemon_py}"

    # Verify python tools/host.py --help succeeds
    proc = subprocess.run(
        [sys.executable, str(host_py), "--help"],
        capture_output=True,
        text=True,
    )
    assert proc.returncode == 0, f"tools/host.py --help failed: {proc.stderr}"
    assert "subcommand" in proc.stdout.lower() or "usage:" in proc.stdout.lower()


def test_host_daemon_no_cache_headers_and_compact_font_sizes():
    """Verify host workstation sends Cache-Control: no-cache headers and uses compact font sizes."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    tmpl_path = workspace_root / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    assert tmpl_path.exists()

    tmpl_content = tmpl_path.read_text(encoding="utf-8")
    # Verify compact font sizes matching dashboard aesthetic
    assert ".header-title {\n      font-size: 10.5px;" in tmpl_content
    assert ".panel-header {\n      background: rgba(10, 28, 36, 0.85);\n      border-bottom: 1px solid var(--xerxes-border-mid);\n      padding: 5px 10px;\n      font-size: 9px;" in tmpl_content
    assert ".panel-content {\n      display: block;\n      flex: 1;\n      flex-direction: column;\n      overflow-y: auto;\n      padding: 6px 8px;\n      font-size: 9.5px;" in tmpl_content

    runner = DaemonRunner(
        repo_root=workspace_root,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
        release=False,
    )
    server = HostDaemonServer("127.0.0.1", 0, runner)
    host, port = server.server_address
    server_thread = threading.Thread(target=server.serve_forever, daemon=True)
    server_thread.start()
    try:
        time.sleep(0.1)
        req = urllib.request.Request(f"http://{host}:{port}/")
        with urllib.request.urlopen(req) as resp:
            cache_ctrl = resp.headers.get("Cache-Control", "")
            assert "no-cache" in cache_ctrl
            assert "no-store" in cache_ctrl
            assert "must-revalidate" in cache_ctrl
            assert resp.headers.get("Pragma") == "no-cache"
    finally:
        server.shutdown()
        server_thread.join(timeout=2)


