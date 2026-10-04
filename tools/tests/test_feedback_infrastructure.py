"""Unit and regression tests for feedback directory infrastructure.

Verifies UUID tracking in SQLite, granular WORM_<id>.md and CR_<commit>.md exports,
rename detection, duplicate ID resolution, commit update tracking, auto-merging,
and UI synchronization states.
"""

from pathlib import Path
import uuid

import pytest

from model.code_review import (
    CommentModel,
    CommitUpdateModel,
    FileReviewStatus,
    FileStateModel,
    ReviewSessionModel,
    ReviewSeverity,
    ReviewStatus,
)
from model.worm_report import WormCategory, WormDatabaseModel, WormReportModel, WormSeverity, WormStatus
from provider.code_review.markdown_exporter import MarkdownReviewExporter
from provider.code_review.sqlite_store import SQLiteReviewStore
from provider.worm_report.markdown_exporter import MarkdownWormExporter
from provider.worm_report.sqlite_store import SQLiteWormStore


def test_worm_uuid_persistence_in_sqlite(tmp_path: Path) -> None:
    """Verify worm UUID is persisted to and loaded from SQLite, with UUID querying."""
    db_file = tmp_path / "worms.sqlite"
    store = SQLiteWormStore(db_file)

    test_uuid = str(uuid.uuid4())
    worm = WormReportModel(
        id="WORM-001",
        uuid=test_uuid,
        title="Test Worm for UUID Persistence",
        status=WormStatus.OPEN,
        severity=WormSeverity.HIGH,
        category=WormCategory.DRIVER,
    )
    db = WormDatabaseModel(worms=[worm])
    store.save_database(db)

    # Reload from SQLite
    loaded_db = store.load_database()
    assert len(loaded_db.worms) == 1
    loaded_worm = loaded_db.worms[0]
    assert loaded_worm.uuid == test_uuid
    assert loaded_worm.id == "WORM-001"

    # Query directly by UUID
    retrieved = store.get_worm_by_uuid(test_uuid)
    assert retrieved is not None
    assert retrieved.id == "WORM-001"
    assert retrieved.title == "Test Worm for UUID Persistence"


def test_worm_granular_export_and_rename_detection(tmp_path: Path) -> None:
    """Verify WORM_<id>.md exports and detection of file renames via UUID."""
    feedback_dir = tmp_path / "feedback"
    feedback_dir.mkdir(parents=True, exist_ok=True)
    db_file = tmp_path / "worms.sqlite"
    store = SQLiteWormStore(db_file)

    worm_uuid = str(uuid.uuid4())
    worm = WormReportModel(
        id="WORM-100",
        uuid=worm_uuid,
        title="Carrier Board Battery Holder Clearance",
        status=WormStatus.OPEN,
        severity=WormSeverity.MEDIUM,
        category=WormCategory.CONTROLLER,
        description="Battery holder clearance issue",
    )
    db = WormDatabaseModel(worms=[worm])
    store.save_database(db)

    exporter = MarkdownWormExporter(repo_root=tmp_path)
    main_worms_md = feedback_dir / "WORMS.md"
    exporter.export_markdown(db, main_worms_md, store=store)

    # Verify individual file created
    individual_file = feedback_dir / "WORM_100.md"
    assert individual_file.exists()
    content = individual_file.read_text(encoding="utf-8")
    assert f"- **UUID**: `{worm_uuid}`" in content
    assert "Carrier Board Battery Holder Clearance" in content

    # Simulate renaming the file to WORM_200.md
    renamed_file = feedback_dir / "WORM_200.md"
    individual_file.rename(renamed_file)
    assert not individual_file.exists()
    assert renamed_file.exists()

    # Scan and sync feedback dir
    stats = exporter.scan_and_sync_feedback_dir(feedback_dir, db, store)
    assert stats["renamed"] == 1
    assert db.worms[0].id == "WORM-200"

    # Verify SQLite was updated
    from_store = store.get_worm_by_uuid(worm_uuid)
    assert from_store is not None
    assert from_store.id == "WORM-200"


def test_worm_duplicate_id_auto_resolution(tmp_path: Path) -> None:
    """Verify duplicate worm IDs with distinct UUIDs are disambiguated on export."""
    feedback_dir = tmp_path / "feedback"
    feedback_dir.mkdir(parents=True, exist_ok=True)
    db_file = tmp_path / "worms.sqlite"
    store = SQLiteWormStore(db_file)

    worm1 = WormReportModel(
        id="WORM-050",
        uuid=str(uuid.uuid4()),
        title="First duplicate worm",
        status=WormStatus.OPEN,
        severity=WormSeverity.LOW,
        category=WormCategory.DRIVER,
    )
    worm2 = WormReportModel(
        id="WORM-050",  # Duplicate ID
        uuid=str(uuid.uuid4()),
        title="Second duplicate worm",
        status=WormStatus.OPEN,
        severity=WormSeverity.HIGH,
        category=WormCategory.PLATFORM,
    )

    db = WormDatabaseModel(worms=[worm1, worm2])
    exporter = MarkdownWormExporter(repo_root=tmp_path)
    main_worms_md = feedback_dir / "WORMS.md"

    # Export with duplicate resolution
    exporter.export_markdown(db, main_worms_md, store=store)

    # Assert IDs were disambiguated
    ids = [w.id for w in db.worms]
    assert len(ids) == len(set(ids)), "All worm IDs must be unique after duplicate resolution"
    assert "WORM-050" in ids
    assert len(ids) == 2

    # Verify individual files exist for both
    for w in db.worms:
        clean_id = w.id.removeprefix("WORM-").removeprefix("WORM_")
        assert (feedback_dir / f"WORM_{clean_id}.md").exists()


def test_code_review_uuid_and_commit_updates_in_sqlite(tmp_path: Path) -> None:
    """Verify comment UUID and commit update history tracking in SQLite store."""
    db_file = tmp_path / "code_review.sqlite"
    store = SQLiteReviewStore(db_file)

    comment_uuid = str(uuid.uuid4())
    session_uuid = str(uuid.uuid4())

    comment = CommentModel(
        id="c12345",
        uuid=comment_uuid,
        file_path="src/view.py",
        start_line=15,
        end_line=20,
        severity=ReviewSeverity.MUST_FIX,
        body="Missing parameter validation",
        author="Reviewer",
        code_snippet="def view(): pass",
        created_at="2026-09-27T00:00:00Z",
    )

    update = CommitUpdateModel(
        id=str(uuid.uuid4()),
        session_uuid=session_uuid,
        original_commit="abc1111",
        current_commit="def2222",
        action="AMEND",
        notes="Amended commit with regression test",
    )

    session = ReviewSessionModel(
        title="Test Review",
        uuid=session_uuid,
        commit_hash="def2222",
        original_commit="abc1111",
        update_action="AMEND",
        commit_history=[update],
        comments=[comment],
        files={"src/view.py": FileStateModel(path="src/view.py", status=FileReviewStatus.PENDING)},
    )

    store.save_session(session)

    # Reload session
    loaded = store.load_session()
    assert loaded is not None
    assert loaded.uuid == session_uuid
    assert len(loaded.comments) == 1
    assert loaded.comments[0].uuid == comment_uuid
    assert len(loaded.commit_history) == 1
    assert loaded.commit_history[0].original_commit == "abc1111"
    assert loaded.commit_history[0].current_commit == "def2222"

    # Query comment by UUID
    retrieved_c = store.get_comment_by_uuid(comment_uuid)
    assert retrieved_c is not None
    assert retrieved_c.id == "c12345"
    assert retrieved_c.body == "Missing parameter validation"

    # Query commit history
    history = store.get_commit_history()
    assert len(history) == 1
    assert history[0].action == "AMEND"


def test_code_review_granular_markdown_export_and_auto_merge(tmp_path: Path) -> None:
    """Verify CR_<commit>.md export and auto-merging of comments."""
    feedback_dir = tmp_path / "feedback"
    feedback_dir.mkdir(parents=True, exist_ok=True)
    exporter = MarkdownReviewExporter(repo_root=tmp_path)

    comment_uuid = str(uuid.uuid4())
    comment = CommentModel(
        id="c55555",
        uuid=comment_uuid,
        file_path="model/src/lib.rs",
        start_line=42,
        end_line=45,
        severity=ReviewSeverity.PROPOSAL,
        body="Consider using namedtuple or dataclass",
        author="Reviewer",
        code_snippet="x = 1",
        created_at="2026-09-27T00:00:00Z",
    )

    session = ReviewSessionModel(
        title="Commit Review",
        commit_hash="7180a9d9",
        comments=[comment],
        files={"model/src/lib.rs": FileStateModel(path="model/src/lib.rs", status=FileReviewStatus.PENDING)},
    )

    # Export commit markdown
    cr_path = exporter.export_commit_markdown(session, "7180a9d9", feedback_dir)
    assert cr_path.exists()
    content = cr_path.read_text(encoding="utf-8")
    assert f"<!-- comment-uuid: {comment_uuid} -->" in content
    assert "Consider using namedtuple or dataclass" in content

    # Create empty session and merge feedback
    fresh_session = ReviewSessionModel(
        title="Fresh Review",
        commit_hash="7180a9d9",
        comments=[],
    )
    merged = exporter.merge_commit_feedback(fresh_session, cr_path)
    assert len(merged.comments) == 1
    assert merged.comments[0].uuid == comment_uuid
    assert merged.comments[0].body == "Consider using namedtuple or dataclass"


def test_feedback_ui_elements_in_templates() -> None:
    """Verify UI loading overlays and sync feedback buttons in Jinja2 templates."""
    templates_dir = Path(__file__).resolve().parent.parent / "dashboard" / "templates"

    worm_template = (templates_dir / "worm_report.html.j2").read_text(encoding="utf-8")
    assert "feedback-loading-modal" in worm_template
    assert "syncFeedback()" in worm_template
    assert "/api/sync_feedback" in worm_template

    cr_template = (templates_dir / "code_review.html.j2").read_text(encoding="utf-8")
    assert "feedbackLoadingModal" in cr_template
    assert "btnSyncFeedback" in cr_template
    assert "syncFeedback()" in cr_template
    assert "/api/sync_feedback" in cr_template


def test_code_review_no_feedback_leaves_no_markdown_files(tmp_path: Path) -> None:
    """Verify that when a review has no comments, CR_<commit>.md and feedback/CR.md are not created."""
    feedback_dir = tmp_path / "feedback"
    feedback_dir.mkdir(parents=True, exist_ok=True)
    exporter = MarkdownReviewExporter(repo_root=tmp_path)

    # 1. Empty session: export_commit_markdown returns None and does not create file
    empty_session = ReviewSessionModel(
        title="Empty Review",
        commit_hash="2b5920a88fe94244d61109e5a934998b8ed58f34",
        comments=[],
    )
    res = exporter.export_commit_markdown(empty_session, "2b5920a88fe94244d61109e5a934998b8ed58f34", feedback_dir)
    assert res is None
    assert not (feedback_dir / "CR_2b5920a88fe94244d61109e5a934998b8ed58f34.md").exists()

    # 2. Session with comments for a different commit: commit_sha should NOT be exported
    other_comment = CommentModel(
        id="c1",
        uuid=str(uuid.uuid4()),
        file_path="tools/dashboard.py",
        start_line=10,
        end_line=10,
        severity=ReviewSeverity.MUST_FIX,
        body="Finding on commit 1",
        commit="11111111",
        created_at="2026-09-27T00:00:00Z",
    )
    mixed_session = ReviewSessionModel(
        title="Mixed Review",
        commit_hash="2b5920a88fe94244d61109e5a934998b8ed58f34",
        comments=[other_comment],
    )
    # Target commit 2b5920a8 has no comments
    res2 = exporter.export_commit_markdown(mixed_session, "2b5920a88fe94244d61109e5a934998b8ed58f34", feedback_dir)
    assert res2 is None
    assert not (feedback_dir / "CR_2b5920a88fe94244d61109e5a934998b8ed58f34.md").exists()

    # Target commit 11111111 has comments and is exported
    res3 = exporter.export_commit_markdown(mixed_session, "11111111", feedback_dir)
    assert res3 is not None
    assert res3.exists()
    content = res3.read_text(encoding="utf-8")
    assert "Finding on commit 1" in content
    res3.unlink()

    # 3. ReviewServer save_and_sync with empty comments inside feedback_dir does not create CR.md
    from provider.code_review.server import ReviewServer
    from provider.vcs.git_engine import get_git_root

    db_file = tmp_path / "test_store.sqlite"
    state_file = tmp_path / "test_state.json"
    server = ReviewServer(
        host="127.0.0.1",
        port=0,
        repo_root=get_git_root(),
        markdown_output=feedback_dir / "CR.md",
        state_file=state_file,
        sqlite_file=db_file,
        bind_and_activate=False,
    )
    server.save_and_sync()
    assert not (feedback_dir / "CR.md").exists()
    assert not (feedback_dir / f"CR_{server.session.commit_hash}.md").exists()


def test_baseline_reports_default_to_target_and_preserve_feedback_granularity(tmp_path: Path) -> None:
    """Verify baseline WORMS.md and CR.md default to target/ and are excluded from feedback/.

    Only granular WORM_<id>.md and CR_<commit>.md files are stored in feedback/.
    """
    from provider.code_review.server import ReviewServer
    from provider.worm_report.server import WormReportServer

    mock_repo = tmp_path / "repo"
    mock_repo.mkdir()
    (mock_repo / "target").mkdir()
    (mock_repo / "feedback").mkdir()

    # Verify ReviewServer defaults to target/CR.md and feedback/
    cr_server = ReviewServer(
        host="127.0.0.1",
        port=0,
        repo_root=mock_repo,
        bind_and_activate=False,
    )
    assert cr_server.markdown_output == mock_repo / "target" / "CR.md"
    assert cr_server.feedback_dir == mock_repo / "feedback"

    # Verify WormReportServer defaults to target/WORMS.md and feedback/
    worm_server = WormReportServer(
        host="127.0.0.1",
        port=0,
        repo_root=mock_repo,
        bind_and_activate=False,
    )
    assert worm_server.markdown_output == mock_repo / "target" / "WORMS.md"
    assert worm_server.feedback_dir == mock_repo / "feedback"

    # Add a worm and sync
    test_worm = WormReportModel(
        id="WORM-999",
        uuid=str(uuid.uuid4()),
        title="Test Granular Report",
        status=WormStatus.OPEN,
        severity=WormSeverity.MEDIUM,
        category=WormCategory.INFRASTRUCTURE,
    )
    worm_server.database.worms.append(test_worm)
    worm_server.save_and_sync()

    # Main WORMS.md is written to target/, individual worm is written to feedback/
    assert (mock_repo / "target" / "WORMS.md").is_file()
    assert (mock_repo / "feedback" / "WORM_999.md").is_file()
    assert not (mock_repo / "feedback" / "WORMS.md").exists()
