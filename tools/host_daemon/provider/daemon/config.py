"""Configuration management, target discovery, and session persistence for Xerxes Host Daemon."""

import json
import os
from pathlib import Path
import sys
from typing import Any, Dict, List, Optional

if sys.version_info >= (3, 11):
    import tomllib
else:
    import tomli as tomllib

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
    except OSError:
        return None
    except ValueError:
        return None
    return None


def save_last_session(repo_root: Path, session: Dict[str, Any]) -> None:
    """Persist target selection from the active connected session."""
    if "PYTEST_CURRENT_TEST" in os.environ and repo_root.resolve() == Path(__file__).resolve().parents[4]:
        return
    session_path = get_session_file(repo_root)
    try:
        session_path.parent.mkdir(parents=True, exist_ok=True)
        session_path.write_text(json.dumps(session, indent=2), encoding="utf-8")
    except OSError:
        pass


def infer_arm_target(
    package: str,
    bin_name: Optional[str] = None,
    chip: Optional[str] = None,
    repo_root: Optional[Path] = None,
) -> str:
    """Infer target ARM architecture triple from `Cargo.toml` metadata, `board.toml`, or `chip`.

    Raises `ValueError` if the target triple cannot be inferred (never falls back to a default chip
    or target architecture).
    """
    resolved_chip = chip.strip() if chip and chip.strip().lower() != "none" else None

    if repo_root is not None:
        app_cargo = repo_root / "app" / "Cargo.toml"
        if app_cargo.is_file():
            manifest = tomllib.loads(app_cargo.read_text(encoding="utf-8"))
            pkg_meta = (manifest.get("package") or {}).get("metadata") or {}
            bin_meta_map = pkg_meta.get("binaries") or {}
            if bin_name and bin_name in bin_meta_map:
                bmeta = bin_meta_map[bin_name]
                explicit_target = str(bmeta.get("target") or "").strip()
                if explicit_target and explicit_target.lower() != "none":
                    return explicit_target
                if not resolved_chip and bmeta.get("chip"):
                    resolved_chip = str(bmeta["chip"]).strip()

        if not resolved_chip:
            board_toml = repo_root / "board" / "board.toml"
            if board_toml.is_file():
                board_data = tomllib.loads(board_toml.read_text(encoding="utf-8"))
                boards = board_data.get("boards", {})
                if package in boards and boards[package].get("chip"):
                    resolved_chip = str(boards[package]["chip"]).strip()

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

    raise ValueError(
        f"Cannot infer ARM target architecture for package='{package}', bin='{bin_name}', chip='{chip}'; "
        "declare [package.metadata.binaries.<bin>] in Cargo.toml or pass --target/--chip explicitly."
    )


def discover_target_packages(repo_root: Path) -> List[Dict[str, Any]]:
    """Scan workspace `Cargo.toml` manifests for embedded application packages and binary metadata."""
    discovered: List[Dict[str, Any]] = []
    ignored_dirs = {"target", "build", ".cargo", "docs", "feedback", "attachments", "tests", "tools"}

    for item in sorted(repo_root.iterdir(), key=lambda p: p.name):
        if not item.is_dir() or item.name in ignored_dirs or item.name.startswith("."):
            continue

        cargo_path = item / "Cargo.toml"
        if not cargo_path.is_file():
            continue

        try:
            data = tomllib.loads(cargo_path.read_text(encoding="utf-8"))
        except (OSError, ValueError):
            continue

        pkg_data = data.get("package") or {}
        pkg_name = str(pkg_data.get("name") or "").strip()
        default_run = str(pkg_data.get("default-run") or "").strip() or None
        binaries = [str(b.get("name")).strip() for b in (data.get("bin") or []) if b.get("name")]
        if not pkg_name or not binaries:
            continue

        bin_metadata_table = ((pkg_data.get("metadata") or {}).get("binaries")) or {}
        binary_configs: Dict[str, Dict[str, str]] = {}
        for b_name in binaries:
            bmeta = bin_metadata_table.get(b_name)
            if isinstance(bmeta, dict) and bmeta.get("target") and bmeta.get("chip"):
                binary_configs[b_name] = {
                    "target": str(bmeta["target"]).strip(),
                    "chip": str(bmeta["chip"]).strip(),
                    "board": str(bmeta.get("board") or "").strip(),
                }

        if not binary_configs:
            continue

        default_bin = default_run if (default_run and default_run in binaries) else binaries[0]
        binaries_ordered = [default_bin] + [b for b in binaries if b != default_bin]
        default_cfg = binary_configs.get(default_bin) or next(iter(binary_configs.values()))

        discovered.append(
            {
                "package": pkg_name,
                "binaries": binaries_ordered,
                "default_bin": default_bin,
                "inferred_target": default_cfg["target"],
                "chip": default_cfg["chip"],
                "binary_configs": binary_configs,
            }
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
        "--show-system-logs",
        "--show-logs",
        "--debug-server",
        "--halt-on-reset",
        "--gdb-core0-port",
        "--gdb-port",
        "--gdb-core1-port",
        "--rtt-bridge-port",
        "--probe",
    }

    value_flags = (
        "--port",
        "--host",
        "--browser",
        "--gdb-core0-port",
        "--gdb-port",
        "--gdb-core1-port",
        "--rtt-bridge-port",
        "--probe",
    )

    idx = 0
    while idx < len(args_list):
        arg = args_list[idx]
        if arg in value_flags:
            idx += 2
            continue
        if any(arg.startswith(f"{vf}=") for vf in value_flags):
            idx += 1
            continue
        if arg in non_target_flags:
            idx += 1
            continue

        return True

    return False
