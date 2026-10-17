#!/usr/bin/env python3
"""Regression unit test for WORM-SLIMY-SNAIL-582.

Verifies that:
1. tools/host_daemon.py and tools/host_daemon/cli.py ensure repo root is in sys.path
   so that native webview and eel launchers can be imported without 'No module named tools' error.
2. The ASCII box banner is removed from host_daemon startup console output,
   matching the compact output format of dashboard.py:
   '➜ Host daemon running at http://...'
   '➜ Detached from terminal.'
"""

import os
from pathlib import Path
import subprocess
import sys

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))


def test_worm_slimy_snail_582_webview_import_without_pythonpath():
    """Verify host_daemon launcher can import dashboard webview modules without PYTHONPATH set."""
    host_daemon_py = REPO_ROOT / "tools" / "host_daemon.py"
    assert host_daemon_py.exists(), f"File not found: {host_daemon_py}"

    # Run python snippet in a subprocess with empty PYTHONPATH
    env = os.environ.copy()
    env.pop("PYTHONPATH", None)

    code = (
        "import sys\n"
        f"from pathlib import Path\n"
        f"sys.path.insert(0, str(Path('{host_daemon_py}').resolve().parent))\n"
        "import host_daemon\n"
        "from host_daemon.cli import launch_host_daemon_app\n"
        "assert callable(launch_host_daemon_app)\n"
        "from tools.dashboard.provider.webview.launcher import launch_webview\n"
        "assert callable(launch_webview)\n"
    )

    proc = subprocess.run(
        [sys.executable, "-c", code],
        cwd=str(REPO_ROOT),
        capture_output=True,
        text=True,
        env=env,
    )
    assert proc.returncode == 0, f"Import failed with clean sys.path: {proc.stderr}"


def test_worm_slimy_snail_582_no_box_banner_in_output():
    """Verify that host_daemon startup output does not include the ASCII box banner."""
    cli_py = REPO_ROOT / "tools" / "host_daemon" / "cli.py"
    cli_code = cli_py.read_text(encoding="utf-8")

    # Invariant: The ASCII header banner must be removed
    assert "======================================================================" not in cli_code
    assert "  ⚡ SYSTEM SHOCK 2 // XERXES HOST DAEMON WORKSTATION" not in cli_code
