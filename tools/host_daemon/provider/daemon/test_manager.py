"""On-device pytest discovery, attribute filtering, and live execution manager for `host.py` Test Mode UI."""

from collections import deque
from enum import Enum
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import threading
import time
from typing import Any, Deque, Dict, List, Optional, Sequence
import xml.etree.ElementTree as ET

from device_tests.model import (
    DetectedBoard,
    ScenarioTier,
    discover_device_tests,
    resolve_binary_board_type,
)

MAX_TEST_CONSOLE_LINES: int = 3000
SUBPROCESS_TERMINATE_TIMEOUT_SEC: float = 2.0
RECENT_LIFECYCLE_WINDOW_SEC: float = 2.5
RUNNING_SYNC_INTRA_WEIGHT: float = 0.65
RUNNING_DEFAULT_INTRA_WEIGHT: float = 0.35
MIN_RUNNING_PROGRESS_PCT: int = 5
MAX_RUNNING_PROGRESS_PCT: int = 99
COMPLETE_PROGRESS_PCT: int = 100
PYTEST_VERBOSE_RESULT_RE = re.compile(r"^(\S+::\S+)\s+(PASSED|FAILED|SKIPPED|ERROR)\b")


class TestRunStep(str, Enum):
    """Lifecycle states of a `host.py` Test Mode UI execution run."""

    IDLE = "idle"
    RUNNING = "running"
    COMPLETE = "complete"
    FAILED = "failed"
    CANCELLED = "cancelled"


class TargetLifecyclePhase(str, Enum):
    """Granular target hardware & test execution phases surfaced in the right-panel status card."""

    IDLE = "idle"
    FLASHING = "flashing"
    MODE_SWITCH = "mode_switch"
    REBOOTING = "rebooting"
    EXECUTING_TEST = "executing_test"
    COMPLETE = "complete"
    FAILED = "failed"
    CANCELLED = "cancelled"


def _parse_manual_filter(raw_manual: Any) -> Optional[bool]:
    """Normalize a manual filter query/payload value into `True`, `False`, or `None` (all)."""
    if raw_manual is None:
        return None
    if isinstance(raw_manual, bool):
        return raw_manual
    text = str(raw_manual).strip().lower()
    if text in ("", "all", "*"):
        return None
    if text in ("true", "1", "yes", "manual"):
        return True
    if text in ("false", "0", "no", "automated", "auto"):
        return False
    return None


def _parse_scenarios_filter(raw_scenarios: Any) -> Optional[List[str]]:
    """Normalize a scenario filter string or sequence into canonical `ScenarioTier` values or `None`."""
    if raw_scenarios is None:
        return None
    if isinstance(raw_scenarios, str):
        tokens = [tok.strip().lower() for tok in re.split(r"[,|\s]+", raw_scenarios) if tok.strip()]
    elif isinstance(raw_scenarios, Sequence):
        tokens = [str(tok).strip().lower() for tok in raw_scenarios if str(tok).strip()]
    else:
        return None

    valid_values = {tier.value for tier in ScenarioTier}
    if not tokens or any(tok in ("all", "*") for tok in tokens):
        return None
    if "default" in tokens:
        expanded: List[str] = [ScenarioTier.SMOKE.value, ScenarioTier.INTEGRATION.value]
        for tok in tokens:
            if tok != "default" and tok in valid_values and tok not in expanded:
                expanded.append(tok)
        return expanded

    filtered = [tok for tok in tokens if tok in valid_values]
    return filtered or None


class DeviceTestRunnerManager:
    """Manages `@device_test` scanning, attribute filtering, and background `pytest` execution for `host.py`."""

    def __init__(self, runner: Any) -> None:
        """Initialize test manager bound to a `DaemonRunner`."""
        self.runner = runner
        self._lock = threading.Lock()
        self._active_proc: Optional[subprocess.Popen[str]] = None
        self._worker_thread: Optional[threading.Thread] = None
        self._cancelled: bool = False
        self._cached_board: Optional[DetectedBoard] = None

        self._active: bool = False
        self._step: TestRunStep = TestRunStep.IDLE
        self._total: int = 0
        self._completed: int = 0
        self._passed: int = 0
        self._failed: int = 0
        self._skipped: int = 0
        self._current_test: Optional[str] = None
        self._error: Optional[str] = None
        self._ordered_node_ids: List[str] = []
        self._results_by_node_id: Dict[str, Dict[str, Any]] = {}
        self._console: Deque[str] = deque(maxlen=MAX_TEST_CONSOLE_LINES)

    def probe_detected_board(self) -> Optional[DetectedBoard]:
        """Query the attached board via `runner.query_target_metadata()` and cache the result."""
        if not hasattr(self.runner, "query_target_metadata"):
            return self._cached_board
        meta = self.runner.query_target_metadata()
        board = DetectedBoard.from_metadata_response(meta) if isinstance(meta, dict) and meta.get("connected") else None
        with self._lock:
            self._cached_board = board
        return board

    def _resolve_active_board_type(self, detected_board: Optional[DetectedBoard]) -> Optional[str]:
        """Resolve the active target `board_type` from physical probe metadata or the configured target binary."""
        if detected_board is not None and detected_board.board_type:
            return detected_board.board_type
        if getattr(self.runner, "package_selection_mode", False):
            return None
        configured_bin = getattr(self.runner, "bin_name", None)
        return resolve_binary_board_type(configured_bin, self.runner.repo_root)

    def scan_tests(
        self,
        filters: Optional[Dict[str, Any]] = None,
        *,
        probe_board: bool = False,
    ) -> Dict[str, Any]:
        """Scan `tools/device_tests/suites/` and apply attribute filters while returning available facets."""
        flt = filters or {}
        if probe_board:
            detected_board = self.probe_detected_board()
        else:
            with self._lock:
                detected_board = self._cached_board

        detected_bt = detected_board.board_type if detected_board is not None else None
        active_board_type = self._resolve_active_board_type(detected_board)
        repo_root: Path = self.runner.repo_root

        scenarios_filter = _parse_scenarios_filter(flt.get("scenarios") or flt.get("scenario"))
        manual_filter = _parse_manual_filter(flt.get("manual"))
        raw_board_filter = str(flt.get("board") or "").strip() or None
        raw_binary_filter = str(flt.get("target_binary") or flt.get("binary") or "").strip()
        binary_filter = (
            raw_binary_filter if raw_binary_filter and raw_binary_filter.lower() not in ("all", "*") else None
        )
        project_filter = str(flt.get("project") or "").strip() or None
        query_filter = str(flt.get("query") or flt.get("q") or "").strip() or None

        is_explicit_board_override = raw_board_filter is not None and raw_board_filter.lower() not in (
            "*",
            "all",
            "any",
            "auto",
        )

        # When 'All Binaries' is selected (binary_filter is None) and the user hasn't explicitly overridden
        # the board glob to a different board, exclude tests and binary facets targeting a different board_type.
        facet_board_scope = raw_board_filter if is_explicit_board_override else active_board_type
        effective_board_filter = (
            raw_board_filter
            if is_explicit_board_override
            else (active_board_type if binary_filter is None else raw_board_filter)
        )

        all_tests = discover_device_tests(
            repo_root,
            board=facet_board_scope,
            detected_board_type=detected_bt or active_board_type,
        )
        projects = sorted({str(item["project"]) for item in all_tests if item.get("project")})
        binaries = sorted({str(item.get("target_binary") or "none") for item in all_tests})
        boards = sorted({str(pat) for item in all_tests for pat in item.get("boards", []) if pat})

        filtered_tests = discover_device_tests(
            repo_root,
            project=project_filter,
            scenarios=scenarios_filter,
            board=effective_board_filter,
            target_binary=binary_filter,
            manual=manual_filter,
            query=query_filter,
            detected_board_type=detected_bt or active_board_type,
        )

        with self._lock:
            results_snapshot = {k: dict(v) for k, v in self._results_by_node_id.items()}

        enriched: List[Dict[str, Any]] = []
        for item in filtered_tests:
            merged = dict(item)
            prior = results_snapshot.get(item["node_id"])
            if prior is not None:
                merged["last_result"] = prior
            enriched.append(merged)

        return {
            "tests": enriched,
            "total_available": len(all_tests),
            "detected_board": detected_board.to_dict() if detected_board is not None else None,
            "facets": {
                "projects": projects,
                "scenarios": [tier.value for tier in ScenarioTier],
                "binaries": binaries,
                "boards": boards,
            },
        }

    def start_run(
        self,
        payload: Dict[str, Any],
        *,
        daemon_host: str,
        daemon_port: int,
    ) -> Dict[str, Any]:
        """Start an asynchronous `pytest` run for the selected or filtered `@device_test` items."""
        self.cancel_run(silent=True)

        repo_root: Path = self.runner.repo_root
        requested_ids_raw = payload.get("node_ids")
        requested_set = (
            {str(nid).strip() for nid in requested_ids_raw if str(nid).strip()}
            if isinstance(requested_ids_raw, list) and len(requested_ids_raw) > 0
            else None
        )

        with self._lock:
            detected_bt = self._cached_board.board_type if self._cached_board is not None else None

        if requested_set is not None:
            all_discovered = discover_device_tests(repo_root, detected_board_type=detected_bt)
            selected_entries = [entry for entry in all_discovered if entry["node_id"] in requested_set]
        else:
            filters = payload.get("filters") if isinstance(payload.get("filters"), dict) else payload
            scan_res = self.scan_tests(filters, probe_board=False)
            selected_entries = list(scan_res["tests"])

        if not selected_entries:
            raise ValueError("No @device_test cases matched the requested selection or attribute filters")

        enable_manual = bool(payload.get("enable_manual", True))
        board_arg = str(payload.get("board") or (payload.get("filters") or {}).get("board") or "auto").strip() or "auto"
        ordered_ids = [str(entry["node_id"]) for entry in selected_entries]

        with self._lock:
            self._cancelled = False
            self._active = True
            self._step = TestRunStep.RUNNING
            self._total = len(ordered_ids)
            self._completed = 0
            self._passed = 0
            self._failed = 0
            self._skipped = 0
            self._current_test = ordered_ids[0]
            self._error = None
            self._ordered_node_ids = ordered_ids
            self._console.clear()
            for idx, entry in enumerate(selected_entries):
                nid = str(entry["node_id"])
                self._results_by_node_id[nid] = {
                    "node_id": nid,
                    "name": entry["name"],
                    "project": entry["project"],
                    "status": "RUNNING" if idx == 0 else "PENDING",
                    "duration_s": None,
                    "detail": "",
                }

        worker = threading.Thread(
            target=self._execute_pytest_worker,
            kwargs={
                "ordered_ids": ordered_ids,
                "enable_manual": enable_manual,
                "board_arg": board_arg,
                "daemon_host": daemon_host,
                "daemon_port": daemon_port,
            },
            daemon=True,
        )
        with self._lock:
            self._worker_thread = worker
        worker.start()
        return self.get_status()

    def is_active(self) -> bool:
        """Return True when an on-device `pytest` run is currently active."""
        with self._lock:
            return self._active

    def record_device_cli_line(self, text: str, *, is_input: bool, device_ts: str) -> None:
        """Append a live CLI/FS command or response line tagged with its target `[device_ts]` to `_console`."""
        cleaned = text.rstrip("\r\n")
        if not cleaned.strip():
            return
        with self._lock:
            if not self._active:
                return
            if is_input:
                self._console.append(f"  [{device_ts}] {cleaned}")
            else:
                self._console.append(f"  [{device_ts}]   {cleaned}")

    def record_cli_exchange(
        self,
        command: str,
        output: str,
        *,
        device_ts: str,
        prefix: str = "shell>",
    ) -> None:
        """Record a synchronous command and multiline output exchange with `[device_ts]` into `_console`."""
        clean_cmd = command.strip()
        with self._lock:
            if not self._active:
                return
            if clean_cmd:
                self._console.append(f"  [{device_ts}] {prefix} {clean_cmd}")
            for raw_line in output.splitlines():
                stripped = raw_line.rstrip()
                if stripped:
                    self._console.append(f"  [{device_ts}]   {stripped}")

    def _append_console(self, line: str) -> None:
        """Append a line to the test console buffer and update live per-test status on verbose lines."""
        match = PYTEST_VERBOSE_RESULT_RE.match(line.strip())
        with self._lock:
            self._console.append(line)
            if not match:
                return
            raw_node_id, outcome = match.group(1), match.group(2)
            matched_key = next(
                (nid for nid in self._ordered_node_ids if raw_node_id.endswith(nid) or nid.endswith(raw_node_id)),
                None,
            )
            if matched_key is None:
                return

            normalized_status = "FAILED" if outcome == "ERROR" else outcome
            entry = self._results_by_node_id.get(matched_key)
            if entry is not None and entry.get("status") in ("PENDING", "RUNNING"):
                entry["status"] = normalized_status
                self._completed += 1
                if normalized_status == "PASSED":
                    self._passed += 1
                elif normalized_status == "SKIPPED":
                    self._skipped += 1
                else:
                    self._failed += 1

            next_pending = next(
                (
                    nid
                    for nid in self._ordered_node_ids
                    if self._results_by_node_id.get(nid, {}).get("status") == "PENDING"
                ),
                None,
            )
            self._current_test = next_pending
            if next_pending is not None:
                self._results_by_node_id[next_pending]["status"] = "RUNNING"

    def _ingest_junit_xml(self, junit_path: Path) -> None:
        """Populate per-test `duration_s` and failure/skip `detail` from pytest's JUnit XML output."""
        if not junit_path.is_file():
            return
        try:
            tree = ET.parse(junit_path)
        except (ET.ParseError, OSError):
            return

        with self._lock:
            for testcase in tree.getroot().iter("testcase"):
                tc_name = testcase.attrib.get("name", "")
                duration = float(testcase.attrib.get("time", "0") or 0.0)
                matched_key = next(
                    (nid for nid in self._ordered_node_ids if nid.endswith(f"::{tc_name}")),
                    None,
                )
                if matched_key is None:
                    continue

                failure_el = testcase.find("failure")
                error_el = testcase.find("error")
                skipped_el = testcase.find("skipped")
                if failure_el is not None or error_el is not None:
                    err_node = failure_el if failure_el is not None else error_el
                    assert err_node is not None
                    status = "FAILED"
                    detail = (err_node.attrib.get("message") or (err_node.text or "")).strip()[:500]
                elif skipped_el is not None:
                    status = "SKIPPED"
                    detail = (skipped_el.attrib.get("message") or (skipped_el.text or "")).strip()[:500]
                else:
                    status = "PASSED"
                    detail = ""

                rec = self._results_by_node_id.get(matched_key)
                if rec is not None:
                    was_counted = rec.get("status") in ("PASSED", "FAILED", "SKIPPED")
                    rec["status"] = status
                    rec["duration_s"] = round(duration, 3)
                    rec["detail"] = detail
                    if not was_counted:
                        self._completed += 1
                        if status == "PASSED":
                            self._passed += 1
                        elif status == "SKIPPED":
                            self._skipped += 1
                        else:
                            self._failed += 1

    def _execute_pytest_worker(
        self,
        *,
        ordered_ids: List[str],
        enable_manual: bool,
        board_arg: str,
        daemon_host: str,
        daemon_port: int,
    ) -> None:
        """Run `pytest` in a child process against `daemon_host:daemon_port` and stream live output."""
        repo_root: Path = self.runner.repo_root
        fd, junit_raw = tempfile.mkstemp(prefix="xerxes_ui_device_tests_", suffix=".xml")
        os.close(fd)
        junit_path = Path(junit_raw)

        cmd: List[str] = [
            sys.executable,
            "-m",
            "pytest",
            "-c",
            str(repo_root / "pyproject.toml"),
            "--rootdir",
            str(repo_root),
            "-o",
            "addopts=",
            "-v",
            *ordered_ids,
            "--board",
            board_arg,
            "--host-addr",
            daemon_host,
            "--host-port",
            str(daemon_port),
            "--junitxml",
            str(junit_path),
        ]
        if enable_manual:
            cmd.append("--manual")

        env = dict(os.environ)
        env["PYTHONUNBUFFERED"] = "1"
        py_paths = [
            str(repo_root / "tools"),
            str(repo_root / "tools" / "validation"),
            str(repo_root / "tools" / "helpers"),
            str(repo_root / "tools" / "dashboard"),
        ]
        if env.get("PYTHONPATH"):
            py_paths.append(env["PYTHONPATH"])
        env["PYTHONPATH"] = os.pathsep.join(py_paths)

        self._append_console(f"$ {' '.join(cmd)}")
        returncode = 1
        try:
            proc = subprocess.Popen(
                cmd,
                cwd=str(repo_root),
                env=env,
                stdin=subprocess.DEVNULL,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                bufsize=1,
            )
            with self._lock:
                self._active_proc = proc

            if proc.stdout is not None:
                for raw_line in iter(proc.stdout.readline, ""):
                    stripped = raw_line.rstrip("\r\n")
                    if stripped:
                        self._append_console(stripped)
            returncode = proc.wait()
        except OSError as exc:
            with self._lock:
                self._error = str(exc)
            self._append_console(f"[ERROR] Failed to launch pytest: {exc}")
        finally:
            with self._lock:
                self._active_proc = None
            self._ingest_junit_xml(junit_path)
            try:
                junit_path.unlink(missing_ok=True)
            except OSError:
                pass

            if hasattr(self.runner, "update_manual_prompt"):
                self.runner.update_manual_prompt({"active": False})

            with self._lock:
                self._active = False
                self._current_test = None
                if self._cancelled:
                    self._step = TestRunStep.CANCELLED
                    for nid in self._ordered_node_ids:
                        rec = self._results_by_node_id.get(nid)
                        if rec is not None and rec.get("status") in ("PENDING", "RUNNING"):
                            rec["status"] = "CANCELLED"
                elif returncode in (0, 5) and self._failed == 0:
                    self._step = TestRunStep.COMPLETE
                else:
                    self._step = TestRunStep.FAILED
                    if self._error is None and returncode not in (0, 5):
                        self._error = f"pytest exited with status {returncode}"

    def cancel_run(self, *, silent: bool = False) -> Dict[str, Any]:
        """Terminate any active `pytest` subprocess and clear pending manual prompts."""
        with self._lock:
            self._cancelled = True
            proc = self._active_proc

        if proc is not None and proc.poll() is None:
            try:
                proc.terminate()
                proc.wait(timeout=SUBPROCESS_TERMINATE_TIMEOUT_SEC)
            except (OSError, subprocess.TimeoutExpired):
                try:
                    proc.kill()
                except OSError:
                    pass

        worker = self._worker_thread
        if worker is not None and worker.is_alive() and threading.current_thread() is not worker:
            worker.join(timeout=SUBPROCESS_TERMINATE_TIMEOUT_SEC)

        if hasattr(self.runner, "update_manual_prompt"):
            self.runner.update_manual_prompt({"active": False})

        if not silent:
            with self._lock:
                self._active = False
                self._step = TestRunStep.CANCELLED
                self._current_test = None
                self._console.append("[WARN] Device test run cancelled by user.")
            if hasattr(self.runner, "log"):
                self.runner.log("Device test execution cancelled by user", level="WARN")

        return self.get_status()

    def _extract_runner_activity(self) -> Dict[str, Any]:
        """Extract raw target synchronization, mode, reboot, and command telemetry from `self.runner`."""
        if hasattr(self.runner, "get_target_activity"):
            return dict(self.runner.get_target_activity())

        sync_st = getattr(self.runner, "sync_status", None) or {}
        history = getattr(self.runner, "command_history", None) or []
        return {
            "phase": "flashing" if sync_st.get("active") else "idle",
            "detail": str(sync_st.get("message", "Ready")),
            "updated_at": 0.0,
            "mode": str(getattr(self.runner, "active_mode", "TEST")),
            "mode_switch_count": int(getattr(self.runner, "mode_switch_count", 0)),
            "reboot_count": int(getattr(self.runner, "reboot_count", 0)),
            "last_command": str(history[-1]) if history else "",
            "sync_active": bool(sync_st.get("active", False)),
            "sync_step": str(sync_st.get("step", "idle")),
            "sync_progress": int(sync_st.get("progress", 0)),
            "sync_message": str(sync_st.get("message", "")),
            "target_bin": getattr(self.runner, "bin_name", None),
            "target_package": getattr(self.runner, "package", None),
            "connected": bool(getattr(self.runner, "is_connected", False)),
            "flashed": bool(getattr(self.runner, "is_flashed", False)),
        }

    def _compute_progress_pct(
        self,
        *,
        active: bool,
        step: TestRunStep,
        total: int,
        completed: int,
        sync_active: bool,
        sync_progress: int,
    ) -> int:
        """Compute an interactive `[0..100]` progress percentage across suite and flash stages."""
        if step == TestRunStep.IDLE or total <= 0:
            return 0
        if step == TestRunStep.COMPLETE:
            return COMPLETE_PROGRESS_PCT
        if not active:
            return min(COMPLETE_PROGRESS_PCT, round((completed / total) * COMPLETE_PROGRESS_PCT))

        intra = (
            (max(MIN_RUNNING_PROGRESS_PCT, sync_progress) / float(COMPLETE_PROGRESS_PCT)) * RUNNING_SYNC_INTRA_WEIGHT
            if sync_active
            else RUNNING_DEFAULT_INTRA_WEIGHT
        )
        raw_pct = round(((completed + intra) / total) * COMPLETE_PROGRESS_PCT)
        return max(MIN_RUNNING_PROGRESS_PCT, min(MAX_RUNNING_PROGRESS_PCT, raw_pct))

    def _build_target_activity(
        self,
        *,
        raw_activity: Dict[str, Any],
        active: bool,
        step: TestRunStep,
        total: int,
        completed: int,
        passed: int,
        failed: int,
        skipped: int,
        current_test: Optional[str],
        current_name: Optional[str],
        error: Optional[str],
    ) -> Dict[str, Any]:
        """Resolve human-readable phase, label, headline, and detail for the right-panel status card."""
        now = time.monotonic()
        sync_active = bool(raw_activity.get("sync_active", False))
        sync_step = str(raw_activity.get("sync_step") or "idle")
        sync_progress = int(raw_activity.get("sync_progress", 0))
        sync_message = str(raw_activity.get("sync_message") or "")
        raw_phase = str(raw_activity.get("phase") or "idle")
        raw_detail = str(raw_activity.get("detail") or "")
        updated_at = float(raw_activity.get("updated_at") or 0.0)
        is_recent_event = updated_at > 0.0 and (now - updated_at) <= RECENT_LIFECYCLE_WINDOW_SEC

        target_bin = str(raw_activity.get("target_bin") or "none")
        mode = str(raw_activity.get("mode") or "TEST")
        last_command = str(raw_activity.get("last_command") or "")

        if sync_active or raw_phase == TargetLifecyclePhase.FLASHING.value:
            phase = TargetLifecyclePhase.FLASHING.value
            label = "FLASHING TARGET"
            headline = sync_message or raw_detail or f"Flashing {target_bin}..."
            detail = f"[{sync_step}] {target_bin} ({sync_progress}%)"
        elif raw_phase == TargetLifecyclePhase.REBOOTING.value and is_recent_event:
            phase = TargetLifecyclePhase.REBOOTING.value
            label = "TARGET REBOOT"
            headline = raw_detail or "Rebooting target MCU over SWD..."
            detail = f"Target: {target_bin} • Reboots: {raw_activity.get('reboot_count', 0)}"
        elif raw_phase == TargetLifecyclePhase.MODE_SWITCH.value and is_recent_event:
            phase = TargetLifecyclePhase.MODE_SWITCH.value
            label = "MODE SWITCH"
            headline = raw_detail or f"Target mode switched to {mode}"
            detail = last_command or f"Active mode: {mode} • Target: {target_bin}"
        elif active:
            phase = TargetLifecyclePhase.EXECUTING_TEST.value
            label = "RUNNING TEST"
            index = min(completed + 1, max(1, total))
            short_name = current_name or (current_test.split("::")[-1] if current_test else "suite")
            headline = f"[{index}/{total}] Running {short_name}"
            detail = last_command or f"Target binary: {target_bin} • Mode: {mode}"
        elif step == TestRunStep.COMPLETE:
            phase = TargetLifecyclePhase.COMPLETE.value
            label = "SUITE COMPLETE"
            headline = f"Completed {total}/{total} tests ({passed} passed, {skipped} skipped)"
            detail = f"Target binary: {target_bin} • Mode: {mode}"
        elif step == TestRunStep.FAILED:
            phase = TargetLifecyclePhase.FAILED.value
            label = "SUITE FAILED"
            headline = error or f"Test run failed ({failed} failed of {total})"
            detail = last_command or f"Target binary: {target_bin} • Mode: {mode}"
        elif step == TestRunStep.CANCELLED:
            phase = TargetLifecyclePhase.CANCELLED.value
            label = "RUN CANCELLED"
            headline = f"Cancelled after {completed}/{total} tests"
            detail = f"Target binary: {target_bin} • Mode: {mode}"
        else:
            phase = TargetLifecyclePhase.IDLE.value
            label = "READY"
            headline = "Ready to execute on-device @device_test suites"
            detail = f"Target binary: {target_bin} • Mode: {mode}"

        return {
            **raw_activity,
            "phase": phase,
            "label": label,
            "headline": headline,
            "detail": detail,
        }

    def get_status(self) -> Dict[str, Any]:
        """Return a thread-safe snapshot of the current or most recent Test Mode UI run."""
        manual_dict = (
            self.runner.manual_prompt.to_dict() if hasattr(self.runner, "manual_prompt") else {"active": False}
        )
        raw_activity = self._extract_runner_activity()
        with self._lock:
            ordered_results = [
                dict(self._results_by_node_id[nid]) for nid in self._ordered_node_ids if nid in self._results_by_node_id
            ]
            current_rec = self._results_by_node_id.get(self._current_test or "") if self._current_test else None
            current_name = (
                str(current_rec.get("name")) if isinstance(current_rec, dict) and current_rec.get("name") else None
            )
            progress_pct = self._compute_progress_pct(
                active=self._active,
                step=self._step,
                total=self._total,
                completed=self._completed,
                sync_active=bool(raw_activity.get("sync_active", False)),
                sync_progress=int(raw_activity.get("sync_progress", 0)),
            )
            target_activity = self._build_target_activity(
                raw_activity=raw_activity,
                active=self._active,
                step=self._step,
                total=self._total,
                completed=self._completed,
                passed=self._passed,
                failed=self._failed,
                skipped=self._skipped,
                current_test=self._current_test,
                current_name=current_name,
                error=self._error,
            )
            return {
                "active": self._active,
                "step": self._step.value,
                "total": self._total,
                "completed": self._completed,
                "passed": self._passed,
                "failed": self._failed,
                "skipped": self._skipped,
                "progress_pct": progress_pct,
                "current_test": self._current_test,
                "error": self._error,
                "results": ordered_results,
                "results_map": {k: dict(v) for k, v in self._results_by_node_id.items()},
                "console": list(self._console),
                "manual_prompt": manual_dict,
                "target_activity": target_activity,
                "detected_board": self._cached_board.to_dict() if self._cached_board is not None else None,
            }
