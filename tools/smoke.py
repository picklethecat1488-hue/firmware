#!/usr/bin/env python3
"""End-to-end smoke tests for firmware debug builds, Forge/Host HTTP workstations, and JS suites."""

from __future__ import annotations

import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import threading
from typing import Any, Dict, Optional, Tuple
import urllib.request

import pytest

REPO_ROOT = Path(__file__).resolve().parent.parent
TOOLS_DIR = REPO_ROOT / "tools"
DASHBOARD_DIR = TOOLS_DIR / "dashboard"
DAEMON_DIR = TOOLS_DIR / "host_daemon"

for _p in (str(REPO_ROOT), str(TOOLS_DIR), str(DASHBOARD_DIR), str(DAEMON_DIR)):
    if _p not in sys.path:
        sys.path.insert(0, _p)

LOCALHOST = "127.0.0.1"
EPHEMERAL_PORT = 0
HTTP_OK = 200
HTTP_TIMEOUT_SECONDS = 20.0
MAX_NODE_CONCURRENCY = 8


def _http_get(url: str) -> Tuple[int, Dict[str, str], bytes]:
    """Perform a local HTTP GET request and return ``(status, lower_headers, body)``."""
    req = urllib.request.Request(url, headers={"Accept-Encoding": "identity"})
    with urllib.request.urlopen(req, timeout=HTTP_TIMEOUT_SECONDS) as resp:
        headers = {k.lower(): v for k, v in resp.headers.items()}
        return resp.status, headers, resp.read()


class TestFirmwareSmoke:
    """Executes firmware/host builds, Forge & Host Daemon HTTP workstations, and JS unit suites."""

    root_dir = REPO_ROOT

    def run_python(
        self, args: list[str], extra_env: Optional[dict[str, str]] = None
    ) -> subprocess.CompletedProcess[str]:
        """Execute a Python tool script from the firmware repository root."""
        env = os.environ.copy()
        if extra_env:
            env.update(extra_env)
        cmd = [sys.executable, *args]
        res = subprocess.run(cmd, cwd=self.root_dir, capture_output=True, text=True, env=env, check=False)
        assert res.returncode == 0, f"Command failed: {' '.join(cmd)}\nSTDOUT:\n{res.stdout}\nSTDERR:\n{res.stderr}"
        return res

    def test_debug_firmware_and_host_builds(self) -> None:
        """Verify debug embedded firmware and host CLI/FS binaries build cleanly."""
        fw_res = subprocess.run(
            ["./tools/build/build_firmware.sh", "--debug-only"],
            cwd=self.root_dir,
            capture_output=True,
            text=True,
            check=False,
        )
        assert fw_res.returncode == 0, f"Firmware debug build failed:\n{fw_res.stdout}\n{fw_res.stderr}"

        host_res = subprocess.run(
            ["./tools/build/build_host_tools.sh", "--debug"],
            cwd=self.root_dir,
            capture_output=True,
            text=True,
            check=False,
        )
        assert host_res.returncode == 0, f"Host tools debug build failed:\n{host_res.stdout}\n{host_res.stderr}"

    def test_forge_workstation_and_js_diff_endpoints(self, tmp_path: Path) -> None:
        """Verify forge.py CLI subcommands, DashboardServer UI routes, and JS file diff visibility."""
        from provider.dashboard.server import DashboardServer

        self.run_python(["tools/forge.py", "--help"])
        self.run_python(["tools/forge.py", "list-commits"])
        self.run_python(["tools/forge.py", "list-worms"])
        self.run_python(["tools/forge.py", "list-reviews"])

        fb_dir = tmp_path / "feedback"
        fb_dir.mkdir(parents=True, exist_ok=True)
        server = DashboardServer(
            host=LOCALHOST,
            port=EPHEMERAL_PORT,
            repo_root=self.root_dir,
            sqlite_worm_file=tmp_path / "worms.sqlite",
            sqlite_review_file=tmp_path / "code_review.sqlite",
            markdown_worms_path=fb_dir / "WORMS.md",
            markdown_review_path=fb_dir / "CR.md",
            feedback_dir=fb_dir,
        )
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        base_url = server.get_url()

        try:
            for route in ("/", "/review", "/worms"):
                st, _, body = _http_get(f"{base_url}{route}")
                assert st == HTTP_OK
                html = body.decode("utf-8")
                assert 'id="app-boot-config"' in html
                assert "onclick=" not in html

            rt_status, js_headers, rt_bytes = _http_get(f"{base_url}/js/core/runtime_context.js")
            assert rt_status == HTTP_OK
            assert "javascript" in js_headers.get("content-type", "")
            assert b"getRuntimeContext" in rt_bytes

            for js_rel in ("tools/js/core/runtime_context.js", "tools/js/daemon/daemon_app.js"):
                diff_status, _, diff_bytes = _http_get(f"{base_url}/api/diff?commit=working&file={js_rel}")
                assert diff_status == HTTP_OK
                diff_json: Dict[str, Any] = json.loads(diff_bytes.decode("utf-8"))
                assert diff_json["file_path"] == js_rel

                raw_status, _, raw_bytes = _http_get(f"{base_url}/api/raw?commit=working&file={js_rel}")
                assert raw_status == HTTP_OK
                assert len(raw_bytes) > 0
        finally:
            server.shutdown()
            server.server_close()
            thread.join(timeout=5.0)

    def test_host_daemon_workstation_endpoints(self) -> None:
        """Verify host.py CLI and HostDaemonServer serve shared /js/* modules and status APIs."""
        from host_daemon.provider.daemon.runner import DaemonRunner
        from host_daemon.provider.daemon.server import HostDaemonServer

        self.run_python(["tools/host.py", "--help"])
        status_proc = subprocess.run(
            [sys.executable, "tools/host.py", "status"],
            cwd=self.root_dir,
            capture_output=True,
            text=True,
            check=False,
        )
        assert status_proc.returncode in (0, 1)

        runner = DaemonRunner(repo_root=self.root_dir, package_selection_mode=True)
        server = HostDaemonServer(LOCALHOST, EPHEMERAL_PORT, runner)
        actual_port = server.server_address[1]
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        base_url = f"http://{LOCALHOST}:{actual_port}"

        try:
            ui_status, _, ui_bytes = _http_get(f"{base_url}/")
            assert ui_status == HTTP_OK
            ui_html = ui_bytes.decode("utf-8")
            assert 'id="app-boot-config"' in ui_html
            assert "/js/daemon/daemon_app.js" in ui_html
            assert "onclick=" not in ui_html

            for asset_path in ("/js/daemon/daemon_app.js", "/js/core/runtime_context.js"):
                js_status, js_headers, js_bytes = _http_get(f"{base_url}{asset_path}")
                assert js_status == HTTP_OK
                assert "javascript" in js_headers.get("content-type", "")
                assert len(js_bytes) > 0

            for api_path in ("/api/status", "/api/logs", "/api/bringup/scripts", "/api/targets"):
                api_status, api_headers, api_bytes = _http_get(f"{base_url}{api_path}")
                assert api_status == HTTP_OK
                assert "application/json" in api_headers.get("content-type", "")
                json.loads(api_bytes.decode("utf-8"))
        finally:
            runner.stop()
            server.shutdown()
            server.server_close()
            thread.join(timeout=5.0)

    def test_validation_and_parallel_js_unit_tests(self) -> None:
        """Verify validate_js.py and parallel Node ES module unit tests pass 100%."""
        self.run_python(["tools/validation/validate_js.py"])

        node_bin = shutil.which("node")
        assert node_bin is not None, "node executable must be installed in environment"
        test_files = sorted(str(p) for p in (self.root_dir / "tools" / "tests" / "js").glob("*.test.mjs"))
        assert len(test_files) > 0
        workers = min(os.cpu_count() or 4, MAX_NODE_CONCURRENCY)
        cmd = [node_bin, "--test", f"--test-concurrency={workers}", *test_files]
        res = subprocess.run(cmd, cwd=self.root_dir, capture_output=True, text=True, check=False)
        assert res.returncode == 0, f"Node unit tests failed:\nSTDOUT:\n{res.stdout}\nSTDERR:\n{res.stderr}"


if __name__ == "__main__":
    sys.exit(pytest.main([__file__, "-s", "-v", "-n", "0"]))
