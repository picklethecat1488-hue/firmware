"""VS Code editor launch integration for dashboard workstation."""

from pathlib import Path
import shutil
import subprocess
import sys
from typing import Optional, Union


def open_in_vscode(
    file_path: Union[str, Path],
    line: Optional[int] = None,
    column: Optional[int] = None,
    repo_root: Optional[Path] = None,
) -> bool:
    """Open a file at an optional line and column in Visual Studio Code.

    Attempts to invoke the `code` CLI tool first. If `code` is not directly on PATH,
    checks common macOS installation paths. If no `code` executable is available,
    falls back to the platform URL scheme handler (e.g. `open vscode://file/...` on macOS).

    Args:
        file_path: Relative or absolute path to the target file.
        line: Optional 1-indexed target line number.
        column: Optional 1-indexed target column number.
        repo_root: Optional repository root to resolve relative paths against.

    Returns:
        True if the editor or URL handler command was launched successfully, False otherwise.
    """
    if not file_path:
        return False

    raw_path = Path(file_path)
    if repo_root is not None and not raw_path.is_absolute():
        target_path = (Path(repo_root) / raw_path).resolve()
    else:
        target_path = raw_path.resolve()

    target_spec = str(target_path)
    if line is not None and line > 0:
        if column is not None and column > 0:
            target_spec = f"{target_spec}:{line}:{column}"
        else:
            target_spec = f"{target_spec}:{line}"

    # Strategy 1: Look for 'code' executable
    code_bin = shutil.which("code")
    if not code_bin and sys.platform == "darwin":
        candidates = [
            "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code",
            "/Applications/Visual Studio Code - Insiders.app/Contents/Resources/app/bin/code",
            str(Path.home() / "Applications/Visual Studio Code.app/Contents/Resources/app/bin/code"),
            str(Path.home() / "Applications/Visual Studio Code - Insiders.app/Contents/Resources/app/bin/code"),
        ]
        for candidate in candidates:
            if Path(candidate).is_file():
                code_bin = candidate
                break

    if code_bin:
        try:
            subprocess.run(
                [str(code_bin), "-g", target_spec],
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
            return True
        except Exception:
            pass

    # Strategy 2: Fall back to system URL scheme handler (vscode://file/...)
    url_target = target_spec if target_spec.startswith("/") else f"/{target_spec}"
    vscode_url = f"vscode://file{url_target}"

    try:
        if sys.platform == "darwin":
            subprocess.run(
                ["open", vscode_url],
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
            return True
        elif sys.platform.startswith("linux"):
            if shutil.which("xdg-open"):
                subprocess.run(
                    ["xdg-open", vscode_url],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                )
                return True
        elif sys.platform == "win32":
            subprocess.run(
                ["cmd", "/c", "start", "", vscode_url],
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
            return True
    except Exception:
        pass

    return False
