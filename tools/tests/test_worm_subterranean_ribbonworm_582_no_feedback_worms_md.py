#!/usr/bin/env python3
"""Regression test for WORM-SUBTERRANEAN-RIBBONWORM-582.

Verifies that the monolithic feedback/WORMS.md file is removed from the git repository,
ensuring that only granular individual WORM_<id>.md files exist in feedback/ and that
the dashboard server defaults aggregated markdown export to target/WORMS.md.
"""

from pathlib import Path
import subprocess
import sys


def test_worm_subterranean_ribbonworm_582_no_feedback_worms_md_in_repo():
    """Verify feedback/WORMS.md does not exist in workspace or git tracking."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    feedback_worms_md = workspace_root / "feedback" / "WORMS.md"

    # Invariant 1: feedback/WORMS.md must not exist on disk
    assert not feedback_worms_md.exists(), (
        "feedback/WORMS.md must not exist in the repository! "
        "Only granular WORM_<id>.md files should be tracked in feedback/."
    )

    # Invariant 2: feedback/WORMS.md must not be tracked in git index
    res = subprocess.run(
        ["git", "ls-files", "feedback/WORMS.md"],
        cwd=workspace_root,
        capture_output=True,
        text=True,
        check=True,
    )
    assert not res.stdout.strip(), "feedback/WORMS.md must not be tracked by git!"


def test_worm_subterranean_ribbonworm_582_dashboard_list_worms_works_without_feedback_worms_md():
    """Verify tools/dashboard.py list-worms functions properly without feedback/WORMS.md."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cmd = [sys.executable, str(workspace_root / "tools" / "dashboard.py"), "list-worms"]

    res = subprocess.run(cmd, cwd=workspace_root, capture_output=True, text=True, check=False)
    assert res.returncode == 0, f"dashboard.py list-worms failed: {res.stderr}"
    assert "SYSTEM SHOCK 2 // XERXES WORM MATRIX" in res.stdout

    # Verify running list-worms did NOT recreate feedback/WORMS.md
    assert not (workspace_root / "feedback" / "WORMS.md").exists(), (
        "Running dashboard list-worms must not recreate feedback/WORMS.md."
    )
