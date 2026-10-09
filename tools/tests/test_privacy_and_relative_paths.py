"""Regression unit test for WORM-016: Privacy, relative paths, and Windows support verification.

Guards against:
1. Leaking personal information (usernames, local user home directories) in git-tracked files.
2. Embedding absolute file URLs or hardcoded local paths in documentation, configs, and skills.
3. Leaking personal information in bug/worm and code report feedback and tracebacks across Unix and Windows.
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
TEST_USERNAME = "".join(["da", "parker"])


def test_no_personal_username_in_tracked_files() -> None:
    """Verify git tracked files do not contain the personal username."""
    cmd = ["git", "grep", "-i", "-I", TEST_USERNAME]
    proc = subprocess.run(cmd, cwd=WORKSPACE_ROOT, capture_output=True, text=True)
    assert proc.returncode != 0, f"Found occurrences of personal username in git tracked files:\n{proc.stdout}"


def test_elide_personal_info_tracebacks_and_paths() -> None:
    """Verify elide_personal_info redacts usernames and local home directories on POSIX systems."""
    traceback_sample = f"""
Traceback (most recent call last):
  File "/Users/{TEST_USERNAME}/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    return subprocess.run(cmd, check=True)
  File "/Users/{TEST_USERNAME}/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    stdout, stderr = process.communicate(input, timeout=timeout)
FileNotFoundError: [Errno 2] No such file or directory: 'pytest-of-{TEST_USERNAME}/pytest-123'
"""
    cleaned = elide_personal_info(traceback_sample)
    assert TEST_USERNAME not in cleaned
    assert "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py" in cleaned
    assert "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py" in cleaned
    assert "pytest-of-<username>/pytest-123" in cleaned


def test_elide_personal_info_windows_paths() -> None:
    """Verify elide_personal_info redacts Windows paths, backslashes, drive letters, and URLs."""
    win_sample = f"""
Traceback (most recent call last):
  File "C:\\Users\\{TEST_USERNAME}\\gh\\firmware\\tools\\runner.py", line 12, in <module>
    main()
  File "C:/Users/{TEST_USERNAME}/gh/firmware/tools/cli.py", line 45, in main
    test()
FileNotFoundError: [Errno 2] No such file or directory: 'C:\\Users\\{TEST_USERNAME}\\AppData\\Local\\Temp\\pytest-of-{TEST_USERNAME}\\test1'
Link: file:///C:/Users/{TEST_USERNAME}/gh/firmware/app/src/main.rs
Legacy: D:\\Documents and Settings\\{TEST_USERNAME}\\Desktop\\notes.txt
"""
    cleaned = elide_personal_info(win_sample)
    assert TEST_USERNAME not in cleaned
    assert r"C:\Users\<username>\gh\firmware\tools\runner.py" in cleaned
    assert "C:/Users/<username>/gh/firmware/tools/cli.py" in cleaned
    assert r"C:\Users\<username>\AppData\Local\Temp\pytest-of-<username>\test1" in cleaned
    assert "file:///C:/Users/<username>/gh/firmware/app/src/main.rs" in cleaned
    assert r"D:\Documents and Settings\<username>\Desktop\notes.txt" in cleaned


def test_skills_and_configs_use_relative_paths() -> None:
    """Verify skill files, cargo config, and documentation do not contain absolute file:///Users/ links."""
    # 1. Skill file
    skill_file = WORKSPACE_ROOT / ".agents" / "skills" / "cat-fountain-development" / "SKILL.md"
    assert skill_file.exists(), "SKILL.md must exist"
    skill_content = skill_file.read_text(encoding="utf-8")
    assert "file:///Users/" not in skill_content
    assert TEST_USERNAME not in skill_content
    assert "[controller/controllers.toml](controller/controllers.toml)" in skill_content
    assert "[shell.toml](shell.toml)" in skill_content
    assert "Relative Paths & Privacy Hygiene Mandate" in skill_content

    # 2. Cargo config
    cargo_config = WORKSPACE_ROOT / ".cargo" / "config.toml"
    assert cargo_config.exists(), ".cargo/config.toml must exist"
    cargo_content = cargo_config.read_text(encoding="utf-8")
    assert "file:///Users/" not in cargo_content
    assert TEST_USERNAME not in cargo_content
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
        assert TEST_USERNAME not in doc_text, f"{doc_name} must not contain personal username"


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
        description=f"Encountered failure under /Users/{TEST_USERNAME}/gh/firmware/tools/runner.sh",
        reproduction_steps=[f"Run pytest under /private/var/folders/pytest-of-{TEST_USERNAME}/test_1"],
        expected_behavior=f"Expected clean execution without leaking /Users/{TEST_USERNAME} paths",
        actual_behavior=f"Leaked /Users/{TEST_USERNAME}/miniforge3/env",
        logs=f'File "/Users/{TEST_USERNAME}/gh/firmware/tools/cli.py", line 10',
        resolution_notes=f"Fixed by {TEST_USERNAME}",
    )
    database = WormDatabaseModel(
        title="Privacy Test Database",
        summary=f"Summary mentioning /Users/{TEST_USERNAME}",
        worms=[worm],
    )

    rendered_worm = exporter.render_worm_markdown(worm)
    assert TEST_USERNAME not in rendered_worm
    assert "<username>" in rendered_worm

    rendered_db = exporter.render_markdown(database)
    assert TEST_USERNAME not in rendered_db
    assert "<username>" in rendered_db

    exported_file = tmp_path / "WORMS.md"
    exporter.export_markdown(database, exported_file, feedback_dir=tmp_path / "feedback")
    content = exported_file.read_text(encoding="utf-8")
    assert TEST_USERNAME not in content
    assert "<username>" in content

    exported_worm_file = tmp_path / "feedback" / "WORM_999.md"
    assert exported_worm_file.exists()
    worm_content = exported_worm_file.read_text(encoding="utf-8")
    assert TEST_USERNAME not in worm_content
    assert "<username>" in worm_content


def test_code_review_exporter_automatically_elides_personal_info(tmp_path: Path) -> None:
    """Verify MarkdownReviewExporter automatically sanitizes personal info when exporting."""
    exporter = MarkdownReviewExporter(repo_root=tmp_path)
    session = ReviewSessionModel(
        title="Privacy Review",
        verdict=ReviewStatus.CHANGES_REQUESTED,
        summary=f"Review by {TEST_USERNAME} for /Users/{TEST_USERNAME}/gh/firmware",
        comments=[
            CommentModel(
                id="c99",
                uuid="aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
                file_path="app/src/main.rs",
                start_line=1,
                end_line=5,
                severity=ReviewSeverity.MUST_FIX,
                body=f"Please check /Users/{TEST_USERNAME}/gh/firmware/app/src/main.rs",
                code_snippet=f"// Author: {TEST_USERNAME}\nlet path = '/Users/{TEST_USERNAME}';",
                created_at=datetime.now(timezone.utc).isoformat(),
                commit="12345678",
            )
        ],
    )

    rendered_cr = exporter.render_markdown(session)
    assert TEST_USERNAME not in rendered_cr
    assert "<username>" in rendered_cr

    out_file = tmp_path / "CR.md"
    exporter.export_markdown(session, out_file)
    content = out_file.read_text(encoding="utf-8")
    assert TEST_USERNAME not in content
    assert "<username>" in content

    commit_out = exporter.export_commit_markdown(session, "12345678", tmp_path / "feedback")
    assert commit_out is not None and commit_out.exists()
    commit_content = commit_out.read_text(encoding="utf-8")
    assert TEST_USERNAME not in commit_content
    assert "<username>" in commit_content
