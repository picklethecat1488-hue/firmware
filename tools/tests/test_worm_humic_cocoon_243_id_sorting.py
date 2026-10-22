#!/usr/bin/env python3
"""Regression functional test for WORM-HUMIC-COCOON-243.

Ensures that:
1. SQLiteWormStore.load_database() sorts bugs/worms chronologically by creation timestamp
   (and legacy sequential ID numbers), rather than by the arbitrary random 3-digit suffix of
   Docker-pattern IDs.
2. In the dashboard worm reporting template (worm_report.html.j2), sorting by "id_desc"
   orders worms from newest to oldest, and "id_asc" orders worms from oldest to newest,
   correctly respecting creation dates for both Docker-pattern and legacy sequential IDs.
3. Node.js functional evaluation of the extracted JavaScript sort logic verifies that
   newest to oldest (id_desc) and oldest to newest (id_asc) sort orders work as intended.
"""

import json
from pathlib import Path
import re
import subprocess
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


def test_template_sorting_functional_via_node():
    """Verify JavaScript sort logic in worm_report.html.j2 orders newest to oldest and vice versa."""
    tpl_path = Path(__file__).resolve().parent.parent / "dashboard" / "templates" / "worm_report.html.j2"
    tpl_content = tpl_path.read_text(encoding="utf-8")

    # Extract the sorting switch block from getFilteredAndSortedWorms()
    match = re.search(r"list\.sort\(\(a,\s*b\)\s*=>\s*\{(.*?)\n\s*\}\);", tpl_content, re.DOTALL)
    assert match, "Could not extract list.sort block from worm_report.html.j2"
    sort_body = match.group(1)

    # Test dataset with mixed legacy and docker-pattern IDs and conflicting random suffixes
    mock_worms = [
        {"id": "WORM-001", "created_at": "2026-10-03 19:26:02 UTC", "title": "Oldest", "severity": "LOW", "status": "RESOLVED"},
        {"id": "WORM-TRANSLUCENT-TAPEWORM-953", "created_at": "2026-10-15 10:00:00 UTC", "title": "Middle High", "severity": "HIGH", "status": "RESOLVED"},
        {"id": "WORM-SETOSE-TUBEWORM-177", "created_at": "2026-10-21 23:57:15 UTC", "title": "Newer Low", "severity": "MEDIUM", "status": "RESOLVED"},
        {"id": "WORM-HUMIC-COCOON-243", "created_at": "2026-10-22 00:00:15 UTC", "title": "Newest", "severity": "MEDIUM", "status": "OPEN"},
    ]

    node_script = f"""
    const severityOrder = {{ "CRITICAL": 0, "HIGH": 1, "MEDIUM": 2, "LOW": 3 }};
    const statusOrder = {{ "OPEN": 0, "IN_PROGRESS": 1, "PLANNED": 2, "RESOLVED": 3, "CLOSED": 4 }};

    function sortWorms(worms, sortMode) {{
        const list = worms.slice();
        let currentWormSort = sortMode;
        list.sort((a, b) => {{
            {sort_body}
        }});
        return list.map(w => w.id);
    }}

    const inputWorms = {json.dumps(mock_worms)};
    const descResult = sortWorms(inputWorms, 'id_desc');
    const ascResult = sortWorms(inputWorms, 'id_asc');

    console.log(JSON.stringify({{ desc: descResult, asc: ascResult }}));
    """

    res = subprocess.run(
        ["node", "-e", node_script],
        capture_output=True,
        text=True,
        check=True,
    )
    data = json.loads(res.stdout.strip())

    # id_desc (Newest first) must order from latest created_at to earliest
    expected_desc = [
        "WORM-HUMIC-COCOON-243",
        "WORM-SETOSE-TUBEWORM-177",
        "WORM-TRANSLUCENT-TAPEWORM-953",
        "WORM-001",
    ]
    assert data["desc"] == expected_desc, (
        f"id_desc must sort newest to oldest. Expected {expected_desc}, got {data['desc']}"
    )

    # id_asc (Oldest first) must order from earliest created_at to latest
    expected_asc = [
        "WORM-001",
        "WORM-TRANSLUCENT-TAPEWORM-953",
        "WORM-SETOSE-TUBEWORM-177",
        "WORM-HUMIC-COCOON-243",
    ]
    assert data["asc"] == expected_asc, (
        f"id_asc must sort oldest to newest. Expected {expected_asc}, got {data['asc']}"
    )
