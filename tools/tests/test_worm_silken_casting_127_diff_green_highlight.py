"""Regression test for WORM-SILKEN-CASTING-127: Diff viewer missing green highlight for line changes.

Verifies that:
1. diff_component.html.j2 and code_review.html.j2 apply green highlights (rgba(34, 197, 94, 0.22))
   and #22c55e to diff additions and line changes, rather than cyan/teal (rgba(20, 217, 196, ...)).
2. Side-by-side diff additions (.row-insert, .row-add, .row-replace td.new-code) apply the green background.
3. Unified diff additions (.row-add, .add, .insert, .row-insert) apply the green background, correctly
   matching classes emitted by renderUnified() for DiffLineType.ADD.
4. Diff stats addition badges (.diff-stats-add) use green #22c55e.
"""

from pathlib import Path
import re


def test_diff_component_green_highlight_css() -> None:
    templates_dir = Path(__file__).resolve().parent.parent / "dashboard" / "templates"
    diff_comp_path = templates_dir / "diff_component.html.j2"
    assert diff_comp_path.exists(), f"Missing template: {diff_comp_path}"

    content = diff_comp_path.read_text(encoding="utf-8")

    # 1. diff-stats-add should use green #22c55e instead of xerxes-teal
    assert re.search(r"\.diff-stats-add\s*\{\s*color:\s*#22c55e", content), (
        ".diff-stats-add must use green color #22c55e"
    )

    # 2. SBS line insertions and replacements must have green background rgba(34, 197, 94, 0.22)
    assert re.search(
        r"\.diff-sbs-row\.row-insert\s+td\.new-code[\s\S]*?background:\s*rgba\(34,\s*197,\s*94,\s*0\.22\)",
        content,
    ), "SBS .row-insert td.new-code must use green background rgba(34, 197, 94, 0.22)"

    assert re.search(
        r"\.diff-sbs-row\.row-replace\s+td\.new-code[\s\S]*?background:\s*rgba\(34,\s*197,\s*94,\s*0\.22\)",
        content,
    ), "SBS .row-replace td.new-code must use green background rgba(34, 197, 94, 0.22)"

    # 3. SBS row-add should also be supported
    assert ".diff-sbs-row.row-add" in content, (
        "SBS view must support .diff-sbs-row.row-add selector"
    )

    # 4. Unified diff rows for additions must match .row-add, .add, .insert, .row-insert
    # and apply green background rgba(34, 197, 94, 0.22)
    assert re.search(
        r"\.diff-unified-row[\s\S]*?(?:\.add|\.row-add|\.insert)[\s\S]*?background:\s*rgba\(34,\s*197,\s*94,\s*0\.22\)",
        content,
    ), "Unified diff rows for additions must use green background rgba(34, 197, 94, 0.22)"

    # Check that .diff-unified-row.add or .diff-unified-row.row-add is explicitly targeted
    assert ".diff-unified-row.add" in content or ".diff-unified-row.row-add" in content, (
        "Unified view must style .diff-unified-row.add or .diff-unified-row.row-add"
    )

    # 5. Check text color for diff addition markers in unified view
    assert re.search(
        r"\.diff-unified-row[\s\S]*?(?:\.add|\.row-add|\.row-insert)[\s\S]*?\.diff-unified-marker\s*\{\s*color:\s*#86efac",
        content,
    ), "Unified diff addition marker must use color #86efac"

    # 6. Ensure no lingering cyan/teal (rgba(20, 217, 196, 0.22)) background in diff_component.html.j2
    assert "rgba(20, 217, 196, 0.22)" not in content, (
        "diff_component.html.j2 must not use teal rgba(20, 217, 196, 0.22) for diff addition backgrounds"
    )


def test_code_review_green_diff_add_color() -> None:
    templates_dir = Path(__file__).resolve().parent.parent / "dashboard" / "templates"
    cr_path = templates_dir / "code_review.html.j2"
    assert cr_path.exists(), f"Missing template: {cr_path}"

    content = cr_path.read_text(encoding="utf-8")

    # --bg-diff-add should be green rgba(34, 197, 94, 0.22)
    assert "--bg-diff-add: rgba(34, 197, 94, 0.22);" in content, (
        "code_review.html.j2 --bg-diff-add must be rgba(34, 197, 94, 0.22)"
    )
