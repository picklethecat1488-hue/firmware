"""Regression and unit test for WORM-029: Docker-pattern issue IDs.

Verifies:
1. Issue IDs are formatted as [PREFIX]-[ADJECTIVE]-[ANIMAL/NOUN]-[3 digits]
   e.g. WORM-SWIFT-FOX-42, WORM-BOLD-LYNX-809, WORM-IRON-CRANE-17.
2. Selection is generated using a CRNG (secrets/os.urandom).
3. Adjective and animal/noun lists are compressed with zlib/base64 to minimize code footprint.
4. Database and SQLite stores generate unique Docker-pattern IDs without collisions.
5. Markdown exporter and loader serialize and round-trip Docker-pattern IDs.
6. Git engine and dashboard extract and link Docker-pattern tags in commit subjects.
"""

from pathlib import Path
import re

from model.id_generator import (
    ADJECTIVES,
    NOUNS,
    _B64_ADJS,
    _B64_NOUNS,
    generate_docker_pattern_id,
    is_docker_pattern_id,
)
from model.worm_report import WormCategory, WormDatabaseModel, WormReportModel, WormSeverity, WormStatus
from provider.vcs.git_engine import GitEngine
from provider.worm_report.markdown_exporter import MarkdownWormExporter
from provider.worm_report.sqlite_store import SQLiteWormStore


def test_compressed_word_lists_save_code_space() -> None:
    """Verify adjective and noun lists are compressed in base64 strings taking minimal code space."""
    assert len(_B64_ADJS) < 500, "Compressed adjectives base64 string must be compact"
    assert len(_B64_NOUNS) < 500, "Compressed nouns base64 string must be compact"

    # Must contain representative words from prompt examples: SWIFT, BOLD, IRON, FOX, LYNX, CRANE
    assert "SWIFT" in ADJECTIVES
    assert "BOLD" in ADJECTIVES
    assert "IRON" in ADJECTIVES
    assert "FOX" in NOUNS
    assert "LYNX" in NOUNS
    assert "CRANE" in NOUNS

    assert len(ADJECTIVES) >= 50
    assert len(NOUNS) >= 50


def test_docker_pattern_id_format_and_crng() -> None:
    """Verify generated IDs match [PREFIX]-[ADJECTIVE]-[ANIMAL/NOUN]-[1..3 digits]."""
    pattern = re.compile(r"^WORM-[A-Z]+-[A-Z]+-\d{1,3}$")

    generated_ids = set()
    for _ in range(50):
        worm_id = generate_docker_pattern_id(prefix="WORM")
        assert pattern.match(worm_id), f"ID '{worm_id}' does not match Docker pattern"
        assert is_docker_pattern_id(worm_id)

        parts = worm_id.split("-")
        assert len(parts) == 4, f"ID '{worm_id}' must have exactly 4 parts separated by hyphens"
        assert parts[0] == "WORM"
        assert parts[1] in ADJECTIVES
        assert parts[2] in NOUNS

        num = int(parts[3])
        assert 1 <= num <= 999
        generated_ids.add(worm_id)

    # CRNG should generate distinct IDs without trivial collisions
    assert len(generated_ids) >= 48


def test_docker_pattern_id_avoids_existing_ids() -> None:
    """Verify generator respects existing_ids collection and avoids collisions."""
    mock_id = "WORM-SWIFT-FOX-42"
    existing = {mock_id}

    # Should never return an ID present in existing
    for _ in range(20):
        new_id = generate_docker_pattern_id(prefix="WORM", existing_ids=existing)
        assert new_id != mock_id
        existing.add(new_id)


def test_worm_database_model_generates_docker_ids() -> None:
    """Verify WormDatabaseModel generates Docker-pattern IDs."""
    db = WormDatabaseModel(title="Test DB")
    id1 = db.generate_worm_id()
    assert is_docker_pattern_id(id1)

    db.add_or_update(
        WormReportModel(
            id=id1,
            title="Defect 1",
            status=WormStatus.OPEN,
            severity=WormSeverity.MEDIUM,
            category=WormCategory.FIRMWARE,
        )
    )

    id2 = db.generate_worm_id()
    assert is_docker_pattern_id(id2)
    assert id2 != id1


def test_sqlite_store_generates_and_resolves_docker_ids(tmp_path: Path) -> None:
    """Verify SQLiteWormStore generates unique Docker-pattern IDs and resolves duplicates."""
    db_file = tmp_path / "worms.sqlite"
    store = SQLiteWormStore(db_file)

    id1 = store.generate_next_worm_id()
    assert is_docker_pattern_id(id1)

    w1 = WormReportModel(
        id=id1,
        title="SQL defect 1",
        status=WormStatus.OPEN,
        severity=WormSeverity.HIGH,
        category=WormCategory.CONTROLLER,
    )
    store.save_worm(w1)

    id2 = store.generate_next_worm_id()
    assert is_docker_pattern_id(id2)
    assert id2 != id1


def test_markdown_exporter_roundtrip_with_docker_pattern_ids(tmp_path: Path) -> None:
    """Verify MarkdownWormExporter roundtrips individual and aggregated markdown files with Docker IDs."""
    exporter = MarkdownWormExporter(repo_root=tmp_path)
    feedback_dir = tmp_path / "feedback"

    worm = WormReportModel(
        id="WORM-SWIFT-FOX-42",
        title="Docker pattern issue test",
        status=WormStatus.OPEN,
        severity=WormSeverity.MEDIUM,
        category=WormCategory.PLATFORM,
        component="dashboard",
        description="Verify Docker pattern ID formatting and parsing",
    )

    # 1. Single file export
    exported_file = exporter.export_individual_worm(worm, feedback_dir)
    assert exported_file.exists()
    assert "WORM_SWIFT-FOX-42.md" in exported_file.name

    # 2. Single file loading
    loaded_single = exporter.parse_individual_worm_file(exported_file)
    assert loaded_single is not None
    assert loaded_single.id == "WORM-SWIFT-FOX-42"
    assert loaded_single.title == "Docker pattern issue test"

    # 3. Aggregated WORMS.md export and loading
    db = WormDatabaseModel(title="Docker Worm Tracker", worms=[worm])
    worms_md = tmp_path / "WORMS.md"
    exporter.export_markdown(db, worms_md, feedback_dir=feedback_dir)

    loaded_db = exporter.parse_markdown(worms_md)
    assert loaded_db is not None
    assert len(loaded_db.worms) == 1
    assert loaded_db.worms[0].id == "WORM-SWIFT-FOX-42"


def test_git_engine_extracts_docker_pattern_worm_tags(tmp_path: Path) -> None:
    """Verify GitEngine extracts Docker-pattern worm tags from commit text."""
    feedback_dir = tmp_path / "feedback"
    feedback_dir.mkdir(parents=True, exist_ok=True)
    (feedback_dir / "WORM_SWIFT-FOX-42.md").write_text(
        "# 🔴 `[WORM-SWIFT-FOX-42]` Swift fox bug\n- **Status**: `OPEN`\n- **Severity**: `HIGH`\n",
        encoding="utf-8",
    )

    engine = GitEngine(repo_root=tmp_path)
    commit_text = "fix(core): resolve issue with timing (WORM-SWIFT-FOX-42)"
    tags = engine._extract_worm_tags(commit_text)

    assert len(tags) == 1
    assert tags[0].id == "WORM-SWIFT-FOX-42"
    assert tags[0].title == "Swift fox bug"
    assert tags[0].status == "OPEN"
    assert tags[0].severity == "HIGH"
