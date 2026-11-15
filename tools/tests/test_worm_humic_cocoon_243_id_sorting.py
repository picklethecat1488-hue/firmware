#!/usr/bin/env python3
"""Regression functional test for WORM-HUMIC-COCOON-243.

Ensures that SQLiteWormStore.load_database() sorts bugs/worms chronologically by creation
timestamp (and legacy sequential ID numbers), rather than by the arbitrary random 3-digit
suffix of Docker-pattern IDs.
"""

from pathlib import Path
import tempfile

from model.worm_report import WormCategory, WormDatabaseModel, WormReportModel, WormSeverity, WormStatus
from provider.worm_report.sqlite_store import SQLiteWormStore


def test_sqlite_store_sorts_by_creation_timestamp():
    """Verify SQLiteWormStore sorts worms by creation time rather than random 3-digit number."""
    with tempfile.TemporaryDirectory() as tmp_dir:
        db_path = Path(tmp_dir) / "test_worms.sqlite"
        store = SQLiteWormStore(db_path)

        # Create 3 worms where alphabetical/number order contradicts chronological order:
        # WORM-001 (legacy, created first)
        # WORM-TRANSLUCENT-TAPEWORM-953 (number 953, created middle)
        # WORM-SETOSE-TUBEWORM-177 (number 177, created last)
        w1 = WormReportModel(
            id="WORM-001",
            title="First Legacy Bug",
            status=WormStatus.RESOLVED,
            severity=WormSeverity.LOW,
            category=WormCategory.FIRMWARE,
            created_at="2026-10-03 19:26:02 UTC",
        )
        w2 = WormReportModel(
            id="WORM-TRANSLUCENT-TAPEWORM-953",
            title="Middle Bug with High Random Number",
            status=WormStatus.RESOLVED,
            severity=WormSeverity.LOW,
            category=WormCategory.FIRMWARE,
            created_at="2026-10-15 10:00:00 UTC",
        )
        w3 = WormReportModel(
            id="WORM-SETOSE-TUBEWORM-177",
            title="Newest Bug with Lower Random Number",
            status=WormStatus.OPEN,
            severity=WormSeverity.MEDIUM,
            category=WormCategory.FIRMWARE,
            created_at="2026-10-21 23:57:15 UTC",
        )

        store.save_database(WormDatabaseModel(title="Test DB", worms=[w3, w1, w2]))

        loaded = store.load_database()
        ids = [w.id for w in loaded.worms]

        # Chronological order should be oldest to newest: w1, w2, w3
        assert ids == ["WORM-001", "WORM-TRANSLUCENT-TAPEWORM-953", "WORM-SETOSE-TUBEWORM-177"], (
            f"Expected chronological order [WORM-001, WORM-TRANSLUCENT-TAPEWORM-953, WORM-SETOSE-TUBEWORM-177], but got {ids}"
        )
