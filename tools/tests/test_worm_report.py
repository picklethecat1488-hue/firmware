"""Unit and regression tests for Bug Report CLI, models, and markdown tracker."""

import base64
import json
from pathlib import Path
import subprocess
import sys
import threading
import time
import urllib.request
import pytest

from model.worm_report import (
    WormAttachmentModel,
    WormCategory,
    WormDatabaseModel,
    WormReportModel,
    WormSeverity,
    WormStatus,
)
from provider.worm_report.markdown_exporter import MarkdownWormExporter
from provider.worm_report.server import WormReportRequestHandler, WormReportServer
from provider.worm_report.sqlite_store import SQLiteWormStore


def test_worm_report_model_lifecycle() -> None:
    """Verify bug report model instantiation, validation, and status transitions."""
    bug = WormReportModel(
        id="WORM-101",
        title="Test Bug",
        status=WormStatus.OPEN,
        severity=WormSeverity.HIGH,
        category=WormCategory.DRIVER,
        component="test_component",
        description="Detailed failure description",
        reproduction_steps=["step 1", "step 2"],
        expected_behavior="Expected outcome",
        actual_behavior="Actual outcome",
        logs="Sample error logs",
        attachments=[
            WormAttachmentModel(
                id="att-1",
                filename="test.png",
                file_type="screenshot",
                file_path="build/test.png",
                description="Test preview",
            )
        ],
    )
    assert bug.id == "WORM-101"
    assert bug.status == WormStatus.OPEN
    assert bug.severity == WormSeverity.HIGH
    assert len(bug.attachments) == 1
    assert bug.attachments[0].filename == "test.png"

    # Test serialization
    data = bug.model_dump(mode="json")
    assert data["id"] == "WORM-101"
    assert data["severity"] == "HIGH"

    restored = WormReportModel.model_validate(data)
    assert restored.id == bug.id
    assert restored.category == WormCategory.DRIVER


def test_worm_database_metrics_and_management(tmp_path: Path) -> None:
    """Verify bug collection aggregation, ID generation, and markdown export."""
    db = WormDatabaseModel(title="Unit Test Tracker")
    assert db.generate_worm_id() == "WORM-001"

    b1 = WormReportModel(
        id="WORM-001",
        title="First defect",
        status=WormStatus.OPEN,
        severity=WormSeverity.CRITICAL,
        category=WormCategory.CONTROLLER,
    )
    b2 = WormReportModel(
        id="WORM-002",
        title="Second defect",
        status=WormStatus.RESOLVED,
        severity=WormSeverity.LOW,
        category=WormCategory.DRIVER,
    )
    db.add_or_update(b1)
    db.add_or_update(b2)

    assert db.generate_worm_id() == "WORM-003"
    assert db.count_by_status()[WormStatus.OPEN.value] == 1
    assert db.count_by_status()[WormStatus.RESOLVED.value] == 1
    assert db.count_by_severity()[WormSeverity.CRITICAL.value] == 1
    assert db.count_by_category()[WormCategory.CONTROLLER.value] == 1
    assert db.count_by_category()[WormCategory.DRIVER.value] == 1

    # Export to markdown and JSON
    exporter = MarkdownWormExporter(repo_root=tmp_path)
    md_file = tmp_path / "WORMS.md"
    json_file = tmp_path / "worms_state.json"

    saved_md = exporter.export_markdown(db, md_file)
    assert saved_md.exists()
    content = saved_md.read_text(encoding="utf-8")
    assert "Unit Test Tracker" in content
    assert "[WORM-001]" in content
    assert "[WORM-002]" in content
    assert "CRITICAL" in content

    saved_json = exporter.export_state_json(db, json_file)
    assert saved_json.exists()
    loaded_db = exporter.load_state_json(saved_json)
    assert loaded_db is not None
    assert len(loaded_db.worms) == 2


def test_worm_report_server_initialization(tmp_path: Path) -> None:
    """Verify WormReportServer initialization, state loading, and persistence."""
    md_file = tmp_path / "WORMS.md"
    json_file = tmp_path / "worms_state.json"

    server = WormReportServer(
        host="127.0.0.1",
        port=8799,
        repo_root=tmp_path,
        markdown_output=md_file,
        state_file=json_file,
        fresh=True,
    )
    assert server.database.title == "Firmware Worm Tracker"
    assert len(server.database.worms) == 0

    # Add bug and sync
    bug = WormReportModel(
        id="WORM-001",
        title="Server test bug",
        status=WormStatus.OPEN,
        severity=WormSeverity.MEDIUM,
        category=WormCategory.UI,
    )
    server.database.add_or_update(bug)
    server.save_and_sync()

    assert md_file.exists()
    assert json_file.exists()
    server.server_close()

    # Re-initialize without fresh to ensure persistence reload
    reloaded_server = WormReportServer(
        host="127.0.0.1",
        port=8799,
        repo_root=tmp_path,
        markdown_output=md_file,
        state_file=json_file,
        fresh=False,
    )
    assert len(reloaded_server.database.worms) == 1
    assert reloaded_server.database.worms[0].id == "WORM-001"
    reloaded_server.server_close()


def test_worm_report_server_api_exit_and_document_upload(tmp_path: Path) -> None:
    """Verify WormReportServer document upload handling and exit endpoint."""
    md_file = tmp_path / "WORMS.md"
    json_file = tmp_path / "worms_state.json"
    att_dir = tmp_path / "attachments"

    server = WormReportServer(
        host="127.0.0.1",
        port=8798,
        repo_root=tmp_path,
        markdown_output=md_file,
        state_file=json_file,
        attachments_dir=att_dir,
        fresh=True,
    )
    t = threading.Thread(target=server.serve_forever, daemon=True)
    t.start()

    try:
        # Test document upload via /api/upload
        doc_content = b"%PDF-1.4 Mock PDF Content"
        b64_doc = base64.b64encode(doc_content).decode("ascii")
        req_data = json.dumps(
            {
                "filename": "test_spec.pdf",
                "file_type": "document",
                "content_base64": b64_doc,
                "description": "Pasted specification document",
            }
        ).encode("utf-8")
        req = urllib.request.Request(
            f"{server.get_url()}/api/upload",
            data=req_data,
            headers={"Content-Type": "application/json"},
        )
        with urllib.request.urlopen(req) as resp:
            assert resp.status == 200
            res = json.loads(resp.read().decode("utf-8"))
            assert res["filename"] == "test_spec.pdf"
            assert res["file_type"] == "document"
            assert (att_dir / "test_spec.pdf").exists()

        # Test /api/exit endpoint
        exit_req = urllib.request.Request(
            f"{server.get_url()}/api/exit",
            data=b"{}",
            headers={"Content-Type": "application/json"},
        )
        with urllib.request.urlopen(exit_req) as resp:
            assert resp.status == 200
            res = json.loads(resp.read().decode("utf-8"))
            assert res["status"] == "saved_and_exited"

        t.join(timeout=2.0)
    finally:
        server.server_close()


def test_sqlite_bug_store_lifecycle(tmp_path: Path) -> None:
    """Verify SQLiteWormStore schema initialization, atomic upsert, query, and cascade delete."""
    db_file = tmp_path / "worms.sqlite"
    store = SQLiteWormStore(db_file)
    assert db_file.exists()

    # 1. Test empty database load
    db = store.load_database()
    assert db.title == "Firmware Engineering Worm Tracker"
    assert len(db.worms) == 0

    # 2. Add bug with attachment and reproduction steps
    att = WormAttachmentModel(
        id="att-001",
        filename="schematic_error.png",
        file_type="screenshot",
        file_path="build/attachments/schematic_error.png",
        size_bytes=1024,
        description="Error screenshot",
        created_at="2026-09-20 12:00:00 UTC",
    )
    bug = WormReportModel(
        id="WORM-001",
        title="Sample Defect",
        status=WormStatus.OPEN,
        severity=WormSeverity.HIGH,
        category=WormCategory.BOARD,
        component="carrier_board",
        description="Trace clearance violation",
        reproduction_steps=["Open schematic", "Run DRC"],
        expected_behavior="0 errors",
        actual_behavior="1 error",
        logs="Error log output",
        attachments=[att],
        created_at="2026-09-20 12:00:00 UTC",
        updated_at="2026-09-20 12:00:00 UTC",
    )
    db.add_or_update(bug)
    store.save_database(db)

    # 3. Reload and assert all data preserved
    reloaded = store.load_database()
    assert len(reloaded.worms) == 1
    b = reloaded.get_worm("WORM-001")
    assert b is not None
    assert b.title == "Sample Defect"
    assert b.status == WormStatus.OPEN
    assert b.severity == WormSeverity.HIGH
    assert b.reproduction_steps == ["Open schematic", "Run DRC"]
    assert len(b.attachments) == 1
    assert b.attachments[0].filename == "schematic_error.png"

    # 4. Update status and notes
    b.status = WormStatus.RESOLVED
    b.resolution_notes = "Rerouted trace on F.Cu"
    b.resolved_at = "2026-09-20 12:30:00 UTC"
    store.save_worm(b)

    reloaded2 = store.load_database()
    b2 = reloaded2.get_worm("WORM-001")
    assert b2 is not None
    assert b2.status == WormStatus.RESOLVED
    assert b2.resolution_notes == "Rerouted trace on F.Cu"

    # 5. Test JSON export and import round trip
    json_path = tmp_path / "exported.json"
    store.export_to_json(json_path)
    assert json_path.exists()

    db_file2 = tmp_path / "imported.sqlite"
    store2 = SQLiteWormStore(db_file2)
    imported = store2.import_from_json(json_path)
    assert len(imported.worms) == 1
    assert imported.worms[0].id == "WORM-001"

    # 6. Delete bug
    store.delete_worm("WORM-001")
    reloaded3 = store.load_database()
    assert len(reloaded3.worms) == 0


def test_server_sqlite_integration(tmp_path: Path) -> None:
    """Verify WormReportServer seamlessly syncs between SQLite backing store, JSON, and Markdown."""
    sqlite_file = tmp_path / "worms.sqlite"
    state_file = tmp_path / "worms_state.json"
    md_file = tmp_path / "WORMS.md"

    server = WormReportServer(
        repo_root=tmp_path,
        sqlite_file=sqlite_file,
        state_file=state_file,
        markdown_output=md_file,
        bind_and_activate=False,
    )

    # 1. Add a bug through server database
    bug = WormReportModel(
        id="WORM-069",
        title="SQLite Backing Store Verification",
        status=WormStatus.OPEN,
        severity=WormSeverity.MEDIUM,
        category=WormCategory.INFRASTRUCTURE,
        description="Verify SQLite database integration",
    )
    server.database.add_or_update(bug)
    server.save_and_sync()

    # 2. Verify files created and synced
    assert sqlite_file.exists()
    assert state_file.exists()
    assert md_file.exists()

    # 3. Spawn a new server instance pointing to the same SQLite store
    server2 = WormReportServer(
        repo_root=tmp_path,
        sqlite_file=sqlite_file,
        state_file=state_file,
        markdown_output=md_file,
        bind_and_activate=False,
    )
    loaded_bug = server2.database.get_worm("WORM-069")
    assert loaded_bug is not None
    assert loaded_bug.title == "SQLite Backing Store Verification"
    assert loaded_bug.status == WormStatus.OPEN


def test_regression_bug_076_feedback_tools_sqlite_only():
    """Verify references to markdown and state files are removed from dashboard.py CLI flags."""
    res = subprocess.run([sys.executable, "tools/dashboard.py", "--help"], capture_output=True, text=True, check=True)
    assert "--output" not in res.stdout, "dashboard.py should not have --output flag"
    assert "--state-file" not in res.stdout, "dashboard.py should not have --state-file flag"
    assert "WORMS.md" not in res.stdout, "dashboard.py should not reference WORMS.md"
    assert "worms_state.json" not in res.stdout, "dashboard.py should not reference worms_state.json"
    assert "CR.md" not in res.stdout, "dashboard.py should not reference CR.md"
    assert "cr_feedback.json" not in res.stdout, "dashboard.py should not reference cr_feedback.json"


def test_regression_bug_079_no_duplicate_worm_ids_and_generator():
    """Verify bug report workstation does not generate duplicate bug IDs using bugs.length."""
    templates_dir = Path(__file__).resolve().parent.parent / "dashboard" / "templates"
    template_file = templates_dir / "worm_report.html.j2"
    assert template_file.exists()
    content = template_file.read_text(encoding="utf-8")

    # 1. Frontend template must NOT generate bug ID using db.worms.length + 1
    assert "db.worms.length + 1" not in content, (
        "Frontend worm_report.html.j2 must not generate bug IDs from db.worms.length + 1 as missing IDs cause collisions"
    )

    # 2. Frontend must define generateNextWormId using max ID index
    assert "generateNextWormId" in content, (
        "Frontend worm_report.html.j2 must define generateNextWormId calculating max bug index"
    )

    # 3. Backend WormDatabaseModel must correctly skip to max + 1 when IDs are non-contiguous
    db = WormDatabaseModel(title="Non-contiguous test")
    db.worms = [
        WormReportModel(
            id="WORM-001", title="B1", status=WormStatus.OPEN, severity=WormSeverity.LOW, category=WormCategory.DRIVER
        ),
        WormReportModel(
            id="WORM-002", title="B2", status=WormStatus.OPEN, severity=WormSeverity.LOW, category=WormCategory.DRIVER
        ),
        WormReportModel(
            id="WORM-078", title="B78", status=WormStatus.OPEN, severity=WormSeverity.LOW, category=WormCategory.DRIVER
        ),
    ]
    # Length is 3, but max is 78. Next ID MUST be WORM-079, NOT WORM-004
    assert db.generate_worm_id() == "WORM-079"


def test_regression_bug_114_rmw_markdown_sync_and_file_watch(tmp_path: Path) -> None:
    """Verify bug report tool uses R+M+W to sync WORMS.md changes into SQLite and database."""
    sqlite_file = tmp_path / "worms.sqlite"
    state_file = tmp_path / "worms_state.json"
    md_file = tmp_path / "WORMS.md"

    server = WormReportServer(
        repo_root=tmp_path,
        sqlite_file=sqlite_file,
        state_file=state_file,
        markdown_output=md_file,
        bind_and_activate=False,
    )

    # 1. Initial bug saved
    b1 = WormReportModel(
        id="WORM-001",
        title="Initial Bug",
        status=WormStatus.OPEN,
        severity=WormSeverity.LOW,
        category=WormCategory.DRIVER,
    )
    server.database.add_or_update(b1)
    server.save_and_sync()

    # 2. External edit: user marks WORM-001 as RESOLVED in markdown
    md_text = md_file.read_text(encoding="utf-8")
    md_text = md_text.replace("`OPEN`", "`RESOLVED`")
    time.sleep(0.05)  # Ensure mtime difference
    md_file.write_text(md_text, encoding="utf-8")

    # 3. Trigger file watch check
    updated = server.check_file_watch()
    assert updated is True
    loaded_b1 = server.database.get_worm("WORM-001")
    assert loaded_b1 is not None
    assert loaded_b1.status == WormStatus.RESOLVED

    # 4. Verify SQLite was updated via R+M+W
    sqlite_b1 = server.sqlite_store.load_database().get_worm("WORM-001")
    assert sqlite_b1 is not None
    assert sqlite_b1.status == WormStatus.RESOLVED


def test_regression_bug_169_git_lfs_tracking_and_policy() -> None:
    """Verify attachments are documented as tracked in GitHub LFS and non-confidential."""
    repo_root = Path(__file__).resolve().parent.parent.parent

    # Check GEMINI.md
    gemini_file = repo_root / "GEMINI.md"
    assert gemini_file.exists()
    gemini_text = gemini_file.read_text(encoding="utf-8")
    assert "GitHub LFS" in gemini_text or "Git LFS" in gemini_text
    assert "non-confidential" in gemini_text

    # Check bug_report template
    template_file = repo_root / "tools" / "dashboard" / "templates" / "worm_report.html.j2"
    tpl_text = template_file.read_text(encoding="utf-8")
    assert "attachment-lfs-notice" in tpl_text
    assert "GitHub LFS" in tpl_text


def test_regression_bug_170_feedback_dir_watch_and_resolution_preservation(tmp_path: Path) -> None:
    """Verify Feedback directory watch detects external bug resolution and preserves resolution on Save."""
    feedback_dir = tmp_path / "feedback"
    feedback_dir.mkdir(parents=True)
    db_file = tmp_path / "worms.sqlite"
    state_file = tmp_path / "worms_state.json"
    md_file = feedback_dir / "WORMS.md"

    server = WormReportServer(
        repo_root=tmp_path,
        feedback_dir=feedback_dir,
        sqlite_file=db_file,
        state_file=state_file,
        markdown_output=md_file,
        bind_and_activate=False,
    )

    # 1. Add bug 166 as OPEN
    b166 = WormReportModel(
        id="WORM-166",
        title="Silkscreen text is mirrored",
        status=WormStatus.OPEN,
        severity=WormSeverity.HIGH,
        category=WormCategory.UI,
        description="F.SilkS text is vertically flipped",
    )
    server.database.add_or_update(b166)
    server.save_and_sync()

    bug166_md = feedback_dir / "WORM_166.md"
    assert bug166_md.is_file()
    assert "- **Status**: `OPEN`" in bug166_md.read_text(encoding="utf-8")

    # 2. External edit: user marks WORM_166.md as RESOLVED
    time.sleep(0.02)
    b166_content = bug166_md.read_text(encoding="utf-8")
    b166_content = b166_content.replace("- **Status**: `OPEN`", "- **Status**: `RESOLVED`")
    bug166_md.write_text(b166_content, encoding="utf-8")

    # 3. Server file watch detects edit
    changed = server.check_file_watch()
    assert changed is True
    reloaded_b166 = server.database.get_worm("WORM-166")
    assert reloaded_b166.status == WormStatus.RESOLVED

    # 4. User modifies ANOTHER bug or triggers save_and_sync
    server.save_and_sync()

    # 5. Assert WORM-166 REMAINS RESOLVED across memory, markdown, and SQLite
    final_b166 = server.database.get_worm("WORM-166")
    assert final_b166.status == WormStatus.RESOLVED
    assert "- **Status**: `RESOLVED`" in bug166_md.read_text(encoding="utf-8")
    assert server.sqlite_store.load_database().get_worm("WORM-166").status == WormStatus.RESOLVED


def test_regression_bug_172_vertical_text_panel_and_cli_expansion() -> None:
    """Verify text panels in bug report tool and CLI console in code review tool extend vertically."""
    repo_root = Path(__file__).resolve().parent.parent.parent

    # 1. Verify code_review.html.j2 has vertical CLI console resizing
    cr_template = (repo_root / "tools" / "dashboard" / "templates" / "code_review.html.j2").read_text(encoding="utf-8")
    assert "cli-resizer" in cr_template
    assert "cliResizer" in cr_template
    assert "setupCliResizer" in cr_template
    assert "cursor: ns-resize" in cr_template

    # 2. Verify worm_report.html.j2 has vertical text panel expansion
    br_template = (repo_root / "tools" / "dashboard" / "templates" / "worm_report.html.j2").read_text(encoding="utf-8")
    assert "panel-expand-btn" in br_template
    assert "expanded-vertical" in br_template
    assert "toggleExpandPanel" in br_template
    assert "resize: both" in br_template or "resize: vertical" in br_template


def test_regression_bug_181_no_file_descriptor_leak_in_sqlite_and_server(tmp_path: Path) -> None:
    """Verify SQLite stores, markdown exporter, and server do not leak file descriptors."""
    db_file = tmp_path / "worms.sqlite"
    feedback_dir = tmp_path / "feedback"
    feedback_dir.mkdir()
    md_file = tmp_path / "WORMS.md"
    state_file = tmp_path / "worms_state.json"

    def get_open_fd_count() -> int:
        fd_dir = Path("/dev/fd") if Path("/dev/fd").exists() else Path("/proc/self/fd")
        if fd_dir.exists():
            try:
                return len(list(fd_dir.iterdir()))
            except Exception:
                return 0
        return 0

    # 1. Test SQLiteWormStore does not leak connections/FDs on repeated load/save
    store = SQLiteWormStore(db_file)
    initial_db = WormDatabaseModel(title="FD Leak Test")
    initial_db.add_or_update(
        WormReportModel(
            id="WORM-001",
            title="FD Test Bug",
            status=WormStatus.OPEN,
            severity=WormSeverity.LOW,
            category=WormCategory.INFRASTRUCTURE,
        )
    )
    store.save_database(initial_db)

    baseline_fds = get_open_fd_count()
    if baseline_fds > 0:
        for _ in range(30):
            loaded = store.load_database()
            store.save_database(loaded)
        after_sqlite_fds = get_open_fd_count()
        assert after_sqlite_fds <= baseline_fds + 1, (
            f"SQLiteWormStore leaked file descriptors: baseline={baseline_fds}, after={after_sqlite_fds}"
        )

    # 2. Test MarkdownWormExporter does not re-write identical bug files
    exporter = MarkdownWormExporter(repo_root=tmp_path)
    test_worm = initial_db.worms[0]
    out_file = exporter.export_individual_worm(test_worm, feedback_dir)
    assert out_file.exists()
    initial_mtime = out_file.stat().st_mtime_ns

    time.sleep(0.01)
    out_file_2 = exporter.export_individual_worm(test_worm, feedback_dir)
    assert out_file_2 == out_file
    second_mtime = out_file_2.stat().st_mtime_ns
    assert second_mtime == initial_mtime, "export_individual_worm must not rewrite identical file contents"

    # 3. Test WormReportServer save_and_sync and check_file_watch do not leak FDs
    server = WormReportServer(
        host="127.0.0.1",
        port=8776,
        repo_root=tmp_path,
        markdown_output=md_file,
        feedback_dir=feedback_dir,
        state_file=state_file,
        sqlite_file=db_file,
        bind_and_activate=False,
    )

    baseline_server_fds = get_open_fd_count()
    if baseline_server_fds > 0:
        for _ in range(20):
            server.save_and_sync()
            server.check_file_watch()
        after_server_fds = get_open_fd_count()
        assert after_server_fds <= baseline_server_fds + 1, (
            f"WormReportServer leaked file descriptors: baseline={baseline_server_fds}, after={after_server_fds}"
        )


def test_regression_bug_186_bug_report_server_attachments_dir(tmp_path: Path) -> None:
    """Verify WormReportServer defaults attachments directory to attachments/."""
    server = WormReportServer(
        host="127.0.0.1",
        port=8776,
        repo_root=tmp_path,
        markdown_output=tmp_path / "WORMS.md",
        feedback_dir=tmp_path / "feedback",
        state_file=tmp_path / "worms_state.json",
        sqlite_file=tmp_path / "worms.sqlite",
        bind_and_activate=False,
    )
    assert server.attachments_dir == tmp_path / "attachments"


def test_regression_bug_264_recreate_db_after_build_dir_removed(tmp_path: Path) -> None:
    """Verify WORM-264: SQLite stores seamlessly recover and recreate schema after build directory removal."""
    import shutil

    build_dir = tmp_path / "build"
    db_file = build_dir / "worms.sqlite"
    store = SQLiteWormStore(db_file)
    store.save_worm(
        WormReportModel(
            id="WORM-001",
            title="Test Bug",
            status=WormStatus.OPEN,
            severity=WormSeverity.MEDIUM,
            category=WormCategory.INFRASTRUCTURE,
        )
    )
    assert db_file.exists()

    # Simulate rm -rf build/
    shutil.rmtree(build_dir)
    assert not build_dir.exists()

    # Saving/loading on the store must recreate directory and schema without OperationalError
    store.save_worm(
        WormReportModel(
            id="WORM-002",
            title="New Bug",
            status=WormStatus.OPEN,
            severity=WormSeverity.MEDIUM,
            category=WormCategory.INFRASTRUCTURE,
        )
    )
    assert db_file.exists()
    db = store.load_database()
    assert any(b.id == "WORM-002" for b in db.worms)


def test_regression_bug_269_scoped_attachments_same_filename(tmp_path: Path) -> None:
    """Verify WORM-269: Encode attachments scoped by bug ID so multiple bugs can have attachments with the same name."""
    import threading
    import urllib.request

    att_dir = tmp_path / "attachments"
    md_file = tmp_path / "WORMS.md"
    fb_dir = tmp_path / "feedback"
    db_file = tmp_path / "worms.sqlite"
    json_file = tmp_path / "worms_state.json"

    server = WormReportServer(
        host="127.0.0.1",
        port=0,
        repo_root=tmp_path,
        feedback_dir=fb_dir,
        markdown_output=md_file,
        state_file=json_file,
        sqlite_file=db_file,
        attachments_dir=att_dir,
        fresh=True,
    )
    t = threading.Thread(target=server.serve_forever, daemon=True)
    t.start()

    try:
        base_url = server.get_url()

        # 1. Upload daemon.log for WORM-268
        payload_268 = json.dumps(
            {
                "worm_id": "WORM-268",
                "filename": "daemon.log",
                "file_type": "document",
                "content_text": "daemon log for bug 268",
                "description": "Build log for WORM-268",
            }
        ).encode("utf-8")
        req1 = urllib.request.Request(
            f"{base_url}/api/upload",
            data=payload_268,
            headers={"Content-Type": "application/json"},
        )
        with urllib.request.urlopen(req1) as resp:
            assert resp.status == 200
            res1 = json.loads(resp.read().decode("utf-8"))
            assert res1["filename"] == "daemon.log"
            assert res1["file_path"] == "attachments/WORM-268/daemon.log"

        file_268 = att_dir / "WORM-268" / "daemon.log"
        assert file_268.exists()
        assert file_268.read_text(encoding="utf-8") == "daemon log for bug 268"

        # 2. Upload another attachment with the EXACT SAME filename daemon.log for WORM-269
        payload_269 = json.dumps(
            {
                "worm_id": "WORM-269",
                "filename": "daemon.log",
                "file_type": "document",
                "content_text": "daemon log for bug 269",
                "description": "Build log for WORM-269",
            }
        ).encode("utf-8")
        req2 = urllib.request.Request(
            f"{base_url}/api/upload",
            data=payload_269,
            headers={"Content-Type": "application/json"},
        )
        with urllib.request.urlopen(req2) as resp:
            assert resp.status == 200
            res2 = json.loads(resp.read().decode("utf-8"))
            assert res2["filename"] == "daemon.log"
            assert res2["file_path"] == "attachments/WORM-269/daemon.log"

        file_269 = att_dir / "WORM-269" / "daemon.log"
        assert file_269.exists()
        assert file_269.read_text(encoding="utf-8") == "daemon log for bug 269"

        # 3. Assert WORM-268 attachment was NOT overwritten
        assert file_268.read_text(encoding="utf-8") == "daemon log for bug 268"

        # 4. Verify HTTP serving of both attachments
        with urllib.request.urlopen(f"{base_url}/attachments/WORM-268/daemon.log") as resp:
            assert resp.status == 200
            assert resp.read() == b"daemon log for bug 268"

        with urllib.request.urlopen(f"{base_url}/attachments/WORM-269/daemon.log") as resp:
            assert resp.status == 200
            assert resp.read() == b"daemon log for bug 269"

        # 5. Verify database and markdown round trip for both bugs
        b268 = WormReportModel(
            id="WORM-268",
            title="Bug 268",
            status=WormStatus.RESOLVED,
            severity=WormSeverity.MEDIUM,
            category=WormCategory.GENERAL,
            attachments=[WormAttachmentModel.model_validate(res1)],
        )
        b269 = WormReportModel(
            id="WORM-269",
            title="Bug 269",
            status=WormStatus.OPEN,
            severity=WormSeverity.LOW,
            category=WormCategory.INFRASTRUCTURE,
            attachments=[WormAttachmentModel.model_validate(res2)],
        )
        server.database.add_or_update(b268)
        server.database.add_or_update(b269)
        server.save_and_sync()

        # Check markdown export
        md_268 = fb_dir / "WORM_268.md"
        md_269 = fb_dir / "WORM_269.md"
        assert md_268.exists()
        assert md_269.exists()
        assert "[daemon.log](attachments/WORM-268/daemon.log)" in md_268.read_text(encoding="utf-8")
        assert "[daemon.log](attachments/WORM-269/daemon.log)" in md_269.read_text(encoding="utf-8")

        # Check round-trip parsing from markdown
        parsed_268 = server.exporter.parse_individual_worm_file(md_268)
        assert parsed_268 is not None
        assert len(parsed_268.attachments) == 1
        assert parsed_268.attachments[0].filename == "daemon.log"
        assert parsed_268.attachments[0].file_path == "attachments/WORM-268/daemon.log"

        parsed_269 = server.exporter.parse_individual_worm_file(md_269)
        assert parsed_269 is not None
        assert len(parsed_269.attachments) == 1
        assert parsed_269.attachments[0].filename == "daemon.log"
        assert parsed_269.attachments[0].file_path == "attachments/WORM-269/daemon.log"
    finally:
        server.server_close()


def test_regression_bug_270_bug_report_server_handles_broken_pipe_gracefully(
    capsys: pytest.CaptureFixture[str],
) -> None:
    """Verify WORM-270: BrokenPipeError and client disconnects are handled cleanly without printing tracebacks in WormReportServer."""
    from unittest.mock import MagicMock
    from provider.worm_report.server import WormReportRequestHandler, WormReportServer

    server = WormReportServer.__new__(WormReportServer)

    # 1. Verify BrokenPipeError in handle_error does not print traceback to stderr
    try:
        raise BrokenPipeError(32, "Broken pipe")
    except BrokenPipeError:
        server.handle_error(None, ("127.0.0.1", 58468))

    captured = capsys.readouterr()
    assert "Traceback" not in captured.err
    assert "BrokenPipeError" not in captured.err

    # 2. Verify unexpected errors are still reported to super().handle_error
    try:
        raise RuntimeError("Real unexpected server failure")
    except RuntimeError:
        server.handle_error(None, ("127.0.0.1", 58468))

    captured = capsys.readouterr()
    assert "Real unexpected server failure" in captured.err

    # 3. Verify handler _send_json and _send_html gracefully handle BrokenPipeError
    handler = WormReportRequestHandler.__new__(WormReportRequestHandler)
    mock_wfile = MagicMock()
    mock_wfile.write.side_effect = BrokenPipeError(32, "Broken pipe")
    handler.wfile = mock_wfile
    handler.send_response = MagicMock()
    handler.send_header = MagicMock()
    handler.end_headers = MagicMock()
    handler.close_connection = False

    # Should not raise exception, and should set close_connection = True
    handler._send_json({"status": "ok"})
    assert handler.close_connection is True

    handler.close_connection = False
    handler._send_html("<html><body>test</body></html>")
    assert handler.close_connection is True


def test_worm_report_server_target_defaults_and_build_migration(tmp_path: Path) -> None:
    """Verify WormReportServer defaults temporary storage to target/ and migrates existing build/ store."""
    repo = tmp_path / "repo"
    repo.mkdir()
    build_dir = repo / "build"
    build_dir.mkdir()
    target_dir = repo / "target"

    # Seed build/worms.sqlite
    build_store = SQLiteWormStore(build_dir / "worms.sqlite")
    build_store.save_worm(
        WormReportModel(
            id="WORM-123",
            title="Migrated Bug",
            status=WormStatus.OPEN,
            severity=WormSeverity.HIGH,
            category=WormCategory.INFRASTRUCTURE,
        )
    )

    # Instantiate server without explicit paths
    server = WormReportServer(
        host="127.0.0.1",
        port=0,
        repo_root=repo,
        bind_and_activate=False,
    )

    # Assert paths default to target/
    assert server.markdown_output == target_dir / "WORMS.md"
    assert server.state_file == target_dir / "worms_state.json"
    assert server.sqlite_file == target_dir / "worms.sqlite"
    assert (target_dir / "worms.sqlite").exists()

    # Verify bug was migrated and accessible
    target_store = SQLiteWormStore(server.sqlite_file)
    db = target_store.load_database()
    assert any(b.id == "WORM-123" and b.title == "Migrated Bug" for b in db.worms)


def test_worm_report_template_saves_and_updates_view_when_no_active_worm(tmp_path: Path) -> None:
    """Verify worm_report.html.j2 creates bug model, unshifts to db, and calls loadActiveWorm() on save."""
    template_path = Path(__file__).resolve().parent.parent / "dashboard" / "templates" / "worm_report.html.j2"
    assert template_path.is_file()
    content = template_path.read_text(encoding="utf-8")

    # Verify saveActiveWorm handles when bug is null / activeWormId not found
    assert "let worm = db.worms.find(w => w.id === activeWormId);" in content
    assert "if (!worm) {" in content
    assert "generateNextWormId()" in content
    assert "db.worms.unshift(worm);" in content
    assert "activeWormId = newId;" in content
    # Verify loadActiveWorm() is called after saving to update header and active selection view
    assert "loadActiveWorm();" in content


def test_regression_bug_001_remove_hardware_categories(tmp_path: Path) -> None:
    """Verify hardware-specific categories (PCB, CAD, SIMULATION) are removed from WormCategory and dashboard."""
    from model.worm_report import WormCategory, WormDatabaseModel, WormReportModel, WormSeverity, WormStatus
    from provider.worm_report.markdown_exporter import MarkdownWormExporter
    from provider.worm_report.server import WormReportServer

    # 1. Assert hardware categories are NOT in WormCategory
    assert not hasattr(WormCategory, "PCB"), "PCB should be removed from WormCategory"
    assert not hasattr(WormCategory, "CAD"), "CAD should be removed from WormCategory"
    assert not hasattr(WormCategory, "SIMULATION"), "SIMULATION should be removed from WormCategory"

    # 2. Assert firmware categories ARE present
    expected_categories = {
        "FIRMWARE",
        "CONTROLLER",
        "DRIVER",
        "PLATFORM",
        "BOARD",
        "MODEL",
        "SHELL",
        "INFRASTRUCTURE",
        "UI",
        "GENERAL",
    }
    actual_categories = {c.value for c in WormCategory}
    assert expected_categories.issubset(actual_categories)

    # 3. Verify server initializes with firmware category defaults and categories list has no hardware categories
    server = WormReportServer(
        repo_root=tmp_path,
        sqlite_file=tmp_path / "worms.sqlite",
        markdown_output=tmp_path / "WORMS.md",
        bind_and_activate=False,
    )
    categories = [c.value for c in WormCategory]
    assert "PCB" not in categories
    assert "CAD" not in categories
    assert "SIMULATION" not in categories

    # 4. Verify markdown export does not hardcode hardware categories
    db = WormDatabaseModel(title="Firmware Worm Tracker")
    db.worms.append(
        WormReportModel(
            id="WORM-001",
            title="Firmware Test",
            severity=WormSeverity.MEDIUM,
            status=WormStatus.OPEN,
            category=WormCategory.DRIVER,
        )
    )
    exporter = MarkdownWormExporter(repo_root=tmp_path)
    md_text = exporter.render_markdown(db)
    assert "**`PCB`**" not in md_text
    assert "**`CAD`**" not in md_text
    assert "**`SIMULATION`**" not in md_text
    assert "**`DRIVER`**" in md_text


def test_regression_bug_002_update_bug_to_worm(tmp_path: Path) -> None:
    """Verify WORM-002: issue tracking terminology updated to WORM and icon updated to worm."""
    import model.worm_report as worm_mod

    # 1. Models must use WORM naming and generate WORM- prefixes
    assert hasattr(worm_mod, "WormReportModel"), "WormReportModel must exist"
    assert hasattr(worm_mod, "WormDatabaseModel"), "WormDatabaseModel must exist"
    db = worm_mod.WormDatabaseModel(title="Xerxes Worm Tracker")
    assert db.generate_worm_id().startswith("WORM-")

    # 2. UI templates must contain the worm icon 🪱 and WORM terminology
    diff_tpl_path = Path(__file__).resolve().parent.parent / "dashboard" / "templates" / "diff_view.html.j2"
    diff_tpl = diff_tpl_path.read_text(encoding="utf-8")
    assert "🪱" in diff_tpl, "diff_view.html.j2 must feature worm icon 🪱"
    assert "Worm" in diff_tpl or "WORM" in diff_tpl, "diff_view.html.j2 must feature WORM terminology"

    worm_tpl_path = Path(__file__).resolve().parent.parent / "dashboard" / "templates" / "worm_report.html.j2"
    assert worm_tpl_path.is_file(), "worm_report.html.j2 must exist"
    worm_tpl = worm_tpl_path.read_text(encoding="utf-8")
    assert "WORM" in worm_tpl
    assert "🪱" in worm_tpl or "Worm" in worm_tpl


def test_regression_worm_018_component_autocomplete_datalist() -> None:
    """Verify WORM-018: populate component textbox with autocomplete datalist from prior worm history."""
    import jinja2
    from model.worm_report import WormCategory, WormDatabaseModel, WormReportModel, WormSeverity, WormStatus

    templates_dir = Path(__file__).resolve().parent.parent / "dashboard" / "templates"
    tpl_path = templates_dir / "worm_report.html.j2"
    assert tpl_path.is_file(), "worm_report.html.j2 must exist"
    content = tpl_path.read_text(encoding="utf-8")

    # 1. Component textbox must bind to datalist via list attribute
    assert 'id="worm-component"' in content
    assert 'list="worm-component-list"' in content or 'list="component-list"' in content, (
        "worm-component input must have a list attribute referencing a datalist"
    )

    # 2. Datalist element must exist
    assert '<datalist id="worm-component-list">' in content or '<datalist id="component-list">' in content, (
        "worm_report.html.j2 must contain datalist for component autocomplete"
    )

    # 3. Dynamic JS updater must be present
    assert "updateComponentDatalist" in content, (
        "worm_report.html.j2 must define updateComponentDatalist to keep autocomplete in sync"
    )

    # 4. Jinja2 render with prior worm history renders unique option elements
    db = WormDatabaseModel(title="Autocomplete Test")
    db.worms = [
        WormReportModel(
            id="WORM-001",
            title="W1",
            status=WormStatus.OPEN,
            severity=WormSeverity.LOW,
            category=WormCategory.DRIVER,
            component="carrier_board",
        ),
        WormReportModel(
            id="WORM-002",
            title="W2",
            status=WormStatus.OPEN,
            severity=WormSeverity.LOW,
            category=WormCategory.DRIVER,
            component="driver",
        ),
        WormReportModel(
            id="WORM-003",
            title="W3",
            status=WormStatus.OPEN,
            severity=WormSeverity.LOW,
            category=WormCategory.DRIVER,
            component="carrier_board",  # duplicate, must be deduplicated
        ),
    ]

    env = jinja2.Environment(
        loader=jinja2.FileSystemLoader(str(templates_dir)),
        autoescape=jinja2.select_autoescape(["html", "xml"]),
        trim_blocks=True,
        lstrip_blocks=True,
    )
    tpl = env.get_template("worm_report.html.j2")
    rendered = tpl.render(
        database=db,
        database_json=db.model_dump_json(),
        statuses=[s.value for s in WormStatus],
        severities=[s.value for s in WormSeverity],
        categories=[c.value for c in WormCategory],
    )

    assert '<option value="carrier_board">' in rendered or '<option value="carrier_board"></option>' in rendered
    assert '<option value="driver">' in rendered or '<option value="driver"></option>' in rendered
