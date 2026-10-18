"""Regression unit test verifying preservation of custom / arbitrary markdown sections in worm reports.

Guards against regressions where agent inputs or user-added subsections (such as
`#### Trace Analysis (digital.csv)`) in `feedback/WORM_<id>.md` were discarded when
viewing, syncing, or saving items in the Worms View.
"""

from pathlib import Path
import tempfile

from model.worm_report import (
    WormCategory,
    WormDatabaseModel,
    WormReportModel,
    WormSectionModel,
    WormSeverity,
    WormStatus,
)
from provider.worm_report.markdown_exporter import MarkdownWormExporter
from provider.worm_report.sqlite_store import SQLiteWormStore


def test_custom_section_parsed_and_rendered_byte_for_byte() -> None:
    """Verify that arbitrary custom markdown sections are parsed and rendered identically."""
    markdown_content = (
        "# \U0001f534 `[WORM-TEST-001]` Motor commutation anomaly\n\n"
        "- **UUID**: `12345678-1234-5678-1234-567812345678`\n"
        "- **ID**: `WORM-TEST-001`\n"
        "- **Status**: `OPEN`\n"
        "- **Severity**: `HIGH`\n"
        "- **Category**: `FIRMWARE`\n"
        "- **Component**: `cat_fountain`\n\n"
        "#### Description\n\n"
        "Motor current is flat at 7 mA.\n\n"
        "#### Behavior Comparison\n\n"
        "- **Expected**: Continuous 25 kHz PWM switching across commutating phases.\n"
        "- **Actual**: Only UH and VL pulse briefly for 2.88 us.\n\n"
        "#### Trace Analysis (`digital.csv`)\n\n"
        "1. Pulses occur at exact ~10.0 ms intervals.\n"
        "2. Channel 2 (UH) and Channel 3 (VL) toggle together.\n\n"
        "#### Execution / Console Logs\n\n"
        "```text\n"
        "10.027651 WARN Motor Controller: Low load / dry detected\n"
        "```\n\n"
        "#### Resolution Notes\n\n"
        "Pending commutation fix.\n"
    )

    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_path = Path(tmp_dir)
        worm_file = tmp_path / "WORM_TEST-001.md"
        worm_file.write_text(markdown_content, encoding="utf-8")

        exporter = MarkdownWormExporter(tmp_path)
        bug = exporter.parse_individual_worm_file(worm_file)

        assert bug is not None
        assert bug.id == "WORM-TEST-001"
        assert bug.description == "Motor current is flat at 7 mA."
        assert len(bug.custom_sections) == 1
        assert bug.custom_sections[0].title == "Trace Analysis (`digital.csv`)"
        assert "Pulses occur at exact ~10.0 ms intervals." in bug.custom_sections[0].content

        rendered = exporter.render_worm_markdown(bug)
        assert "#### Trace Analysis (`digital.csv`)" in rendered
        assert "Pulses occur at exact ~10.0 ms intervals." in rendered
        assert "#### Behavior Comparison" in rendered

        # Export to file and verify it matches
        out_file = exporter.export_individual_worm(bug, tmp_path)
        assert out_file.exists()
        reloaded = exporter.parse_individual_worm_file(out_file)
        assert reloaded is not None
        assert len(reloaded.custom_sections) == 1
        assert reloaded.custom_sections[0].title == "Trace Analysis (`digital.csv`)"


def test_custom_sections_persisted_in_sqlite() -> None:
    """Verify SQLite store preserves custom_sections through save and load."""
    with tempfile.TemporaryDirectory() as tmp_dir:
        db_path = Path(tmp_dir) / "worms.sqlite"
        store = SQLiteWormStore(db_path)

        worm = WormReportModel(
            id="WORM-TEST-002",
            uuid="87654321-4321-8765-4321-876543210987",
            title="Custom section SQLite test",
            status=WormStatus.OPEN,
            severity=WormSeverity.MEDIUM,
            category=WormCategory.DRIVER,
            description="Testing SQLite roundtrip.",
            custom_sections=[
                WormSectionModel(title="Root Cause Analysis", content="Discovered race condition in FIFO."),
                WormSectionModel(title="Hardware Measurements", content="Scope probe on pin 16 showed 3.3V flat."),
            ],
        )

        db = WormDatabaseModel(title="Test DB", worms=[worm])
        store.save_database(db)

        # Reload from fresh store connection
        store2 = SQLiteWormStore(db_path)
        loaded_db = store2.load_database()
        assert len(loaded_db.worms) == 1
        loaded_worm = loaded_db.worms[0]
        assert len(loaded_worm.custom_sections) == 2
        assert loaded_worm.custom_sections[0].title == "Root Cause Analysis"
        assert loaded_worm.custom_sections[0].content == "Discovered race condition in FIFO."
        assert loaded_worm.custom_sections[1].title == "Hardware Measurements"
        assert loaded_worm.custom_sections[1].content == "Scope probe on pin 16 showed 3.3V flat."


def test_merge_databases_preserves_custom_sections() -> None:
    """Verify merge_databases propagates custom_sections from parsed markdown into base database."""
    base_bug = WormReportModel(
        id="WORM-TEST-003",
        uuid="abcdefab-cdef-abcd-efab-cdefabcdefab",
        title="Merge test",
        status=WormStatus.OPEN,
        severity=WormSeverity.LOW,
        category=WormCategory.UI,
        description="Base description.",
    )
    md_bug = WormReportModel(
        id="WORM-TEST-003",
        uuid="abcdefab-cdef-abcd-efab-cdefabcdefab",
        title="Merge test",
        status=WormStatus.OPEN,
        severity=WormSeverity.LOW,
        category=WormCategory.UI,
        description="Base description.",
        custom_sections=[
            WormSectionModel(title="Investigation Notes", content="Agent verified findings with logic analyzer.")
        ],
    )

    base_db = WormDatabaseModel(title="Base", worms=[base_bug])
    md_db = WormDatabaseModel(title="MD", worms=[md_bug])

    exporter = MarkdownWormExporter(Path("."))
    merged = exporter.merge_databases(base_db, md_db)
    assert len(merged.worms) == 1
    assert len(merged.worms[0].custom_sections) == 1
    assert merged.worms[0].custom_sections[0].title == "Investigation Notes"
    assert "logic analyzer" in merged.worms[0].custom_sections[0].content
