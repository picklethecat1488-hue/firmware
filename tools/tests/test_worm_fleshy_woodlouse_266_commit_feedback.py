"""Regression unit test for WORM-FLESHY-WOODLOUSE-266.

Verifies:
1. .gitignore contains feedback/, attachments/, and feedback/attachments/ to decouple
   worm/bug and review tracking to dedicated detached branches (refs/heads/worms or refs/heads/bugs)
   and keep code feature branches clean from untracked working tree pollution.
2. GitEngine.stage_file() and commit_files() provide automatic force-add fallback when explicitly
   staging or committing files that match .gitignore rules (such as feedback files), so users
   can commit updated feedbacks through the VCS UI without being blocked by gitignore or
   having to manually run 'git add -f' from the shell.
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


def test_gitignore_decouples_feedback_and_attachments() -> None:
    """Verify that root .gitignore ignores feedback/ and attachments/ to decouple to detached branches."""
    repo_root = Path(__file__).resolve().parents[2]
    gitignore_path = repo_root / ".gitignore"
    assert gitignore_path.exists(), ".gitignore must exist"

    lines = [line.strip() for line in gitignore_path.read_text(encoding="utf-8").splitlines()]
    # feedback/ must be ignored on code branches to prevent untracked file pollution
    assert "feedback/" in lines, (
        "feedback/ must be ignored in .gitignore, as reviews and worms are decoupled to detached branches."
    )
    # Binary attachments must remain ignored
    assert "attachments/" in lines or any("attachments/" in line for line in lines), (
        "attachments/ must remain ignored in .gitignore."
    )
    assert any("feedback/attachments/" in line for line in lines), (
        "feedback/attachments/ must remain ignored in .gitignore."
    )


def test_stage_and_commit_feedback_file_with_gitignore_fallback(temp_repo: GitEngine) -> None:
    """Verify GitEngine can stage and commit feedback files even when feedback/ is in .gitignore."""
    # Write .gitignore ignoring feedback/ (matching repository configuration)
    gitignore = temp_repo.repo_root / ".gitignore"
    gitignore.write_text("feedback/\nattachments/\nfeedback/attachments/\n", encoding="utf-8")
    run_git_command(["add", ".gitignore"], cwd=temp_repo.repo_root)
    run_git_command(["commit", "-m", "Ignore feedback and attachments on code branch"], cwd=temp_repo.repo_root)

    feedback_dir = temp_repo.repo_root / "feedback"
    feedback_dir.mkdir(parents=True, exist_ok=True)

    worm_file = feedback_dir / "WORM_TEST-SAMPLE-002.md"
    worm_file.write_text("# Test Worm Report\nStatus: OPEN\n", encoding="utf-8")

    # Regular git add fails on ignored path
    res = subprocess.run(
        ["git", "add", "--", "feedback/WORM_TEST-SAMPLE-002.md"],
        cwd=str(temp_repo.repo_root),
        capture_output=True,
        text=True,
    )
    assert res.returncode != 0
    assert "ignored by one of your .gitignore files" in res.stderr

    # GitEngine stage_file must succeed via force-add fallback
    temp_repo.stage_file("feedback/WORM_TEST-SAMPLE-002.md")
    diff_staged = run_git_command(["diff", "--cached", "--name-only"], cwd=temp_repo.repo_root).strip()
    assert "feedback/WORM_TEST-SAMPLE-002.md" in diff_staged

    # GitEngine commit_files must succeed via force-add fallback
    commit_sha = temp_repo.commit_files(
        message="feat(worm): register test worm",
        file_paths=["feedback/WORM_TEST-SAMPLE-002.md"],
    )
    assert len(commit_sha) >= 7

    # Verify commit contains the feedback file
    show_files = run_git_command(["show", "--name-only", "--format=", commit_sha], cwd=temp_repo.repo_root).strip()
    assert "feedback/WORM_TEST-SAMPLE-002.md" in show_files


def test_stage_and_commit_ignored_file_fallback(temp_repo: GitEngine) -> None:
    """Verify GitEngine provides force-add fallback when staging or committing any ignored file."""
    # Write a .gitignore ignoring special_dir/
    gitignore = temp_repo.repo_root / ".gitignore"
    gitignore.write_text("special_dir/\n", encoding="utf-8")
    run_git_command(["add", ".gitignore"], cwd=temp_repo.repo_root)
    run_git_command(["commit", "-m", "Add gitignore"], cwd=temp_repo.repo_root)

    special_dir = temp_repo.repo_root / "special_dir"
    special_dir.mkdir(parents=True, exist_ok=True)
    special_file = special_dir / "test_file.txt"
    special_file.write_text("hello\n", encoding="utf-8")

    # Explicit stage_file must force-add without failing
    temp_repo.stage_file("special_dir/test_file.txt")

    # Verify staged
    diff_staged = run_git_command(["diff", "--cached", "--name-only"], cwd=temp_repo.repo_root).strip()
    assert "special_dir/test_file.txt" in diff_staged

    # Explicit commit_files must succeed
    commit_sha = temp_repo.commit_files(
        message="feat: add special file",
        file_paths=["special_dir/test_file.txt"],
    )
    assert len(commit_sha) >= 7
    log_files = run_git_command(["show", "--name-only", "--format=", commit_sha], cwd=temp_repo.repo_root).strip()
    assert "special_dir/test_file.txt" in log_files
