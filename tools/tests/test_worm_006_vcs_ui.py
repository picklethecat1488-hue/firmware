"""Reproduction and regression unit tests for WORM-006: VCS UI worm tags and status."""

from pathlib import Path
import sqlite3
import pytest

from model.vcs import CommitNodeModel
from provider.vcs.git_engine import GitEngine


def test_reproduction_worm_006_commit_worm_tags_normalized_and_resolved(tmp_path: Path) -> None:
    """Verify WORM-006: Commit worm tags normalize legacy BUG-xxx to WORM-xxx and reflect RESOLVED status from worms.sqlite."""
    repo = tmp_path / "repo"
    repo.mkdir()
    target_dir = repo / "target"
    target_dir.mkdir()

    # Create target/worms.sqlite with WORM-002 marked RESOLVED
    worms_db = target_dir / "worms.sqlite"
    conn = sqlite3.connect(str(worms_db))
    conn.execute(
        "CREATE TABLE worms (id TEXT PRIMARY KEY, title TEXT, status TEXT, severity TEXT, category TEXT, description TEXT)"
    )
    conn.execute(
        "INSERT INTO worms (id, title, status, severity, category, description) VALUES ('WORM-002', 'Update BUG to WORM', 'RESOLVED', 'MEDIUM', 'INFRASTRUCTURE', 'Test description')"
    )
    conn.commit()
    conn.close()

    # Also create a legacy target/bugs.sqlite with BUG-002 marked OPEN (simulating stale database before migration)
    bugs_db = target_dir / "bugs.sqlite"
    conn = sqlite3.connect(str(bugs_db))
    conn.execute(
        "CREATE TABLE bugs (id TEXT PRIMARY KEY, title TEXT, status TEXT, severity TEXT, category TEXT, description TEXT)"
    )
    conn.execute(
        "INSERT INTO bugs (id, title, status, severity, category, description) VALUES ('BUG-002', 'Old Bug 2', 'OPEN', 'MEDIUM', 'INFRASTRUCTURE', 'Old')"
    )
    conn.commit()
    conn.close()

    engine = GitEngine(repo_root=repo)
    tags = engine._extract_worm_tags("fix(dashboard): update issue tracking from BUG to WORM and use worm icon (BUG-002)")

    assert len(tags) == 1
    # Tag ID MUST be normalized to canonical WORM-002 (never remain BUG-002)
    assert tags[0].id == "WORM-002", f"Expected canonical WORM-002, got {tags[0].id}"
    # Status MUST be resolved from worms.sqlite (never remain OPEN from stale bugs.sqlite)
    assert tags[0].status == "RESOLVED", f"Expected RESOLVED status, got {tags[0].status}"
