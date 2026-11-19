"""Target `.program_metadata` parsing, ELF discovery, and target specification validation on host.py."""

import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Any, Dict, List, Optional

EMBEDDED_BUILD_REL_DIRS: tuple[str, ...] = (
    "target/thumbv6m-none-eabi/debug",
    "target/thumbv6m-none-eabi/release",
    "target/thumbv8m.main-none-eabihf/debug",
    "target/thumbv8m.main-none-eabihf/release",
    "target/out/debug/embedded/cat_detector",
    "target/out/debug/embedded/carrier_board",
)

ATTACHED_INVALID_META_RE = re.compile(r"AttachedTargetMetadataInvalid\s*\(([^)]+)\)")


def discover_compiled_elfs(repo_root: Path, preferred_elf: Optional[Path] = None) -> List[Path]:
    """Return candidate compiled embedded ELF binaries under `target/`, putting `preferred_elf` first."""
    candidates: List[Path] = []
    seen_resolved: set[Path] = set()

    if preferred_elf is not None and preferred_elf.exists() and preferred_elf.is_file():
        resolved = preferred_elf.resolve()
        seen_resolved.add(resolved)
        candidates.append(preferred_elf)

    for rel_dir in EMBEDDED_BUILD_REL_DIRS:
        directory = repo_root / rel_dir
        if not directory.is_dir():
            continue
        for child in sorted(directory.iterdir()):
            if not child.is_file() or child.suffix != "":
                continue
            resolved = child.resolve()
            if resolved not in seen_resolved:
                seen_resolved.add(resolved)
                candidates.append(child)
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
    if "board_type" in kv and "chip" in kv:
        return {
            "checksum": kv.get("checksum"),
            "chip": kv["chip"],
            "board_type": kv["board_type"],
            "program_metadata_board_type": kv.get("program_metadata_board_type", kv["board_type"]),
            "provisioned_board_type": kv.get("provisioned_board_type"),
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

    for scalar_field in ("flash_start", "flash_write_size", "flash_erase_size", "stack_scan_limit"):
        if scalar_field in device_meta and scalar_field in target_meta:
            if device_meta[scalar_field] != target_meta[scalar_field]:
                return False

    if "partitions" in device_meta and "partitions" in target_meta:
        if device_meta["partitions"] != target_meta["partitions"]:
            return False

    return True


def _resolve_host_fs_cmd_prefix(repo_root: Path, host_fs_path: Optional[Path] = None) -> List[str]:
    """Return direct `host_fs` binary command prefix (falling back to `cargo run -q -p host_fs --`)."""
    candidates: List[Path] = []
    if host_fs_path is not None:
        candidates.append(host_fs_path)
    candidates.extend(
        [
            repo_root / "target" / "debug" / "host_fs",
            repo_root / "target" / "release" / "host_fs",
        ]
    )
    if sys.platform in ("win32", "win64"):
        candidates = [p.with_suffix(".exe") for p in candidates]

    found = next((p for p in candidates if p.exists()), None)
    if found is not None:
        return [str(found)]
    return ["cargo", "run", "--quiet", "--package", "host_fs", "--"]


def run_host_fs_query_metadata(
    repo_root: Path,
    elf_path: Path,
    *,
    elf_only: bool = False,
    host_fs_path: Optional[Path] = None,
    timeout: float = 15.0,
) -> tuple[int, str]:
    """Invoke `host_fs --elf <elf_path> query_metadata [--elf-only] --json` and return `(rc, output)`."""
    cmd = _resolve_host_fs_cmd_prefix(repo_root, host_fs_path) + [
        "--elf",
        str(elf_path),
        "query_metadata",
    ]
    if elf_only:
        cmd.append("--elf-only")
    cmd.append("--json")

    env = dict(os.environ)
    env.pop("CARGO_BUILD_TARGET", None)
    env.pop("CARGO_ENCODED_RUSTFLAGS", None)

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

    combined = f"{proc.stdout}\n{proc.stderr}".strip()
    return proc.returncode, combined


def probe_attached_target_metadata(
    repo_root: Path,
    *,
    preferred_elf: Optional[Path] = None,
    host_fs_path: Optional[Path] = None,
) -> Dict[str, Any]:
    """Probe the attached target using compiled ELFs and validate metadata against `preferred_elf`."""
    elf_candidates = discover_compiled_elfs(repo_root, preferred_elf=preferred_elf)
    if not elf_candidates:
        return {
            "connected": False,
            "matches_target": False,
            "error": "No compiled firmware ELFs available under target/ to resolve .program_metadata geometry",
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

    seen_geometry_keys: set[tuple[str, Any, Any]] = set()
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
            cand_spec.get("program_metadata_size"),
        )
        if geom_key in seen_geometry_keys:
            continue
        seen_geometry_keys.add(geom_key)

        rc_dev, out_dev = run_host_fs_query_metadata(
            repo_root,
            candidate_elf,
            elf_only=False,
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
        if "no debug probes connected" in out_dev.lower():
            return {
                "connected": False,
                "matches_target": False,
                "error": "No debug probes connected",
            }

        attached_unprogrammed = ATTACHED_INVALID_META_RE.search(out_dev)
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
