"""Reproduction and regression unit tests for WORM-007: Diff and Code Review word wrap."""

from pathlib import Path


def test_reproduction_worm_007_code_review_and_diff_word_wrap_css() -> None:
    """Verify WORM-007: diff-word-wrap styles correctly constrain tables, wraps, and grid columns to 100% without overflowing."""
    templates_dir = Path(__file__).resolve().parent.parent / "dashboard" / "templates"
    cr_html = (templates_dir / "code_review.html.j2").read_text(encoding="utf-8")
    diff_comp_html = (templates_dir / "diff_component.html.j2").read_text(encoding="utf-8")

    # 1. In code_review.html.j2, .unified-wrap has width: max-content by default.
    # When .diff-word-wrap is active, .unified-wrap MUST be overridden to width: 100% and min-width: 0 to allow wrapping.
    assert ".diff-word-wrap .unified-wrap" in cr_html
    assert "width: 100% !important;" in cr_html

    # 2. In code_review.html.j2, .unified-row grid 4th column must use minmax(0, 1fr) when wrapped so text doesn't push width to max-content
    assert ".diff-word-wrap .unified-row" in cr_html
    assert "minmax(0, 1fr)" in cr_html

    # 3. In code_review.html.j2, side-by-side rows use .sbs-row td.old-code and .sbs-row td.new-code
    assert ".diff-word-wrap .sbs-row td.old-code" in cr_html
    assert ".diff-word-wrap .sbs-row td.new-code" in cr_html

    # 4. In code_review.html.j2, .sbs-table has min-width: max-content by default; .diff-word-wrap must override table min-width
    assert ".diff-word-wrap .sbs-table" in cr_html or ".diff-word-wrap table.sbs-table" in cr_html

    # 5. In diff_component.html.j2, .diff-unified-row grid 4th column must also use minmax(0, 1fr) when wrapped
    assert ".diff-word-wrap .diff-unified-row" in diff_comp_html
