#!/usr/bin/env python3
"""Regression unit test for WORM-TUNNELING-SILKWORM-860.

Verifies terminal formatting, coloration, font weights, and link support in:
1. tools/dashboard/provider/terminal.py (Python ANSI to HTML engine)
2. tools/dashboard/templates/code_review.html.j2 (ANSI parsing, styling, and link handling)
3. tools/host_daemon/templates/host_daemon.html.j2 (ANSI parsing, styling, and link handling)
"""

import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.dashboard.provider.terminal import ansi_to_html


def test_worm_tunneling_silkworm_860_ansi_colors_and_weights():
    """Verify ANSI color codes and font weights are converted to styled HTML."""
    # Green bold text like cargo output: "\x1b[1;32mFinished\x1b[0m"
    cargo_sample = "\x1b[1;32mFinished\x1b[0m `release` profile"
    html = ansi_to_html(cargo_sample)

    assert "term-bold" in html or "font-weight: bold" in html or "font-weight:bold" in html
    assert "term-fg-green" in html or "#" in html or "color:" in html
    assert "Finished" in html
    assert "\x1b" not in html, "Raw ANSI escape codes must not leak into output"

    # Red error text
    error_sample = "\x1b[31mError:\x1b[0m compilation failed"
    err_html = ansi_to_html(error_sample)
    assert "term-fg-red" in err_html or "color:" in err_html
    assert "Error:" in err_html
    assert "\x1b" not in err_html

    # Cyan text
    cyan_sample = "\x1b[36mchannel: RTT\x1b[0m"
    cyan_html = ansi_to_html(cyan_sample)
    assert "term-fg-cyan" in cyan_html or "color:" in cyan_html
    assert "channel: RTT" in cyan_html


def test_worm_tunneling_silkworm_860_links_and_file_paths():
    """Verify web links and file paths are converted to clickable links."""
    # Web URL
    url_sample = "Docs available at https://embassy.dev/book/ and http://localhost:8080"
    url_html = ansi_to_html(url_sample)
    assert '<a href="https://embassy.dev/book/"' in url_html
    assert 'class="term-link' in url_html
    assert '<a href="http://localhost:8080"' in url_html

    # Absolute file path
    abs_path = "Running `/Users/daparker/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell`"
    path_html = ansi_to_html(abs_path)
    assert "term-link" in path_html
    assert "/Users/daparker/gh/firmware/tools/runner.sh" in path_html
    assert "target/thumbv6m-none-eabi/release/cat_detector_shell" in path_html


def test_worm_tunneling_silkworm_860_html_escaping():
    """Verify HTML special characters are escaped to prevent XSS."""
    unsafe = '<script>alert("xss")</script> & "quotes"'
    escaped = ansi_to_html(unsafe)
    assert "<script>" not in escaped
    assert "&lt;script&gt;" in escaped
    assert "&amp;" in escaped


def test_worm_tunneling_silkworm_860_templates_contain_terminal_formatting():
    """Verify code_review and host_daemon templates implement terminal formatting."""
    workspace_root = Path(__file__).resolve().parent.parent.parent

    # Check code_review.html.j2
    cr_path = workspace_root / "tools" / "dashboard" / "templates" / "code_review.html.j2"
    assert cr_path.exists()
    cr_content = cr_path.read_text(encoding="utf-8")
    assert "ansiToHtml" in cr_content or "ansitohtml" in cr_content.lower()
    assert "term-link" in cr_content
    assert "term-bold" in cr_content

    # Check host_daemon.html.j2
    daemon_path = workspace_root / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"
    assert daemon_path.exists()
    daemon_content = daemon_path.read_text(encoding="utf-8")
    assert "ansiToHtml" in daemon_content or "ansitohtml" in daemon_content.lower()
    assert "term-link" in daemon_content
    assert "term-bold" in daemon_content
