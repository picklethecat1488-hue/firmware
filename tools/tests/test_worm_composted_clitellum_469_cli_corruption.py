#!/usr/bin/env python3
"""Regression unit tests for WORM-COMPOSTED-CLITELLUM-469: Corrupted CLI output.

Guards against two root causes of CLI workstation output corruption:
1. ANSI OSC 8 hyperlink sequences (e.g. from Cargo release profile notices) leaking
   unparsed escape codes ('8;;https://...' and '8;;') into the terminal view.
2. Inappropriate 'display: flex' on #cliView and #fsView, which caused child log
   lines (<div class="log-line">) to be rendered side-by-side as narrow horizontal
   columns instead of stacking vertically line-by-line.
"""

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.dashboard.provider.terminal import ansi_to_html


def test_osc8_canonical_hyperlink_parsing():
    """Verify canonical Cargo OSC 8 hyperlinks are converted to styled HTML links."""
    cargo_sample = (
        "    Finished \x1b]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\x1b\\"
        "`release` profile [optimized + debuginfo]\x1b]8;;\x1b\\ target(s) in 0.19s"
    )
    html = ansi_to_html(cargo_sample)

    assert "8;;" not in html, f"Raw '8;;' must not leak into HTML output: {html}"
    assert "\x1b" not in html, "Raw escape codes must not leak into HTML output"
    assert (
        '<a href="https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles"' in html
    ), f"Hyperlink tag missing from output: {html}"
    assert "`release` profile [optimized + debuginfo]" in html
    assert "target(s) in 0.19s" in html
    assert 'class="term-link"' in html


def test_osc8_orphaned_hyperlink_parsing():
    """Verify orphaned 8;;URL...8;; sequences (e.g. after escape stripping) are parsed."""
    raw_sample = (
        "    Finished 8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles"
        "`release` profile [optimized + debuginfo]8;; target(s) in 0.22s"
    )
    html = ansi_to_html(raw_sample)

    assert "8;;" not in html, f"Raw '8;;' must not leak into HTML output: {html}"
    assert (
        '<a href="https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles"' in html
    ), f"Hyperlink tag missing from output: {html}"
    assert "`release` profile [optimized + debuginfo]" in html
    assert "target(s) in 0.22s" in html


def test_osc8_bel_terminator_parsing():
    """Verify OSC 8 sequences terminated by \\x07 (BEL) are cleanly parsed."""
    sample = "Docs: \x1b]8;;https://example.com/docs\x07Click Here\x1b]8;;\x07 for info"
    html = ansi_to_html(sample)

    assert "8;;" not in html
    assert "\x07" not in html
    assert '<a href="https://example.com/docs"' in html
    assert "Click Here" in html
    assert "for info" in html


def test_osc8_combined_with_sgr_formatting():
    """Verify OSC 8 hyperlinks work in combination with SGR colors and styles."""
    sample = (
        "\x1b[1;32m    Finished\x1b[0m \x1b]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\x1b\\"
        "`release` profile\x1b]8;;\x1b\\ in 0.12s"
    )
    html = ansi_to_html(sample)

    assert "8;;" not in html
    assert "term-bold" in html or "font-weight: bold" in html
    assert "term-fg-green" in html or "color:" in html
    assert '<a href="https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles"' in html
    assert "`release` profile" in html


def test_host_daemon_template_cli_view_layout_invariants():
    """Verify #cliView and #fsView are never styled with unconstrained display: flex."""
    template_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    content = template_path.read_text(encoding="utf-8")

    # In HTML template tags, display should be block or none, never flex
    cli_match = re.search(r'<div[^>]*id="cliView"[^>]*style="([^"]*)"', content)
    assert cli_match is not None, "id='cliView' element not found in host_daemon.html.j2"
    cli_style = cli_match.group(1)
    assert "flex" not in cli_style, f"cliView must not have flex display in inline style: {cli_style}"
    assert "block" in cli_style or "none" in cli_style

    fs_match = re.search(r'<div[^>]*id="fsView"[^>]*style="([^"]*)"', content)
    assert fs_match is not None, "id='fsView' element not found in host_daemon.html.j2"
    fs_style = fs_match.group(1)
    assert "flex" not in fs_style, f"fsView must not have flex display in inline style: {fs_style}"
    assert "block" in fs_style or "none" in fs_style

    # In changeMode() JS function, switching views must set display to block, never flex
    assert 'cliView.style.display = "block"' in content
    assert 'fsView.style.display = "block"' in content
    assert 'cliView.style.display = "flex"' not in content
    assert 'fsView.style.display = "flex"' not in content

    # In CSS, .panel-content must include display: block and flex-direction: column
    panel_content_re = re.search(r'\.panel-content\s*\{([^}]+)\}', content)
    assert panel_content_re is not None, ".panel-content CSS class definition not found"
    panel_css = panel_content_re.group(1)
    assert "display: block;" in panel_css, ".panel-content must specify display: block"
    assert "flex-direction: column;" in panel_css, ".panel-content must specify flex-direction: column defensively"


def test_client_templates_contain_osc8_parsers():
    """Verify both host_daemon and code_review client templates parse OSC 8 hyperlinks."""
    cr_path = REPO_ROOT / "tools" / "dashboard" / "templates" / "code_review.html.j2"
    daemon_path = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"

    cr_content = cr_path.read_text(encoding="utf-8")
    daemon_content = daemon_path.read_text(encoding="utf-8")

    assert "8;" in cr_content, "code_review.html.j2 must handle OSC 8 hyperlinks"
    assert "8;" in daemon_content, "host_daemon.html.j2 must handle OSC 8 hyperlinks"
