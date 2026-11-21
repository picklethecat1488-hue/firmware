"""Target `.program_metadata` parsing, Cargo output-directory discovery, and ELF validation on host.py."""

import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time
from typing import Any, Dict, List, Optional

from host_daemon.model.transport import (
    InterfaceAvailability,
    InterfaceStatus,
    TargetInterfaceType,
    parse_interface_status_output,
    query_target_interface_status,
)

ATTACHED_INVALID_META_RE = re.compile(r"AttachedTargetMetadataInvalid\s*\(([^)]+)\)")
ACTIVE_TARGET_ELF_FILENAME: str = "active_target.elf"
BUILD_PROFILES: tuple[str, ...] = ("debug", "release")
DEFAULT_QUERY_METADATA_TIMEOUT_SEC: float = 30.0
MAX_METADATA_PROBE_RETRIES: int = 2
METADATA_PROBE_RETRY_BACKOFF_SEC: float = 0.6
_CARGO_TARGET_DIR_CACHE: Dict[Path, Path] = {}


def format_debugger_device_name(chip: Optional[str]) -> Optional[str]:
    """Return the canonical debugger device identifier (e.g. `'RP2040'`, `'MCXN947'`) for a chip string."""
    cleaned = str(chip or "").strip()
    return cleaned.upper() if cleaned else None


def sync_active_target_elf_link(repo_root: Path, elf_path: Optional[Path]) -> Optional[Path]:
    """Atomically point `<cargo_target_dir>/active_target.elf` at `elf_path` so IDE debuggers track debug & release builds."""
    if elf_path is None or not elf_path.is_file():
        return None
    try:
        target_dir = get_cargo_target_directory(repo_root)
    except (OSError, RuntimeError, ValueError):
        target_dir = repo_root / "target"

    try:
        target_dir.mkdir(parents=True, exist_ok=True)
        link_path = target_dir / ACTIVE_TARGET_ELF_FILENAME
        resolved_elf = elf_path.resolve()
        tmp_link = target_dir / f".{ACTIVE_TARGET_ELF_FILENAME}.{os.getpid()}.tmp"
        if tmp_link.exists() or tmp_link.is_symlink():
            tmp_link.unlink()
        try:
            tmp_link.symlink_to(resolved_elf)
            os.replace(tmp_link, link_path)
        except OSError:
            if tmp_link.exists() or tmp_link.is_symlink():
                tmp_link.unlink()
            shutil.copy2(resolved_elf, link_path)
        return link_path
    except OSError:
        return None


def check_metadata_transport_status(
    repo_root: Path,
    raw_output: str,
    *,
    interface: TargetInterfaceType = TargetInterfaceType.SWD_USB,
    host_debug_path: Optional[Path] = None,
    probe_selector: Optional[str] = None,
    serial_port: Optional[Path] = None,
    ble_address: Optional[str] = None,
) -> InterfaceStatus:
    """Resolve structured target interface availability from JSON output or `host_debug probe-status --json`."""
    parsed = parse_interface_status_output(raw_output, default_interface=interface)
    if parsed is not None:
        return parsed
    prefix = (
        _resolve_host_debug_cmd_prefix(repo_root, host_debug_path) if interface == TargetInterfaceType.SWD_USB else None
    )
    return query_target_interface_status(
        repo_root,
        interface=interface,
        host_debug_cmd_prefix=prefix,
        probe_selector=probe_selector,
        serial_port=serial_port,
        ble_address=ble_address,
    )


def get_cargo_target_directory(repo_root: Path) -> Path:
    """Resolve the single Cargo output directory via `cargo metadata --format-version 1 --no-deps`."""
    resolved_root = repo_root.resolve()
    cached = _CARGO_TARGET_DIR_CACHE.get(resolved_root)
    if cached is not None:
        return cached

    env = dict(os.environ)
    env.pop("CARGO_BUILD_TARGET", None)
    env.pop("CARGO_ENCODED_RUSTFLAGS", None)

    try:
        proc = subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"],
            cwd=str(resolved_root),
            env=env,
            capture_output=True,
            text=True,
            timeout=15.0,
            check=False,
        )
    except (OSError, subprocess.SubprocessError) as exc:
        raise RuntimeError(f"Failed to invoke `cargo metadata` in {resolved_root}: {exc}") from exc

    if proc.returncode != 0:
        raise RuntimeError(f"`cargo metadata` failed in {resolved_root}: {proc.stderr.strip()}")

    data = json.loads(proc.stdout)
    target_dir_raw = str(data.get("target_directory") or "").strip()
    if not target_dir_raw:
        raise RuntimeError(f"`cargo metadata` returned an empty target_directory in {resolved_root}")

    target_dir = Path(target_dir_raw)
    _CARGO_TARGET_DIR_CACHE[resolved_root] = target_dir
    return target_dir


def discover_compiled_elfs(repo_root: Path, preferred_elf: Optional[Path] = None) -> List[Path]:
    """Return compiled embedded ELF binaries from Cargo's output directory, putting `preferred_elf` first."""
    from device_tests.model import discover_app_binaries

    candidates: List[Path] = []
    seen_resolved: set[Path] = set()

    if preferred_elf is not None and preferred_elf.exists() and preferred_elf.is_file():
        resolved = preferred_elf.resolve()
        seen_resolved.add(resolved)
        candidates.append(preferred_elf)

    target_dir = get_cargo_target_directory(repo_root)
    app_binaries = discover_app_binaries(repo_root)
    if preferred_elf is None:
        from host_daemon.provider.daemon.config import load_last_session

        last_sess = load_last_session(repo_root)
        if isinstance(last_sess, dict):
            sess_bin = str(last_sess.get("bin") or "").strip()
            sess_spec = app_binaries.get(sess_bin) if sess_bin else None
            if sess_spec is not None and sess_spec.arm_target and sess_spec.bin_name:
                sess_profile = "release" if bool(last_sess.get("release", False)) else "debug"
                for prof in (sess_profile, *[p for p in BUILD_PROFILES if p != sess_profile]):
                    sess_elf = target_dir / sess_spec.arm_target / prof / sess_spec.bin_name
                    if sess_elf.is_file():
                        resolved_sess = sess_elf.resolve()
                        if resolved_sess not in seen_resolved:
                            seen_resolved.add(resolved_sess)
                            candidates.append(sess_elf)

    for spec in app_binaries.values():
        assert spec.arm_target is not None and spec.bin_name is not None
        for profile in BUILD_PROFILES:
            elf_path = target_dir / spec.arm_target / profile / spec.bin_name
            if not elf_path.is_file():
                continue
            resolved = elf_path.resolve()
            if resolved not in seen_resolved:
                seen_resolved.add(resolved)
                candidates.append(elf_path)
    return candidates


def parse_program_metadata_output(raw_output: str) -> Optional[Dict[str, Any]]:
    """Parse JSON or key-value stdout emitted by `host_fs query_metadata` into a metadata dict."""
    cleaned = raw_output.strip()
    if not cleaned:
        return None

    for line in reversed(cleaned.splitlines()):
        candidate = line.strip()
        if candidate.startswith("{") and candidate.endswith("}"):
            try:
                payload = json.loads(candidate)
            except json.JSONDecodeError:
                payload = None
            if isinstance(payload, dict) and payload.get("board_type") and payload.get("chip"):
                return payload

    if "error:" in cleaned.lower():
        return None

    kv: Dict[str, Any] = {}
    for line in cleaned.splitlines():
        if ":" in line and not line.strip().startswith("partition["):
            key, val = line.split(":", 1)
            kv[key.strip().lower()] = val.strip()
    if "board_type" in kv and "chip" in kv and kv["board_type"] and kv["chip"]:
        raw_rel = kv.get("release_build")
        parsed_rel: Optional[bool] = None
        if raw_rel is not None:
            parsed_rel = str(raw_rel).strip().lower() in ("true", "1", "yes")
        return {
            "checksum": kv.get("checksum"),
            "chip": kv["chip"],
            "board_type": kv["board_type"],
            "program_metadata_board_type": kv.get("program_metadata_board_type", kv["board_type"]),
            "provisioned_board_type": kv.get("provisioned_board_type"),
            "bin_name": kv.get("bin_name"),
            "release_build": parsed_rel,
            "trace_mode": kv.get("trace_mode"),
            "source": kv.get("source", "program_metadata"),
        }
    return None


def validate_metadata_matches_target(
    device_meta: Optional[Dict[str, Any]],
    target_meta: Optional[Dict[str, Any]],
) -> bool:
    """Validate that on-device `.program_metadata` matches the target ELF specification on `host.py`."""
    if not device_meta or not target_meta:
        return False

    dev_crc = str(device_meta.get("checksum") or "").strip().upper()
    tgt_crc = str(target_meta.get("checksum") or "").strip().upper()
    if not dev_crc or not tgt_crc or dev_crc != tgt_crc:
        return False

    if str(device_meta.get("chip") or "").strip().lower() != str(target_meta.get("chip") or "").strip().lower():
        return False

    dev_prog_bt = str(device_meta.get("program_metadata_board_type") or device_meta.get("board_type") or "").strip()
    tgt_prog_bt = str(target_meta.get("program_metadata_board_type") or target_meta.get("board_type") or "").strip()
    if dev_prog_bt != tgt_prog_bt:
        return False

    dev_bin = str(device_meta.get("bin_name") or "").strip()
    tgt_bin = str(target_meta.get("bin_name") or "").strip()
    if dev_bin and tgt_bin and dev_bin != tgt_bin:
        return False

    dev_rel = device_meta.get("release_build")
    tgt_rel = target_meta.get("release_build")
    if isinstance(dev_rel, bool) and isinstance(tgt_rel, bool) and dev_rel != tgt_rel:
        return False

    dev_tmode = str(device_meta.get("trace_mode") or "").strip().lower()
    tgt_tmode = str(target_meta.get("trace_mode") or "").strip().lower()
    if dev_tmode and tgt_tmode and dev_tmode != tgt_tmode:
        return False

    for scalar_field in ("flash_start", "flash_write_size", "flash_erase_size", "stack_scan_limit"):
        if scalar_field in device_meta and scalar_field in target_meta:
            if device_meta[scalar_field] != target_meta[scalar_field]:
                return False

    if "partitions" in device_meta and "partitions" in target_meta:
        if device_meta["partitions"] != target_meta["partitions"]:
            return False

    return True


def _resolve_host_fs_cmd_prefix(repo_root: Path, host_fs_path: Optional[Path] = None) -> List[str]:
    """Return direct `host_fs` binary command prefix from Cargo's output directory."""
    candidates: List[Path] = []
    if host_fs_path is not None:
        candidates.append(host_fs_path)

    target_dir = get_cargo_target_directory(repo_root)
    for profile in BUILD_PROFILES:
        candidates.append(target_dir / profile / "host_fs")
    if sys.platform in ("win32", "win64"):
        candidates = [p.with_suffix(".exe") for p in candidates]

    found = next((p for p in candidates if p.exists()), None)
    if found is not None:
        return [str(found)]
    return ["cargo", "run", "--quiet", "--package", "host_fs", "--"]


def _resolve_host_debug_cmd_prefix(repo_root: Path, host_debug_path: Optional[Path] = None) -> List[str]:
    """Return direct `host_debug` binary command prefix from Cargo's output directory."""
    candidates: List[Path] = []
    if host_debug_path is not None:
        candidates.append(host_debug_path)

    target_dir = get_cargo_target_directory(repo_root)
    for profile in BUILD_PROFILES:
        candidates.append(target_dir / profile / "host_debug")
    if sys.platform in ("win32", "win64"):
        candidates = [p.with_suffix(".exe") for p in candidates]

    found = next((p for p in candidates if p.exists()), None)
    if found is not None:
        return [str(found)]
    return ["cargo", "run", "--quiet", "--package", "host_debug", "--"]


def run_host_fs_query_metadata(
    repo_root: Path,
    elf_path: Path,
    *,
    elf_only: bool = False,
    gdb_bridge: Optional[str] = None,
    host_fs_path: Optional[Path] = None,
    timeout: float = DEFAULT_QUERY_METADATA_TIMEOUT_SEC,
) -> tuple[int, str]:
    """Invoke `host_fs --elf <elf_path> [--gdb-bridge <addr>] query_metadata [--elf-only] --json` and return `(rc, output)`."""
    cmd = _resolve_host_fs_cmd_prefix(repo_root, host_fs_path) + [
        "--elf",
        str(elf_path),
    ]
    if gdb_bridge and not elf_only:
        cmd.extend(["--gdb-bridge", str(gdb_bridge)])
    cmd.append("query_metadata")
    if elf_only:
        cmd.append("--elf-only")
    cmd.append("--json")

    env = dict(os.environ)
    env.pop("CARGO_BUILD_TARGET", None)
    env.pop("CARGO_ENCODED_RUSTFLAGS", None)

    max_attempts = 1 if elf_only else (MAX_METADATA_PROBE_RETRIES + 1)
    rc_last = 1
    combined_last = ""
    for attempt in range(max_attempts):
        try:
            proc = subprocess.run(
                cmd,
                cwd=str(repo_root),
                env=env,
                capture_output=True,
                text=True,
                timeout=timeout,
                check=False,
            )
        except (OSError, subprocess.SubprocessError) as exc:
            return 1, str(exc)

        combined_last = f"{proc.stdout}\n{proc.stderr}".strip()
        rc_last = proc.returncode
        if rc_last == 0 or elf_only or attempt >= max_attempts - 1:
            break
        status = check_metadata_transport_status(
            repo_root,
            combined_last,
        )
        if not status.is_retryable:
            break
        time.sleep(METADATA_PROBE_RETRY_BACKOFF_SEC)

    return rc_last, combined_last


def probe_attached_target_metadata(
    repo_root: Path,
    *,
    preferred_elf: Optional[Path] = None,
    host_fs_path: Optional[Path] = None,
    gdb_bridge: Optional[str] = None,
) -> Dict[str, Any]:
    """Probe the attached target using compiled ELFs and validate metadata against `preferred_elf`."""
    try:
        elf_candidates = discover_compiled_elfs(repo_root, preferred_elf=preferred_elf)
    except (OSError, RuntimeError, ValueError) as exc:
        return {
            "connected": False,
            "matches_target": False,
            "error": str(exc),
        }

    if not elf_candidates:
        return {
            "connected": False,
            "matches_target": False,
            "error": "No compiled firmware ELFs available in Cargo target_directory to resolve .program_metadata geometry",
        }

    expected_target_meta: Optional[Dict[str, Any]] = None
    if preferred_elf is not None and preferred_elf.exists():
        rc_elf, out_elf = run_host_fs_query_metadata(
            repo_root,
            preferred_elf,
            elf_only=True,
            host_fs_path=host_fs_path,
        )
        if rc_elf == 0:
            expected_target_meta = parse_program_metadata_output(out_elf)

    seen_geometry_keys: set[tuple[str, Any]] = set()
    last_error = "No debug probes connected"

    for candidate_elf in elf_candidates:
        rc_spec, out_spec = run_host_fs_query_metadata(
            repo_root,
            candidate_elf,
            elf_only=True,
            host_fs_path=host_fs_path,
        )
        if rc_spec != 0:
            continue
        cand_spec = parse_program_metadata_output(out_spec)
        if not cand_spec:
            continue

        geom_key = (
            str(cand_spec.get("chip", "")).lower(),
            cand_spec.get("program_metadata_addr"),
        )
        if geom_key in seen_geometry_keys:
            continue
        seen_geometry_keys.add(geom_key)

        rc_dev, out_dev = run_host_fs_query_metadata(
            repo_root,
            candidate_elf,
            elf_only=False,
            gdb_bridge=gdb_bridge,
            host_fs_path=host_fs_path,
        )
        if rc_dev == 0:
            dev_meta = parse_program_metadata_output(out_dev)
            if dev_meta is not None:
                reference_meta = expected_target_meta or cand_spec
                matches = validate_metadata_matches_target(dev_meta, reference_meta)
                return {
                    "connected": True,
                    "board_type": dev_meta["board_type"],
                    "chip": dev_meta["chip"],
                    "source": dev_meta.get("source", "program_metadata"),
                    "checksum": dev_meta.get("checksum"),
                    "elf_checksum": reference_meta.get("checksum"),
                    "matches_target": matches,
                    "device_metadata": dev_meta,
                    "target_metadata": reference_meta,
                }

        last_error = out_dev
        status_dev = parse_interface_status_output(out_dev)
        if (
            status_dev is not None and status_dev.availability == InterfaceAvailability.DISCONNECTED
        ) or "no debug probes connected" in out_dev.lower():
            return {
                "connected": False,
                "matches_target": False,
                "interface": TargetInterfaceType.SWD_USB.value,
                "availability": InterfaceAvailability.DISCONNECTED.value,
                "in_use": False,
                "error": "No debug probes connected",
            }

        if status_dev is not None and status_dev.in_use:
            return {
                "connected": False,
                "matches_target": False,
                "interface": status_dev.interface.value,
                "availability": status_dev.availability.value,
                "in_use": True,
                "error": status_dev.error or "Target interface is currently in use",
            }

        attached_unprogrammed = (
            status_dev is not None and status_dev.availability == InterfaceAvailability.INVALID_METADATA
        ) or bool(ATTACHED_INVALID_META_RE.search(out_dev))
        if attached_unprogrammed:
            reference_meta = expected_target_meta or cand_spec
            return {
                "connected": True,
                "board_type": cand_spec["board_type"],
                "chip": cand_spec["chip"],
                "source": "attached_unprogrammed",
                "checksum": None,
                "elf_checksum": reference_meta.get("checksum"),
                "matches_target": False,
                "device_metadata": None,
                "target_metadata": reference_meta,
            }

    return {
        "connected": False,
        "matches_target": False,
        "error": last_error,
    }
