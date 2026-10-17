#!/usr/bin/env python3
"""Regression unit test for WORM-NOCTURNAL-BURROWER-260: Copy icons and text selection preservation.

Verifies that:
1. host_daemon.html.j2 includes copy buttons/icons in the logging monitor and command workstation headers.
2. host_daemon.html.j2 defines copyPanelText and copyActiveWorkstationText with clipboard export.
3. host_daemon.html.j2 preserves partial user text selection during periodic output polling,
   preventing selection resets while the user highlights text before Cmd+C.
"""

from pathlib import Path
import sys

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))


def test_template_copy_icons_present():
    """Verify host_daemon.html.j2 has copy buttons for logging monitor and workstation."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")

    # Panel header copy buttons
    assert 'id="btnCopyLog"' in content
    assert 'id="btnCopyWorkstation"' in content or 'id="btnCopyCli"' in content
    assert "copyPanelText" in content
    assert "copyActiveWorkstationText" in content
    assert "clipboard.writeText" in content


def test_template_selection_preservation_logic():
    """Verify output polling avoids destroying active DOM selection."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")

    # Selection detection check
    assert "isSelectionInside" in content or "getSelection" in content
    # Append-based or change-guarded updates rather than unconditional innerHTML wipe
    assert "lastLogCount" in content
    assert "lastCliCount" in content
    assert "lastFsCount" in content
