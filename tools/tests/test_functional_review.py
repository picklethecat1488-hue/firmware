"""Unit and regression tests for Firmware Functional Review, Detached Worm Branch, and Target Flash Guarantee."""

import json
from pathlib import Path
import sqlite3
import subprocess
import threading
import urllib.request
import pytest

from model.worm_report import (
    WormCategory,
    WormDatabaseModel,
    WormReportModel,
    WormReviewTaskModel,
    WormReviewTaskStatus,
    WormSeverity,
    WormStatus,
)
from provider.worm_report.detached_branch import DetachedBranchWormStore
from provider.worm_report.markdown_exporter import MarkdownWormExporter
from provider.worm_report.server import WormReportServer
from provider.worm_report.sqlite_store import SQLiteWormStore
from dashboard.cli import main as cli_main


def create_test_git_repo(path: Path) -> Path:
    """Create a minimal isolated git repository for detached branch testing."""
    repo_dir = path / "git_repo"
    repo_dir.mkdir(parents=True, exist_ok=True)
    subprocess.run(["git", "init"], cwd=repo_dir, check=True, capture_output=True)
    subprocess.run(["git", "config", "user.email", "tester@example.com"], cwd=repo_dir, check=True, capture_output=True)
    subprocess.run(["git", "config", "user.name", "Test Engineer"], cwd=repo_dir, check=True, capture_output=True)
    subprocess.run(["git", "config", "commit.gpgsign", "false"], cwd=repo_dir, check=True, capture_output=True)
    (repo_dir / "README.md").write_text("# Test Repo\n", encoding="utf-8")
    subprocess.run(["git", "add", "README.md"], cwd=repo_dir, check=True, capture_output=True)
    subprocess.run(["git", "commit", "-m", "Initial commit"], cwd=repo_dir, check=True, capture_output=True)
    subprocess.run(["git", "branch", "-M", "main"], cwd=repo_dir, check=True, capture_output=True)
    return repo_dir


def test_worm_review_task_model() -> None:
    """Verify WormReviewTaskModel initialization, defaults, and serialization."""
    task = WormReviewTaskModel(
        id="task-w1",
        title="Verify motor PWM frequency with scope",
        command="python tools/host.py cli ping",
        status=WormReviewTaskStatus.PENDING,
        notes="Scope ch1 on pin GP14",
    )
    assert task.id == "task-w1"
    assert task.title == "Verify motor PWM frequency with scope"
    assert task.command == "python tools/host.py cli ping"
    assert task.status == WormReviewTaskStatus.PENDING
    assert task.notes == "Scope ch1 on pin GP14"

    dumped = task.model_dump(mode="json")
    assert dumped["status"] == "PENDING"
    assert dumped["command"] == "python tools/host.py cli ping"

    loaded = WormReviewTaskModel.model_validate(dumped)
    assert loaded.id == task.id
    assert loaded.status == WormReviewTaskStatus.PENDING


def test_worm_report_model_functional_review_fields() -> None:
    """Verify WormReportModel supports commit_ids, review_tasks, rejection_notes, and timestamps."""
    worm = WormReportModel(
        id="WORM-TEST-001",
        title="Check functional review lifecycle in firmware",
        status=WormStatus.RESOLVED,
        severity=WormSeverity.HIGH,
        category=WormCategory.DRIVER,
        commit_ids=["beef456"],
        review_tasks=[
            WormReviewTaskModel(
                id="task-1",
                title="Run host cli ping",
                command="python tools/host.py cli ping",
                status=WormReviewTaskStatus.COMPLETED,
            )
        ],
        rejection_notes="Firmware timed out during communication",
        accepted_at=None,
        rejected_at="2026-11-04 11:30:00 UTC",
    )

    assert worm.commit_ids == ["beef456"]
    assert len(worm.review_tasks) == 1
    assert worm.review_tasks[0].status == WormReviewTaskStatus.COMPLETED
    assert worm.rejection_notes == "Firmware timed out during communication"
    assert worm.rejected_at == "2026-11-04 11:30:00 UTC"

    dumped = worm.model_dump(mode="json")
    assert dumped["commit_ids"] == ["beef456"]
    assert dumped["review_tasks"][0]["id"] == "task-1"
    assert dumped["rejection_notes"] == "Firmware timed out during communication"

    restored = WormReportModel.model_validate(dumped)
    assert restored.commit_ids == worm.commit_ids
    assert len(restored.review_tasks) == 1
    assert restored.review_tasks[0].title == "Run host cli ping"


def test_sqlite_store_functional_review_roundtrip_and_migration(tmp_path: Path) -> None:
    """Verify SQLiteWormStore saves and restores functional review fields and handles schema migrations."""
    db_file = tmp_path / "test_worms.sqlite"
    store = SQLiteWormStore(db_path=db_file)

    worm = WormReportModel(
        id="WORM-SQLITE-001",
        title="SQLite Functional Review Test",
        status=WormStatus.RESOLVED,
        severity=WormSeverity.HIGH,
        category=WormCategory.CONTROLLER,
        commit_ids=["deadbeef"],
        review_tasks=[
            WormReviewTaskModel(
                id="task-sql-1",
                title="Verify PID coefficients",
                command="python tools/host.py cli pid",
                status=WormReviewTaskStatus.PENDING,
            )
        ],
        rejection_notes=None,
        accepted_at=None,
        rejected_at=None,
    )
    store.save_worm(worm)

    loaded = store.get_worm("WORM-SQLITE-001")
    assert loaded is not None
    assert loaded.commit_ids == ["deadbeef"]
    assert len(loaded.review_tasks) == 1
    assert loaded.review_tasks[0].id == "task-sql-1"
    assert loaded.review_tasks[0].title == "Verify PID coefficients"
    assert loaded.rejection_notes is None

    # Test migration on legacy database schema
    legacy_db_file = tmp_path / "legacy.sqlite"
    conn = sqlite3.connect(legacy_db_file)
    conn.execute("""
        CREATE TABLE worms (
            id TEXT PRIMARY KEY,
            uuid TEXT,
            title TEXT NOT NULL,
            severity TEXT NOT NULL,
            status TEXT NOT NULL,
            category TEXT NOT NULL,
            component TEXT,
            description TEXT,
            reproduction_steps TEXT,
            expected_behavior TEXT,
            actual_behavior TEXT,
            logs TEXT,
            resolution_notes TEXT,
            created_at TEXT,
            updated_at TEXT,
            resolved_at TEXT
        )
    """)
    conn.execute("""
        INSERT INTO worms (id, title, severity, status, category)
        VALUES ('WORM-LEGACY-001', 'Legacy Worm', 'LOW', 'OPEN', 'GENERAL')
    """)
    conn.commit()
    conn.close()

    # Reopening with SQLiteWormStore should run migration without error
    legacy_store = SQLiteWormStore(db_path=legacy_db_file)
    legacy_worm = legacy_store.get_worm("WORM-LEGACY-001")
    assert legacy_worm is not None
    assert legacy_worm.commit_ids == []
    assert legacy_worm.review_tasks == []
    assert legacy_worm.rejection_notes is None


def test_markdown_exporter_functional_review_sections(tmp_path: Path) -> None:
    """Verify MarkdownWormExporter correctly writes and parses attached commits, tasks, and rejection notes."""
    feedback_dir = tmp_path / "feedback"
    feedback_dir.mkdir(parents=True, exist_ok=True)
    exporter = MarkdownWormExporter(repo_root=tmp_path)

    worm = WormReportModel(
        id="WORM-MD-001",
        title="Markdown Review Test",
        status=WormStatus.RESOLVED,
        severity=WormSeverity.CRITICAL,
        category=WormCategory.DRIVER,
        description="Verify review sections.",
        commit_ids=["3333333", "4444444"],
        review_tasks=[
            WormReviewTaskModel(
                id="task-md-1",
                title="Execute motor step test",
                command="python tools/host.py cli step",
                status=WormReviewTaskStatus.COMPLETED,
            ),
            WormReviewTaskModel(
                id="task-md-2",
                title="Inspect sensor telemetry",
                command="",
                status=WormReviewTaskStatus.PENDING,
            ),
        ],
        rejection_notes="Jitter detected on optical encoder",
        rejected_at="2026-11-04 12:15:00 UTC",
    )

    exported_path = exporter.export_individual_worm(worm, feedback_dir)
    content = exported_path.read_text(encoding="utf-8")
    assert "#### Attached Commits" in content
    assert "`3333333`" in content
    assert "`4444444`" in content
    assert "#### Review Tasks & Functional Checklist" in content
    assert "Execute motor step test | `python tools/host.py cli step`" in content
    assert "<!-- id:task-md-1 -->" in content
    assert "Inspect sensor telemetry" in content
    assert "<!-- id:task-md-2 -->" in content
    assert "#### Rejection Feedback" in content
    assert "Jitter detected on optical encoder" in content

    # Parse exported markdown back
    parsed = exporter.parse_individual_worm_file(exported_path)
    assert parsed is not None
    assert parsed.id == "WORM-MD-001"
    assert parsed.commit_ids == ["3333333", "4444444"]
    assert len(parsed.review_tasks) == 2
    assert parsed.review_tasks[0].title == "Execute motor step test"
    assert parsed.review_tasks[0].status == WormReviewTaskStatus.COMPLETED
    assert parsed.review_tasks[0].command == "python tools/host.py cli step"
    assert parsed.review_tasks[1].title == "Inspect sensor telemetry"
    assert parsed.review_tasks[1].status == WormReviewTaskStatus.PENDING
    assert parsed.rejection_notes == "Jitter detected on optical encoder"


def test_detached_branch_worm_store(tmp_path: Path) -> None:
    """Verify DetachedBranchWormStore commits and loads from detached git branch without dirtying working tree."""
    repo_dir = create_test_git_repo(tmp_path)
    store = DetachedBranchWormStore(repo_root=repo_dir, branch_name="worms")

    # Initially empty
    data = store.load_database()
    assert data is None

    # Commit data to detached branch
    db = WormDatabaseModel(
        title="Detached Worm Tracker",
        worms=[
            WormReportModel(
                id="WORM-DETACHED-001",
                title="Detached Test Worm",
                status=WormStatus.OPEN,
                severity=WormSeverity.MEDIUM,
                category=WormCategory.DRIVER,
                commit_ids=["beef789"],
            )
        ],
    )

    commit_sha = store.commit_database(
        database=db,
        commit_message="Initial worm database sync",
    )
    assert commit_sha is not None
    assert len(commit_sha) == 40

    # Ensure working tree was NOT modified or checked out
    status = subprocess.run(["git", "status", "--porcelain"], cwd=repo_dir, check=True, capture_output=True, text=True)
    assert status.stdout.strip() == ""
    curr_branch = subprocess.run(
        ["git", "branch", "--show-current"], cwd=repo_dir, check=True, capture_output=True, text=True
    ).stdout.strip()
    assert curr_branch == "main"

    # Load from detached branch
    loaded = store.load_database()
    assert loaded is not None
    assert len(loaded.worms) == 1
    assert loaded.worms[0].id == "WORM-DETACHED-001"
    assert loaded.worms[0].commit_ids == ["beef789"]


def test_worm_server_functional_review_api_endpoints(tmp_path: Path) -> None:
    """Verify WormReportServer REST endpoints: accept, reject, attach_commit, add_task, update_task, run_command."""
    feedback_dir = tmp_path / "feedback"
    feedback_dir.mkdir(parents=True, exist_ok=True)
    db_file = tmp_path / "worms.sqlite"

    store = SQLiteWormStore(db_path=db_file)
    test_worm = WormReportModel(
        id="WORM-SERVER-001",
        title="Server API Worm",
        status=WormStatus.RESOLVED,
        severity=WormSeverity.MEDIUM,
        category=WormCategory.CONTROLLER,
    )
    store.save_worm(test_worm)

    server = WormReportServer(
        host="127.0.0.1",
        port=0,
        repo_root=tmp_path,
        feedback_dir=feedback_dir,
        sqlite_file=db_file,
    )
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    base_url = server.get_url()

    try:
        # 1. Attach commit
        req = urllib.request.Request(
            f"{base_url}/api/worms/attach_commit",
            data=json.dumps({"id": "WORM-SERVER-001", "commit_id": "def4567"}).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urllib.request.urlopen(req) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            assert data["status"] == "ok"
            assert "def4567" in data["worm"]["commit_ids"]

        # 2. Add review task
        req = urllib.request.Request(
            f"{base_url}/api/worms/add_task",
            data=json.dumps(
                {
                    "id": "WORM-SERVER-001",
                    "title": "Run firmware echo test",
                    "command": "echo 'Firmware verification output'",
                }
            ).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urllib.request.urlopen(req) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            assert data["status"] == "ok"
            tasks = data["worm"]["review_tasks"]
            assert len(tasks) == 1
            task_id = tasks[0]["id"]
            assert tasks[0]["title"] == "Run firmware echo test"

        # 3. Update task status
        req = urllib.request.Request(
            f"{base_url}/api/worms/update_task",
            data=json.dumps(
                {
                    "id": "WORM-SERVER-001",
                    "task_id": task_id,
                    "status": "COMPLETED",
                }
            ).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urllib.request.urlopen(req) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            assert data["status"] == "ok"
            assert data["worm"]["review_tasks"][0]["status"] == "COMPLETED"

        # 4. Run task command
        req = urllib.request.Request(
            f"{base_url}/api/worms/run_command",
            data=json.dumps(
                {
                    "id": "WORM-SERVER-001",
                    "command": "echo 'Firmware verification output'",
                }
            ).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urllib.request.urlopen(req) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            assert data["status"] == "ok"
            assert "Firmware verification output" in data["stdout"]
            assert data["exit_code"] == 0

        # 5. Reject worm -> transitions RESOLVED to OPEN
        req = urllib.request.Request(
            f"{base_url}/api/worms/reject",
            data=json.dumps(
                {
                    "id": "WORM-SERVER-001",
                    "notes": "Firmware echo failed parity bit validation.",
                }
            ).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urllib.request.urlopen(req) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            assert data["status"] == "ok"
            assert data["worm"]["status"] == "OPEN"
            assert data["worm"]["rejection_notes"] == "Firmware echo failed parity bit validation."
            assert data["worm"]["rejected_at"] is not None

        # Re-resolve worm to test accept
        worm_obj = store.get_worm("WORM-SERVER-001")
        worm_obj.status = WormStatus.RESOLVED
        store.save_worm(worm_obj)

        # 6. Accept worm -> transitions RESOLVED to CLOSED
        req = urllib.request.Request(
            f"{base_url}/api/worms/accept",
            data=json.dumps(
                {
                    "id": "WORM-SERVER-001",
                    "notes": "Verified and accepted by tester.",
                }
            ).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urllib.request.urlopen(req) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            assert data["status"] == "ok"
            assert data["worm"]["status"] == "CLOSED"
            assert data["worm"]["accepted_at"] is not None

    finally:
        server.server_close()


def test_cli_functional_review_subcommands(
    tmp_path: Path, capsys: pytest.CaptureFixture[str], monkeypatch: pytest.MonkeyPatch
) -> None:
    """Verify CLI commands: attach-commit, add-task, resolve-task, accept-worm, reject-worm, and list-worms --open."""
    repo_dir = create_test_git_repo(tmp_path)
    monkeypatch.chdir(repo_dir)
    db_file = repo_dir / "worms.sqlite"
    store = SQLiteWormStore(db_path=db_file)

    worm = WormReportModel(
        id="WORM-CLI-001",
        title="CLI Review Worm",
        status=WormStatus.RESOLVED,
        severity=WormSeverity.HIGH,
        category=WormCategory.CONTROLLER,
    )
    store.save_worm(worm)

    # 1. attach-commit
    cli_main(["--db-file", str(db_file), "attach-commit", "--id", "WORM-CLI-001", "--commit", "a1b2c3d"])
    updated = store.get_worm("WORM-CLI-001")
    assert "a1b2c3d" in updated.commit_ids

    # 2. add-task
    cli_main(
        [
            "--db-file",
            str(db_file),
            "add-task",
            "--id",
            "WORM-CLI-001",
            "--title",
            "Firmware Step",
            "--command",
            "echo test",
        ]
    )
    updated = store.get_worm("WORM-CLI-001")
    assert len(updated.review_tasks) == 1
    task_id = updated.review_tasks[0].id
    assert updated.review_tasks[0].title == "Firmware Step"

    # 3. resolve-task
    cli_main(
        [
            "--db-file",
            str(db_file),
            "resolve-task",
            "--id",
            "WORM-CLI-001",
            "--task-id",
            task_id,
            "--status",
            "COMPLETED",
        ]
    )
    updated = store.get_worm("WORM-CLI-001")
    assert updated.review_tasks[0].status == WormReviewTaskStatus.COMPLETED

    # 4. list-worms --open keeps RESOLVED visible as [RESOLVED (AWAITING REVIEW)]
    capsys.readouterr()
    cli_main(["--db-file", str(db_file), "list-worms", "--open"])
    captured = capsys.readouterr()
    assert "WORM-CLI-001" in captured.out
    assert "AWAITING REVIEW" in captured.out

    # 5. reject-worm
    cli_main(["--db-file", str(db_file), "reject-worm", "--id", "WORM-CLI-001", "--notes", "CLI rejected firmware fix"])
    out_rej, err_rej = capsys.readouterr()
    updated = store.get_worm("WORM-CLI-001")
    assert updated.status == WormStatus.OPEN, f"STDOUT: {out_rej}\nSTDERR: {err_rej}\nUPDATED: {updated}"
    assert updated.rejection_notes == "CLI rejected firmware fix"
    assert updated.rejected_at is not None

    # Re-resolve for accept test
    updated.status = WormStatus.RESOLVED
    store.save_worm(updated)

    # 6. accept-worm
    cli_main(["--db-file", str(db_file), "accept-worm", "--id", "WORM-CLI-001", "--notes", "CLI accepted firmware fix"])
    updated = store.get_worm("WORM-CLI-001")
    assert updated.status == WormStatus.CLOSED
    assert updated.accepted_at is not None

    # Verify list-worms --open now hides the CLOSED worm
    capsys.readouterr()
    cli_main(["--db-file", str(db_file), "list-worms", "--open"])
    captured = capsys.readouterr()
    assert "WORM-CLI-001" not in captured.out


def test_migrate_resolved_worms_cli(tmp_path: Path, capsys: pytest.CaptureFixture) -> None:
    """Verify migrate-resolved-worms CLI handles dry-run, timestamp filtering, exclude, and --all."""
    db_file = tmp_path / "test_worms.sqlite"
    store = SQLiteWormStore(db_path=db_file)

    w_old = WormReportModel(
        id="WORM-LEGACY-001",
        title="Legacy resolved worm",
        status=WormStatus.RESOLVED,
        severity=WormSeverity.MEDIUM,
        category=WormCategory.FIRMWARE,
        created_at="2026-09-01 10:00:00 UTC",
        resolved_at="2026-09-01 12:00:00 UTC",
        accepted_at=None,
    )
    w_recent = WormReportModel(
        id="WORM-RECENT-002",
        title="Recently resolved worm awaiting review",
        status=WormStatus.RESOLVED,
        severity=WormSeverity.HIGH,
        category=WormCategory.UI,
        created_at="2026-11-05 14:15:00 UTC",
        resolved_at="2026-11-05 14:20:00 UTC",
        accepted_at=None,
    )
    w_open = WormReportModel(
        id="WORM-OPEN-003",
        title="Open defect",
        status=WormStatus.OPEN,
        severity=WormSeverity.LOW,
        category=WormCategory.FIRMWARE,
    )

    store.save_worm(w_old)
    store.save_worm(w_recent)
    store.save_worm(w_open)

    # 1. Dry run
    capsys.readouterr()
    cli_main(["--db-file", str(db_file), "migrate-resolved-worms", "--dry-run"])
    captured = capsys.readouterr()
    assert "[DRY-RUN]" in captured.out
    assert "WORM-LEGACY-001" in captured.out
    assert "WORM-RECENT-002" not in captured.out

    # Status must not change after dry-run
    assert store.get_worm("WORM-LEGACY-001").status == WormStatus.RESOLVED

    # 2. Exclude flag
    capsys.readouterr()
    cli_main(
        [
            "--db-file",
            str(db_file),
            "migrate-resolved-worms",
            "--all",
            "--exclude",
            "WORM-RECENT-002",
            "--dry-run",
        ]
    )
    captured = capsys.readouterr()
    assert "WORM-LEGACY-001" in captured.out
    assert "WORM-RECENT-002" not in captured.out

    # 3. Default migration (migrates worms resolved <= 2026-11-05 14:00:00 UTC)
    cli_main(["--db-file", str(db_file), "migrate-resolved-worms"])
    updated_old = store.get_worm("WORM-LEGACY-001")
    updated_recent = store.get_worm("WORM-RECENT-002")
    updated_open = store.get_worm("WORM-OPEN-003")

    assert updated_old.status == WormStatus.CLOSED
    assert updated_old.accepted_at == "2026-09-01 12:00:00 UTC"
    assert updated_recent.status == WormStatus.RESOLVED
    assert updated_recent.accepted_at is None
    assert updated_open.status == WormStatus.OPEN

    # 4. Migrate remaining with --all
    cli_main(["--db-file", str(db_file), "migrate-resolved-worms", "--all"])
    updated_recent_all = store.get_worm("WORM-RECENT-002")
    assert updated_recent_all.status == WormStatus.CLOSED
    assert updated_recent_all.accepted_at is not None


def test_sqlite_store_auto_migration_and_markdown_protection(tmp_path: Path) -> None:
    """Verify SQLiteWormStore schema migration of legacy resolved worms and markdown merge protection."""
    db_file = tmp_path / "legacy_test.sqlite"

    # Seed legacy worms directly into SQLite without metadata flag
    conn = sqlite3.connect(str(db_file))
    conn.execute(
        """
        CREATE TABLE worms (
            id TEXT PRIMARY KEY, uuid TEXT, title TEXT NOT NULL, status TEXT NOT NULL,
            severity TEXT NOT NULL, category TEXT NOT NULL, component TEXT DEFAULT '',
            description TEXT DEFAULT '', reproduction_steps TEXT DEFAULT '[]',
            expected_behavior TEXT DEFAULT '', actual_behavior TEXT DEFAULT '',
            logs TEXT DEFAULT '', commit_ids TEXT DEFAULT '[]', review_tasks TEXT DEFAULT '[]',
            created_at TEXT DEFAULT '', updated_at TEXT DEFAULT '', resolved_at TEXT,
            resolution_notes TEXT DEFAULT '', rejection_notes TEXT DEFAULT '',
            accepted_at TEXT, rejected_at TEXT, custom_sections TEXT DEFAULT '[]'
        )
        """
    )
    conn.execute(
        """
        CREATE TABLE metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )
        """
    )
    conn.execute(
        """
        INSERT INTO worms (id, uuid, title, status, severity, category, resolved_at)
        VALUES ('WORM-SEED-001', 'u-seed-1', 'Legacy seed worm', 'RESOLVED', 'MEDIUM', 'FIRMWARE', '2026-09-10 12:00:00 UTC')
        """
    )
    conn.commit()
    conn.close()

    # Initializing SQLiteWormStore runs _ensure_schema and auto-migrates
    store = SQLiteWormStore(db_path=db_file)
    worm = store.get_worm("WORM-SEED-001")
    assert worm is not None
    assert worm.status == WormStatus.CLOSED
    assert worm.accepted_at == "2026-09-10 12:00:00 UTC"

    # Verify metadata flag is recorded
    with store._get_connection() as c:
        row = c.execute("SELECT value FROM metadata WHERE key = 'legacy_resolved_migrated'").fetchone()
        assert row is not None and row["value"] == "1"

    # Test Markdown merge protection: stale markdown with RESOLVED must not regress CLOSED worm
    exporter = MarkdownWormExporter(repo_root=tmp_path)
    base_db = WormDatabaseModel(worms=[worm])
    stale_md_db = WormDatabaseModel(
        worms=[
            WormReportModel(
                id="WORM-SEED-001",
                uuid="u-seed-1",
                title="Legacy seed worm",
                status=WormStatus.RESOLVED,
                severity=WormSeverity.MEDIUM,
                category=WormCategory.FIRMWARE,
                resolved_at="2026-09-10 12:00:00 UTC",
                accepted_at=None,
            )
        ]
    )
    merged = exporter.merge_databases(base_db, stale_md_db)
    merged_worm = merged.get_worm("WORM-SEED-001")
    assert merged_worm.status == WormStatus.CLOSED
    assert merged_worm.accepted_at == "2026-09-10 12:00:00 UTC"


def test_api_migrate_legacy_worms(tmp_path: Path) -> None:
    """Verify REST API endpoint POST /api/worms/migrate_legacy."""
    db_file = tmp_path / "api_test.sqlite"
    server = WormReportServer(
        host="127.0.0.1",
        port=0,
        repo_root=tmp_path,
        sqlite_file=db_file,
        bind_and_activate=True,
    )

    t = threading.Thread(target=server.serve_forever, daemon=True)
    t.start()

    try:
        url = server.get_url()

        # Add a legacy resolved worm directly to server database
        server.database.worms.append(
            WormReportModel(
                id="WORM-API-001",
                title="API legacy worm",
                status=WormStatus.RESOLVED,
                severity=WormSeverity.LOW,
                category=WormCategory.FIRMWARE,
                resolved_at="2026-09-15 10:00:00 UTC",
                accepted_at=None,
            )
        )
        server.sqlite_store.save_database(server.database)

        # POST /api/worms/migrate_legacy
        req = urllib.request.Request(
            f"{url}/api/worms/migrate_legacy",
            data=json.dumps({"all": True}).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urllib.request.urlopen(req) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            assert data["status"] == "ok"
            assert data["count"] >= 1
            assert "WORM-API-001" in data["migrated_ids"]

        migrated_worm = server.database.get_worm("WORM-API-001")
        assert migrated_worm.status == WormStatus.CLOSED
        assert migrated_worm.accepted_at == "2026-09-15 10:00:00 UTC"
    finally:
        server.server_close()
        server.shutdown()
        t.join(timeout=2.0)
