"""Unit and regression tests for UI Scale Selector and Commit Selection parity.

Addresses Code Review Findings:
- 8715cab2: UI scale selector in dashboard & host toolbars with Command+/Command- shortcuts
- 5d8eff1f: Commit selection highlight styling in code review and smartlog UI
"""

import re
import shutil
import subprocess
import tempfile
from pathlib import Path
import pytest



TEMPLATES_DIR = Path(__file__).resolve().parent.parent / "dashboard" / "templates"
HOST_TEMPLATES_DIR = Path(__file__).resolve().parent.parent / "host_daemon" / "templates"


def test_ui_scale_selector_present_in_all_templates() -> None:
    """Verify that all dashboard and host daemon templates define the UI scale selector."""
    files_to_check = [
        TEMPLATES_DIR / "diff_view.html.j2",
        TEMPLATES_DIR / "code_review.html.j2",
        TEMPLATES_DIR / "worm_report.html.j2",
        HOST_TEMPLATES_DIR / "host_daemon.html.j2",
    ]

    expected_scales = ["75%", "80%", "90%", "100%", "110%", "125%", "150%"]

    for file_path in files_to_check:
        assert file_path.exists(), f"File {file_path} not found"
        content = file_path.read_text(encoding="utf-8")

        # 1. Check for select element
        assert 'id="uiScaleSelect"' in content, f"{file_path.name} must contain select id='uiScaleSelect'"

        # 2. Check for all scale options
        for scale in expected_scales:
            assert f'value="{scale}"' in content, f"{file_path.name} missing scale option {scale}"

        # 3. Check for JS engine functions
        assert "applyUiScale" in content, f"{file_path.name} missing applyUiScale function"
        assert "adjustUiScale" in content, f"{file_path.name} missing adjustUiScale function"
        assert "resetUiScale" in content, f"{file_path.name} missing resetUiScale function"
        assert "initUiScale" in content, f"{file_path.name} missing initUiScale function"
        assert "ui_scale" in content, f"{file_path.name} missing localStorage ui_scale persistence"

        # 4. Check for keyboard shortcuts (+, -, 0)
        assert 'e.key === "="' in content or "e.code === 'Equal'" in content or 'e.key === "+"' in content, (
            f"{file_path.name} missing zoom in shortcut"
        )
        assert 'e.key === "-"' in content or "e.code === 'Minus'" in content, (
            f"{file_path.name} missing zoom out shortcut"
        )
        assert 'e.key === "0"' in content or "e.code === 'Digit0'" in content, (
            f"{file_path.name} missing zoom reset shortcut"
        )


def test_commit_selection_parity_in_code_review() -> None:
    """Verify finding 5d8eff1f: commit-item.selected styles and logic in code_review.html.j2."""
    cr_path = TEMPLATES_DIR / "code_review.html.j2"
    content = cr_path.read_text(encoding="utf-8")

    # CSS styles
    assert ".commit-item.selected" in content, "code_review.html.j2 missing .commit-item.selected styling"
    assert ".commit-item.selected.active" in content, "code_review.html.j2 missing .commit-item.selected.active styling"

    # JS renderCommitsList logic
    assert "isSelected" in content, "renderCommitsList must determine isSelected status"
    assert 'selected' in content, "renderCommitsList must apply selected class"


def test_templates_js_syntax_validation() -> None:
    """Validate JS syntax of script blocks in all updated templates using node -c."""
    node_bin = shutil.which("node")
    if not node_bin:
        pytest.skip("Node.js not available for JavaScript syntax checking")

    files_to_check = [
        TEMPLATES_DIR / "diff_view.html.j2",
        TEMPLATES_DIR / "code_review.html.j2",
        TEMPLATES_DIR / "worm_report.html.j2",
        HOST_TEMPLATES_DIR / "host_daemon.html.j2",
    ]

    for file_path in files_to_check:
        content = file_path.read_text(encoding="utf-8")
        # Extract <script> blocks
        script_blocks = re.findall(r"<script[^>]*>(.*?)</script>", content, re.DOTALL)
        assert script_blocks, f"No script blocks found in {file_path.name}"

        for idx, block in enumerate(script_blocks):
            # Sanitize Jinja expressions for node syntax checker
            clean_js = re.sub(r"\{\{.*?\}\}", "null", block)
            clean_js = re.sub(r"\{%.*?%\}", "", clean_js)
            clean_js = re.sub(r"\{#.*?#\}", "", clean_js)

            with tempfile.NamedTemporaryFile("w", suffix=".js", delete=False, encoding="utf-8") as tf:
                tf.write(clean_js)
                tf_path = tf.name

            try:
                res = subprocess.run([node_bin, "-c", tf_path], capture_output=True, text=True)
                assert res.returncode == 0, f"JS syntax error in {file_path.name} block {idx}:\n{res.stderr}"
            finally:
                Path(tf_path).unlink(missing_ok=True)
