"""Regression unit test for code review findings a5bbb33b and cfc27a10.

Verifies:
1. a5bbb33b: host_daemon directory hierarchy mirrors tools/dashboard (model/, provider/, templates/, static/, cli.py).
2. a5bbb33b: Canonical imports work cleanly from model and provider packages without legacy backward-compatibility shims.
3. cfc27a10: format_clickable_choice formats interactive prompt choices with OSC 8 hyperlinks and ANSI colors.
4. cfc27a10 / Component 4B: HostDaemonServer renders structured #app-boot-config JSON with zero inline HTML event handlers and serves /js/daemon/daemon_app.js ES module.
"""

import json
from pathlib import Path
import sys
import threading
import urllib.request

from bs4 import BeautifulSoup

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))
_tools_dir = REPO_ROOT / "tools"
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

from host_daemon.cli import format_clickable_choice
from host_daemon.model.manual_prompt import ManualPromptState
from host_daemon.provider.daemon.runner import DaemonRunner
from host_daemon.provider.daemon.server import HostDaemonServer


def test_cr_a5bbb33b_host_daemon_library_structure():
    """Verify host_daemon follows the modular dashboard hierarchy (model/, provider/, templates/, cli.py)."""
    hd_dir = REPO_ROOT / "tools" / "host_daemon"
    assert (hd_dir / "model").is_dir(), "tools/host_daemon/model directory must exist"
    assert (hd_dir / "model" / "manual_prompt.py").is_file(), "tools/host_daemon/model/manual_prompt.py must exist"
    assert (hd_dir / "provider").is_dir(), "tools/host_daemon/provider directory must exist"
    assert not (hd_dir / "provider" / "bringup").exists(), "Legacy bringup_manager must be ported out to device_tests"
    assert (hd_dir / "provider" / "daemon" / "runner.py").is_file(), "runner.py must exist in provider/daemon/"
    assert (hd_dir / "provider" / "daemon" / "server.py").is_file(), "server.py must exist in provider/daemon/"
    assert (hd_dir / "templates" / "host_daemon.html.j2").is_file(), "templates/host_daemon.html.j2 must exist"
    assert (hd_dir / "cli.py").is_file(), "cli.py must exist at host_daemon root"

    # Enforce Rule 3: No legacy flat shims remain
    assert not (hd_dir / "bringup.py").exists(), "Legacy tools/host_daemon/bringup.py must not exist"
    assert not (hd_dir / "runner.py").exists(), "Legacy tools/host_daemon/runner.py must not exist"
    assert not (hd_dir / "server.py").exists(), "Legacy tools/host_daemon/server.py must not exist"


def test_cr_a5bbb33b_model_and_provider_classes():
    """Verify ManualPromptState, DaemonRunner, and HostDaemonServer can be instantiated and operated."""
    prompt_state = ManualPromptState()
    assert prompt_state.active is False
    activated = prompt_state.apply_update(
        {
            "active": True,
            "test_name": "test_motor_calibration_routine",
            "project": "cat_detector",
            "procedure": "Fill reservoir",
            "prompt": "Confirm steady flow",
        }
    )
    assert activated is True
    assert prompt_state.heartbeat_count == 1

    runner = DaemonRunner(repo_root=REPO_ROOT)
    assert isinstance(runner.manual_prompt, ManualPromptState)
    server = HostDaemonServer("127.0.0.1", 0, runner)
    assert server is not None
    server.server_close()


def test_cr_cfc27a10_format_clickable_choices():
    """Verify format_clickable_choice formats options with OSC 8 hyperlinks and ANSI colors."""
    p_pass = format_clickable_choice("Pass", "P", "pass", "\033[1;32m")
    assert "\033]8;;command:pass\033\\" in p_pass
    assert "[P]ass" in p_pass
    assert "\033]8;;\033\\" in p_pass
    assert "\033[1;32m" in p_pass

    p_fail = format_clickable_choice("Fail", "F", "fail", "\033[1;31m")
    assert "\033]8;;command:fail\033\\" in p_fail
    assert "[F]ail" in p_fail
    assert "\033]8;;\033\\" in p_fail
    assert "\033[1;31m" in p_fail

    p_skip = format_clickable_choice("Skip", "S", "skip", "\033[1;33m")
    assert "\033]8;;command:skip\033\\" in p_skip
    assert "[S]kip" in p_skip
    assert "\033]8;;\033\\" in p_skip
    assert "\033[1;33m" in p_skip


def test_host_daemon_renders_boot_config_json_and_serves_es_modules():
    """Verify HostDaemonServer emits structured #app-boot-config JSON with zero inline handlers and serves /js/daemon/*."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        package="cat_detector",
        bin_name="cat_detector_shell",
        target="thumbv6m-none-eabi",
        release=True,
        speed=4000,
    )
    server = HostDaemonServer("127.0.0.1", 0, runner)
    port = server.server_address[1]
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()

    try:
        with urllib.request.urlopen(f"http://127.0.0.1:{port}/", timeout=5) as resp:
            assert resp.status == 200
            html = resp.read().decode("utf-8")

        soup = BeautifulSoup(html, "html.parser")
        boot_el = soup.find("script", id="app-boot-config")
        assert boot_el is not None, "Missing <script id='app-boot-config' type='application/json'>"
        assert boot_el.get("type") == "application/json"

        boot_cfg = json.loads(boot_el.text)
        assert boot_cfg["package"] == "cat_detector"
        assert boot_cfg["binName"] == "cat_detector_shell"
        assert boot_cfg["target"] == "thumbv6m-none-eabi"
        assert boot_cfg["release"] is True
        assert boot_cfg["speed"] == 4000

        module_script = soup.find("script", attrs={"type": "module", "src": "/js/daemon/daemon_app.js"})
        assert module_script is not None, "Missing <script type='module' src='/js/daemon/daemon_app.js'>"
        assert soup.find("script", attrs={"src": "/static/common.js"}) is None

        for tag in soup.find_all(True):
            inline_handlers = [attr for attr in tag.attrs if attr.lower().startswith("on")]
            assert not inline_handlers, f"Found forbidden inline event handler {inline_handlers} on <{tag.name}>"

        for mod_name in (
            "daemon_app.js",
            "rtt_stream.js",
            "cli_console.js",
            "target_config.js",
            "worm_reporter.js",
        ):
            with urllib.request.urlopen(f"http://127.0.0.1:{port}/js/daemon/{mod_name}", timeout=5) as mod_resp:
                assert mod_resp.status == 200
                assert "javascript" in (mod_resp.headers.get("Content-Type") or "")
    finally:
        server.shutdown()
        server.server_close()
