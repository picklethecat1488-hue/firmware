"""Reproduction and regression unit tests for WORM-014: Unresolved worms in VCS UI.

Verifies that:
1. Bug/worm IDs mentioned in commit messages that do not exist in the repository's
   worm tracker database or markdown files (such as ported BUG-273 / BUG-274) are NOT
   tagged as phantom OPEN worms in the VCS UI smartlog.
2. Commits in the current repository history do not exhibit ghost WORM-273 or WORM-274 tags.
"""

from pathlib import Path
import sqlite3
import pytest

from model.vcs import CommitNodeModel
from provider.vcs.git_engine import GitEngine


def test_reproduction_worm_014_nonexistent_worms_not_tagged_as_open(tmp_path: Path) -> None:
    """Verify WORM-014: Non-existent worm IDs are not assigned phantom OPEN tags."""
    repo = tmp_path / "repo"
    repo.mkdir()
    target_dir = repo / "target"
    target_dir.mkdir()

    # Create target/worms.sqlite with only WORM-011 marked RESOLVED
    worms_db = target_dir / "worms.sqlite"
    conn = sqlite3.connect(str(worms_db))
    conn.execute(
        "CREATE TABLE worms (id TEXT PRIMARY KEY, title TEXT, status TEXT, severity TEXT, category TEXT, description TEXT)"
    )
    conn.execute(
        "INSERT INTO worms (id, title, status, severity, category, description) "
        "VALUES ('WORM-011', 'Traceback in dashboard', 'RESOLVED', 'MEDIUM', 'INFRASTRUCTURE', 'Desc')"
    )
    conn.commit()
    conn.close()

    engine = GitEngine(repo_root=repo)

    # 1. Commit mentioning ported hardware bug BUG-274 (which does not exist in firmware repo)
    tags_274 = engine._extract_worm_tags(
        "feat(dashboard): display code review and worm report as modal workstations (BUG-274)"
    )
    # Must NOT produce phantom WORM-274 tag with OPEN status
    assert len(tags_274) == 0, f"Expected no tags for non-existent BUG-274, but got: {tags_274}"

    # 2. Commit mentioning WORM-011 and ported BUG-273
    tags_mixed = engine._extract_worm_tags(
        "fix(dashboard): raise soft FD limit, cache commit diffs, and handle EMFILE (WORM-011 / BUG-273)"
    )
    assert len(tags_mixed) == 1, f"Expected exactly 1 tag (WORM-011), but got: {tags_mixed}"
    assert tags_mixed[0].id == "WORM-011"
    assert tags_mixed[0].status == "RESOLVED"
    assert not any(t.id == "WORM-273" for t in tags_mixed), "WORM-273 must not be tagged"


def test_reproduction_worm_014_repo_smartlog_no_ghost_worms() -> None:
    """Verify WORM-014: The repository's smartlog does not display ghost WORM-273 or WORM-274."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    engine = GitEngine(repo_root=workspace_root)
    nodes = engine.get_smartlog_dag()

    ghost_tags = []
    for node in nodes:
        for tag in node.worm_tags:
            if tag.id in ("WORM-273", "WORM-274", "BUG-273", "BUG-274"):
                ghost_tags.append((node.short_hash, tag.id, tag.status))

    assert not ghost_tags, f"Found ghost worm tags in repository smartlog: {ghost_tags}"
