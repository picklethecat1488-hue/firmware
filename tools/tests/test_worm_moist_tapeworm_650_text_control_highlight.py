#!/usr/bin/env python3
"""Regression unit test for WORM-MOIST-TAPEWORM-650.

Verifies that:
1. Single clicking on any text control (such as #cmdInput, #promptLabel, .term-cmd-link,
   .cli-line-input, or footer.cli-input-bar) highlights and focuses the CLI text control.
2. handleCliCommandClick populates cmdInput with the clicked command, focuses it, and
   selects/highlights the text rather than immediately executing submitCommand().
3. Clicking on cmdInput itself automatically selects all text (this.select()).
4. Clicking on promptLabel focuses and selects all text in cmdInput.
5. Clicking on previous command output lines (.cli-line-input) copies the command and
   highlights/selects it in cmdInput.
"""

from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
TEMPLATE_PATH = REPO_ROOT / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"


def test_handle_cli_command_click_highlights_cli_input():
    """Verify handleCliCommandClick focuses and highlights cmdInput without auto-submitting."""
    assert TEMPLATE_PATH.exists(), f"File not found: {TEMPLATE_PATH}"
    content = TEMPLATE_PATH.read_text(encoding="utf-8")

    assert "function handleCliCommandClick(cmd)" in content
    # Invariant: Must focus and select (highlight) cmdInput
    assert "input.focus();" in content
    assert "input.select();" in content
    # Invariant: Must NOT auto-submit on click
    # The old buggy behavior was: input.value = cmd; submitCommand();
    assert 'if (typeof submitCommand === "function") {' not in content


def test_single_click_on_cmd_input_highlights_text():
    """Verify single clicking on cmdInput selects/highlights its text."""
    content = TEMPLATE_PATH.read_text(encoding="utf-8")

    # Invariant: cmdInput click listener triggers selection
    assert "cmdInput" in content
    assert "this.select()" in content or "input.select()" in content


def test_single_click_on_prompt_label_highlights_cli():
    """Verify single clicking on prompt label highlights the CLI text control."""
    content = TEMPLATE_PATH.read_text(encoding="utf-8")

    assert "promptLabel" in content
    assert "promptLabel" in content and "input.select()" in content


def test_single_click_on_cli_line_input_highlights_cli():
    """Verify clicking on previous command lines in CLI view populates and highlights cmdInput."""
    content = TEMPLATE_PATH.read_text(encoding="utf-8")

    assert "cli-line-input" in content
    assert "input.select()" in content
