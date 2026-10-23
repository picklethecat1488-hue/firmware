#!/usr/bin/env python3
"""Regression functional unit test for WORM-SLIMY-ROOTER-821.

Verifies that:
1. In the host UI, system logs are hidden by default (#logPanel display: none).
2. The UI provides an option on startup (#cfgShowLogsCheckbox in target configuration modal
   and --show-system-logs CLI argument) to show system logs if desired.
3. Runner and server track and persist show_system_logs preference.
"""

import json
from pathlib import Path
import sys
from unittest.mock import MagicMock
from bs4 import BeautifulSoup

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))
_tools_dir = REPO_ROOT / "tools"
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

from host_daemon.cli import parse_arguments  # noqa: E402
from host_daemon.provider.daemon.config import has_explicit_target_args  # noqa: E402
from host_daemon.provider.daemon.runner import DaemonRunner  # noqa: E402
from host_daemon.provider.daemon.server import HostDaemonRequestHandler, HostDaemonServer  # noqa: E402


def test_daemon_runner_show_system_logs_default_false():
    """Verify DaemonRunner sets show_system_logs to False by default."""
    runner = DaemonRunner(repo_root=REPO_ROOT)
    assert runner.show_system_logs is False, "show_system_logs must be False by default"


def test_cli_parser_show_system_logs_flag():
    """Verify CLI parser supports --show-system-logs and --show-logs flags."""
    args = parse_arguments([])
    assert hasattr(args, "show_system_logs")
    assert args.show_system_logs is False

    args_flag = parse_arguments(["--show-system-logs"])
    assert args_flag.show_system_logs is True

    args_alias = parse_arguments(["--show-logs"])
    assert args_alias.show_system_logs is True


def test_has_explicit_target_args_ignores_show_system_logs():
    """Verify --show-system-logs does not falsely count as an explicit target specification."""
    assert has_explicit_target_args(["--show-system-logs"]) is False
    assert has_explicit_target_args(["--show-logs"]) is False


def test_template_hides_log_panel_by_default():
    """Verify host_daemon.html.j2 hides logPanel by default and provides cfgShowLogsCheckbox."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")
    soup = BeautifulSoup(content, "html.parser")

    log_panel = soup.find("div", id="logPanel")
    assert log_panel is not None, "Could not find <div id='logPanel'>"
    style = log_panel.get("style", "")
    assert "display: none" in style or "display:none" in style, (
        f"logPanel must have display: none by default in template, got style='{style}'"
    )

    v_splitter = soup.find("div", id="vSplitter")
    assert v_splitter is not None, "Could not find <div id='vSplitter'>"
    splitter_style = v_splitter.get("style", "")
    assert "display: none" in splitter_style or "display:none" in splitter_style, (
        f"vSplitter must have display: none by default in template, got style='{splitter_style}'"
    )

    toggle_btn = soup.find("button", id="btnToggleLog")
    assert toggle_btn is not None, "Could not find <button id='btnToggleLog'>"
    assert "Show Log" in toggle_btn.text, f"btnToggleLog should read 'Show Log' by default, got '{toggle_btn.text}'"

    cfg_show_logs = soup.find("input", id="cfgShowLogsCheckbox")
    assert cfg_show_logs is not None, "Could not find <input id='cfgShowLogsCheckbox'> in target config modal"


def test_api_target_configure_updates_show_system_logs():
    """Verify /api/target/configure accepts and updates show_system_logs."""
    runner = DaemonRunner(repo_root=REPO_ROOT, show_system_logs=False)
    server = HostDaemonServer("127.0.0.1", 0, runner)

    try:
        handler = HostDaemonRequestHandler.__new__(HostDaemonRequestHandler)
        handler.server = server
        handler.path = "/api/target/configure"

        config_body = json.dumps(
            {
                "package": "cat_detector",
                "bin": "cat_detector_shell",
                "show_system_logs": True,
            }
        ).encode("utf-8")

        handler.headers = {"Content-Length": str(len(config_body))}
        handler.rfile = MagicMock()
        handler.rfile.read.return_value = config_body
        handler._send_json = MagicMock()

        handler.do_POST()
        assert runner.show_system_logs is True, "Runner show_system_logs should be updated to True"
    finally:
        server.server_close()
