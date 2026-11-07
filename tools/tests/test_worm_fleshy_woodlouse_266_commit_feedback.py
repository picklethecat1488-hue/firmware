"""Regression unit test for WORM-FLESHY-WOODLOUSE-266.

Verifies:
1. Feedback files in feedback/ (e.g. feedback/WORM_*.md, feedback/CR_*.md) are discovered
   by GitEngine.get_working_tree_files() as feedback files (not ignored by git).
2. GitEngine.stage_file() successfully stages untracked and modified feedback files into git index.
3. GitEngine.commit_files() successfully commits feedback files to repository without gitignore errors.
4. GitEngine.stage_file() and commit_files() provide force-add fallback if a file was gitignored,
   honoring explicit user actions in the VCS UI.
5. .gitignore does not ignore feedback/ directory while preserving attachments/ ignores.
"""

from pathlib import Path
import subprocess

import pytest

from dashboard.provider.vcs.git_engine import GitEngine, run_git_command


@pytest.fixture
def temp_repo(tmp_path: Path) -> GitEngine:
    """Initialize a temporary git repository with initial commit and .gitignore."""
    repo_dir = tmp_path / "repo"
    repo_dir.mkdir(parents=True, exist_ok=True)

    run_git_command(["init", "-b", "main"], cwd=repo_dir)
    run_git_command(["config", "user.name", "Test User"], cwd=repo_dir)
    run_git_command(["config", "user.email", "test@example.com"], cwd=repo_dir)

    # Initial file and commit
    readme = repo_dir / "README.md"
    readme.write_text("# Test Repo\n", encoding="utf-8")
    run_git_command(["add", "README.md"], cwd=repo_dir)
    run_git_command(["commit", "-m", "Initial commit"], cwd=repo_dir)

    return GitEngine(repo_dir)


def test_gitignore_does_not_ignore_feedback_dir() -> None:
    """Verify that root .gitignore does not ignore the feedback directory."""
    repo_root = Path(__file__).resolve().parents[2]
    gitignore_path = repo_root / ".gitignore"
    assert gitignore_path.exists(), ".gitignore must exist"

    lines = gitignore_path.read_text(encoding="utf-8").splitlines()
    # feedback/ must not be present as an ignored line
    assert "feedback/" not in [line.strip() for line in lines], (
        "feedback/ must not be ignored in .gitignore, as review and worm markdown files are tracked in git."
    )
    # Binary attachments must remain ignored
    assert any("feedback/attachments/" in line for line in lines), (
        "feedback/attachments/ must remain ignored in .gitignore."
    )


def test_untracked_feedback_file_discovered_in_working_tree(temp_repo: GitEngine) -> None:
    """Verify that an untracked feedback file is discovered by get_working_tree_files()."""
    feedback_dir = temp_repo.repo_root / "feedback"
    feedback_dir.mkdir(parents=True, exist_ok=True)

    worm_file = feedback_dir / "WORM_TEST-SAMPLE-001.md"
    worm_file.write_text("# Test Worm Report\nStatus: OPEN\n", encoding="utf-8")

    working_files = temp_repo.get_working_tree_files()
    found = [f for f in working_files if f.path == "feedback/WORM_TEST-SAMPLE-001.md"]
    assert len(found) == 1, "Untracked feedback file must be discovered in working tree"
    assert found[0].is_feedback is True
    assert found[0].is_untracked is True
    assert found[0].is_staged is False


def test_stage_and_commit_feedback_file(temp_repo: GitEngine) -> None:
    """Verify GitEngine can stage and commit an untracked feedback file without errors."""
    feedback_dir = temp_repo.repo_root / "feedback"
    feedback_dir.mkdir(parents=True, exist_ok=True)

    worm_file = feedback_dir / "WORM_TEST-SAMPLE-002.md"
    worm_file.write_text("# Test Worm Report\nStatus: OPEN\n", encoding="utf-8")

    # Stage through GitEngine
    temp_repo.stage_file("feedback/WORM_TEST-SAMPLE-002.md")

    working_files = temp_repo.get_working_tree_files()
    staged = [f for f in working_files if f.path == "feedback/WORM_TEST-SAMPLE-002.md"]
    assert len(staged) == 1
    assert staged[0].is_staged is True

    # Commit through GitEngine
    commit_sha = temp_repo.commit_files(
        message="feat(worm): register test worm",
        file_paths=["feedback/WORM_TEST-SAMPLE-002.md"],
    )
    assert len(commit_sha) >= 7

    # Verify commit contains the feedback file
    show_files = run_git_command(["show", "--name-only", "--format=", commit_sha], cwd=temp_repo.repo_root).strip()
    assert "feedback/WORM_TEST-SAMPLE-002.md" in show_files


def test_stage_and_commit_ignored_file_fallback(temp_repo: GitEngine) -> None:
    """Verify GitEngine provides force-add fallback when staging or committing an ignored file."""
    # Write a .gitignore ignoring special_feedback/
    gitignore = temp_repo.repo_root / ".gitignore"
    gitignore.write_text("special_feedback/\n", encoding="utf-8")
    run_git_command(["add", ".gitignore"], cwd=temp_repo.repo_root)
    run_git_command(["commit", "-m", "Add gitignore"], cwd=temp_repo.repo_root)

    special_dir = temp_repo.repo_root / "special_feedback"
    special_dir.mkdir(parents=True, exist_ok=True)
    special_file = special_dir / "WORM_SPECIAL-003.md"
    special_file.write_text("# Special Worm\n", encoding="utf-8")

    # Explicit stage_file must force-add without failing
    temp_repo.stage_file("special_feedback/WORM_SPECIAL-003.md")

    # Verify staged
    diff_staged = run_git_command(["diff", "--cached", "--name-only"], cwd=temp_repo.repo_root).strip()
    assert "special_feedback/WORM_SPECIAL-003.md" in diff_staged

    # Explicit commit_files must succeed
    commit_sha = temp_repo.commit_files(
        message="feat: add special feedback",
        file_paths=["special_feedback/WORM_SPECIAL-003.md"],
    )
    assert len(commit_sha) >= 7
    log_files = run_git_command(["show", "--name-only", "--format=", commit_sha], cwd=temp_repo.repo_root).strip()
    assert "special_feedback/WORM_SPECIAL-003.md" in log_files
