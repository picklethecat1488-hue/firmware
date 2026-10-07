"""Regression unit test for WORM-016: Privacy and relative paths verification.

Guards against:
1. Leaking personal information (usernames, local user home directories) in git-tracked files.
2. Embedding absolute file URLs or hardcoded local paths in documentation, configs, and skills.
3. Leaking personal information in bug/worm and code report feedback and tracebacks.
"""

from datetime import datetime, timezone
from pathlib import Path
import subprocess
import sys

from model.code_review import CommentModel, ReviewSessionModel, ReviewSeverity, ReviewStatus
from model.worm_report import WormCategory, WormDatabaseModel, WormReportModel, WormSeverity, WormStatus
from provider.code_review.markdown_exporter import MarkdownReviewExporter
from provider.sanitizer import elide_personal_info, get_usernames_to_redact
from provider.worm_report.markdown_exporter import MarkdownWormExporter


WORKSPACE_ROOT = Path(__file__).resolve().parent.parent.parent


def test_no_personal_username_in_tracked_files() -> None:
    """Verify git tracked files do not contain the personal username 'daparker'."""
    cmd = ["git", "grep", "-i", "-I", "daparker"]
    proc = subprocess.run(cmd, cwd=WORKSPACE_ROOT, capture_output=True, text=True)
    assert proc.returncode != 0, (
        f"Found occurrences of 'daparker' in git tracked files:\n{proc.stdout}"
    )


def test_elide_personal_info_tracebacks_and_paths() -> None:
    """Verify elide_personal_info redacts usernames and local home directories."""
    traceback_sample = """
Traceback (most recent call last):
  File "/Users/daparker/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    return subprocess.run(cmd, check=True)
  File "/Users/daparker/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    stdout, stderr = process.communicate(input, timeout=timeout)
FileNotFoundError: [Errno 2] No such file or directory: 'pytest-of-daparker/pytest-123'
"""
    cleaned = elide_personal_info(traceback_sample)
    assert "daparker" not in cleaned
    assert "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py" in cleaned
    assert "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py" in cleaned
    assert "pytest-of-<username>/pytest-123" in cleaned


def test_skills_and_configs_use_relative_paths() -> None:
    """Verify skill files, cargo config, and documentation do not contain absolute file:///Users/ links."""
    # 1. Skill file
    skill_file = WORKSPACE_ROOT / ".agents" / "skills" / "cat-fountain-development" / "SKILL.md"
    assert skill_file.exists(), "SKILL.md must exist"
    skill_content = skill_file.read_text(encoding="utf-8")
    assert "file:///Users/" not in skill_content
    assert "daparker" not in skill_content
    assert "[controller/controllers.toml](controller/controllers.toml)" in skill_content
    assert "[shell.toml](shell.toml)" in skill_content
    assert "Relative Paths & Privacy Hygiene Mandate" in skill_content

    # 2. Cargo config
    cargo_config = WORKSPACE_ROOT / ".cargo" / "config.toml"
    assert cargo_config.exists(), ".cargo/config.toml must exist"
    cargo_content = cargo_config.read_text(encoding="utf-8")
    assert "file:///Users/" not in cargo_content
    assert "daparker" not in cargo_content
    assert 'runner = "tools/runner.sh"' in cargo_content

    # 3. Contributing & Gemini docs
    for doc_name in [
        "CONTRIBUTING.md",
        "GEMINI.md",
        "app/carrier_board.md",
        "app/cat_detector.md",
        "docs/hardware_firmware_codesign.md",
    ]:
        doc_path = WORKSPACE_ROOT / doc_name
        assert doc_path.exists(), f"{doc_name} must exist"
        doc_text = doc_path.read_text(encoding="utf-8")
        assert "file:///Users/" not in doc_text, f"{doc_name} must not contain file:///Users/ links"
        assert "daparker" not in doc_text, f"{doc_name} must not contain personal username"


def test_worm_exporter_automatically_elides_personal_info(tmp_path: Path) -> None:
    """Verify MarkdownWormExporter automatically sanitizes personal info when rendering and exporting."""
    exporter = MarkdownWormExporter(repo_root=tmp_path)
    worm = WormReportModel(
        id="WORM-999",
        uuid="99999999-9999-9999-9999-999999999999",
        title="Test Bug with Personal Info",
        status=WormStatus.OPEN,
        severity=WormSeverity.MEDIUM,
        category=WormCategory.FIRMWARE,
        description="Encountered failure under /Users/daparker/gh/firmware/tools/runner.sh",
        reproduction_steps=["Run pytest under /private/var/folders/pytest-of-daparker/test_1"],
        expected_behavior="Expected clean execution without leaking /Users/daparker paths",
        actual_behavior="Leaked /Users/daparker/miniforge3/env",
        logs='File "/Users/daparker/gh/firmware/tools/cli.py", line 10',
        resolution_notes="Fixed by daparker",
    )
    database = WormDatabaseModel(
        title="Privacy Test Database",
        summary="Summary mentioning /Users/daparker",
        worms=[worm],
    )

    rendered_worm = exporter.render_worm_markdown(worm)
    assert "daparker" not in rendered_worm
    assert "<username>" in rendered_worm

    rendered_db = exporter.render_markdown(database)
    assert "daparker" not in rendered_db
    assert "<username>" in rendered_db

    exported_file = tmp_path / "WORMS.md"
    exporter.export_markdown(database, exported_file, feedback_dir=tmp_path / "feedback")
    content = exported_file.read_text(encoding="utf-8")
    assert "daparker" not in content
    assert "<username>" in content

    exported_worm_file = tmp_path / "feedback" / "WORM_999.md"
    assert exported_worm_file.exists()
    worm_content = exported_worm_file.read_text(encoding="utf-8")
    assert "daparker" not in worm_content
    assert "<username>" in worm_content


def test_code_review_exporter_automatically_elides_personal_info(tmp_path: Path) -> None:
    """Verify MarkdownReviewExporter automatically sanitizes personal info when exporting."""
    exporter = MarkdownReviewExporter(repo_root=tmp_path)
    session = ReviewSessionModel(
        title="Privacy Review",
        verdict=ReviewStatus.CHANGES_REQUESTED,
        summary="Review by daparker for /Users/daparker/gh/firmware",
        comments=[
            CommentModel(
                id="c99",
                uuid="aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
                file_path="app/src/main.rs",
                start_line=1,
                end_line=5,
                severity=ReviewSeverity.MUST_FIX,
                body="Please check /Users/daparker/gh/firmware/app/src/main.rs",
                code_snippet="// Author: daparker\nlet path = '/Users/daparker';",
                created_at=datetime.now(timezone.utc).isoformat(),
                commit="12345678",
            )
        ],
    )

    rendered_cr = exporter.render_markdown(session)
    assert "daparker" not in rendered_cr
    assert "<username>" in rendered_cr

    out_file = tmp_path / "CR.md"
    exporter.export_markdown(session, out_file)
    content = out_file.read_text(encoding="utf-8")
    assert "daparker" not in content
    assert "<username>" in content

    commit_out = exporter.export_commit_markdown(session, "12345678", tmp_path / "feedback")
    assert commit_out is not None and commit_out.exists()
    commit_content = commit_out.read_text(encoding="utf-8")
    assert "daparker" not in commit_content
    assert "<username>" in commit_content
