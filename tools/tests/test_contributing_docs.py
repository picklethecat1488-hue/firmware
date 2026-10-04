"""Regression tests for CONTRIBUTING.md developer documentation."""

from pathlib import Path

WORKSPACE_ROOT = Path(__file__).resolve().parent.parent.parent


def test_contributing_contains_dashboard_documentation() -> None:
    """Verify CONTRIBUTING.md documents the Xerxes dashboard, CLI subcommands, code reviews, and worm tracking."""
    contributing_path = WORKSPACE_ROOT / "CONTRIBUTING.md"
    assert contributing_path.exists(), "CONTRIBUTING.md must exist in workspace root"

    content = contributing_path.read_text(encoding="utf-8")

    # Section header
    assert "Dashboard" in content or "dashboard.py" in content

    # Workstation launch & CLI commands
    assert "tools/dashboard.py" in content
    assert "list-reviews" in content
    assert "list-worms" in content
    assert "add-worm" in content
    assert "resolve-worm" in content
    assert "set-verdict" in content

    # Core view modes
    assert "Diff View" in content or "diff" in content.lower()
    assert "Code Review" in content or "code review" in content.lower()
    assert "Worm" in content or "worm" in content.lower()

    # Persistence & sync
    assert "target/code_review.sqlite" in content or "target/worms.sqlite" in content or "SQLite" in content
    assert "feedback/" in content
