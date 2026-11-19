"""Regression tests for WORM-UNDULATING-REDWORM-56: Validate Mermaid diagram syntax across documentation."""

import re
import shutil
import subprocess
from pathlib import Path

WORKSPACE_ROOT = Path(__file__).resolve().parent.parent.parent


def test_readme_mermaid_syntax_structural() -> None:
    """Verify documentation mermaid diagrams do not use space-separated unquoted identifiers for subgraphs or edges."""
    readme_path = WORKSPACE_ROOT / "README.md"
    arch_doc_path = WORKSPACE_ROOT / "docs" / "firmware_architecture.md"
    assert readme_path.exists(), "README.md must exist in workspace root"
    assert arch_doc_path.exists(), "docs/firmware_architecture.md must exist"
    assert "```mermaid" in arch_doc_path.read_text(encoding="utf-8"), (
        "docs/firmware_architecture.md must contain the workspace architecture mermaid diagrams"
    )

    md_files = [readme_path, *sorted((WORKSPACE_ROOT / "docs").glob("*.md"))]
    total_blocks = 0

    for md_path in md_files:
        content = md_path.read_text(encoding="utf-8")
        mermaid_blocks = re.findall(r"```mermaid\s*\n(.*?)```", content, re.DOTALL)
        total_blocks += len(mermaid_blocks)

        for block in mermaid_blocks:
            lines = block.strip().splitlines()
            for line in lines:
                line_str = line.strip()
                if not line_str or line_str.startswith("%%"):
                    continue

                # Subgraph syntax check: subgraph <id> [<title>]
                if line_str.startswith("subgraph"):
                    match = re.match(r"^subgraph\s+([A-Za-z0-9_]+)\s*(\[.*\])?$", line_str)
                    assert match is not None, (
                        f"Invalid subgraph definition in {md_path.relative_to(WORKSPACE_ROOT)}: {line_str}"
                    )

                # Edge syntax check: ensure arrows don't point to multi-word unquoted identifiers
                arrow_match = re.search(r"-->(\|[^|]*\|)?\s*([A-Za-z0-9_]+(\s+[A-Za-z0-9_]+)+)$", line_str)
                assert arrow_match is None, f"Invalid edge target in {md_path.relative_to(WORKSPACE_ROOT)}: {line_str}"

    assert total_blocks > 0, "At least one mermaid block must be validated across documentation"


def test_all_markdown_mermaid_blocks_compile_with_mmdc() -> None:
    """If mermaid-cli or npx is available, verify all markdown mermaid blocks compile cleanly."""
    npx_bin = shutil.which("npx")
    if not npx_bin:
        return

    # Check if mermaid-cli is already installed locally without downloading via npx
    check = subprocess.run(
        [npx_bin, "--no-install", "@mermaid-js/mermaid-cli", "--version"],
        capture_output=True,
        text=True,
    )
    if check.returncode != 0:
        return

    # Find all markdown files in workspace
    md_files = [
        p for p in WORKSPACE_ROOT.rglob("*.md") if not any(part in p.parts for part in ["target", ".venv", ".git"])
    ]

    import tempfile

    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_out = Path(tmp_dir) / "out.svg"
        puppeteer_config = Path(tmp_dir) / "puppeteer-config.json"
        puppeteer_config.write_text('{"args": ["--no-sandbox", "--disable-setuid-sandbox"]}', encoding="utf-8")
        for md_file in md_files:
            content = md_file.read_text(encoding="utf-8", errors="ignore")
            if "```mermaid" not in content:
                continue

            mermaid_blocks = re.findall(r"```mermaid\s*\n(.*?)```", content, re.DOTALL)
            for idx, block in enumerate(mermaid_blocks):
                cmd = [npx_bin, "@mermaid-js/mermaid-cli", "-p", str(puppeteer_config), "-i", "-", "-o", str(tmp_out)]
                proc = subprocess.run(
                    cmd,
                    input=block,
                    text=True,
                    capture_output=True,
                )
                if proc.returncode != 0 and "ProtocolError" in proc.stderr:
                    proc = subprocess.run(
                        cmd,
                        input=block,
                        text=True,
                        capture_output=True,
                    )
                if proc.returncode != 0 and (
                    "Failed to launch the browser process" in proc.stderr
                    or "error while loading shared libraries" in proc.stderr
                ):
                    return
                assert proc.returncode == 0, (
                    f"Mermaid syntax error in {md_file.relative_to(WORKSPACE_ROOT)} (block {idx}):\n{proc.stderr}"
                )
