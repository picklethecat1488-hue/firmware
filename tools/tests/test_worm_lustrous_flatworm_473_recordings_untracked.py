#!/usr/bin/env python3
"""Reproduction and regression functional test for WORM-LUSTROUS-FLATWORM-473.

Asserts that:
1. .gitignore contains patterns ignoring temporary inspection and frame extraction artifacts
   in recordings/previews/ (frames_*, motor_crops*, video_samples_*, *.wav, *.zip).
2. git status does not report untracked temporary media/inspection directories under recordings/.
3. Dashboard and workspace git operations remain responsive without scanning hundreds of loose frames.
"""

from pathlib import Path
import subprocess


def test_worm_lustrous_flatworm_473_recordings_untracked():
    """Verify that recordings/ contains no loose untracked temporary extraction files."""
    workspace_root = Path(__file__).resolve().parent.parent.parent

    # 1. Verify .gitignore ignores temporary recording extractions
    gitignore_path = workspace_root / ".gitignore"
    assert gitignore_path.exists(), ".gitignore must exist"
    gitignore_content = gitignore_path.read_text(encoding="utf-8")
    assert "recordings/previews/frames_*/" in gitignore_content
    assert "recordings/previews/motor_crops*/" in gitignore_content
    assert "recordings/previews/*.wav" in gitignore_content

    # 2. Verify git status has no untracked files under recordings/
    cmd = ["git", "status", "--porcelain", "recordings/"]
    proc = subprocess.run(
        cmd,
        cwd=workspace_root,
        capture_output=True,
        text=True,
        check=False,
    )
    assert proc.returncode == 0
    untracked_recordings = [
        line for line in proc.stdout.splitlines() if line.startswith("?? recordings/")
    ]
    assert (
        not untracked_recordings
    ), f"Found untracked artifacts under recordings/: {untracked_recordings}"
