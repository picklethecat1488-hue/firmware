#!/usr/bin/env python3
"""Regression unit test for Code Review CR-c4bf6929 (CR_6decfd3cf857f7d8cb849df1bf4710a5567d3804.md).

Verifies that:
1. tools/host_daemon/static/ contains dedicated application icons:
   - app.icns, favicon.ico, icon-512.png, favicon.svg
   - Distinct from dashboard's code review icons.
2. tools/host_daemon/cli.py resolves dedicated host daemon application icon.
3. tools/host_daemon/cli.py supports pywebview and eel display modes:
   - --browser {webview,eel}
   - --webview, --eel, --app aliases
   - --no-detach, --foreground flags
4. Default behavior detaches from terminal when run without --no-detach.
5. launch_host_daemon_app opens a dedicated window using webview or eel.
"""

from pathlib import Path
import sys
from unittest.mock import MagicMock, patch

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.host_daemon.cli import (
    get_host_daemon_icon_path,
    launch_host_daemon_app,
    parse_arguments,
)


def test_cr_c4bf6929_dedicated_app_icons_exist():
    """Verify dedicated application icon assets exist for host daemon."""
    static_dir = REPO_ROOT / "tools" / "host_daemon" / "static"
    assert static_dir.exists(), f"Static directory not found: {static_dir}"

    icns_path = static_dir / "app.icns"
    ico_path = static_dir / "favicon.ico"
    png_path = static_dir / "icon-512.png"
    svg_path = static_dir / "favicon.svg"

    assert icns_path.exists(), f"app.icns missing at {icns_path}"
    assert icns_path.stat().st_size > 1000, "app.icns must not be empty"

    assert ico_path.exists(), f"favicon.ico missing at {ico_path}"
    assert png_path.exists(), f"icon-512.png missing at {png_path}"
    assert svg_path.exists(), f"favicon.svg missing at {svg_path}"

    # Verify that the icon is distinct from dashboard's icon
    dashboard_icns = REPO_ROOT / "tools" / "dashboard" / "provider" / "code_review" / "static" / "app.icns"
    if dashboard_icns.exists():
        assert icns_path.read_bytes() != dashboard_icns.read_bytes(), (
            "Host daemon must use a distinct application icon from dashboard."
        )


def test_cr_c4bf6929_icon_path_resolution():
    """Verify get_host_daemon_icon_path resolves platform icons."""
    with patch("sys.platform", "darwin"):
        icon = get_host_daemon_icon_path()
        assert icon is not None
        assert icon.name == "app.icns"

    with patch("sys.platform", "win32"):
        icon = get_host_daemon_icon_path()
        assert icon is not None
        assert icon.name == "favicon.ico"

    with patch("sys.platform", "linux"):
        icon = get_host_daemon_icon_path()
        assert icon is not None
        assert icon.name in ("icon-512.png", "favicon.ico")


def test_cr_c4bf6929_cli_arguments():
    """Verify host_daemon argument parser supports webview, eel, and detach flags."""
    # Default browser is webview
    args = parse_arguments([])
    assert args.browser == "webview"
    assert args.no_detach is False

    # --eel sets browser=eel
    args_eel = parse_arguments(["--eel"])
    assert args_eel.browser == "eel"

    # --webview sets browser=webview
    args_wv = parse_arguments(["--webview"])
    assert args_wv.browser == "webview"

    # --no-detach / --foreground sets no_detach=True
    args_nodetach = parse_arguments(["--no-detach"])
    assert args_nodetach.no_detach is True

    args_fg = parse_arguments(["--foreground"])
    assert args_fg.no_detach is True


def test_cr_c4bf6929_launch_app_webview():
    """Verify launch_host_daemon_app invokes launch_webview with icon and title."""
    with patch("tools.dashboard.provider.webview.launcher.launch_webview") as mock_launch:
        mock_launch.return_value = True
        res = launch_host_daemon_app("http://127.0.0.1:8899", browser="webview")
        assert res is True
        assert mock_launch.called
        kwargs = mock_launch.call_args[1]
        assert "url" in kwargs
        assert "HOST DAEMON" in kwargs.get("title", "")
        assert kwargs.get("icon_path") is not None
        assert kwargs["icon_path"].name in ("app.icns", "favicon.ico", "icon-512.png")


def test_cr_c4bf6929_launch_app_eel():
    """Verify launch_host_daemon_app invokes launch_eel when requested."""
    with patch("tools.dashboard.provider.eel.launcher.launch_eel") as mock_launch:
        mock_launch.return_value = True
        res = launch_host_daemon_app("http://127.0.0.1:8899", browser="eel")
        assert res is True
        assert mock_launch.called
