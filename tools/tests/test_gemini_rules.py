"""Regression tests validating GEMINI.md modular structure and firmware back-compat mandate (BUG-003).

Ensures:
1. GEMINI.md contains core agent skills and workflow invariants only.
2. GEMINI.md includes the firmware target back-compatibility mandate:
   "Unless duly noted, any firmware code should apply to all firmware targets in the repository."
3. Subsystem-specific architectural guides are modularized into dedicated documents under docs/.
4. All subsystem guide links in GEMINI.md point to existing, non-empty files in docs/.
"""

from pathlib import Path
import re


def get_repo_root() -> Path:
    """Return repository root path."""
    return Path(__file__).resolve().parents[2]


def test_gemini_markdown_core_skills_and_back_compat_mandate() -> None:
    """Verify GEMINI.md contains core agent rules and firmware back-compat mandate."""
    repo_root = get_repo_root()
    gemini_path = repo_root / "GEMINI.md"
    assert gemini_path.exists(), "GEMINI.md must exist in repository root"
    content = gemini_path.read_text(encoding="utf-8")

    # 1. Verification & Process hygiene
    assert "./tools/verify.sh" in content
    assert "Process Management & Rerun Hygiene" in content

    # 2. Regression unit testing mandate
    assert "Regression Unit Testing Mandate" in content

    # 3. Firmware target back-compatibility mandate (BUG-003)
    mandate_phrase = "unless duly noted, any firmware code should apply to all firmware targets in the repository"
    assert mandate_phrase in content.lower(), (
        f"Missing firmware target back-compatibility mandate in GEMINI.md: '{mandate_phrase}'"
    )

    # 4. Strict Code Cleanliness & Hygiene
    assert "No Dead Code" in content
    assert "No Backward Compatibility Shims" in content
    assert "Parameter & Signature Hygiene" in content

    # 5. Workflow & Issue Tracking (Worm Tracker & Single-Bug Focus)
    assert "Worm Tracker" in content
    assert "Single-Bug Focus" in content
    assert "User-Managed Code Review & Autonomous PR Prohibition" in content

    # 6. Modular Subsystem Guides section
    assert "## Modular Subsystem & Domain Architecture Guides" in content


def test_modular_subsystem_guides_exist_and_linked_in_gemini() -> None:
    """Verify domain-specific guides under docs/ exist, are non-empty, and are referenced in GEMINI.md."""
    repo_root = get_repo_root()
    gemini_path = repo_root / "GEMINI.md"
    assert gemini_path.exists()
    content = gemini_path.read_text(encoding="utf-8")

    expected_guides = [
        "docs/mcu_decoupling.md",
        "docs/peripheral_sharing.md",
        "docs/controller_design.md",
        "docs/logging_tracing.md",
        "docs/hardware_firmware_codesign.md",
        "docs/vcs_code_review.md",
    ]

    for guide_rel in expected_guides:
        guide_path = repo_root / guide_rel
        assert guide_path.exists(), f"Subsystem guide {guide_rel} must exist under docs/"
        assert guide_path.stat().st_size > 100, f"Subsystem guide {guide_rel} must not be empty"
        assert guide_rel in content or guide_path.name in content, (
            f"GEMINI.md must reference subsystem guide {guide_rel}"
        )


def test_gemini_documentation_style_uses_rust_syntax() -> None:
    """Verify Section 4 of GEMINI.md uses canonical Rust syntax examples for coding style."""
    repo_root = get_repo_root()
    gemini_path = repo_root / "GEMINI.md"
    assert gemini_path.exists()
    content = gemini_path.read_text(encoding="utf-8")

    # Enums & Newtypes with Rust derive
    assert "Strongly Typed Enums & Newtypes" in content
    assert "defmt::Format" in content

    # Constants with pub const
    assert "pub const" in content or "ALL_CAPS named constants" in content

    # Pattern matching with match
    assert "match" in content
    assert "if let Some" in content

