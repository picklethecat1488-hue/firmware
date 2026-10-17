#!/usr/bin/env python3
"""Regression unit test for WORM-SETOSE-SETAE-521.

Verifies that:
1. tools/dashboard/templates/code_review.html.j2 implements fuzzy history search:
   - Intercepts Ctrl-R (or Cmd-R) shortcut.
   - Provides a fuzzy history selector overlay/modal.
   - Filters history items with fuzzy matching.
   - Allows selecting history commands to populate the CLI input.
2. tools/host_daemon/templates/host_daemon.html.j2 implements fuzzy history search:
   - Intercepts Ctrl-R (or Cmd-R) shortcut.
   - Provides a fuzzy history selector overlay/modal.
   - Filters history items with fuzzy matching.
   - Allows selecting history commands to populate the CLI input.
"""

from pathlib import Path


def test_worm_setose_setae_521_code_review_fuzzy_history():
    """Verify fuzzy history search in code_review.html.j2."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cr_template_path = workspace_root / "tools" / "dashboard" / "templates" / "code_review.html.j2"

    assert cr_template_path.exists(), f"File not found: {cr_template_path}"
    content = cr_template_path.read_text(encoding="utf-8")

    # Invariant 1: Intercepts Ctrl-R (case-insensitive r with ctrlKey or metaKey)
    assert "ctrlKey" in content and ("'r'" in content or '"r"' in content or "'R'" in content)

    # Invariant 2: Fuzzy history container element and input
    assert "cliFuzzyHistory" in content
    assert "fuzzySearchInput" in content
    assert "fuzzyResultsList" in content
    assert "reverse-i-search" in content

    # Invariant 3: Fuzzy matching logic and navigation handlers
    assert "fuzzyMatch" in content
    assert "openCliFuzzyHistory" in content
    assert "closeCliFuzzyHistory" in content
    assert "selectFuzzyCommand" in content


def test_worm_setose_setae_521_host_daemon_fuzzy_history():
    """Verify fuzzy history search in host_daemon.html.j2."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    daemon_template_path = workspace_root / "tools" / "host_daemon" / "templates" / "host_daemon.html.j2"

    assert daemon_template_path.exists(), f"File not found: {daemon_template_path}"
    content = daemon_template_path.read_text(encoding="utf-8")

    # Invariant 1: Intercepts Ctrl-R
    assert "ctrlKey" in content and ("'r'" in content or '"r"' in content or "'R'" in content)

    # Invariant 2: Fuzzy history container element and input
    assert "cliFuzzyHistory" in content
    assert "fuzzySearchInput" in content
    assert "fuzzyResultsList" in content
    assert "reverse-i-search" in content

    # Invariant 3: Fuzzy matching logic and navigation handlers
    assert "fuzzyMatch" in content
    assert "openCliFuzzyHistory" in content
    assert "closeCliFuzzyHistory" in content
    assert "selectFuzzyCommand" in content
