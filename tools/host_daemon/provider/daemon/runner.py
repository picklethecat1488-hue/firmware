"""Runner for building targets, flashing devices, streaming RTT, and executing CLI/FS tools."""

from collections import deque
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import threading
import time
from typing import Any, Deque, Dict, List, Optional, Tuple

from host_daemon.model.manual_prompt import ManualPromptState
from host_daemon.model.metadata import (
    ACTIVE_TARGET_ELF_FILENAME,
    format_debugger_device_name,
    get_cargo_target_directory,
    probe_attached_target_metadata,
    sync_active_target_elf_link,
)
from host_daemon.model.trace_mode import (
    TraceMode,
    get_cargo_feature_args,
    get_external_trace_viewer,
    normalize_trace_mode,
)
from host_daemon.model.transport import (
    InterfaceAvailability,
    TargetInterfaceType,
    parse_interface_status_output,
    query_target_interface_status,
)
from host_daemon.provider.daemon.config import discover_target_packages, save_last_session
from host_daemon.provider.daemon.rerun_bridge import open_trace_in_rerun
from host_daemon.provider.daemon.test_manager import DeviceTestRunnerManager
from host_daemon.provider.daemon.trace_store import TraceSessionStore

RTT_STDIN_SHUTDOWN_TIMEOUT_SEC: float = 2.5
RTT_TERMINATE_TIMEOUT_SEC: float = 2.0
RTT_STREAM_JOIN_TIMEOUT_SEC: float = 1.0
RTT_STARTUP_EVENT_TIMEOUT_SEC: float = 20.0
RTT_BOOT_READY_TIMEOUT_SEC: float = 5.0
RTT_CONSOLE_READY_SETTLE_SEC: float = 0.20
DEBUG_SERVER_STARTUP_TIMEOUT_SEC: float = 20.0
DEFAULT_GDB_CORE0_PORT: int = 3333
DEFAULT_GDB_CORE1_PORT: int = 3334
DEFAULT_RTT_BRIDGE_PORT: int = 50000
PROBE_RELEASE_SETTLE_SEC: float = 0.25
PROBE_RETRY_BACKOFF_SEC: float = 0.6
MAX_PROBE_OPEN_RETRIES: int = 2
DEFAULT_CLI_OUTPUT_TIMEOUT_SEC: float = 2.5
CLI_POLL_INTERVAL_SEC: float = 0.02
CLI_QUIET_SETTLE_SEC: float = 0.30
CLI_QUIET_SETTLE_MAX_SEC: float = 2.0
CLI_QUIET_TIMEOUT_RATIO: float = 0.45
CLI_COMPLETION_SETTLE_SEC: float = 0.25
ANSI_CLEAR_LINE_SEQ: str = "\x1b[2K"
SHELL_PROMPT_PREFIX: str = "shell>"
ANSI_ESCAPE_RE = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")
DEFMT_DEVICE_TS_RE = re.compile(r"^\s*(\d+\.\d+)\s+(?:INFO|WARN|ERROR|DEBUG|TRACE)\b")
RTT_SESSION_BANNER_PREFIXES: Tuple[str, ...] = (
    "RTT connected. Active channels:",
    "- Up Channel:",
    "- Down Channel:",
    "RTT Session Started",
    "Interactive RTT console active.",
    "Type 'help' to print usage.",
    "|\\      _,,,---,,_",
    "Zzz   /",
    "|,4-  )",
    "'---''(_",
)


class DaemonRunner:
    """Manages firmware compilation, device flashing, RTT streaming, and CLI/FS command execution."""

    def __init__(
        self,
        repo_root: Path,
        target: Optional[str] = None,
        package: Optional[str] = None,
        bin_name: Optional[str] = None,
        release: bool = False,
        speed: int = 1000,
        chip: Optional[str] = None,
        package_selection_mode: bool = False,
        show_system_logs: bool = False,
        enable_debug_server: bool = False,
        halt_on_reset: bool = False,
        gdb_core0_port: int = DEFAULT_GDB_CORE0_PORT,
        gdb_core1_port: int = DEFAULT_GDB_CORE1_PORT,
        rtt_bridge_port: int = DEFAULT_RTT_BRIDGE_PORT,
        probe_selector: Optional[str] = None,
        trace_mode: Optional[str] = None,
    ) -> None:
        """Initialize DaemonRunner with workspace configuration and build parameters."""
        self.repo_root = repo_root

        resolved_pkg = package
        resolved_bin = bin_name
        resolved_target = target
        resolved_chip = chip
        if (repo_root / "app" / "Cargo.toml").is_file():
            discovered = discover_target_packages(repo_root)
            if discovered:
                pkg_entry = (
                    next((p for p in discovered if p["package"] == resolved_pkg), None)
                    if resolved_pkg
                    else discovered[0]
                )
                if pkg_entry is not None:
                    if not resolved_pkg:
                        resolved_pkg = str(pkg_entry["package"])
                    if not resolved_bin:
                        resolved_bin = str(pkg_entry["default_bin"])
                    bin_cfg = (pkg_entry.get("binary_configs") or {}).get(resolved_bin or "") or {}
                    if not resolved_target:
                        resolved_target = str(bin_cfg.get("target") or pkg_entry.get("inferred_target") or "") or None
                    if not resolved_chip and bin_cfg.get("chip"):
                        resolved_chip = str(bin_cfg["chip"])

        self.target: Optional[str] = resolved_target
        self.package: Optional[str] = resolved_pkg
        self.bin_name: Optional[str] = resolved_bin
        self.release = release
        self.speed = speed
        self.chip: Optional[str] = resolved_chip
        self.package_selection_mode = package_selection_mode
        self.show_system_logs = show_system_logs
        self.enable_debug_server: bool = bool(enable_debug_server)
        self.halt_on_reset: bool = bool(halt_on_reset)
        self.gdb_core0_port: int = int(gdb_core0_port)
        self.gdb_core1_port: int = int(gdb_core1_port)
        self.rtt_bridge_port: int = int(rtt_bridge_port)
        self._saved_user_debug_ports: Optional[Tuple[int, int, int]] = None
        self.probe_selector: Optional[str] = probe_selector
        self.trace_mode: TraceMode = normalize_trace_mode(trace_mode)

        self.elf_path: Optional[Path] = self.resolve_current_elf_path()
        if self.elf_path is not None:
            sync_active_target_elf_link(self.repo_root, self.elf_path)
        self.host_cli_path: Optional[Path] = None
        self.host_fs_path: Optional[Path] = None
        self.host_debug_path: Optional[Path] = None
        self.manual_prompt = ManualPromptState()
        self.test_manager = DeviceTestRunnerManager(self)
        self.trace_store = TraceSessionStore()
        self._streams_live_cli: bool = True

        self.active_mode: str = "CLI"
        self._lock = threading.Lock()
        self._fs_lock = threading.RLock()
        self.logs: Deque[Dict[str, Any]] = deque(maxlen=5000)
        self.cli_outputs: Deque[Dict[str, Any]] = deque(maxlen=5000)
        self.fs_outputs: Deque[Dict[str, Any]] = deque(maxlen=5000)
        self._log_seq: int = 0
        self._cli_seq: int = 0
        self._cli_clear_seq: int = 0
        self._cmd_done_seq: int = 0
        self._cmd_done_clear_floor: int = 0
        self._last_cmd_error: Optional[str] = None
        self._json_events_active: bool = False
        self._last_shell_idle_epoch: Optional[int] = None
        self._fs_seq: int = 0
        self.command_history: List[str] = []
        self._boot_mono: float = time.monotonic()
        self._last_device_ts_sec: Optional[float] = None
        self._last_device_ts_mono: float = self._boot_mono
        self._cached_metadata: Optional[Dict[str, Any]] = None
        self._cached_metadata_elf: Optional[Path] = None

        self.rtt_process: Optional[subprocess.Popen] = None
        self._streaming_thread: Optional[threading.Thread] = None
        self.debug_process: Optional[subprocess.Popen] = None
        self._debug_thread: Optional[threading.Thread] = None
        self.debug_server_active: bool = False
        self.debug_server_info: Dict[str, Any] = {}
        self.is_flashed = False
        self.is_connected = False
        self.status_message = "Awaiting Target Configuration" if package_selection_mode else "Initialized"
        self.sync_logs: Deque[str] = deque(maxlen=2000)
        self.is_sync_cancelled: bool = False
        self._active_sync_process: Optional[subprocess.Popen] = None

        self.reboot_count: int = 0
        self.mode_switch_count: int = 0
        self.last_command: str = ""
        self._lifecycle_phase: str = "idle"
        self._lifecycle_detail: str = "Ready"
        self._lifecycle_updated_at: float = 0.0

        self.flashed_commit: Optional[str] = None
        self.sync_status: Dict[str, Any] = {
            "active": False,
            "step": "idle",
            "step_index": 0,
            "total_steps": 4,
            "progress": 0,
            "message": "Idle",
            "target": self.target,
            "package": self.package,
            "bin": self.bin_name,
            "profile": "release" if self.release else "debug",
            "trace_mode": self.trace_mode.value,
            "speed": self.speed,
            "flashed_commit": None,
            "error": None,
            "console": [],
        }
        self._sync_thread: Optional[threading.Thread] = None

    def log_sync_console(self, line: str) -> None:
        """Append a line to the sync console log buffer and update sync_status."""
        with self._lock:
            self.sync_logs.append(line)
            self.sync_status["console"] = list(self.sync_logs)

    def cancel_sync(self) -> Dict[str, Any]:
        """Abort running synchronization process and mark status cancelled."""
        self.is_sync_cancelled = True
        proc = self._active_sync_process
        if proc and proc.poll() is None:
            try:
                proc.terminate()
                proc.wait(timeout=1.0)
            except Exception:
                try:
                    proc.kill()
                except Exception:
                    pass
        self._update_sync(
            "cancelled", 0, 0, "Target synchronization cancelled by user.", active=False, error="Sync cancelled by user"
        )
        self.log("Target synchronization cancelled by user", level="WARN")
        return {"success": True, "step": "cancelled"}

    def _reset_device_clock(self) -> None:
        """Reset synchronized target device uptime anchor upon MCU flash, reset, or RTT session start."""
        now = time.monotonic()
        with self._lock:
            self._boot_mono = now
            self._last_device_ts_sec = None
            self._last_device_ts_mono = now

    def _invalidate_metadata_cache(self) -> None:
        """Clear cached `.program_metadata` verification state on target binary/config transitions."""
        with self._lock:
            self._cached_metadata = None
            self._cached_metadata_elf = None

    def get_device_timestamp_str(self) -> str:
        """Return the current target MCU monotonic timestamp string (`e.g. '0.714500s'`) synchronized from `defmt`."""
        now = time.monotonic()
        with self._lock:
            if self._last_device_ts_sec is not None:
                est = self._last_device_ts_sec + max(0.0, now - self._last_device_ts_mono)
            else:
                est = max(0.0, now - self._boot_mono)
        return f"{est:.6f}s"

    def _record_lifecycle(self, phase: str, detail: str, *, mark_recent: bool = True) -> None:
        """Record a thread-safe target lifecycle event (`flashing`, `mode_switch`, `rebooting`, `idle`)."""
        with self._lock:
            self._lifecycle_phase = phase
            self._lifecycle_detail = detail
            self._lifecycle_updated_at = time.monotonic() if mark_recent else 0.0

    def get_target_activity(self) -> Dict[str, Any]:
        """Return a thread-safe snapshot of target flash, mode-switch, reboot, and command telemetry."""
        with self._lock:
            return {
                "phase": self._lifecycle_phase,
                "detail": self._lifecycle_detail,
                "updated_at": self._lifecycle_updated_at,
                "mode": self.active_mode,
                "mode_switch_count": self.mode_switch_count,
                "reboot_count": self.reboot_count,
                "last_command": self.last_command,
                "sync_active": bool(self.sync_status.get("active", False)),
                "sync_step": str(self.sync_status.get("step", "idle")),
                "sync_progress": int(self.sync_status.get("progress", 0)),
                "sync_message": str(self.sync_status.get("message", "")),
                "target_bin": self.bin_name,
                "target_package": self.package,
                "connected": self.is_connected,
                "flashed": self.is_flashed,
            }

    def _update_sync(
        self,
        step: str,
        step_index: int,
        progress: int,
        message: str,
        active: bool = True,
        error: Optional[str] = None,
    ) -> None:
        """Update the thread-safe target synchronization status."""
        with self._lock:
            self.sync_status.update(
                {
                    "active": active,
                    "step": step,
                    "step_index": step_index,
                    "total_steps": 4,
                    "progress": progress,
                    "message": message,
                    "target": self.target,
                    "package": self.package,
                    "bin": self.bin_name,
                    "profile": "release" if self.release else "debug",
                    "trace_mode": self.trace_mode.value,
                    "speed": self.speed,
                    "flashed_commit": self.flashed_commit,
                    "error": error,
                    "console": list(self.sync_logs),
                }
            )
            if active:
                self._lifecycle_phase = "flashing"
                self._lifecycle_detail = message
                self._lifecycle_updated_at = time.monotonic()
            else:
                self._lifecycle_phase = "idle"
                self._lifecycle_detail = message
                self._lifecycle_updated_at = 0.0
        self.log(f"[Sync] {message}")

    def resolve_current_elf_path(self) -> Optional[Path]:
        """Resolve the single Cargo output ELF path for the active target/profile/binary, if built."""
        if not self.target or not self.bin_name or not (self.repo_root / "Cargo.toml").is_file():
            return None
        try:
            target_dir = get_cargo_target_directory(self.repo_root)
        except (OSError, RuntimeError, ValueError):
            return None
        profile_dir = "release" if self.release else "debug"
        elf = target_dir / self.target / profile_dir / self.bin_name
        return elf if elf.is_file() else None

    def update_config(
        self,
        target: Optional[str] = None,
        package: Optional[str] = None,
        bin_name: Optional[str] = None,
        release: Optional[bool] = None,
        speed: Optional[int] = None,
        chip: Optional[str] = None,
        show_system_logs: Optional[bool] = None,
        enable_debug_server: Optional[bool] = None,
        halt_on_reset: Optional[bool] = None,
        gdb_core0_port: Optional[int] = None,
        gdb_core1_port: Optional[int] = None,
        rtt_bridge_port: Optional[int] = None,
        probe_selector: Optional[str] = None,
        trace_mode: Optional[str] = None,
        *,
        persist_session: bool = True,
    ) -> None:
        """Dynamically update runner configuration parameters and resolved paths."""
        trace_mode_changed = False
        with self._lock:
            if target:
                self.target = target
            if package:
                self.package = package
            if bin_name:
                self.bin_name = bin_name
            if release is not None:
                self.release = release
            if speed:
                self.speed = speed
            if chip:
                self.chip = chip
            if show_system_logs is not None:
                self.show_system_logs = show_system_logs
            if enable_debug_server is not None:
                self.enable_debug_server = bool(enable_debug_server)
            if halt_on_reset is not None:
                self.halt_on_reset = bool(halt_on_reset)
            if gdb_core0_port is not None:
                self.gdb_core0_port = int(gdb_core0_port)
            if gdb_core1_port is not None:
                self.gdb_core1_port = int(gdb_core1_port)
            if rtt_bridge_port is not None:
                self.rtt_bridge_port = int(rtt_bridge_port)
            if probe_selector is not None:
                self.probe_selector = probe_selector or None
            if trace_mode is not None:
                normalized_tm = normalize_trace_mode(trace_mode)
                trace_mode_changed = normalized_tm != self.trace_mode
                self.trace_mode = normalized_tm

            binary_identity_changed = (
                any(val is not None for val in (target, package, bin_name, release)) or trace_mode_changed
            )
            resolved_elf = self.resolve_current_elf_path()
            if resolved_elf is not None:
                self.elf_path = resolved_elf
            elif binary_identity_changed:
                self.elf_path = None
            if self.elf_path is not None:
                sync_active_target_elf_link(self.repo_root, self.elf_path)
            self._cached_metadata = None
            self._cached_metadata_elf = None

            self.sync_status.update(
                {
                    "target": self.target,
                    "package": self.package,
                    "bin": self.bin_name,
                    "profile": "release" if self.release else "debug",
                    "trace_mode": self.trace_mode.value,
                    "speed": self.speed,
                }
            )
        if trace_mode_changed:
            self.trace_store.clear()
        if persist_session:
            save_last_session(
                self.repo_root,
                {
                    "package": self.package,
                    "bin": self.bin_name,
                    "release": self.release,
                    "target": self.target,
                    "speed": self.speed,
                    "chip": self.chip,
                    "trace_mode": self.trace_mode.value,
                    "enable_debug_server": self.enable_debug_server,
                    "halt_on_reset": self.halt_on_reset,
                    "gdb_core0_port": self.gdb_core0_port,
                    "gdb_core1_port": self.gdb_core1_port,
                },
            )
        self.log(
            f"Updated configuration: package={self.package}, bin={self.bin_name}, target={self.target}, "
            f"profile={'release' if self.release else 'debug'}, trace_mode={self.trace_mode.value}, speed={self.speed}, "
            f"debug_server={self.enable_debug_server} (GDB :{self.gdb_core0_port}/:{self.gdb_core1_port})"
        )

    def sync(self) -> Dict[str, Any]:
        """Perform full synchronization: compile target, compile tools, flash, and connect RTT."""
        self.is_sync_cancelled = False
        with self._lock:
            self.sync_logs.clear()
            self.sync_status["console"] = []
        try:
            # 1. Compile target firmware
            if self.is_sync_cancelled:
                return dict(self.sync_status)
            self._update_sync("compiling_target", 1, 25, "Compiling target firmware...")
            self.build_target()

            # 2. Compile host tools
            if self.is_sync_cancelled:
                return dict(self.sync_status)
            self._update_sync(
                "compiling_host_tools",
                2,
                50,
                "Compiling host utilities (host_cli, host_fs, host_debug)...",
            )
            self.build_host_tools()

            # 3. Flash target device (release probe first)
            if self.is_sync_cancelled:
                return dict(self.sync_status)
            probe_was_busy = (self.rtt_process is not None and self.rtt_process.poll() is None) or (
                self.debug_process is not None and self.debug_process.poll() is None
            )
            self.stop_rtt_streaming()
            self.stop_debug_server()
            if probe_was_busy:
                time.sleep(PROBE_RELEASE_SETTLE_SEC)
            self._update_sync("flashing_target", 3, 75, "Flashing target device via probe-rs...")
            if not self.flash_target(use_cargo_run=False):
                raise RuntimeError(self.status_message or "Flash target failed")
            if not self.enable_debug_server:
                time.sleep(PROBE_RELEASE_SETTLE_SEC)

            head_commit = None
            try:
                proc = subprocess.run(
                    ["git", "rev-parse", "HEAD"],
                    cwd=str(self.repo_root),
                    capture_output=True,
                    text=True,
                    check=False,
                )
                if proc.returncode == 0:
                    head_commit = proc.stdout.strip()
            except Exception:
                pass
            self.flashed_commit = head_commit
            with self._lock:
                self.sync_status["flashed_commit"] = head_commit

            # 4. Start RTT streaming (and host_debug GDB server if enabled, with 1 single reset)
            if self.is_sync_cancelled:
                return dict(self.sync_status)
            self.trace_store.clear()
            step4_msg = (
                f"Starting probe-rs GDB server (:{self.gdb_core0_port}/:{self.gdb_core1_port}) & RTT bridge..."
                if self.enable_debug_server
                else "Starting RTT streaming channels..."
            )
            self._update_sync("starting_rtt", 4, 90, step4_msg)
            self.start_rtt_streaming(reset_on_attach=True)

            # Complete
            done_msg = (
                f"Target synchronized & GDB server active on :{self.gdb_core0_port}/:{self.gdb_core1_port}."
                if self.enable_debug_server
                else "Target synchronized & RTT connected successfully."
            )
            self._update_sync("complete", 4, 100, done_msg, active=False)
        except Exception as e:
            if self.is_sync_cancelled:
                return dict(self.sync_status)
            err_msg = f"Synchronization failed: {e}"
            self._update_sync("failed", 0, 0, err_msg, active=False, error=str(e))
            self.log(err_msg, level="ERROR")

        return dict(self.sync_status)

    def sync_async(self) -> None:
        """Trigger target synchronization asynchronously in a background thread."""
        with self._lock:
            if self._sync_thread and self._sync_thread.is_alive():
                return
            self.is_sync_cancelled = False
            self.sync_logs.clear()
            self.sync_status.update(
                {
                    "active": True,
                    "step": "starting",
                    "step_index": 0,
                    "total_steps": 4,
                    "progress": 5,
                    "message": "Starting target synchronization...",
                    "error": None,
                    "console": [],
                }
            )
            self._sync_thread = threading.Thread(target=self.sync, daemon=True)
            self._sync_thread.start()

    def log(self, message: str, level: str = "INFO", channel: str = "daemon") -> None:
        """Append a log message to the thread-safe log buffer and synchronize target `defmt` device timestamps."""
        parsed_device_ts: Optional[float] = None
        if channel == "target":
            plain = ANSI_ESCAPE_RE.sub("", message)
            match = DEFMT_DEVICE_TS_RE.match(plain)
            if match:
                parsed_device_ts = float(match.group(1))

        now_mono = time.monotonic()
        with self._lock:
            if parsed_device_ts is not None:
                self._last_device_ts_sec = parsed_device_ts
                self._last_device_ts_mono = now_mono
            self._log_seq += 1
            entry = {
                "id": self._log_seq,
                "timestamp": datetime.now(timezone.utc).isoformat(),
                "level": level,
                "channel": channel,
                "message": message,
            }
            self.logs.append(entry)

    def log_cli(self, message: str, is_input: bool = False) -> None:
        """Append a line to the CLI console output buffer and stream device-timestamped lines to active test runs."""
        device_ts = self.get_device_timestamp_str()
        with self._lock:
            self._cli_seq += 1
            entry = {
                "id": self._cli_seq,
                "timestamp": datetime.now(timezone.utc).isoformat(),
                "device_ts": device_ts,
                "is_input": is_input,
                "text": message,
            }
            self.cli_outputs.append(entry)

        stripped_msg = message.replace(ANSI_CLEAR_LINE_SEQ, "").strip()
        if stripped_msg and (is_input or not stripped_msg.startswith(RTT_SESSION_BANNER_PREFIXES)):
            self.test_manager.record_device_cli_line(message, is_input=is_input, device_ts=device_ts)

    def log_fs(self, message: str, is_input: bool = False) -> None:
        """Append a line to the FS console output buffer and stream device-timestamped lines to active test runs."""
        device_ts = self.get_device_timestamp_str()
        with self._lock:
            self._fs_seq += 1
            entry = {
                "id": self._fs_seq,
                "timestamp": datetime.now(timezone.utc).isoformat(),
                "device_ts": device_ts,
                "is_input": is_input,
                "text": message,
            }
            self.fs_outputs.append(entry)

        if message.strip():
            self.test_manager.record_device_cli_line(message, is_input=is_input, device_ts=device_ts)

    def set_mode(self, mode: str) -> None:
        """Switch active workstation mode among CLI, FS, TEST, and TRACE, managing probe ownership."""
        mode = mode.upper()
        if mode == self.active_mode:
            return
        prev_mode = self.active_mode
        with self._lock:
            self.active_mode = mode
            self.mode_switch_count += 1
        self._record_lifecycle("mode_switch", f"Switched target mode from {prev_mode} to {mode}")
        if mode == "FS":
            self.log("Switching to Filesystem mode (stopping RTT streaming to release probe)...", channel="system")
            self.stop_rtt_streaming()
            self.stop_debug_server()
            time.sleep(0.15)
        elif mode == "CLI":
            self.log("Switching to CLI mode (starting RTT streaming)...", channel="system")
            if prev_mode == "FS" or not self.is_connected:
                self.start_rtt_streaming()
        elif mode == "TEST":
            self.log("Switching to Test mode (on-device @device_test runner active)...", channel="system")
            if self.is_flashed and not self.is_connected:
                self.start_rtt_streaming()
        elif mode == "TRACE":
            self.log(f"Switching to Trace mode (trace_mode={self.trace_mode.value})...", channel="system")
            if self.is_flashed and not self.is_connected:
                self.start_rtt_streaming()

    def update_manual_prompt(self, payload: Dict[str, Any]) -> Dict[str, Any]:
        """Record a periodic manual test procedure & prompt heartbeat from `pytest`."""
        with self._lock:
            newly_activated = self.manual_prompt.apply_update(payload)
            snapshot = self.manual_prompt.to_dict()
        if newly_activated:
            self.log(
                f"[MANUAL TEST: {snapshot['test_name']}] Procedure: {snapshot['procedure']} | "
                f"Expected: {snapshot['prompt']}",
                channel="manual_test",
            )
            self.log_cli(f"[manual:{snapshot['test_name']}] {snapshot['procedure']} -> {snapshot['prompt']}")
        return snapshot

    def respond_manual_prompt(self, payload: Dict[str, Any]) -> Dict[str, Any]:
        """Record an operator decision (`yes`, `no`, `measurement`, or `cancel` with comment) from the UI."""
        with self._lock:
            snapshot = self.manual_prompt.submit_response(payload)
        resp = snapshot.get("response") or {}
        action = resp.get("action", "")
        detail = (
            f"value={resp.get('measurement')}"
            if action == "measurement"
            else (f"comment={resp.get('comment')}" if resp.get("comment") else "")
        )
        suffix = f" ({detail})" if detail else ""
        self.log(f"[MANUAL TEST RESPONSE] action={action}{suffix}", channel="manual_test")
        return snapshot

    def build_target(self) -> Path:
        """Compile the target embedded firmware binary into Cargo's single target_directory."""
        if not self.package or not self.bin_name or not self.target:
            raise RuntimeError(
                f"Target build configuration is incomplete (package={self.package}, bin={self.bin_name}, target={self.target})"
            )
        feature_args = get_cargo_feature_args(self.trace_mode)
        self.log(
            f"Building target firmware: package={self.package}, bin={self.bin_name}, target={self.target}, "
            f"release={self.release}, trace_mode={self.trace_mode.value}"
        )
        cmd = ["cargo", "build", "--target", self.target, "-p", self.package, "--bin", self.bin_name]
        if self.release:
            cmd.append("--release")
        if feature_args:
            cmd.extend(feature_args)

        ret, out = self._run_command_streaming(cmd, log_to_cli=False, log_channel="build")
        if ret != 0:
            err_msg = f"Failed to build target binary:\n{out}"
            self.log(err_msg, level="ERROR")
            raise RuntimeError(err_msg)

        target_dir = get_cargo_target_directory(self.repo_root)
        profile_dir = "release" if self.release else "debug"
        elf = target_dir / self.target / profile_dir / self.bin_name
        if not elf.exists():
            raise FileNotFoundError(f"Target ELF not found at {elf}")

        self.elf_path = elf
        sync_active_target_elf_link(self.repo_root, elf)
        self.log(f"Target binary built successfully: {elf}")
        return elf

    def build_host_tools(self) -> Tuple[Path, Path]:
        """Compile host utilities into Cargo's single target_directory via `cargo build-host-tools`."""
        self.log("Building host utilities via cargo build-host-tools")
        cmd = ["cargo", "build-host-tools"]
        if self.release:
            cmd.append("--release")

        env = os.environ.copy()
        # Ensure target flags from ARM don't contaminate host build
        env.pop("CARGO_BUILD_TARGET", None)
        env.pop("CARGO_ENCODED_RUSTFLAGS", None)

        ret, out = self._run_command_streaming(cmd, env=env, log_to_cli=False, log_channel="build")
        if ret != 0:
            err_msg = f"Failed to build host tools:\n{out}"
            self.log(err_msg, level="ERROR")
            raise RuntimeError(err_msg)

        target_dir = get_cargo_target_directory(self.repo_root)
        profile_dir = "release" if self.release else "debug"
        exe_suffix = ".exe" if os.name == "nt" else ""
        cli_cand = target_dir / profile_dir / f"host_cli{exe_suffix}"
        fs_cand = target_dir / profile_dir / f"host_fs{exe_suffix}"
        debug_cand = target_dir / profile_dir / f"host_debug{exe_suffix}"

        if not cli_cand.exists() or not fs_cand.exists():
            missing = []
            if not cli_cand.exists():
                missing.append(f"host_cli at {cli_cand}")
            if not fs_cand.exists():
                missing.append(f"host_fs at {fs_cand}")
            err = f"Host tools build completed but binaries not found: {', '.join(missing)}"
            self.log(err, level="ERROR")
            raise FileNotFoundError(err)

        self.host_cli_path = cli_cand
        self.host_fs_path = fs_cand
        if debug_cand.exists():
            self.host_debug_path = debug_cand
        self.log(f"Host tools ready: cli={cli_cand}, fs={fs_cand}, debug={self.host_debug_path}")
        return cli_cand, fs_cand

    def _run_command_streaming(
        self,
        cmd: List[str],
        env: Optional[Dict[str, str]] = None,
        timeout: Optional[float] = None,
        log_to_cli: bool = True,
        log_to_fs: bool = False,
        log_channel: str = "daemon",
    ) -> Tuple[int, str]:
        """Execute a subprocess command while streaming colorized output line-by-line in real time."""
        run_env = os.environ.copy()
        is_host_cargo_alias = len(cmd) >= 2 and cmd[1] in (
            "build-host-tools",
            "host-cli",
            "host-fs",
            "host-debug",
            "code-gen",
        )
        if "cargo" in cmd[0] and (
            is_host_cargo_alias or ("-p" in cmd and any(t in cmd for t in ("host_cli", "host_fs", "host_debug")))
        ):
            run_env.pop("CARGO_BUILD_TARGET", None)
            run_env.pop("CARGO_ENCODED_RUSTFLAGS", None)
        if env:
            run_env.update(env)

        # Force full colorization for cargo, compilers, and CLI tools
        run_env.setdefault("CARGO_TERM_COLOR", "always")
        run_env.setdefault("CLICOLOR_FORCE", "1")

        output_lines: List[str] = []
        try:
            proc = subprocess.Popen(
                cmd,
                cwd=self.repo_root,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                bufsize=1,
                env=run_env,
            )
            self._active_sync_process = proc
            if proc.stdout:
                for line in iter(proc.stdout.readline, ""):
                    stripped = line.rstrip("\r\n")
                    if stripped:
                        output_lines.append(stripped)
                        self.log_sync_console(stripped)
                        if log_to_cli:
                            self.log_cli(stripped)
                        if log_to_fs:
                            self.log_fs(stripped)
                        self.log(stripped, channel=log_channel)
            proc.wait(timeout=timeout)
            return proc.returncode, "\n".join(output_lines)
        except Exception as e:
            err = f"Execution error running {cmd[0]}: {e}"
            self.log_sync_console(err)
            if log_to_cli:
                self.log_cli(err)
            if log_to_fs:
                self.log_fs(err)
            self.log(err, level="ERROR", channel=log_channel)
            return 1, err
        finally:
            self._active_sync_process = None

    def run_host_cli(
        self,
        args: List[str],
        env: Optional[Dict[str, str]] = None,
        timeout: Optional[float] = None,
        use_cargo_run: bool = False,
    ) -> Tuple[int, str]:
        """Subcommand wrapper for host_cli, streaming colorized output in real time."""
        if not use_cargo_run and self.host_cli_path and self.host_cli_path.exists():
            cmd = [str(self.host_cli_path)] + args
        else:
            cmd = ["cargo", "host-cli"] + args
        return self._run_command_streaming(cmd, env=env, timeout=timeout, log_to_cli=True, log_channel="host_cli")

    def run_host_fs(
        self,
        args: List[str],
        env: Optional[Dict[str, str]] = None,
        timeout: Optional[float] = None,
    ) -> Tuple[int, str]:
        """Subcommand wrapper for host_fs via the `cargo host-fs` alias, streaming colorized output in real time."""
        cmd = ["cargo", "host-fs"] + args
        return self._run_command_streaming(
            cmd, env=env, timeout=timeout, log_to_cli=False, log_to_fs=True, log_channel="host_fs"
        )

    def run_probe_rs(
        self,
        args: List[str],
        env: Optional[Dict[str, str]] = None,
        timeout: Optional[float] = None,
    ) -> Tuple[int, str]:
        """Subcommand wrapper for probe-rs, streaming colorized output in real time."""
        probe_bin = shutil.which("probe-rs") or "probe-rs"
        cmd = [probe_bin] + args
        return self._run_command_streaming(cmd, env=env, timeout=timeout, log_to_cli=False, log_channel="probe-rs")

    def detect_chip(self) -> str:
        """Detect the target chip name from ELF metadata using host_cli.

        Explicitly raises RuntimeError if program metadata is missing or target is unsupported.
        """
        if self.chip:
            return self.chip

        if not self.elf_path or not self.elf_path.exists():
            raise RuntimeError(f"Target ELF '{self.elf_path}' not found; cannot detect program metadata")

        if self.host_cli_path and self.host_cli_path.exists():
            cmd = [str(self.host_cli_path), "--elf", str(self.elf_path), "--print-chip"]
        else:
            cmd = ["cargo", "host-cli", "--elf", str(self.elf_path), "--print-chip"]

        env = os.environ.copy()
        env.pop("CARGO_BUILD_TARGET", None)
        env.pop("CARGO_ENCODED_RUSTFLAGS", None)

        try:
            proc = subprocess.run(cmd, cwd=self.repo_root, capture_output=True, text=True, timeout=10, env=env)
            if proc.returncode == 0 and proc.stdout.strip():
                detected = proc.stdout.strip().splitlines()[-1].strip()
                if detected:
                    self.chip = detected
                    return self.chip
        except Exception as e:
            self.log(f"Chip detection execution failed: {e}", level="WARN")

        # Explicitly error out if target does not support host_daemon because it lacks program metadata (CR-6783fe41)
        err_msg = f"Target ELF '{self.elf_path}' does not contain required program metadata for host_daemon"
        self.log(err_msg, level="ERROR")
        raise RuntimeError(err_msg)

    def _should_retry_probe_operation(self, output: str) -> bool:
        """Query USB/transport interface status (`host_fs probe-status --json`) to decide whether a failed probe op is retryable."""
        parsed = parse_interface_status_output(output, default_interface=TargetInterfaceType.SWD_USB)
        if parsed is not None:
            return parsed.is_retryable
        host_fs_prefix = (
            [str(self.host_fs_path)] if self.host_fs_path and self.host_fs_path.exists() else ["cargo", "host-fs"]
        )
        status = query_target_interface_status(
            self.repo_root,
            interface=TargetInterfaceType.SWD_USB,
            host_fs_cmd_prefix=host_fs_prefix,
            probe_selector=self.probe_selector,
        )
        return status.availability != InterfaceAvailability.DISCONNECTED or status.is_retryable

    def flash_target(self, use_cargo_run: bool = True) -> bool:
        """Flash target device by invoking cargo run or probe-rs download."""
        if not self.elf_path or not self.elf_path.exists():
            err_msg = f"Cannot flash target: ELF not found at {self.elf_path}"
            self.log(err_msg, level="ERROR")
            raise FileNotFoundError(err_msg)

        self._invalidate_metadata_cache()
        self._reset_device_clock()

        if use_cargo_run:
            self.log(
                f"Flashing target via cargo run: package={self.package}, bin={self.bin_name}, target={self.target}"
            )
            cmd = ["cargo", "run", "--target", self.target, "-p", self.package, "--bin", self.bin_name]
            if self.release:
                cmd.append("--release")
            cmd.extend(["--", "--dump"])

            ret, out = 1, ""
            for attempt in range(MAX_PROBE_OPEN_RETRIES + 1):
                ret, out = self._run_command_streaming(cmd, log_to_cli=False, log_channel="flash")
                if ret == 0 or attempt >= MAX_PROBE_OPEN_RETRIES or not self._should_retry_probe_operation(out):
                    break
                self.log(
                    f"Transient USB probe timeout during flash (attempt {attempt + 1}); retrying...",
                    level="WARN",
                )
                time.sleep(PROBE_RETRY_BACKOFF_SEC)

            if ret == 0:
                self.is_flashed = True
                self.status_message = "Flashed & Running"
                self.log("Flash download succeeded via cargo run", level="INFO")
                return True
            else:
                self.log(f"cargo run flash failed (exit code {ret}):\n{out}", level="WARN")
                self.status_message = f"Flash failed: {out[:100]}"
                return False
        else:
            chip = self.detect_chip()
            cmd_args = ["download", "--chip", chip, "--speed", str(self.speed)]
            if self.probe_selector:
                cmd_args.extend(["--probe", self.probe_selector])
            cmd_args.append(str(self.elf_path))
            self.log(f"Flashing target via probe-rs: {' '.join(cmd_args)}")
            ret, out = 1, ""
            for attempt in range(MAX_PROBE_OPEN_RETRIES + 1):
                ret, out = self.run_probe_rs(cmd_args)
                if ret == 0 or attempt >= MAX_PROBE_OPEN_RETRIES or not self._should_retry_probe_operation(out):
                    break
                self.log(
                    f"Transient USB probe timeout during probe-rs download (attempt {attempt + 1}); retrying...",
                    level="WARN",
                )
                time.sleep(PROBE_RETRY_BACKOFF_SEC)

            if ret == 0:
                self.is_flashed = True
                self.status_message = "Flashed & Running"
                self.log("Flash download succeeded via probe-rs", level="INFO")
                return True
            else:
                self.log(f"probe-rs download failed (exit code {ret}):\n{out}", level="WARN")
                self.status_message = f"Flash failed: {out[:100]}"
                return False

    def reset_target(self) -> Dict[str, Any]:
        """Reset the target microcontroller via debug probe and reconnect RTT streaming."""
        with self._fs_lock:
            self._invalidate_metadata_cache()
            self._reset_device_clock()
            self._record_lifecycle("rebooting", "Rebooting target MCU over SWD & reconnecting RTT...")
            self.log("Resetting target microcontroller...", channel="system")
            self.stop_rtt_streaming()
            self.stop_debug_server()
            time.sleep(PROBE_RELEASE_SETTLE_SEC)

            if self.enable_debug_server:
                self.start_rtt_streaming(reset_on_attach=True)
                with self._lock:
                    self.reboot_count += 1
                self._record_lifecycle("rebooting", "Target reset & GDB server reconnected")
                return {
                    "success": True,
                    "probe_reset": True,
                    "message": "Target reset completed",
                    "output": "",
                    "debug": self.get_debug_status(),
                }

            chip_name = self.chip
            if not chip_name and self.elf_path and self.elf_path.exists():
                chip_name = self.detect_chip()
            if not chip_name:
                raise RuntimeError(
                    "Target chip could not be inferred for reset; configure target metadata or build target ELF first."
                )

            cmd_args = ["reset", "--chip", chip_name]
            if self.speed:
                cmd_args.extend(["--speed", str(self.speed)])
            if self.probe_selector:
                cmd_args.extend(["--probe", self.probe_selector])

            ret, out = 1, ""
            for attempt in range(MAX_PROBE_OPEN_RETRIES + 1):
                ret, out = self.run_probe_rs(cmd_args)
                if ret == 0 or attempt >= MAX_PROBE_OPEN_RETRIES or not self._should_retry_probe_operation(out):
                    break
                self.log(
                    f"Transient USB probe timeout during reset (attempt {attempt + 1}); retrying...",
                    level="WARN",
                )
                time.sleep(PROBE_RETRY_BACKOFF_SEC)

            reset_success = ret == 0
            if reset_success:
                self.log("Target reset successfully via probe-rs", channel="system")
            else:
                self.log(f"probe-rs reset returned {ret}: {out.strip()}", level="WARN")

            time.sleep(PROBE_RELEASE_SETTLE_SEC)
            self.start_rtt_streaming()
            with self._lock:
                self.reboot_count += 1
            msg = (
                "Target reboot completed & RTT reconnected"
                if reset_success
                else "Target reboot attempted & RTT reconnected"
            )
            self._record_lifecycle("rebooting", msg)
            return {
                "success": True,
                "probe_reset": reset_success,
                "message": "Target reset completed" if reset_success else "Target reset attempted and RTT reconnected",
                "output": out.strip() if out else "",
            }

    def reboot_target(self) -> Dict[str, Any]:
        """Reboot the attached target MCU over SWD and re-attach RTT streaming."""
        res = self.reset_target()
        res["message"] = (
            "Target reboot completed" if res.get("probe_reset") else "Target reboot attempted and RTT reconnected"
        )
        return res

    def query_target_metadata(
        self,
        elf_path: Optional[Path] = None,
        *,
        force_probe: bool = False,
    ) -> Dict[str, Any]:
        """Query `.program_metadata` off the target via `host_fs query_metadata` and validate against target ELF."""
        with self._lock:
            sync_active = bool(self._sync_thread and self._sync_thread.is_alive())
        if sync_active:
            return {
                "connected": False,
                "matches_target": False,
                "error": "Target synchronization is currently in progress",
            }

        with self._fs_lock:
            target_elf = elf_path or self.elf_path or self.resolve_current_elf_path()
            with self._lock:
                if (
                    not force_probe
                    and self.is_connected
                    and target_elf is not None
                    and self._cached_metadata is not None
                    and self._cached_metadata_elf == target_elf
                    and bool(self._cached_metadata.get("matches_target"))
                ):
                    return dict(self._cached_metadata)

            self._record_lifecycle("mode_switch", "Probing target .program_metadata over SWD...")
            if self.debug_server_active:
                bridge_addr = f"127.0.0.1:{self.rtt_bridge_port}"
                res_bridge = probe_attached_target_metadata(
                    self.repo_root,
                    preferred_elf=target_elf,
                    host_fs_path=self.host_fs_path,
                    gdb_bridge=bridge_addr,
                )
                if res_bridge.get("connected") and res_bridge.get("matches_target") and target_elf is not None:
                    with self._lock:
                        self._cached_metadata = dict(res_bridge)
                        self._cached_metadata_elf = target_elf
                self._record_lifecycle("idle", "Target metadata probe complete", mark_recent=False)
                return res_bridge

            rtt_was_running = (
                (self.rtt_process is not None and self.rtt_process.poll() is None)
                or (self._streaming_thread is not None and self._streaming_thread.is_alive())
                or self.debug_server_active
            )
            if rtt_was_running:
                self.stop_rtt_streaming()
                self.stop_debug_server()
                time.sleep(PROBE_RELEASE_SETTLE_SEC)

            try:
                res = probe_attached_target_metadata(
                    self.repo_root,
                    preferred_elf=target_elf,
                    host_fs_path=self.host_fs_path,
                )
                if res.get("connected") and res.get("matches_target") and target_elf is not None:
                    with self._lock:
                        self._cached_metadata = dict(res)
                        self._cached_metadata_elf = target_elf
                return res
            finally:
                if rtt_was_running and self.active_mode in ("CLI", "TEST", "TRACE"):
                    time.sleep(PROBE_RELEASE_SETTLE_SEC)
                    self.start_rtt_streaming(reset_on_attach=False)
                self._record_lifecycle("idle", "Target metadata probe complete", mark_recent=False)

    def _resolve_host_debug_cmd_prefix(self) -> List[str]:
        """Resolve compiled `host_debug` executable path or fall back to `cargo host-debug` alias."""
        if self.host_debug_path and self.host_debug_path.exists():
            return [str(self.host_debug_path)]
        try:
            target_dir = get_cargo_target_directory(self.repo_root)
            exe_suffix = ".exe" if os.name == "nt" else ""
            for prof in ("release", "debug") if self.release else ("debug", "release"):
                cand = target_dir / prof / f"host_debug{exe_suffix}"
                if cand.is_file():
                    self.host_debug_path = cand
                    return [str(cand)]
        except (OSError, RuntimeError, ValueError):
            pass
        return ["cargo", "host-debug"]

    def get_debug_status(self) -> Dict[str, Any]:
        """Return a thread-safe snapshot of the `host_debug` GDB server, active ELF (`debug` or `release`), and device."""
        with self._lock:
            info = dict(self.debug_server_info)
            effective_elf = self.elf_path or self.resolve_current_elf_path()
            effective_chip = str(info.get("chip") or self.chip or "").strip() or None
            profile_name = "release" if self.release else "debug"
            pkg_name = self.package
            bin_name = self.bin_name
            target_triple = self.target
        try:
            target_dir = get_cargo_target_directory(self.repo_root)
        except (OSError, RuntimeError, ValueError):
            target_dir = self.repo_root / "target"
        active_link = target_dir / ACTIVE_TARGET_ELF_FILENAME
        return {
            "enabled": self.enable_debug_server,
            "active": self.debug_server_active,
            "halt_on_reset": self.halt_on_reset,
            "gdb_core0_port": int(info.get("gdb_core0_port") or self.gdb_core0_port),
            "gdb_core1_port": info.get("gdb_core1_port", self.gdb_core1_port),
            "rtt_bridge_port": int(info.get("rtt_bridge_port") or self.rtt_bridge_port),
            "probe": self.probe_selector,
            "cores": int(info.get("cores") or 0),
            "halted_on_reset": bool(info.get("halted_on_reset", False)),
            "gdb_core0_connected": bool(info.get("gdb_core0_connected", False)),
            "gdb_core1_connected": bool(info.get("gdb_core1_connected", False)),
            "executable": str(effective_elf) if effective_elf else None,
            "active_elf_link": str(active_link),
            "device": format_debugger_device_name(effective_chip),
            "chip": effective_chip,
            "package": pkg_name,
            "bin": bin_name,
            "target": target_triple,
            "profile": profile_name,
        }

    def sync_active_symbols(self) -> Dict[str, Any]:
        """Ensure the active `debug` or `release` target ELF exists and refresh `target/active_target.elf`."""
        resolved = self.elf_path or self.resolve_current_elf_path()
        if resolved is None or not resolved.is_file():
            resolved = self.build_target()
        else:
            self.elf_path = resolved
            sync_active_target_elf_link(self.repo_root, resolved)
        return self.get_debug_status()

    def stop_debug_server(self) -> None:
        """Gracefully stop `host_debug` and release the USB debug probe session."""
        proc = self.debug_process
        if proc and proc.poll() is None:
            if proc.stdin:
                try:
                    proc.stdin.close()
                except OSError:
                    pass
                try:
                    proc.wait(timeout=RTT_STDIN_SHUTDOWN_TIMEOUT_SEC)
                except Exception:
                    pass
            if proc.poll() is None:
                try:
                    proc.terminate()
                    proc.wait(timeout=RTT_TERMINATE_TIMEOUT_SEC)
                except Exception:
                    try:
                        proc.kill()
                    except Exception:
                        pass
        self.debug_process = None
        dbg_thread = self._debug_thread
        if dbg_thread and dbg_thread.is_alive() and threading.current_thread() is not dbg_thread:
            dbg_thread.join(timeout=RTT_STREAM_JOIN_TIMEOUT_SEC)
        self._debug_thread = None
        with self._lock:
            self.debug_server_active = False
            self.debug_server_info = {}

    def start_debug_server(
        self,
        *,
        reset: bool = False,
        halt_on_reset: Optional[bool] = None,
    ) -> Dict[str, Any]:
        """Start `host_debug` (`probe-rs` dual-core GDB RSP + `:50000` RTT memory bridge)."""
        if self.elf_path is None:
            self.elf_path = self.resolve_current_elf_path()
        if not self.elf_path or not self.elf_path.exists():
            raise RuntimeError("Target ELF is not built yet; run sync before starting debug server.")
        sync_active_target_elf_link(self.repo_root, self.elf_path)

        self.stop_debug_server()
        effective_halt = self.halt_on_reset if halt_on_reset is None else bool(halt_on_reset)
        cmd = [
            *self._resolve_host_debug_cmd_prefix(),
            "--elf",
            str(self.elf_path),
            "--speed",
            str(self.speed),
            "--gdb-core0-port",
            str(self.gdb_core0_port),
            "--gdb-core1-port",
            str(self.gdb_core1_port),
            "--rtt-bridge-port",
            str(self.rtt_bridge_port),
            "--json-events",
            "--exit-on-stdin-eof",
        ]
        if self.chip:
            cmd.extend(["--chip", self.chip])
        if self.probe_selector:
            cmd.extend(["--probe", self.probe_selector])
        if effective_halt:
            cmd.append("--halt-on-reset")
        elif reset:
            cmd.append("--reset")

        run_env = os.environ.copy()
        run_env.pop("CARGO_BUILD_TARGET", None)
        run_env.pop("CARGO_ENCODED_RUSTFLAGS", None)

        ready_event = threading.Event()
        startup_error: List[str] = []

        def _debug_worker() -> None:
            try:
                self.debug_process = subprocess.Popen(
                    cmd,
                    cwd=str(self.repo_root),
                    stdin=subprocess.PIPE,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.STDOUT,
                    text=True,
                    bufsize=1,
                    env=run_env,
                )
                if self.debug_process.stdout:
                    for line in iter(self.debug_process.stdout.readline, ""):
                        stripped = line.rstrip("\r\n")
                        if not stripped:
                            continue
                        if stripped.startswith("{") and stripped.endswith("}"):
                            try:
                                ev = json.loads(stripped)
                            except json.JSONDecodeError:
                                ev = None
                            if isinstance(ev, dict) and "event" in ev:
                                ev_name = str(ev.get("event") or "")
                                if ev_name == "ready":
                                    with self._lock:
                                        self.debug_server_active = True
                                        self.debug_server_info = {
                                            "chip": ev.get("chip"),
                                            "cores": ev.get("cores", 1),
                                            "gdb_core0_port": ev.get("gdb_core0_port", self.gdb_core0_port),
                                            "gdb_core1_port": ev.get("gdb_core1_port"),
                                            "rtt_bridge_port": ev.get("rtt_bridge_port", self.rtt_bridge_port),
                                            "halted_on_reset": bool(ev.get("halted_on_reset", False)),
                                            "gdb_core0_connected": False,
                                            "gdb_core1_connected": False,
                                        }
                                    self.log(
                                        f"Target GDB server ready on Core 0 :{ev.get('gdb_core0_port')} / "
                                        f"Core 1 :{ev.get('gdb_core1_port')} (RTT bridge :{ev.get('rtt_bridge_port')})",
                                        channel="debug",
                                    )
                                    ready_event.set()
                                elif ev_name in ("gdb_connected", "gdb_disconnected"):
                                    core_idx = int(ev.get("core", 0))
                                    key = f"gdb_core{core_idx}_connected"
                                    with self._lock:
                                        self.debug_server_info[key] = bool(ev.get("connected", False))
                                    self.log(f"GDB client {ev_name} on Core {core_idx}", channel="debug")
                                elif ev_name == "stopped":
                                    with self._lock:
                                        self.debug_server_active = False
                                continue
                        startup_error.append(stripped)
                        self.log(stripped, level="WARN", channel="debug")
            except Exception as exc:
                startup_error.append(str(exc))
                self.log(f"host_debug error: {exc}", level="ERROR", channel="debug")
            finally:
                with self._lock:
                    self.debug_server_active = False
                ready_event.set()

        self._debug_thread = threading.Thread(target=_debug_worker, daemon=True)
        self._debug_thread.start()
        ready_event.wait(timeout=DEBUG_SERVER_STARTUP_TIMEOUT_SEC)
        if not self.debug_server_active:
            detail = "; ".join(startup_error[-3:]) if startup_error else "timed out waiting for host_debug ready event"
            self.stop_debug_server()
            raise RuntimeError(f"Failed to start target debug server: {detail}")
        return self.get_debug_status()

    def resolve_elf_breakpoints(self, specs: List[str]) -> Dict[str, int]:
        """Resolve symbol names and `file:line` DWARF specs against the active target ELF via `host_debug`."""
        if not specs:
            return {}
        if self.elf_path is None:
            self.elf_path = self.resolve_current_elf_path()
        if not self.elf_path or not self.elf_path.exists():
            raise RuntimeError("Target ELF is not built yet; run sync before resolving breakpoints.")
        cmd = [*self._resolve_host_debug_cmd_prefix(), "--elf", str(self.elf_path)]
        for spec in specs:
            cmd.extend(["--resolve", str(spec)])
        run_env = os.environ.copy()
        run_env.pop("CARGO_BUILD_TARGET", None)
        run_env.pop("CARGO_ENCODED_RUSTFLAGS", None)
        res = subprocess.run(cmd, cwd=str(self.repo_root), env=run_env, capture_output=True, text=True, check=False)
        if res.returncode != 0:
            raise RuntimeError(f"host_debug --resolve failed ({res.returncode}): {res.stderr or res.stdout}")
        raw = json.loads(res.stdout.strip().splitlines()[-1])
        return {str(spec): int(addr) for spec, addr in raw.items()}

    def enable_live_debug_server(
        self,
        *,
        gdb_core0_port: Optional[int] = None,
        gdb_core1_port: Optional[int] = None,
        rtt_bridge_port: Optional[int] = None,
        probe_selector: Optional[str] = None,
        halt_on_reset: Optional[bool] = None,
        reset: bool = False,
        resolve: Optional[List[str]] = None,
    ) -> Dict[str, Any]:
        """Enable and start `host_debug` on the running daemon, re-routing RTT over its `:50000` bridge."""
        with self._fs_lock:
            with self._lock:
                if self._saved_user_debug_ports is None:
                    self._saved_user_debug_ports = (
                        self.gdb_core0_port,
                        self.gdb_core1_port,
                        self.rtt_bridge_port,
                    )
            self.update_config(
                enable_debug_server=True,
                halt_on_reset=halt_on_reset,
                gdb_core0_port=gdb_core0_port,
                gdb_core1_port=gdb_core1_port,
                rtt_bridge_port=rtt_bridge_port,
                probe_selector=probe_selector,
                persist_session=False,
            )
            was_busy = (self.rtt_process is not None and self.rtt_process.poll() is None) or self.debug_server_active
            self.stop_rtt_streaming()
            self.stop_debug_server()
            if was_busy:
                time.sleep(PROBE_RELEASE_SETTLE_SEC)
            effective_reset = bool(reset or self.halt_on_reset)
            self.start_debug_server(reset=effective_reset, halt_on_reset=self.halt_on_reset)
            if self.host_cli_path is None:
                try:
                    target_dir = get_cargo_target_directory(self.repo_root)
                    prof = "release" if self.release else "debug"
                    exe_suffix = ".exe" if os.name == "nt" else ""
                    cand = target_dir / prof / f"host_cli{exe_suffix}"
                    if cand.is_file():
                        self.host_cli_path = cand
                except (OSError, RuntimeError, ValueError):
                    pass
            if self.active_mode in ("CLI", "TEST"):
                self.start_rtt_streaming(reset_on_attach=False)
            status = self.get_debug_status()
            if resolve:
                status["resolved"] = self.resolve_elf_breakpoints([str(spec) for spec in resolve])
            return status

    def disable_live_debug_server(self) -> Dict[str, Any]:
        """Stop `host_debug` and return RTT streaming to direct USB probe mode without resetting."""
        with self._fs_lock:
            with self._lock:
                restored_ports = self._saved_user_debug_ports
                self._saved_user_debug_ports = None
            self.update_config(
                enable_debug_server=False,
                halt_on_reset=False,
                gdb_core0_port=restored_ports[0] if restored_ports is not None else None,
                gdb_core1_port=restored_ports[1] if restored_ports is not None else None,
                rtt_bridge_port=restored_ports[2] if restored_ports is not None else None,
                persist_session=False,
            )
            was_active = (self.rtt_process is not None and self.rtt_process.poll() is None) or self.debug_server_active
            self.stop_rtt_streaming()
            self.stop_debug_server()
            if was_active:
                time.sleep(PROBE_RELEASE_SETTLE_SEC)
            if self.active_mode in ("CLI", "TEST") and self.elf_path and self.elf_path.exists():
                self.start_rtt_streaming(reset_on_attach=False)
            return self.get_debug_status()

    def stop_rtt_streaming(self) -> None:
        """Gracefully stop active RTT streaming process to release the USB debug probe cleanly."""
        proc = self.rtt_process
        if proc and proc.poll() is None:
            if proc.stdin:
                try:
                    proc.stdin.close()
                except OSError:
                    pass
                try:
                    proc.wait(timeout=RTT_STDIN_SHUTDOWN_TIMEOUT_SEC)
                except Exception:
                    pass
            if proc.poll() is None:
                try:
                    proc.terminate()
                    proc.wait(timeout=RTT_TERMINATE_TIMEOUT_SEC)
                except Exception:
                    try:
                        proc.kill()
                    except Exception:
                        pass
        self.rtt_process = None
        stream_thread = self._streaming_thread
        if stream_thread and stream_thread.is_alive() and threading.current_thread() is not stream_thread:
            stream_thread.join(timeout=RTT_STREAM_JOIN_TIMEOUT_SEC)
        self._streaming_thread = None
        self.is_connected = False
        with self._lock:
            self._json_events_active = False
            self._last_shell_idle_epoch = None

    def start_rtt_streaming(
        self,
        *,
        reset_on_attach: bool = True,
        _retry_on_boot_timeout: bool = True,
    ) -> None:
        """Start streaming RTT logs and CLI interactions in a background thread."""
        if self.elf_path is None:
            self.elf_path = self.resolve_current_elf_path()
        if not self.elf_path or not self.elf_path.exists():
            return
        if self.host_cli_path is None or not self.host_cli_path.exists():
            try:
                target_dir = get_cargo_target_directory(self.repo_root)
                prof = "release" if self.release else "debug"
                exe_suffix = ".exe" if os.name == "nt" else ""
                cand = target_dir / prof / f"host_cli{exe_suffix}"
                if cand.is_file():
                    self.host_cli_path = cand
            except (OSError, RuntimeError, ValueError):
                return
        if not self.host_cli_path or not self.host_cli_path.exists():
            return

        self.stop_rtt_streaming()
        if not self.enable_debug_server and self.debug_server_active:
            self.stop_debug_server()
            time.sleep(PROBE_RELEASE_SETTLE_SEC)
        self._reset_device_clock()

        halted_for_gdb = False
        if self.enable_debug_server:
            if not self.debug_server_active:
                dbg_info = self.start_debug_server(
                    reset=reset_on_attach,
                    halt_on_reset=self.halt_on_reset if reset_on_attach else False,
                )
            else:
                dbg_info = self.get_debug_status()
            halted_for_gdb = bool(dbg_info.get("halted_on_reset", False))
            bridge_port = int(dbg_info.get("rtt_bridge_port") or self.rtt_bridge_port)
            cmd = [
                str(self.host_cli_path),
                "--elf",
                str(self.elf_path),
                "--gdb-bridge",
                f"127.0.0.1:{bridge_port}",
                "--channel",
                "both",
                "--json-events",
            ]
        else:
            cmd = [
                str(self.host_cli_path),
                "--elf",
                str(self.elf_path),
                "--speed",
                str(self.speed),
                "--channel",
                "both",
                "--json-events",
            ]
            if reset_on_attach:
                cmd.append("--reset")

        started_event = threading.Event()
        boot_ready_event = threading.Event()
        session_ready = False
        has_cli_channel = True

        def _handle_json_event(ev_obj: Dict[str, Any]) -> None:
            nonlocal session_ready, has_cli_channel
            ev_type = str(ev_obj.get("event") or "")
            dev_ts_val = ev_obj.get("device_ts")
            if isinstance(dev_ts_val, (int, float)) and float(dev_ts_val) >= 0.0:
                with self._lock:
                    self._last_device_ts_sec = float(dev_ts_val)
                    self._last_device_ts_mono = time.monotonic()

            if ev_type == "session_started":
                if not session_ready:
                    self._reset_device_clock()
                    with self._lock:
                        self._last_shell_idle_epoch = None
                with self._lock:
                    self._json_events_active = True
                session_ready = True
                has_cli_channel = bool(ev_obj.get("cli_up", True))
                self.is_connected = True
                self.log("RTT streaming process connected")
                started_event.set()
                if not has_cli_channel and not bool(ev_obj.get("defmt", True)):
                    boot_ready_event.set()
            elif ev_type == "defmt":
                level_str = str(ev_obj.get("level") or "INFO").upper()
                formatted = str(ev_obj.get("formatted") or ev_obj.get("message") or "")
                if formatted:
                    self.log(formatted, level=level_str, channel="target")
                if not has_cli_channel:
                    boot_ready_event.set()
            elif ev_type == "cli_output":
                text_line = str(ev_obj.get("text") or "")
                if text_line and not text_line.strip().startswith(RTT_SESSION_BANNER_PREFIXES):
                    self.log_cli(text_line)
            elif ev_type == "shell_idle":
                raw_epoch = ev_obj.get("epoch")
                epoch_val = int(raw_epoch) if isinstance(raw_epoch, int) and not isinstance(raw_epoch, bool) else None
                with self._lock:
                    if epoch_val is None or epoch_val != self._last_shell_idle_epoch:
                        self._last_shell_idle_epoch = epoch_val
                        self._cli_clear_seq += 1
                boot_ready_event.set()
            elif ev_type == "command_completed":
                ok = bool(ev_obj.get("success", True))
                err_msg = ev_obj.get("error")
                with self._lock:
                    self._cmd_done_seq += 1
                    self._cmd_done_clear_floor = self._cli_clear_seq
                    self._last_cmd_error = str(err_msg) if (not ok and err_msg) else (None if ok else "Command failed")
            elif ev_type == "trace_event":
                raw_ts_us = ev_obj.get("ts_us")
                if isinstance(raw_ts_us, (int, float)) and not isinstance(raw_ts_us, bool) and float(raw_ts_us) >= 0.0:
                    with self._lock:
                        self._last_device_ts_sec = float(raw_ts_us) / 1_000_000.0
                        self._last_device_ts_mono = time.monotonic()
                self.trace_store.ingest_event(ev_obj)
            elif ev_type == "session_disconnected":
                self.is_connected = False

        def _stream_worker() -> None:
            nonlocal session_ready
            try:
                self.rtt_process = subprocess.Popen(
                    cmd,
                    stdin=subprocess.PIPE,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.STDOUT,
                    text=True,
                    bufsize=1,
                )
                if self.rtt_process.stdout:
                    for line in iter(self.rtt_process.stdout.readline, ""):
                        stripped = line.rstrip("\r\n")
                        if not stripped:
                            continue
                        if stripped.startswith("{") and stripped.endswith("}"):
                            try:
                                parsed_ev = json.loads(stripped)
                            except json.JSONDecodeError:
                                parsed_ev = None
                            if isinstance(parsed_ev, dict) and "event" in parsed_ev:
                                _handle_json_event(parsed_ev)
                                continue

                        if "RTT Session Started" in stripped or "Interactive RTT console active" in stripped:
                            if not session_ready:
                                self._reset_device_clock()
                            session_ready = True
                            self.is_connected = True
                            self.log("RTT streaming process connected")
                            started_event.set()
                        if "Type 'help' to print usage." in stripped or "MPU stack guard configured" in stripped:
                            boot_ready_event.set()
                        if (
                            "INFO" in stripped
                            or "WARN" in stripped
                            or "ERROR" in stripped
                            or "DEBUG" in stripped
                            or "TRACE" in stripped
                            or "Draining buffered defmt logs" in stripped
                        ):
                            self.log(stripped, channel="target")
                        elif stripped.replace(ANSI_CLEAR_LINE_SEQ, "").strip() == "":
                            pass
                        else:
                            unescaped = stripped.replace(ANSI_CLEAR_LINE_SEQ, "")
                            if unescaped.lstrip().startswith(SHELL_PROMPT_PREFIX):
                                with self._lock:
                                    self._cli_clear_seq += 1
                                boot_ready_event.set()
                            cleaned_line = unescaped.removeprefix("shell> ").removeprefix(SHELL_PROMPT_PREFIX)
                            if cleaned_line:
                                self.log_cli(cleaned_line)
            except Exception as e:
                started_event.set()
                boot_ready_event.set()
                self.log(f"RTT stream error: {e}", level="WARN")
            finally:
                started_event.set()
                boot_ready_event.set()
                self.is_connected = False
                self.log("RTT stream disconnected", level="INFO")

        self._streaming_thread = threading.Thread(target=_stream_worker, daemon=True)
        self._streaming_thread.start()
        if halted_for_gdb:
            return
        started_event.wait(timeout=RTT_STARTUP_EVENT_TIMEOUT_SEC)
        if session_ready and self.rtt_process is not None and self.rtt_process.poll() is None:
            boot_ok = boot_ready_event.wait(timeout=RTT_BOOT_READY_TIMEOUT_SEC)
            if (
                not boot_ok
                and _retry_on_boot_timeout
                and self.rtt_process is not None
                and self.rtt_process.poll() is None
            ):
                self.log(
                    "Target boot/idle signal timed out after RTT attach; issuing recovery reset...",
                    level="WARN",
                )
                self.stop_rtt_streaming()
                if self.enable_debug_server:
                    self.stop_debug_server()
                time.sleep(PROBE_RELEASE_SETTLE_SEC)
                self.start_rtt_streaming(reset_on_attach=True, _retry_on_boot_timeout=False)
                return
            time.sleep(RTT_CONSOLE_READY_SETTLE_SEC)

    def _collect_cli_output(
        self,
        clean_cmd: str,
        raw_cmd: str,
        start_cli_seq: int,
        start_log_seq: int,
        start_clear_seq: int,
        start_done_seq: int,
        timeout: float,
    ) -> str:
        """Wait for and collect target CLI output lines emitted after a command dispatch."""
        deadline = time.monotonic() + max(timeout, CLI_POLL_INTERVAL_SEC)
        last_output_time: Optional[float] = None
        done_seen_time: Optional[float] = None
        prev_count = 0
        new_cli: List[str] = []
        new_logs: List[str] = []
        prompt_seen = False
        post_done_prompt_seen = False
        json_events_active = False
        structured_done = False
        structured_err: Optional[str] = None

        while time.monotonic() < deadline:
            now = time.monotonic()
            with self._lock:
                new_cli = [
                    str(entry["text"])
                    for entry in self.cli_outputs
                    if entry["id"] > start_cli_seq and not entry.get("is_input")
                ]
                new_logs = [str(entry["message"]) for entry in self.logs if entry["id"] > start_log_seq]
                prompt_seen = self._cli_clear_seq > start_clear_seq or any(
                    ANSI_CLEAR_LINE_SEQ in text for text in new_cli
                )
                structured_done = self._cmd_done_seq > start_done_seq
                post_done_prompt_seen = structured_done and (self._cli_clear_seq > self._cmd_done_clear_floor)
                json_events_active = self._json_events_active
                structured_err = self._last_cmd_error if structured_done else None

            meaningful_count = sum(
                1
                for text in new_cli
                if text.replace(ANSI_CLEAR_LINE_SEQ, "").strip() not in ("", "shell>")
                and not text.replace(ANSI_CLEAR_LINE_SEQ, "").strip().startswith(RTT_SESSION_BANNER_PREFIXES)
            )
            if meaningful_count > prev_count:
                prev_count = meaningful_count
                last_output_time = now

            cmd_done_log = structured_done or any(
                "command execution succeeded" in msg or "command execution failed" in msg for msg in new_logs
            )
            effective_prompt_seen = (
                (post_done_prompt_seen or (prompt_seen and (last_output_time is not None or not clean_cmd)))
                if json_events_active
                else prompt_seen
            )
            if (effective_prompt_seen or cmd_done_log) and done_seen_time is None:
                done_seen_time = now

            if done_seen_time is not None:
                if structured_done and prompt_seen:
                    break
                anchor = max(done_seen_time, last_output_time or done_seen_time)
                if (now - anchor) >= CLI_COMPLETION_SETTLE_SEC:
                    break
            elif last_output_time is not None:
                quiet_settle = min(
                    CLI_QUIET_SETTLE_MAX_SEC, max(CLI_QUIET_SETTLE_SEC, timeout * CLI_QUIET_TIMEOUT_RATIO)
                )
                if (now - last_output_time) >= quiet_settle:
                    break

            if self.rtt_process is None or self.rtt_process.poll() is not None:
                break

            time.sleep(CLI_POLL_INTERVAL_SEC)

        cleaned_lines: List[str] = []
        for text in new_cli:
            line = (
                text.replace(ANSI_CLEAR_LINE_SEQ, "").rstrip().removeprefix("shell> ").removeprefix(SHELL_PROMPT_PREFIX)
            )
            if line.strip() and not line.strip().startswith(RTT_SESSION_BANNER_PREFIXES):
                cleaned_lines.append(line)
        if cleaned_lines:
            return "\n".join(cleaned_lines)

        if structured_err is not None or any("command execution failed" in msg for msg in new_logs):
            return f"Command failed: {clean_cmd}"

        final_prompt_ok = (post_done_prompt_seen or not clean_cmd) if json_events_active else prompt_seen
        if final_prompt_ok or structured_done or any("command execution succeeded" in msg for msg in new_logs):
            return f"Executed: {clean_cmd}"

        return f"Dispatched: {raw_cmd}"

    def execute_cli(
        self,
        command: str,
        wait_for_output: bool = False,
        timeout: float = DEFAULT_CLI_OUTPUT_TIMEOUT_SEC,
    ) -> str:
        """Send command to the interactive CLI shell, optionally waiting for target response lines."""
        if wait_for_output:
            with self._fs_lock:
                return self._execute_cli_unlocked(command, wait_for_output=True, timeout=timeout)
        return self._execute_cli_unlocked(command, wait_for_output=False, timeout=timeout)

    def _execute_cli_unlocked(
        self,
        command: str,
        *,
        wait_for_output: bool,
        timeout: float,
    ) -> str:
        """Dispatch a CLI command; serialized via `self._fs_lock` when `wait_for_output=True`."""
        clean_cmd = command.strip()
        if clean_cmd:
            with self._lock:
                self.command_history.append(clean_cmd)
                self.last_command = f"shell> {clean_cmd}"
        self.log_cli(f"shell> {command}", is_input=True)
        with self._lock:
            start_cli_seq = self._cli_seq
            start_log_seq = self._log_seq
            start_clear_seq = self._cli_clear_seq
            start_done_seq = self._cmd_done_seq

        # If RTT process is actively connected and taking stdin:
        if self.rtt_process and self.rtt_process.stdin and self.rtt_process.poll() is None:
            try:
                self.rtt_process.stdin.write(command + "\n")
                self.rtt_process.stdin.flush()
            except OSError as e:
                err = f"Failed to send to RTT: {e}"
                self.log_cli(err)
                return err

            if wait_for_output:
                return self._collect_cli_output(
                    clean_cmd=clean_cmd,
                    raw_cmd=command,
                    start_cli_seq=start_cli_seq,
                    start_log_seq=start_log_seq,
                    start_clear_seq=start_clear_seq,
                    start_done_seq=start_done_seq,
                    timeout=timeout,
                )
            return f"Dispatched: {command}"

        err = "Target CLI is not available (RTT console is not connected). Please synchronize target."
        self.log_cli(err)
        self.log(err, level="ERROR", channel="host_cli")
        return f"Error: {err}"

    def execute_fs(self, command_or_subcommand: str, args: Optional[List[str]] = None) -> str:
        """Execute host_fs utility command by invoking cargo run."""
        clean_subcmd = command_or_subcommand.strip()
        if clean_subcmd:
            full_fs_cmd = f"fs {clean_subcmd}" + (f" {' '.join(args)}" if args else "")
            with self._lock:
                self.command_history.append(full_fs_cmd)
                self.last_command = f"fs> {clean_subcmd}" + (f" {' '.join(args)}" if args else "")
        with self._lock:
            sync_active = bool(self._sync_thread and self._sync_thread.is_alive())
        if sync_active:
            err = "Target synchronization is currently in progress. Please wait for sync to complete."
            self.log_fs(err)
            return f"Error: {err}"

        with self._fs_lock:
            args = args or []
            tokens = command_or_subcommand.strip().split()
            if tokens:
                subcmd = tokens[0]
                extra_args = tokens[1:] + args
            else:
                subcmd = "ls"
                extra_args = args

            self.log_fs(f"fs> {subcmd} {' '.join(extra_args)}", is_input=True)
            self._record_lifecycle("mode_switch", f"Probe switched to host_fs ({subcmd})")

            # Ensure probe is available by pausing RTT streaming and debug server if running
            rtt_was_running = (
                (self.rtt_process is not None and self.rtt_process.poll() is None)
                or (self._streaming_thread is not None and self._streaming_thread.is_alive())
                or self.debug_server_active
            )
            if rtt_was_running:
                self.stop_rtt_streaming()
                self.stop_debug_server()
                time.sleep(PROBE_RELEASE_SETTLE_SEC)

            fs_args: List[str] = []
            if self.elf_path and self.elf_path.exists():
                fs_args.extend(["--elf", str(self.elf_path)])
            fs_args.append(subcmd)
            fs_args.extend(extra_args)

            try:
                ret, out = 1, ""
                for attempt in range(MAX_PROBE_OPEN_RETRIES + 1):
                    ret, out = self.run_host_fs(fs_args)
                    if ret == 0 or attempt >= MAX_PROBE_OPEN_RETRIES or not self._should_retry_probe_operation(out):
                        break
                    self.log(
                        f"Transient USB probe contention during host_fs '{subcmd}' (attempt {attempt + 1}); retrying...",
                        level="WARN",
                    )
                    self.stop_rtt_streaming()
                    self.stop_debug_server()
                    time.sleep(PROBE_RETRY_BACKOFF_SEC)
                out = out.strip() or f"Completed {subcmd} (exit code {ret})"
                return out
            finally:
                if rtt_was_running and self.active_mode in ("CLI", "TEST", "TRACE"):
                    time.sleep(PROBE_RELEASE_SETTLE_SEC)
                    self.start_rtt_streaming(reset_on_attach=False)
                self._record_lifecycle("idle", f"Completed host_fs ({subcmd})", mark_recent=False)

    def get_trace_events(self, after_seq: int = 0, limit: int = 10_000) -> Dict[str, Any]:
        """Return incremental live trace slices/counters, track summaries, and active viewer metadata."""
        payload = self.trace_store.get_events(after_seq=after_seq, limit=limit)
        payload["trace_mode"] = self.trace_mode.value
        payload["external_viewer"] = get_external_trace_viewer(self.trace_mode).value
        payload["chip"] = self.chip
        payload["bin"] = self.bin_name
        return payload

    def control_trace_session(self, action: str) -> Dict[str, Any]:
        """Execute a live trace playback/session control action (`'clear'`, `'pause'`, `'resume'`)."""
        cleaned = str(action or "").strip().lower()
        if cleaned == "clear":
            self.trace_store.clear()
        elif cleaned == "pause":
            self.trace_store.set_recording(False)
        elif cleaned == "resume":
            self.trace_store.set_recording(True)
        else:
            raise ValueError(f"Unsupported trace control action '{action}'. Expected: clear, pause, resume.")
        return {
            "success": True,
            "action": cleaned,
            "recording": self.trace_store.recording,
            "trace_mode": self.trace_mode.value,
        }

    def export_trace_session(self, fmt: str, path: Optional[str] = None) -> Dict[str, Any]:
        """Export the active buffered trace session to disk in `'csv'` or `'perfetto'` format."""
        stem = f"{self.bin_name or 'target'}_{self.trace_mode.value.lower()}"
        return self.trace_store.export_to_disk(
            self.repo_root,
            fmt,
            path,
            chip=self.chip,
            default_stem=stem,
        )

    def open_trace_in_rerun(
        self,
        *,
        spawn: bool = True,
        save_rrd_path: Optional[str] = None,
    ) -> Dict[str, Any]:
        """Stream the active buffered trace session directly into the Rerun Viewer via `rerun-sdk`."""
        rrd_path: Optional[Path] = None
        if save_rrd_path and str(save_rrd_path).strip():
            raw = Path(str(save_rrd_path).strip())
            rrd_path = raw if raw.is_absolute() else (self.repo_root / raw)
        app_id = f"{self.bin_name or 'xerxes'}_{self.trace_mode.value.lower()}"
        return open_trace_in_rerun(
            self.trace_store.snapshot_all(),
            app_id=app_id,
            spawn=spawn,
            save_rrd_path=rrd_path,
        )

    def get_status(self) -> Dict[str, Any]:
        """Return daemon status dictionary."""
        return {
            "status": "running",
            "target": self.target,
            "package": self.package,
            "bin": self.bin_name,
            "profile": "release" if self.release else "debug",
            "trace_mode": self.trace_mode.value,
            "external_viewer": get_external_trace_viewer(self.trace_mode).value,
            "elf": str(self.elf_path) if self.elf_path else None,
            "chip": self.chip,
            "speed": self.speed,
            "connected": self.is_connected,
            "flashed": self.is_flashed,
            "mode": self.active_mode,
            "message": self.status_message,
            "manual_prompt": self.manual_prompt.to_dict(),
            "debug_server": self.get_debug_status(),
        }

    def stop(self) -> None:
        """Terminate active RTT processes, sync tasks, test runs, and release hardware debug probes."""
        self.is_sync_cancelled = True
        self._invalidate_metadata_cache()
        self.test_manager.cancel_run(silent=True)
        self.stop_rtt_streaming()
        self.stop_debug_server()
        if self._active_sync_process and self._active_sync_process.poll() is None:
            try:
                self._active_sync_process.terminate()
                self._active_sync_process.wait(timeout=2)
            except Exception:
                try:
                    self._active_sync_process.kill()
                except Exception:
                    pass
            self._active_sync_process = None

        if self.rtt_process and self.rtt_process.poll() is None:
            try:
                self.rtt_process.terminate()
                self.rtt_process.wait(timeout=2)
            except Exception:
                try:
                    self.rtt_process.kill()
                except Exception:
                    pass
            self.rtt_process = None

        self.is_connected = False
        self.log("Daemon stopped")
