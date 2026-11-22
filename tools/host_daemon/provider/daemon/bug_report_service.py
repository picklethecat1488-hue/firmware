"""Shared target defect (worm report) creation service for `host.py` REST API and CLI."""

import base64
from datetime import datetime, timezone
from pathlib import Path
import re
import sys
from typing import Any, Callable, Dict, List, Optional
import uuid as uuid_pkg

dashboard_dir = Path(__file__).resolve().parents[3] / "dashboard"
if str(dashboard_dir) not in sys.path:
    sys.path.insert(0, str(dashboard_dir))

from model.id_generator import generate_docker_pattern_id
from model.worm_report import (
    WormAttachmentModel,
    WormCategory,
    WormReportModel,
    WormSeverity,
    WormStatus,
)
from provider.sanitizer import elide_personal_info
from provider.worm_report.markdown_exporter import MarkdownWormExporter
from provider.worm_report.sqlite_store import SQLiteWormStore

DEFAULT_BUG_COMPONENT: str = "target"
MAX_SYNTHESIZED_HISTORY_COMMANDS: int = 20
MAX_LOG_SUMMARY_LINES: int = 50
MAX_TEST_LOG_SUMMARY_LINES: int = 30
SCREENSHOT_EXTENSIONS: frozenset[str] = frozenset({".png", ".jpg", ".jpeg", ".gif", ".webp", ".svg"})
LOG_EXTENSIONS: frozenset[str] = frozenset({".log", ".txt", ".out", ".err"})
LEADING_STEP_NUMBER_RE = re.compile(r"^\d+\.\s*")

SEVERITY_BY_NAME: Dict[str, WormSeverity] = {member.value: member for member in WormSeverity}
CATEGORY_BY_NAME: Dict[str, WormCategory] = {member.value: member for member in WormCategory}


def infer_attachment_file_type(file_path: Path) -> str:
    """Infer worm attachment `file_type` (`screenshot`, `log`, or `reference`) from file suffix."""
    suffix = file_path.suffix.lower()
    if suffix in SCREENSHOT_EXTENSIONS:
        return "screenshot"
    if suffix in LOG_EXTENSIONS:
        return "log"
    return "reference"


def build_manual_attachment_payload(file_path: Path, description: Optional[str] = None) -> Dict[str, Any]:
    """Read a local file from disk and encode it as a `/api/target_bug` `manual_attachments` entry."""
    resolved = file_path.expanduser().resolve()
    if not resolved.is_file():
        raise FileNotFoundError(f"Attachment file not found: {file_path}")
    raw_bytes = resolved.read_bytes()
    return {
        "filename": resolved.name,
        "file_type": infer_attachment_file_type(resolved),
        "description": description or f"Attached via host.py bug CLI ({resolved.name})",
        "content_base64": base64.b64encode(raw_bytes).decode("ascii"),
    }


def _normalize_reproduction_steps(raw_steps: Any, runner: Optional[Any] = None) -> List[str]:
    """Normalize explicit reproduction steps or auto-synthesize them from `runner.command_history`."""
    if isinstance(raw_steps, str):
        parsed = [line.strip() for line in raw_steps.splitlines() if line.strip()]
    elif isinstance(raw_steps, list):
        parsed = [str(item).strip() for item in raw_steps if str(item).strip()]
    else:
        parsed = []

    cleaned = [LEADING_STEP_NUMBER_RE.sub("", step).strip() for step in parsed]
    steps = [step for step in cleaned if step]
    if steps or runner is None:
        return steps

    history = list(getattr(runner, "command_history", None) or [])
    if not history:
        return []

    synthesized: List[str] = ["Connect target device via probe-rs / RTT"]
    pkg = getattr(runner, "package", None)
    bin_name = getattr(runner, "bin_name", None)
    if pkg and bin_name:
        synthesized.append(f"Synchronize firmware: package {pkg} ({bin_name})")
    for cmd in history[-MAX_SYNTHESIZED_HISTORY_COMMANDS:]:
        synthesized.append(f"Execute command '{cmd}'")
    synthesized.append("Observe defect/failure on target")
    return synthesized


def _collect_host_daemon_log_content(repo_root: Path, runner: Optional[Any], now_str: str) -> str:
    """Collect `host_daemon.log` (and `host_debug.log` tail when present) for automatic bug report attachment."""
    candidates: List[Path] = []
    debug_candidates: List[Path] = []
    if runner is not None:
        if hasattr(runner, "get_canonical_daemon_log_path"):
            cand = runner.get_canonical_daemon_log_path()
            if isinstance(cand, Path):
                candidates.append(cand)
        if hasattr(runner, "get_log_files"):
            files = runner.get_log_files()
            if isinstance(files, dict):
                if files.get("daemon"):
                    candidates.append(Path(str(files["daemon"])))
                if files.get("debug"):
                    debug_candidates.append(Path(str(files["debug"])))
    candidates.extend(
        [
            repo_root / "target" / "host_daemon.log",
            repo_root / "target" / "logs" / "host_daemon.log",
        ]
    )
    debug_candidates.append(repo_root / "target" / "logs" / "host_debug.log")

    daemon_text = ""
    for path in candidates:
        if path.is_file():
            text = path.read_text(encoding="utf-8", errors="replace")
            if text.strip():
                daemon_text = text
                break

    if not daemon_text and runner is not None:
        lock = getattr(runner, "_lock", None)
        if lock is not None and hasattr(lock, "__enter__"):
            with lock:
                entries = list(getattr(runner, "logs", []) or [])
        else:
            entries = list(getattr(runner, "logs", []) or [])
        daemon_lines = [
            f"[{entry.get('timestamp') or entry.get('time') or ''}] "
            f"[{entry.get('level', 'INFO')}] [{entry.get('channel', 'daemon')}] {entry.get('message', '')}"
            for entry in entries
            if isinstance(entry, dict) and entry.get("channel", "daemon") != "target"
        ]
        if daemon_lines:
            daemon_text = "\n".join(daemon_lines)

    if not daemon_text:
        daemon_text = f"Host daemon log initialized at {now_str}."

    for dbg_path in debug_candidates:
        if dbg_path.is_file():
            dbg_text = dbg_path.read_text(encoding="utf-8", errors="replace").strip()
            if dbg_text:
                daemon_text = f"{daemon_text.rstrip()}\n\n--- host_debug.log ---\n{dbg_text}\n"
            break

    return daemon_text


def _execute_runner_fs_attachment(runner: Optional[Any], fs_cmd: str, label: str) -> str:
    """Execute `runner.execute_fs(fs_cmd)` safely for bug attachment capture."""
    if runner is None:
        return f"No active host daemon session; could not retrieve {label} via '{fs_cmd}'."
    try:
        return str(runner.execute_fs(fs_cmd))
    except (OSError, RuntimeError, ValueError) as exc:
        return f"Failed to retrieve {label}: {exc}"


def create_target_worm_report(
    repo_root: Path,
    payload: Dict[str, Any],
    *,
    runner: Optional[Any] = None,
    test_manager_getter: Optional[Callable[[Any], Any]] = None,
) -> Dict[str, Any]:
    """Create, persist, and export a target worm report from REST API or CLI payload.

    Args:
        repo_root: Repository root containing `target/worms.sqlite`, `feedback/`, and `attachments/`.
        payload: Bug report dictionary (`title`, `severity`, `category`, `component`, `description`,
            `reproduction_steps`, `expected_behavior`, `actual_behavior`, attachment flags, and
            optional `manual_attachments`).
        runner: Active `DaemonRunner` (or mock runner) when invoked inside `host.py` server; `None`
            when invoked offline via standalone `host.py bug`.
        test_manager_getter: Optional callback returning the `DeviceTestRunnerManager` for `runner`.

    Returns:
        Dictionary containing `success`, `worm_id`, `file_path`, and `attachments_count`.

    Raises:
        ValueError: If `title` is empty or missing.
    """
    title = str(payload.get("title") or "").strip()
    if not title:
        raise ValueError("Bug title is required")

    severity_key = str(payload.get("severity") or WormSeverity.MEDIUM.value).strip().upper()
    severity = SEVERITY_BY_NAME.get(severity_key, WormSeverity.MEDIUM)

    category_key = str(payload.get("category") or WormCategory.FIRMWARE.value).strip().upper()
    category = CATEGORY_BY_NAME.get(category_key, WormCategory.FIRMWARE)

    component = str(payload.get("component") or DEFAULT_BUG_COMPONENT).strip() or DEFAULT_BUG_COMPONENT
    description = str(payload.get("description") or "").strip()
    steps = _normalize_reproduction_steps(payload.get("reproduction_steps"), runner=runner)
    expected = str(payload.get("expected_behavior") or "").strip()
    actual = str(payload.get("actual_behavior") or "").strip()

    worm_id = generate_docker_pattern_id("WORM")

    attachments_dir = repo_root / "attachments"
    if not attachments_dir.exists() and (repo_root / "feedback" / "attachments").exists():
        attachments_dir = repo_root / "feedback" / "attachments"
    worm_att_dir = attachments_dir / worm_id
    worm_att_dir.mkdir(parents=True, exist_ok=True)

    attachments: List[WormAttachmentModel] = []
    now_str = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")

    logs_content = ""
    if bool(payload.get("attach_system_logs", True)):
        if runner is not None:
            lock = getattr(runner, "_lock", None)
            if lock is not None:
                with lock:
                    log_entries = list(getattr(runner, "logs", []))
            else:
                log_entries = list(getattr(runner, "logs", []))
            if log_entries:
                lines = [
                    f"[{entry.get('timestamp') or entry.get('time', '')}] [{entry.get('channel', 'daemon')}] "
                    f"[{entry.get('level', 'INFO')}] {entry.get('message', '')}"
                    for entry in log_entries
                ]
                logs_content = "\n".join(lines)
            else:
                logs_content = f"Host daemon session initialized at {now_str}. No logs recorded yet."
        else:
            daemon_log_file = repo_root / "target" / "host_daemon.log"
            if daemon_log_file.is_file():
                logs_content = daemon_log_file.read_text(encoding="utf-8", errors="replace")
            else:
                logs_content = f"Standalone host.py bug report filed at {now_str} (no active host daemon session)."

        log_filename = f"target_system_logs_{worm_id}.log"
        sanitized_logs = elide_personal_info(logs_content)
        (worm_att_dir / log_filename).write_text(sanitized_logs, encoding="utf-8")
        attachments.append(
            WormAttachmentModel(
                id=f"att-{len(attachments) + 1}",
                filename=log_filename,
                file_type="log",
                file_path=f"attachments/{worm_id}/{log_filename}",
                size_bytes=len(sanitized_logs.encode("utf-8")),
                description="Target system and defmt logs from host daemon session",
                created_at=now_str,
            )
        )

    if bool(payload.get("attach_daemon_logs", payload.get("attach_system_logs", True))):
        daemon_raw = _collect_host_daemon_log_content(repo_root, runner, now_str)
        daemon_filename = f"host_daemon_{worm_id}.log"
        sanitized_daemon = elide_personal_info(daemon_raw)
        (worm_att_dir / daemon_filename).write_text(sanitized_daemon, encoding="utf-8")
        attachments.append(
            WormAttachmentModel(
                id=f"att-{len(attachments) + 1}",
                filename=daemon_filename,
                file_type="log",
                file_path=f"attachments/{worm_id}/{daemon_filename}",
                size_bytes=len(sanitized_daemon.encode("utf-8")),
                description="Host daemon API, state transition, debug, and trace backend logs",
                created_at=now_str,
            )
        )

    runner_mode = getattr(runner, "active_mode", "CLI") if runner is not None else "CLI"
    effective_mode = str(payload.get("mode") or runner_mode or "CLI").upper()
    force_test_logs = bool(payload.get("force_attach_test_execution_logs", False))
    should_attach_test_logs = runner is not None and (
        force_test_logs or (effective_mode == "TEST" and bool(payload.get("attach_test_execution_logs", True)))
    )

    test_exec_logs_content = ""
    if should_attach_test_logs and runner is not None:
        tm = test_manager_getter(runner) if test_manager_getter is not None else getattr(runner, "test_manager", None)
        raw_test_console = tm.get_status().get("console") or [] if tm is not None else []
        if raw_test_console:
            test_exec_logs_content = "\n".join(str(line) for line in raw_test_console)
        else:
            test_exec_logs_content = f"Host daemon Test Mode active at {now_str}. No test execution logs recorded yet."

        test_log_filename = f"target_test_execution_logs_{worm_id}.log"
        sanitized_test_logs = elide_personal_info(test_exec_logs_content)
        (worm_att_dir / test_log_filename).write_text(sanitized_test_logs, encoding="utf-8")
        attachments.append(
            WormAttachmentModel(
                id=f"att-{len(attachments) + 1}",
                filename=test_log_filename,
                file_type="log",
                file_path=f"attachments/{worm_id}/{test_log_filename}",
                size_bytes=len(sanitized_test_logs.encode("utf-8")),
                description="Test execution logs from host daemon Test Mode",
                created_at=now_str,
            )
        )

    if bool(payload.get("attach_crash_log", False)):
        crash_out = _execute_runner_fs_attachment(runner, "crash-log", "crash log")
        crash_filename = f"target_crash_log_{worm_id}.log"
        sanitized_crash = elide_personal_info(crash_out)
        (worm_att_dir / crash_filename).write_text(sanitized_crash, encoding="utf-8")
        attachments.append(
            WormAttachmentModel(
                id=f"att-{len(attachments) + 1}",
                filename=crash_filename,
                file_type="log",
                file_path=f"attachments/{worm_id}/{crash_filename}",
                size_bytes=len(sanitized_crash.encode("utf-8")),
                description="Target crash log retrieved via host_fs",
                created_at=now_str,
            )
        )

    if bool(payload.get("attach_telemetry", False)):
        telem_out = _execute_runner_fs_attachment(runner, "export-telemetry", "telemetry")
        telem_filename = f"target_telemetry_{worm_id}.json"
        sanitized_telem = elide_personal_info(telem_out)
        (worm_att_dir / telem_filename).write_text(sanitized_telem, encoding="utf-8")
        attachments.append(
            WormAttachmentModel(
                id=f"att-{len(attachments) + 1}",
                filename=telem_filename,
                file_type="reference",
                file_path=f"attachments/{worm_id}/{telem_filename}",
                size_bytes=len(sanitized_telem.encode("utf-8")),
                description="Target telemetry export retrieved via host_fs",
                created_at=now_str,
            )
        )

    if bool(payload.get("attach_metadata", False)):
        meta_out = _execute_runner_fs_attachment(runner, "cat metadata", "metadata")
        meta_filename = f"target_metadata_{worm_id}.txt"
        sanitized_meta = elide_personal_info(meta_out)
        (worm_att_dir / meta_filename).write_text(sanitized_meta, encoding="utf-8")
        attachments.append(
            WormAttachmentModel(
                id=f"att-{len(attachments) + 1}",
                filename=meta_filename,
                file_type="reference",
                file_path=f"attachments/{worm_id}/{meta_filename}",
                size_bytes=len(sanitized_meta.encode("utf-8")),
                description="Filesystem metadata retrieved via host_fs",
                created_at=now_str,
            )
        )

    manual_atts = payload.get("manual_attachments") or payload.get("attachments") or []
    for att_data in manual_atts:
        if not isinstance(att_data, dict):
            continue
        fname = Path(str(att_data.get("filename") or f"attachment_{uuid_pkg.uuid4().hex[:8]}.bin")).name
        ftype = str(att_data.get("file_type") or "reference")
        desc = str(att_data.get("description") or "Target bug attachment")
        b64 = str(att_data.get("content_base64") or "")
        text = str(att_data.get("content_text") or "")

        dest_path = worm_att_dir / fname
        if dest_path.exists():
            stem = dest_path.stem
            suf = dest_path.suffix
            suffix_idx = 1
            while (worm_att_dir / f"{stem}_{suffix_idx}{suf}").exists():
                suffix_idx += 1
            fname = f"{stem}_{suffix_idx}{suf}"
            dest_path = worm_att_dir / fname

        if b64:
            encoded_part = b64.split(",", 1)[1] if "," in b64 else b64
            raw_bytes = base64.b64decode(encoded_part)
        elif text:
            raw_bytes = elide_personal_info(text).encode("utf-8")
        else:
            raw_bytes = b""
        dest_path.write_bytes(raw_bytes)

        attachments.append(
            WormAttachmentModel(
                id=f"att-{len(attachments) + 1}",
                filename=fname,
                file_type=ftype,
                file_path=f"attachments/{worm_id}/{fname}",
                size_bytes=len(raw_bytes),
                description=desc,
                created_at=now_str,
            )
        )

    sanitized_full_logs = elide_personal_info(logs_content)
    log_lines = sanitized_full_logs.strip().splitlines()
    log_summary = (
        "\n".join(log_lines[-MAX_LOG_SUMMARY_LINES:]) if len(log_lines) > MAX_LOG_SUMMARY_LINES else sanitized_full_logs
    )
    if test_exec_logs_content:
        test_tail = "\n".join(
            elide_personal_info(test_exec_logs_content).strip().splitlines()[-MAX_TEST_LOG_SUMMARY_LINES:]
        )
        log_summary = f"{log_summary}\n\n--- Test Execution Log ---\n{test_tail}".strip()

    worm = WormReportModel(
        id=worm_id,
        uuid=str(uuid_pkg.uuid4()),
        title=elide_personal_info(title),
        status=WormStatus.OPEN,
        severity=severity,
        category=category,
        component=elide_personal_info(component),
        description=elide_personal_info(description),
        reproduction_steps=[elide_personal_info(step) for step in steps],
        expected_behavior=elide_personal_info(expected),
        actual_behavior=elide_personal_info(actual),
        logs=log_summary,
        attachments=attachments,
        created_at=now_str,
        updated_at=now_str,
    )

    db_file = repo_root / "target" / "worms.sqlite"
    db_file.parent.mkdir(parents=True, exist_ok=True)
    store = SQLiteWormStore(db_file)
    store.save_worm(worm)

    feedback_dir = repo_root / "feedback"
    exporter = MarkdownWormExporter(repo_root=repo_root)
    exported_path = exporter.export_individual_worm(worm, feedback_dir=feedback_dir)

    if runner is not None and hasattr(runner, "log"):
        runner.log(
            f"Target bug created: {worm_id} ({title}) with {len(attachments)} attachments",
            level="INFO",
            channel="daemon",
        )

    return {
        "success": True,
        "worm_id": worm_id,
        "file_path": str(exported_path),
        "attachments_count": len(attachments),
    }
