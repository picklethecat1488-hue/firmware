"""Configuration management, target discovery, and session persistence for Xerxes Host Daemon."""

import json
import os
from pathlib import Path
import re
import sys
from typing import Any, Dict, List, Optional

try:
    import tomllib
except ModuleNotFoundError:
    try:
        import tomli as tomllib
    except ModuleNotFoundError:
        tomllib = None  # type: ignore

SESSION_FILE_NAME: str = "host_session.json"


def get_session_file(repo_root: Path) -> Path:
    """Return path to target session persistence file."""
    target_dir = repo_root / "target"
    if target_dir.exists():
        return target_dir / SESSION_FILE_NAME
    return repo_root / f".{SESSION_FILE_NAME}"


def load_last_session(repo_root: Path) -> Optional[Dict[str, Any]]:
    """Load remembered target selection from the last connected session."""
    session_path = get_session_file(repo_root)
    if not session_path.exists():
        fallback = repo_root / f".{SESSION_FILE_NAME}"
        if fallback.exists():
            session_path = fallback
        else:
            return None

    try:
        data = json.loads(session_path.read_text(encoding="utf-8"))
        if isinstance(data, dict) and "package" in data and "bin" in data:
            return data
    except Exception:
        pass
    return None


def save_last_session(repo_root: Path, session: Dict[str, Any]) -> None:
    """Persist target selection from the active connected session."""
    session_path = get_session_file(repo_root)
    try:
        session_path.parent.mkdir(parents=True, exist_ok=True)
        session_path.write_text(json.dumps(session, indent=2), encoding="utf-8")
    except Exception:
        pass


def infer_arm_target(
    package: str,
    bin_name: Optional[str] = None,
    chip: Optional[str] = None,
    repo_root: Optional[Path] = None,
) -> str:
    """Automatically infer target ARM architecture triple from chip or board configuration."""
    resolved_chip = chip
    if not resolved_chip and repo_root:
        board_toml = repo_root / "board" / "board.toml"
        if board_toml.exists() and tomllib:
            try:
                board_data = tomllib.loads(board_toml.read_text(encoding="utf-8"))
                boards = board_data.get("boards", {})
                if package in boards and "chip" in boards[package]:
                    resolved_chip = boards[package]["chip"]
            except Exception:
                pass

    if resolved_chip:
        c = resolved_chip.lower()
        if "rp2040" in c or "cortex-m0" in c:
            return "thumbv6m-none-eabi"
        if "cortex-m33" in c or "mcx" in c or "n947" in c or "n946" in c:
            return "thumbv8m.main-none-eabihf"
        if any(x in c for x in ("cortex-m4f", "cortex-m7f", "stm32f4", "stm32f7", "stm32h7", "nrf52")):
            return "thumbv7em-none-eabihf"
        if "cortex-m4" in c or "cortex-m7" in c:
            return "thumbv7em-none-eabi"
        if "cortex-m3" in c or "stm32f1" in c:
            return "thumbv7m-none-eabi"

    # Default embedded target architecture
    return "thumbv6m-none-eabi"


def discover_target_packages(repo_root: Path) -> List[Dict[str, Any]]:
    """Scan workspace for embedded application packages and binary targets."""
    discovered: List[Dict[str, Any]] = []
    ignored_dirs = {"target", "build", ".cargo", "docs", "feedback", "attachments", "tests", "tools"}

    for item in repo_root.iterdir():
        if not item.is_dir() or item.name in ignored_dirs or item.name.startswith("."):
            continue

        cargo_path = item / "Cargo.toml"
        if not cargo_path.exists():
            continue

        try:
            content = cargo_path.read_text(encoding="utf-8")
            pkg_name = None
            binaries: List[str] = []

            if tomllib:
                data = tomllib.loads(content)
                pkg_name = data.get("package", {}).get("name")
                for b in data.get("bin", []):
                    b_name = b.get("name")
                    if b_name:
                        binaries.append(b_name)
            else:
                # Fallback regex parsing
                pkg_match = re.search(r'\[package\][^\[]*?name\s*=\s*"([^"]+)"', content, re.DOTALL)
                if pkg_match:
                    pkg_name = pkg_match.group(1)
                bin_matches = re.findall(r'\[\[bin\]\][^\[]*?name\s*=\s*"([^"]+)"', content, re.DOTALL)
                binaries.extend(bin_matches)

            # Check if default src/main.rs exists if no [[bin]] specified
            if pkg_name and not binaries and (item / "src" / "main.rs").exists():
                binaries.append(pkg_name)

            if pkg_name and binaries:
                chip_val = "rp2040" if pkg_name == "cat_detector" else None
                inferred = infer_arm_target(pkg_name, binaries[0], chip=chip_val, repo_root=repo_root)
                discovered.append(
                    {
                        "package": pkg_name,
                        "binaries": binaries,
                        "default_bin": binaries[0],
                        "inferred_target": inferred,
                        "chip": chip_val or "rp2040",
                    }
                )
        except Exception:
            continue

    # Guarantee canonical cat_detector package if found or fallback
    if not any(d["package"] == "cat_detector" for d in discovered):
        discovered.insert(
            0,
            {
                "package": "cat_detector",
                "binaries": ["cat_detector_shell", "cat_detector_app", "carrier_board_shell"],
                "default_bin": "cat_detector_shell",
                "inferred_target": "thumbv6m-none-eabi",
                "chip": "rp2040",
            },
        )

    return discovered


def has_explicit_target_args(args_list: Optional[List[str]] = None) -> bool:
    """Check if command line arguments explicitly configure a target, binary, or subcommand.

    Returns False when called with no arguments or only daemon transport / window flags.
    """
    if args_list is None:
        args_list = sys.argv[1:]

    non_target_flags = {
        "--no-detach",
        "--foreground",
        "--detach",
        "--port",
        "--host",
        "--browser",
        "--app",
        "--webview",
        "--eel",
        "--no-browser",
        "--no-app",
        "--restart",
    }

    idx = 0
    while idx < len(args_list):
        arg = args_list[idx]
        if arg in ("--port", "--host", "--browser"):
            idx += 2
            continue
        if arg.startswith("--port=") or arg.startswith("--host=") or arg.startswith("--browser="):
            idx += 1
            continue
        if arg in non_target_flags:
            idx += 1
            continue

        # Found explicit argument (e.g. --target, --package, --bin, --release, or subcommand cli, fs, etc.)
        return True

    return False
