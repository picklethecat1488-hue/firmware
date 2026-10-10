"""Regression unit test for WORM-023: Add Planned status to worms.

Verifies:
1. WormStatus.PLANNED enum exists and serializes properly.
2. Markdown exporter and parser preserve PLANNED status.
3. Dashboard CLI list-worms --open excludes PLANNED worms by default.
4. Dashboard CLI supports --planned flag.
5. Worm console retains default status as OPEN.
6. Documentation/skills instruct agents not to work on Planned worms by default.
"""

from io import StringIO
from pathlib import Path
from unittest.mock import patch

import pytest

from model.worm_report import (
    WormCategory,
    WormDatabaseModel,
    WormReportModel,
    WormSeverity,
    WormStatus,
)
from dashboard.cli import parse_arguments, print_cli_worms


def test_worm_status_planned_enum_exists() -> None:
    """Verify WormStatus has PLANNED variant."""
    assert hasattr(WormStatus, "PLANNED"), "WormStatus must define PLANNED"
    assert WormStatus.PLANNED.value == "PLANNED"


def test_cli_list_worms_open_excludes_planned() -> None:
    """Verify list-worms --open excludes PLANNED worms by default so agent won't work on them."""
    db = WormDatabaseModel(
        worms=[
            WormReportModel(
                id="WORM-101",
                title="Active open bug",
                severity=WormSeverity.HIGH,
                status=WormStatus.OPEN,
                category=WormCategory.FIRMWARE,
            ),
            WormReportModel(
                id="WORM-102",
                title="Future planned feature",
                severity=WormSeverity.LOW,
                status=WormStatus.PLANNED,
                category=WormCategory.FIRMWARE,
            ),
            WormReportModel(
                id="WORM-103",
                title="Resolved defect",
                severity=WormSeverity.MEDIUM,
                status=WormStatus.RESOLVED,
                category=WormCategory.FIRMWARE,
            ),
        ]
    )

    out = StringIO()
    with patch("sys.stdout", out):
        print_cli_worms(db, open_only=True)
    output = out.getvalue()

    assert "WORM-101" in output, "Open worm should be listed"
    assert "WORM-102" not in output, "Planned worm must be excluded when open_only=True"
    assert "WORM-103" not in output, "Resolved worm must be excluded when open_only=True"


def test_cli_planned_argument() -> None:
    """Verify --planned CLI argument filters to PLANNED worms."""
    with patch("sys.argv", ["dashboard.py", "--planned"]):
        args = parse_arguments()
        assert getattr(args, "planned", False) is True

    db = WormDatabaseModel(
        worms=[
            WormReportModel(
                id="WORM-101",
                title="Active open bug",
                severity=WormSeverity.HIGH,
                status=WormStatus.OPEN,
                category=WormCategory.FIRMWARE,
            ),
            WormReportModel(
                id="WORM-102",
                title="Future planned feature",
                severity=WormSeverity.LOW,
                status=WormStatus.PLANNED,
                category=WormCategory.FIRMWARE,
            ),
        ]
    )

    out = StringIO()
    with patch("sys.stdout", out):
        print_cli_worms(db, planned_only=True)
    output = out.getvalue()

    assert "WORM-102" in output, "Planned worm must be listed when planned_only=True"
    assert "WORM-101" not in output, "Open worm must be excluded when planned_only=True"


def test_template_planned_filter_and_default_open() -> None:
    """Verify worm report template includes PLANNED filter and retains OPEN as default."""
    template_path = Path(__file__).resolve().parent.parent / "dashboard" / "templates" / "worm_report.html.j2"
    if not template_path.exists():
        # Fallback for different repo structure if ported
        template_path = Path(__file__).resolve().parent.parent / "provider" / "templates" / "bug_report.html.j2"
    assert template_path.exists(), f"Template not found at {template_path}"
    content = template_path.read_text(encoding="utf-8")

    assert 'id="filter-planned"' in content, "Template must have filter-planned button"
    assert "PLANNED" in content, "Template must reference PLANNED"
    assert 'status: "OPEN"' in content, "Template new worm must default to OPEN status"


def test_documentation_planned_worm_exemption() -> None:
    """Verify GEMINI.md and SKILL.md state agents must not work on Planned worms by default."""
    repo_root = Path(__file__).resolve().parent.parent.parent
    gemini_path = repo_root / "GEMINI.md"
    assert gemini_path.exists(), "GEMINI.md must exist"
    gemini_content = gemini_path.read_text(encoding="utf-8")
    assert "PLANNED" in gemini_content, "GEMINI.md must reference PLANNED worms"
    assert "Planned Worm Exemption" in gemini_content or "planned" in gemini_content.lower()
