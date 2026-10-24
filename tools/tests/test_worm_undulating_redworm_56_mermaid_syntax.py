"""Regression tests for WORM-UNDULATING-REDWORM-56: Validate Mermaid diagram syntax across documentation."""

import re
import shutil
import subprocess
from pathlib import Path

WORKSPACE_ROOT = Path(__file__).resolve().parent.parent.parent


def test_readme_mermaid_syntax_structural() -> None:
    """Verify that README.md mermaid diagram does not use space-separated unquoted identifiers for subgraphs or edges."""
    readme_path = WORKSPACE_ROOT / "README.md"
    assert readme_path.exists(), "README.md must exist in workspace root"

    content = readme_path.read_text(encoding="utf-8")
    assert "```mermaid" in content, "README.md must contain a mermaid diagram"

    # Extract all mermaid blocks
    mermaid_blocks = re.findall(r"```mermaid\s*\n(.*?)```", content, re.DOTALL)
    assert len(mermaid_blocks) > 0, "At least one mermaid block should be found"

    for block in mermaid_blocks:
        lines = block.strip().splitlines()
        for line in lines:
            line_str = line.strip()
            if not line_str or line_str.startswith("%%"):
                continue

            # Subgraph syntax check: subgraph <id> [<title>]
            if line_str.startswith("subgraph"):
                # Subgraph ID cannot have raw spaces before '[' if brackets are used
                # e.g. "subgraph Controller Layer [controller]" is invalid
                match = re.match(r"^subgraph\s+([A-Za-z0-9_]+)\s*(\[.*\])?$", line_str)
                assert match is not None, f"Invalid subgraph definition with space-separated tokens: {line_str}"

            # Edge syntax check: ensure arrows don't point to multi-word unquoted identifiers
            # e.g. "AppBin -->|Spawns tasks| Controller Layer"
            arrow_match = re.search(r"-->(\|[^|]*\|)?\s*([A-Za-z0-9_]+(\s+[A-Za-z0-9_]+)+)$", line_str)
            assert arrow_match is None, (
                f"Invalid edge target with space-separated unquoted identifier in line: {line_str}"
            )


def test_all_markdown_mermaid_blocks_compile_with_mmdc() -> None:
    """If mermaid-cli or npx is available, verify all markdown mermaid blocks compile cleanly."""
    npx_bin = shutil.which("npx")
    if not npx_bin:
        return

    # Check if mermaid-cli is available via npx without re-downloading
    check = subprocess.run([npx_bin, "@mermaid-js/mermaid-cli", "--version"], capture_output=True, text=True)
    if check.returncode != 0:
        return

    # Find all markdown files in workspace
    md_files = [
        p for p in WORKSPACE_ROOT.rglob("*.md") if not any(part in p.parts for part in ["target", ".venv", ".git"])
    ]

    import tempfile

    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_out = Path(tmp_dir) / "out.svg"
        for md_file in md_files:
            content = md_file.read_text(encoding="utf-8", errors="ignore")
            if "```mermaid" not in content:
                continue

            mermaid_blocks = re.findall(r"```mermaid\s*\n(.*?)```", content, re.DOTALL)
            for idx, block in enumerate(mermaid_blocks):
                proc = subprocess.run(
                    [npx_bin, "@mermaid-js/mermaid-cli", "-i", "-", "-o", str(tmp_out)],
                    input=block,
                    text=True,
                    capture_output=True,
                )
                if proc.returncode != 0 and "ProtocolError" in proc.stderr:
                    proc = subprocess.run(
                        [npx_bin, "@mermaid-js/mermaid-cli", "-i", "-", "-o", str(tmp_out)],
                        input=block,
                        text=True,
                        capture_output=True,
                    )
                assert proc.returncode == 0, (
                    f"Mermaid syntax error in {md_file.relative_to(WORKSPACE_ROOT)} (block {idx}):\n{proc.stderr}"
                )
