"""Tests for detached feedback/attachments push and duplicate bug ID collision detection/mitigation.

Verifies:
1. Pushing feedback/ and attachments/ via DetachedBranchWormStore.sync_branch(push=True)
   and GitEngine.sync_repo(push=True).
2. Collision detection and mitigation when merging databases with duplicate worm IDs (different UUIDs).
3. Collision detection and mitigation in WormDatabaseModel.add_or_update().
4. Collision detection and mitigation in MarkdownWormExporter.scan_and_sync_feedback_dir().
"""

import os
from pathlib import Path
import subprocess
import uuid

import pytest

from model.worm_report import (
    WormAttachmentModel,
    WormCategory,
    WormDatabaseModel,
    WormReportModel,
    WormSeverity,
    WormStatus,
)
from provider.vcs.git_engine import GitEngine
from provider.worm_report.detached_branch import DetachedBranchWormStore
from provider.worm_report.markdown_exporter import MarkdownWormExporter


def run_git(args: list[str], cwd: Path) -> subprocess.CompletedProcess:
    """Run a git command in specified working directory."""
    return subprocess.run(
        ["git"] + args,
        cwd=str(cwd),
        capture_output=True,
        text=True,
        check=True,
    )


@pytest.fixture
def repo_with_remote(tmp_path: Path):
    """Create a pair of git repositories: bare remote origin and cloned local working repo."""
    remote_dir = tmp_path / "remote.git"
    remote_dir.mkdir(parents=True)
    subprocess.run(["git", "init", "--bare", str(remote_dir)], check=True, capture_output=True)

    local_dir = tmp_path / "local"
    local_dir.mkdir(parents=True)
    run_git(["init", "-b", "main"], cwd=local_dir)
    run_git(["config", "user.name", "User"], cwd=local_dir)
    run_git(["config", "user.email", "user@workspace"], cwd=local_dir)

    readme = local_dir / "README.md"
    readme.write_text("# Test Repo\n", encoding="utf-8")
    run_git(["add", "README.md"], cwd=local_dir)
    run_git(["commit", "-m", "Initial commit"], cwd=local_dir)
    run_git(["remote", "add", "origin", str(remote_dir)], cwd=local_dir)
    run_git(["push", "-u", "origin", "main"], cwd=local_dir)

    return local_dir, remote_dir


def test_collision_detection_in_add_or_update() -> None:
    """Verify WormDatabaseModel.add_or_update detects duplicate IDs with different UUIDs."""
    db = WormDatabaseModel(title="Test DB")
    w1 = WormReportModel(
        id="WORM-FIERCE-LONGHORN-800",
        uuid=str(uuid.uuid4()),
        title="First Worm",
        status=WormStatus.OPEN,
        severity=WormSeverity.HIGH,
        category=WormCategory.FIRMWARE,
    )
    db.add_or_update(w1)
    assert len(db.worms) == 1
    assert db.worms[0].id == "WORM-FIERCE-LONGHORN-800"

    # Same UUID: update in place
    w1_updated = w1.model_copy()
    w1_updated.title = "First Worm Updated"
    db.add_or_update(w1_updated)
    assert len(db.worms) == 1
    assert db.worms[0].title == "First Worm Updated"

    # Different UUID with SAME ID: COLLISION!
    w2_colliding = WormReportModel(
        id="WORM-FIERCE-LONGHORN-800",
        uuid=str(uuid.uuid4()),
        title="Second Colliding Worm",
        status=WormStatus.OPEN,
        severity=WormSeverity.MEDIUM,
        category=WormCategory.DRIVER,
    )
    assigned_id = db.add_or_update(w2_colliding)

    # Both worms must be preserved!
    assert len(db.worms) == 2
    assert db.worms[0].id == "WORM-FIERCE-LONGHORN-800"
    assert db.worms[0].title == "First Worm Updated"

    assert db.worms[1].id == assigned_id
    assert db.worms[1].id != "WORM-FIERCE-LONGHORN-800"
    assert db.worms[1].title == "Second Colliding Worm"
    assert db.worms[1].id.startswith("WORM-")


def test_collision_detection_in_merge_databases(tmp_path: Path) -> None:
    """Verify MarkdownWormExporter.merge_databases detects and mitigates duplicate ID collisions."""
    exporter = MarkdownWormExporter(tmp_path)

    u1 = str(uuid.uuid4())
    u2 = str(uuid.uuid4())

    base_db = WormDatabaseModel(
        worms=[
            WormReportModel(
                id="WORM-DUPLICATE-ID-123",
                uuid=u1,
                title="Local Worm Alpha",
                status=WormStatus.OPEN,
                severity=WormSeverity.CRITICAL,
                category=WormCategory.FIRMWARE,
            )
        ]
    )

    incoming_db = WormDatabaseModel(
        worms=[
            # Shares the same ID but has a completely different UUID
            WormReportModel(
                id="WORM-DUPLICATE-ID-123",
                uuid=u2,
                title="Remote Worm Beta",
                status=WormStatus.RESOLVED,
                severity=WormSeverity.LOW,
                category=WormCategory.CONTROLLER,
                resolution_notes="Fixed upstream",
            )
        ]
    )

    merged = exporter.merge_databases(base_db, incoming_db)

    # Both worms must survive collision mitigation
    assert len(merged.worms) == 2

    # Original worm retained its ID
    worm_alpha = next(w for w in merged.worms if w.uuid == u1)
    assert worm_alpha.id == "WORM-DUPLICATE-ID-123"
    assert worm_alpha.title == "Local Worm Alpha"

    # Incoming worm received a new unique ID
    worm_beta = next(w for w in merged.worms if w.uuid == u2)
    assert worm_beta.id != "WORM-DUPLICATE-ID-123"
    assert worm_beta.title == "Remote Worm Beta"
    assert worm_beta.resolution_notes == "Fixed upstream"
    assert worm_beta.id.startswith("WORM-")


def test_collision_detection_in_scan_feedback_dir(tmp_path: Path) -> None:
    """Verify scan_and_sync_feedback_dir mitigates collisions when an untracked file reuses an ID."""
    feedback_dir = tmp_path / "feedback"
    feedback_dir.mkdir(parents=True)
    exporter = MarkdownWormExporter(tmp_path)

    u_orig = str(uuid.uuid4())
    db = WormDatabaseModel(
        worms=[
            WormReportModel(
                id="WORM-EXISTING-ID-999",
                uuid=u_orig,
                title="Existing Database Worm",
                status=WormStatus.OPEN,
                severity=WormSeverity.HIGH,
                category=WormCategory.BOARD,
            )
        ]
    )

    # Create a markdown file on disk with the same ID but a different UUID
    u_new = str(uuid.uuid4())
    colliding_file = feedback_dir / "WORM_EXISTING-ID-999.md"
    colliding_file.write_text(
        f"# `[WORM-EXISTING-ID-999]` Conflicting File Worm\n\n"
        f"- **UUID**: `{u_new}`\n"
        f"- **ID**: `WORM-EXISTING-ID-999`\n"
        f"- **Status**: `OPEN`\n"
        f"- **Severity**: `MEDIUM`\n"
        f"- **Category**: `DRIVER`\n\n"
        f"#### Description\n\nColliding description.\n",
        encoding="utf-8",
    )

    stats = exporter.scan_and_sync_feedback_dir(feedback_dir, db)
    assert stats["scanned"] == 1
    assert stats["merged"] == 1

    # Both worms must exist in database!
    assert len(db.worms) == 2
    w1 = db.get_worm_by_uuid(u_orig)
    assert w1 is not None
    assert w1.id == "WORM-EXISTING-ID-999"

    w2 = db.get_worm_by_uuid(u_new)
    assert w2 is not None
    assert w2.id != "WORM-EXISTING-ID-999"
    assert w2.title == "Conflicting File Worm"


def test_sync_branch_pushes_feedback_and_attachments(repo_with_remote) -> None:
    """Verify DetachedBranchWormStore.sync_branch pushes feedback reports and attachments to remote."""
    local_dir, remote_dir = repo_with_remote
    store = DetachedBranchWormStore(local_dir, branch_name="worms")

    # Create a worm with an attachment
    worm_id = "WORM-PUSH-TEST-101"
    att_dir = local_dir / "attachments" / worm_id
    att_dir.mkdir(parents=True)
    sample_log = att_dir / "sample.log"
    sample_log.write_text("Crash log contents\n", encoding="utf-8")

    db = WormDatabaseModel(
        worms=[
            WormReportModel(
                id=worm_id,
                uuid=str(uuid.uuid4()),
                title="Worm to be pushed",
                status=WormStatus.OPEN,
                severity=WormSeverity.HIGH,
                category=WormCategory.FIRMWARE,
                attachments=[
                    WormAttachmentModel(
                        id="att-101",
                        filename="sample.log",
                        file_type="log",
                        file_path=f"attachments/{worm_id}/sample.log",
                        size_bytes=len("Crash log contents\n"),
                    )
                ],
            )
        ]
    )

    # Commit to detached branch
    commit_sha = store.commit_database(
        db,
        commit_message="Add push test worm",
        extra_feedback_dir=local_dir / "feedback",
    )
    assert commit_sha is not None

    # Sync branch with push=True
    res = store.sync_branch(push=True)
    assert res["status"] == "ok"
    assert res["pushed"] is True

    # Verify remote bare repo has refs/heads/worms with the commit
    remote_worm_commit = run_git(["rev-parse", "refs/heads/worms"], cwd=remote_dir).stdout.strip()
    assert remote_worm_commit == res["commit"]

    # Verify remote has worms.json, feedback file, and attachment
    ls_remote = run_git(["ls-tree", "-r", "--name-only", "refs/heads/worms"], cwd=remote_dir).stdout
    assert "worms.json" in ls_remote
    assert "WORMS.md" in ls_remote
    assert f"feedback/WORM_{worm_id.removeprefix('WORM-')}.md" in ls_remote
    assert f"attachments/{worm_id}/sample.log" in ls_remote


def test_git_engine_sync_repo_pushes_detached_branch(repo_with_remote) -> None:
    """Verify GitEngine.sync_repo(push=True) pushes the detached branch."""
    local_dir, remote_dir = repo_with_remote
    engine = GitEngine(local_dir)
    store = DetachedBranchWormStore(local_dir, branch_name="worms")

    db = WormDatabaseModel(
        worms=[
            WormReportModel(
                id="WORM-ENGINE-SYNC-202",
                uuid=str(uuid.uuid4()),
                title="GitEngine sync test",
                status=WormStatus.OPEN,
                severity=WormSeverity.LOW,
                category=WormCategory.UI,
            )
        ]
    )
    store.commit_database(db, commit_message="Worm for GitEngine sync")

    # Add an unpushed local commit to main
    test_code_file = local_dir / "test.txt"
    test_code_file.write_text("unpushed code\n", encoding="utf-8")
    run_git(["add", "test.txt"], cwd=local_dir)
    run_git(["commit", "-m", "Unpushed code commit"], cwd=local_dir)
    local_main_sha = run_git(["rev-parse", "main"], cwd=local_dir).stdout.strip()
    remote_main_before = run_git(["rev-parse", "main"], cwd=remote_dir).stdout.strip()

    # Run sync_repo with push=True
    sync_res = engine.sync_repo(push=True)
    assert sync_res["status"] == "ok"
    assert sync_res["detached"]["pushed"] is True
    assert "Feedback and attachments pushed" in sync_res["message"]

    # Verify remote has the worm ref
    remote_sha = run_git(["rev-parse", "refs/heads/worms"], cwd=remote_dir).stdout.strip()
    assert len(remote_sha) == 40

    # Verify remote main was NOT pushed (code branches must only be pushed via user-managed PRs)
    remote_main_after = run_git(["rev-parse", "main"], cwd=remote_dir).stdout.strip()
    assert remote_main_after == remote_main_before
    assert remote_main_after != local_main_sha
