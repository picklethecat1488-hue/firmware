#!/usr/bin/env python3
"""Regression unit test for host daemon review feedback: Target CLI availability check and 80% UI scale.

Verifies that:
1. DaemonRunner.execute_cli logs an error and returns an error response when RTT is not connected,
   rather than serving simulated mock help text.
2. host_daemon.html.j2 scales all UI text to 80% to match the Xerxes dashboard HUD aesthetic.
3. host_daemon.html.j2 handles target CLI unavailability by disabling the command workstation.
"""

from pathlib import Path
import sys

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.host_daemon.runner import DaemonRunner


def test_runner_cli_not_available_when_rtt_offline():
    """Verify execute_cli returns an error and logs when RTT is not connected."""
    runner = DaemonRunner(repo_root=REPO_ROOT)
    runner.rtt_process = None

    res = runner.execute_cli("help")
    assert "Error:" in res
    assert "Target CLI is not available" in res
    assert any("Target CLI is not available" in e["message"] for e in runner.logs)


def test_template_ui_scale_and_cli_lockout():
    """Verify host_daemon.html.j2 scales UI to 80% and locks CLI when target unavailable."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")

    assert "html { font-size: 80%; }" in content
    assert "Target CLI is not available" in content
    assert "setWorkstationDisabled" in content
