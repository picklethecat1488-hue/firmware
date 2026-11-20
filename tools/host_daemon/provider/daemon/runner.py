"""Runner for building targets, flashing devices, streaming RTT, and executing CLI/FS tools."""

from collections import deque
from datetime import datetime, timezone
import os
from pathlib import Path
import shutil
import subprocess
import threading
import time
from typing import Any, Deque, Dict, List, Optional, Tuple

from host_daemon.model.manual_prompt import ManualPromptState
from host_daemon.model.metadata import get_cargo_target_directory, probe_attached_target_metadata
from host_daemon.provider.daemon.config import discover_target_packages, save_last_session
from host_daemon.provider.daemon.test_manager import DeviceTestRunnerManager

RTT_STDIN_SHUTDOWN_TIMEOUT_SEC: float = 2.5
RTT_TERMINATE_TIMEOUT_SEC: float = 2.0
RTT_STREAM_JOIN_TIMEOUT_SEC: float = 1.0
RTT_STARTUP_EVENT_TIMEOUT_SEC: float = 4.0
RTT_CONSOLE_READY_SETTLE_SEC: float = 0.20
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
RTT_SESSION_BANNER_PREFIXES: Tuple[str, ...] = (
    "RTT connected. Active channels:",
    "- Up Channel:",
    "- Down Channel:",
    "RTT Session Started",
    "Interactive RTT console active.",
    "Type 'help' to print usage.",
)


def _is_transient_probe_error(output: str) -> bool:
    """Return True if subprocess output indicates a transient USB debug probe lock or timeout."""
    lowered = output.lower()
    return (
        "bulk write timed out" in lowered
        or "bulk read timed out" in lowered
        or "failed to open probe" in lowered
        or "usb(custom" in lowered
        or "no debug probes connected" in lowered
        or "no debug probes were found" in lowered
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

        self.elf_path: Optional[Path] = None
        self.host_cli_path: Optional[Path] = None
        self.host_fs_path: Optional[Path] = None
        self.manual_prompt = ManualPromptState()
        self.test_manager = DeviceTestRunnerManager(self)

        self.active_mode: str = "CLI"
        self._lock = threading.Lock()
        self._fs_lock = threading.Lock()
        self.logs: Deque[Dict[str, Any]] = deque(maxlen=5000)
        self.cli_outputs: Deque[Dict[str, Any]] = deque(maxlen=5000)
        self.fs_outputs: Deque[Dict[str, Any]] = deque(maxlen=5000)
        self._log_seq: int = 0
        self._cli_seq: int = 0
        self._cli_clear_seq: int = 0
        self._fs_seq: int = 0
        self.command_history: List[str] = []

        self.rtt_process: Optional[subprocess.Popen] = None
        self._streaming_thread: Optional[threading.Thread] = None
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
    ) -> None:
        """Dynamically update runner configuration parameters and resolved paths."""
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

            resolved_elf = self.resolve_current_elf_path()
            if resolved_elf is not None:
                self.elf_path = resolved_elf

            self.sync_status.update(
                {
                    "target": self.target,
                    "package": self.package,
                    "bin": self.bin_name,
                    "profile": "release" if self.release else "debug",
                    "speed": self.speed,
                }
            )
        save_last_session(
            self.repo_root,
            {
                "package": self.package,
                "bin": self.bin_name,
                "release": self.release,
                "target": self.target,
                "speed": self.speed,
                "chip": self.chip,
            },
        )
        self.log(
            f"Updated configuration: package={self.package}, bin={self.bin_name}, target={self.target}, profile={'release' if self.release else 'debug'}, speed={self.speed}"
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
            self._update_sync("compiling_host_tools", 2, 50, "Compiling host utilities (host_cli, host_fs)...")
            self.build_host_tools()

            # 3. Flash target device (release probe first)
            if self.is_sync_cancelled:
                return dict(self.sync_status)
            rtt_was_running = self.rtt_process is not None and self.rtt_process.poll() is None
            self.stop_rtt_streaming()
            if rtt_was_running:
                time.sleep(PROBE_RELEASE_SETTLE_SEC)
            self._update_sync("flashing_target", 3, 75, "Flashing target device via probe-rs...")
            if not self.flash_target():
                raise RuntimeError(self.status_message or "Flash target failed")

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

            # 4. Start RTT streaming
            if self.is_sync_cancelled:
                return dict(self.sync_status)
            self._update_sync("starting_rtt", 4, 90, "Starting RTT streaming channels...")
            self.start_rtt_streaming()

            # Complete
            self._update_sync("complete", 4, 100, "Target synchronized & RTT connected successfully.", active=False)
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
        """Append a log message to the thread-safe log buffer."""
        with self._lock:
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
        """Append a line to the CLI console output buffer."""
        with self._lock:
            self._cli_seq += 1
            entry = {
                "id": self._cli_seq,
                "timestamp": datetime.now(timezone.utc).isoformat(),
                "is_input": is_input,
                "text": message,
            }
            self.cli_outputs.append(entry)

    def log_fs(self, message: str, is_input: bool = False) -> None:
        """Append a line to the FS console output buffer."""
        with self._lock:
            self._fs_seq += 1
            entry = {
                "id": self._fs_seq,
                "timestamp": datetime.now(timezone.utc).isoformat(),
                "is_input": is_input,
                "text": message,
            }
            self.fs_outputs.append(entry)

    def set_mode(self, mode: str) -> None:
        """Switch active workstation mode among CLI, FS, and TEST, managing probe ownership."""
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
            time.sleep(0.15)
        elif mode == "CLI":
            self.log("Switching to CLI mode (starting RTT streaming)...", channel="system")
            self.start_rtt_streaming()
        elif mode == "TEST":
            self.log("Switching to Test mode (on-device @device_test runner active)...", channel="system")
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
        self.log(
            f"Building target firmware: package={self.package}, bin={self.bin_name}, target={self.target}, release={self.release}"
        )
        cmd = ["cargo", "build", "--target", self.target, "-p", self.package, "--bin", self.bin_name]
        if self.release:
            cmd.append("--release")

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
        self.log(f"Target binary built successfully: {elf}")
        return elf

    def build_host_tools(self) -> Tuple[Path, Path]:
        """Compile host CLI and FS tools into Cargo's single target_directory."""
        self.log("Building host utilities: host_cli and host_fs")
        cmd = ["cargo", "build", "-p", "host_cli", "-p", "host_fs"]
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
        self.log(f"Host tools ready: cli={cli_cand}, fs={fs_cand}")
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
        if "cargo" in cmd[0] and "-p" in cmd and ("host_cli" in cmd or "host_fs" in cmd):
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
            cmd = ["cargo", "run", "-q", "-p", "host_cli", "--"] + args
        return self._run_command_streaming(cmd, env=env, timeout=timeout, log_to_cli=True, log_channel="host_cli")

    def run_host_fs(
        self,
        args: List[str],
        env: Optional[Dict[str, str]] = None,
        timeout: Optional[float] = None,
    ) -> Tuple[int, str]:
        """Subcommand wrapper for host_fs by invoking cargo run, streaming colorized output in real time."""
        cmd = ["cargo", "run", "-q", "-p", "host_fs", "--"] + args
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
            cmd = ["cargo", "run", "-q", "-p", "host_cli", "--", "--elf", str(self.elf_path), "--print-chip"]

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

    def flash_target(self, use_cargo_run: bool = True) -> bool:
        """Flash target device by invoking cargo run or probe-rs download."""
        if not self.elf_path or not self.elf_path.exists():
            err_msg = f"Cannot flash target: ELF not found at {self.elf_path}"
            self.log(err_msg, level="ERROR")
            raise FileNotFoundError(err_msg)

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
                if ret == 0 or not _is_transient_probe_error(out) or attempt >= MAX_PROBE_OPEN_RETRIES:
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
            cmd_args = ["download", "--chip", chip, "--speed", str(self.speed), str(self.elf_path)]
            self.log(f"Flashing target via probe-rs: {' '.join(cmd_args)}")
            ret, out = 1, ""
            for attempt in range(MAX_PROBE_OPEN_RETRIES + 1):
                ret, out = self.run_probe_rs(cmd_args)
                if ret == 0 or not _is_transient_probe_error(out) or attempt >= MAX_PROBE_OPEN_RETRIES:
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
        self._record_lifecycle("rebooting", "Rebooting target MCU over SWD & reconnecting RTT...")
        self.log("Resetting target microcontroller...", channel="system")
        self.stop_rtt_streaming()
        time.sleep(PROBE_RELEASE_SETTLE_SEC)

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

        ret, out = 1, ""
        for attempt in range(MAX_PROBE_OPEN_RETRIES + 1):
            ret, out = self.run_probe_rs(cmd_args)
            if ret == 0 or not _is_transient_probe_error(out) or attempt >= MAX_PROBE_OPEN_RETRIES:
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
        msg = "Target reboot completed & RTT reconnected" if reset_success else "Target reboot attempted & RTT reconnected"
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

    def query_target_metadata(self, elf_path: Optional[Path] = None) -> Dict[str, Any]:
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
            rtt_was_running = (self.rtt_process is not None and self.rtt_process.poll() is None) or (
                self._streaming_thread is not None and self._streaming_thread.is_alive()
            )
            if rtt_was_running:
                self.stop_rtt_streaming()
                time.sleep(PROBE_RELEASE_SETTLE_SEC)

            target_elf = elf_path or self.elf_path or self.resolve_current_elf_path()
            try:
                return probe_attached_target_metadata(
                    self.repo_root,
                    preferred_elf=target_elf,
                    host_fs_path=self.host_fs_path,
                )
            finally:
                if rtt_was_running and self.active_mode in ("CLI", "TEST"):
                    self.start_rtt_streaming()

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

    def start_rtt_streaming(self) -> None:
        """Start streaming RTT logs and CLI interactions in a background thread."""
        if not self.host_cli_path or not self.host_cli_path.exists() or not self.elf_path:
            return

        self.stop_rtt_streaming()

        cmd = [
            str(self.host_cli_path),
            "--elf",
            str(self.elf_path),
            "--speed",
            str(self.speed),
            "--channel",
            "both",
            "--reset",
        ]

        started_event = threading.Event()
        session_ready = False

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
                self.is_connected = True
                self.log("RTT streaming process connected")
                if self.rtt_process.stdout:
                    for line in iter(self.rtt_process.stdout.readline, ""):
                        stripped = line.rstrip("\r\n")
                        if stripped:
                            if "RTT Session Started" in stripped or "Interactive RTT console active" in stripped:
                                session_ready = True
                                started_event.set()
                            if (
                                "INFO" in stripped
                                or "WARN" in stripped
                                or "ERROR" in stripped
                                or "DEBUG" in stripped
                                or "TRACE" in stripped
                                or "Draining buffered defmt logs" in stripped
                            ):
                                self.log(stripped, channel="target")
                            elif stripped in ("shell>", "shell> "):
                                pass
                            elif stripped.replace(ANSI_CLEAR_LINE_SEQ, "").strip() in ("", "shell>"):
                                if ANSI_CLEAR_LINE_SEQ in stripped:
                                    with self._lock:
                                        self._cli_clear_seq += 1
                            else:
                                if ANSI_CLEAR_LINE_SEQ in stripped:
                                    with self._lock:
                                        self._cli_clear_seq += 1
                                cleaned_line = (
                                    stripped.replace(ANSI_CLEAR_LINE_SEQ, "")
                                    .removeprefix("shell> ")
                                    .removeprefix(SHELL_PROMPT_PREFIX)
                                )
                                if cleaned_line:
                                    self.log_cli(cleaned_line)
            except Exception as e:
                started_event.set()
                self.log(f"RTT stream error: {e}", level="WARN")
            finally:
                started_event.set()
                self.is_connected = False
                self.log("RTT stream disconnected", level="INFO")

        self._streaming_thread = threading.Thread(target=_stream_worker, daemon=True)
        self._streaming_thread.start()
        started_event.wait(timeout=RTT_STARTUP_EVENT_TIMEOUT_SEC)
        if session_ready and self.rtt_process is not None and self.rtt_process.poll() is None:
            time.sleep(RTT_CONSOLE_READY_SETTLE_SEC)

    def _collect_cli_output(
        self,
        clean_cmd: str,
        raw_cmd: str,
        start_cli_seq: int,
        start_log_seq: int,
        start_clear_seq: int,
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

            meaningful_count = sum(
                1
                for text in new_cli
                if text.replace(ANSI_CLEAR_LINE_SEQ, "").strip() not in ("", "shell>")
                and not text.replace(ANSI_CLEAR_LINE_SEQ, "").strip().startswith(RTT_SESSION_BANNER_PREFIXES)
            )
            if meaningful_count > prev_count:
                prev_count = meaningful_count
                last_output_time = now

            cmd_done_log = any(
                "command execution succeeded" in msg or "command execution failed" in msg for msg in new_logs
            )
            if (prompt_seen or cmd_done_log) and done_seen_time is None:
                done_seen_time = now

            if done_seen_time is not None:
                anchor = max(done_seen_time, last_output_time or done_seen_time)
                if (now - anchor) >= CLI_COMPLETION_SETTLE_SEC:
                    break
            elif last_output_time is not None:
                quiet_settle = min(CLI_QUIET_SETTLE_MAX_SEC, max(CLI_QUIET_SETTLE_SEC, timeout * CLI_QUIET_TIMEOUT_RATIO))
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

        for msg in new_logs:
            if "command execution failed" in msg:
                return f"Command failed: {clean_cmd}"

        if prompt_seen or any("command execution succeeded" in msg for msg in new_logs):
            return f"Executed: {clean_cmd}"

        return f"Dispatched: {raw_cmd}"

    def execute_cli(
        self,
        command: str,
        wait_for_output: bool = False,
        timeout: float = DEFAULT_CLI_OUTPUT_TIMEOUT_SEC,
    ) -> str:
        """Send command to the interactive CLI shell, optionally waiting for target response lines."""
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

            # Ensure probe is available by pausing RTT streaming if currently running or starting
            rtt_was_running = (self.rtt_process is not None and self.rtt_process.poll() is None) or (
                self._streaming_thread is not None and self._streaming_thread.is_alive()
            )
            if rtt_was_running:
                self.stop_rtt_streaming()
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
                    if ret == 0 or not _is_transient_probe_error(out) or attempt >= MAX_PROBE_OPEN_RETRIES:
                        break
                    self.log(
                        f"Transient USB probe contention during host_fs '{subcmd}' (attempt {attempt + 1}); retrying...",
                        level="WARN",
                    )
                    self.stop_rtt_streaming()
                    time.sleep(PROBE_RETRY_BACKOFF_SEC)
                out = out.strip() or f"Completed {subcmd} (exit code {ret})"
                return out
            finally:
                if rtt_was_running and self.active_mode in ("CLI", "TEST"):
                    self.start_rtt_streaming()
                self._record_lifecycle("idle", f"Completed host_fs ({subcmd})", mark_recent=False)

    def get_status(self) -> Dict[str, Any]:
        """Return daemon status dictionary."""
        return {
            "status": "running",
            "target": self.target,
            "package": self.package,
            "bin": self.bin_name,
            "profile": "release" if self.release else "debug",
            "elf": str(self.elf_path) if self.elf_path else None,
            "chip": self.chip,
            "speed": self.speed,
            "connected": self.is_connected,
            "flashed": self.is_flashed,
            "mode": self.active_mode,
            "message": self.status_message,
            "manual_prompt": self.manual_prompt.to_dict(),
        }

    def stop(self) -> None:
        """Terminate active RTT processes, sync tasks, test runs, and release hardware debug probes."""
        self.is_sync_cancelled = True
        self.test_manager.cancel_run(silent=True)
        self.stop_rtt_streaming()
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
