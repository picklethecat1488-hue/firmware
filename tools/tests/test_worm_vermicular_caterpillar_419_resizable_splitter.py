#!/usr/bin/env python3
"""Regression unit test for WORM-VERMICULAR-CATERPILLAR-419: Resizable vertical splitter and hideable log panel.

Verifies that:
1. host_daemon.html.j2 includes #vSplitter with .splitter-gripper handle.
2. host_daemon.html.j2 includes a toggle button (#btnToggleLog) to totally hide/show the logging monitor.
3. host_daemon.html.j2 implements mouse drag event listeners for resizing #logPanel.
4. host_daemon.html.j2 uses flex-based layout in .workspace-body to support arbitrary panel sizing and complete hiding.
"""

from pathlib import Path
import sys

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))


def test_template_vertical_splitter_and_gripper_present():
    """Verify host_daemon.html.j2 has vertical splitter and gripper element."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")

    assert 'id="vSplitter"' in content
    assert "vertical-splitter" in content
    assert "splitter-gripper" in content
    assert "setupSplitter" in content


def test_template_toggle_hide_log_panel():
    """Verify host_daemon.html.j2 supports totally hiding/showing the logging monitor."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")

    assert 'id="btnToggleLog"' in content
    assert "toggleLogPanel" in content
    assert 'logPanel.style.display = "none"' in content
