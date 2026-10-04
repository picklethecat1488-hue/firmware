"""Regression unit test for WORM-004: Code Review icon should be a wrench (🔧).

Verifies that all UI templates (diff_view.html.j2 and code_review.html.j2) use
the wrench icon (🔧) for Code Review navigation, smartlog action buttons,
and review indicators instead of the magnifying glass icon (🔍).
"""

from pathlib import Path
import jinja2

from model.vcs import CommitNodeModel, DiffViewSessionModel


TEMPLATES_DIR = Path(__file__).resolve().parent.parent / "dashboard" / "templates"


def test_code_review_icon_is_wrench_in_diff_view() -> None:
    """Verify diff_view.html.j2 uses wrench (🔧) instead of magnifying glass (🔍) for Code Review."""
    diff_view_path = TEMPLATES_DIR / "diff_view.html.j2"
    assert diff_view_path.exists()
    content = diff_view_path.read_text(encoding="utf-8")

    # 1. Open Code Review button in Smartlog header
    assert 'id="btnOpenCR"' in content
    assert "🔧" in content
    assert "🔍 CR" not in content, "diff_view.html.j2 should not use 🔍 CR for Code Review action buttons"
    assert "<span>🔧 CR</span>" in content, "diff_view.html.j2 should use <span>🔧 CR</span> for commit CR buttons"

    # 2. Inter-tool link in top navbar
    assert "onOpenInCodeReview()" in content

    # 3. Check rendered output
    env = jinja2.Environment(loader=jinja2.FileSystemLoader(str(TEMPLATES_DIR)))
    tpl = env.get_template("diff_view.html.j2")
    session = DiffViewSessionModel(
        repo_name="firmware-test",
        current_branch="main",
        head_commit_hash="a1b2c3d4e5f6",
        smartlog_nodes=[
            CommitNodeModel(
                commit_hash="a1b2c3d4e5f6",
                short_hash="a1b2c3d",
                date="2026-10-02T12:00:00Z",
                subject="Fix DMA overrun in SPI buffer",
                author="Xerxes AI",
                relative_date="10 minutes ago",
                branches=["main"],
                is_head=True,
            )
        ],
        working_files=[],
        branches=[],
    )
    rendered = tpl.render(session=session)
    assert "🔧" in rendered
    assert "🔍 CR" not in rendered


def test_code_review_icon_is_wrench_in_code_review() -> None:
    """Verify code_review.html.j2 uses wrench (🔧) for Code Review branding."""
    cr_path = TEMPLATES_DIR / "code_review.html.j2"
    assert cr_path.exists()
    content = cr_path.read_text(encoding="utf-8")

    assert "🔧" in content, "code_review.html.j2 should feature the wrench icon for Code Review branding"
