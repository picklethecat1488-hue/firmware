"""Runner for building targets, flashing devices, streaming RTT, and executing CLI/FS tools."""

from collections import deque
from datetime import datetime, timezone
import os
from pathlib import Path
import shutil
import subprocess
import threading
from typing import Any, Deque, Dict, List, Optional, Tuple


class DaemonRunner:
    """Manages firmware compilation, device flashing, RTT streaming, and CLI/FS command execution."""

    def __init__(
        self,
        repo_root: Path,
        target: str = "thumbv6m-none-eabi",
        package: str = "cat_detector",
        bin_name: str = "cat_detector_shell",
        release: bool = False,
        speed: int = 1000,
        chip: Optional[str] = None,
    ) -> None:
        """Initialize DaemonRunner with workspace configuration and build parameters."""
        self.repo_root = repo_root
        self.target = target
        self.package = package
        self.bin_name = bin_name
        self.release = release
        self.speed = speed
        self.chip = chip

        self.elf_path: Optional[Path] = None
        self.host_cli_path: Optional[Path] = None
        self.host_fs_path: Optional[Path] = None

        self.active_mode: str = "CLI"
        self._lock = threading.Lock()
        self.logs: Deque[Dict[str, Any]] = deque(maxlen=5000)
        self.cli_outputs: Deque[Dict[str, Any]] = deque(maxlen=5000)

        self.rtt_process: Optional[subprocess.Popen] = None
        self._streaming_thread: Optional[threading.Thread] = None
        self.is_flashed = False
        self.is_connected = False
        self.status_message = "Initialized"

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
            "error": None,
        }
        self._sync_thread: Optional[threading.Thread] = None

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
                    "error": error,
                }
            )
        self.log(f"[Sync] {message}")

    def sync(self) -> Dict[str, Any]:
        """Perform full synchronization: compile target, compile tools, flash, and connect RTT."""
        try:
            # 1. Compile target firmware
            self._update_sync("compiling_target", 1, 25, "Compiling target firmware...")
            self.build_target()

            # 2. Compile host tools
            self._update_sync("compiling_host_tools", 2, 50, "Compiling host utilities (host_cli, host_fs)...")
            self.build_host_tools()

            # 3. Flash target device (release probe first)
            self.stop_rtt_streaming()
            self._update_sync("flashing_target", 3, 75, "Flashing target device via probe-rs...")
            self.flash_target()

            # 4. Start RTT streaming
            self._update_sync("starting_rtt", 4, 90, "Starting RTT streaming channels...")
            self.start_rtt_streaming()

            # Complete
            self._update_sync("complete", 4, 100, "Target synchronized & RTT connected successfully.", active=False)
        except Exception as e:
            err_msg = f"Synchronization failed: {e}"
            self._update_sync("failed", 0, 0, err_msg, active=False, error=str(e))
            self.log(err_msg, level="ERROR")

        return dict(self.sync_status)

    def sync_async(self) -> None:
        """Trigger target synchronization asynchronously in a background thread."""
        with self._lock:
            if self._sync_thread and self._sync_thread.is_alive():
                return
            self._sync_thread = threading.Thread(target=self.sync, daemon=True)
            self._sync_thread.start()

    def log(self, message: str, level: str = "INFO", channel: str = "daemon") -> None:
        """Append a log message to the thread-safe log buffer."""
        entry = {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "level": level,
            "channel": channel,
            "message": message,
        }
        with self._lock:
            self.logs.append(entry)

    def log_cli(self, message: str, is_input: bool = False) -> None:
        """Append a line to the CLI console output buffer."""
        entry = {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "is_input": is_input,
            "text": message,
        }
        with self._lock:
            self.cli_outputs.append(entry)

    def build_target(self) -> Path:
        """Compile the target embedded firmware binary."""
        self.log(
            f"Building target firmware: package={self.package}, bin={self.bin_name}, target={self.target}, release={self.release}"
        )
        cmd = ["cargo", "build", "--target", self.target, "-p", self.package, "--bin", self.bin_name]
        if self.release:
            cmd.append("--release")

        env = os.environ.copy()
        proc = subprocess.run(cmd, cwd=self.repo_root, capture_output=True, text=True, env=env)
        if proc.returncode != 0:
            err_msg = f"Failed to build target binary:\n{proc.stderr}"
            self.log(err_msg, level="ERROR")
            raise RuntimeError(err_msg)

        profile_dir = "release" if self.release else "debug"
        elf = self.repo_root / "target" / self.target / profile_dir / self.bin_name
        if not elf.exists():
            # Check target/out
            alt_elf = self.repo_root / "target" / "out" / profile_dir / "embedded" / self.package / self.bin_name
            if alt_elf.exists():
                elf = alt_elf

        if not elf.exists():
            raise FileNotFoundError(f"Target ELF not found at {elf}")

        self.elf_path = elf
        self.log(f"Target binary built successfully: {elf}")
        return elf

    def build_host_tools(self) -> Tuple[Path, Path]:
        """Compile host CLI and FS tools."""
        self.log("Building host utilities: host_cli and host_fs")
        cmd = ["cargo", "build", "-p", "host_cli", "-p", "host_fs"]
        if self.release:
            cmd.append("--release")

        env = os.environ.copy()
        # Ensure target flags from ARM don't contaminate host build
        env.pop("CARGO_BUILD_TARGET", None)
        env.pop("CARGO_ENCODED_RUSTFLAGS", None)

        proc = subprocess.run(cmd, cwd=self.repo_root, capture_output=True, text=True, env=env)
        if proc.returncode != 0:
            err_msg = f"Failed to build host tools:\n{proc.stderr}"
            self.log(err_msg, level="ERROR")
            raise RuntimeError(err_msg)

        profile_dir = "release" if self.release else "debug"
        cli_cand = self.repo_root / "target" / profile_dir / "host_cli"
        fs_cand = self.repo_root / "target" / profile_dir / "host_fs"

        if not cli_cand.exists():
            alt = self.repo_root / "target" / "out" / profile_dir / "macos" / "host_cli"
            if alt.exists():
                cli_cand = alt
        if not fs_cand.exists():
            alt = self.repo_root / "target" / "out" / profile_dir / "macos" / "host_fs"
            if alt.exists():
                fs_cand = alt

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
            if proc.stdout:
                for line in iter(proc.stdout.readline, ""):
                    stripped = line.rstrip("\r\n")
                    if stripped:
                        output_lines.append(stripped)
                        if log_to_cli:
                            self.log_cli(stripped)
                        self.log(stripped, channel=log_channel)
            proc.wait(timeout=timeout)
            return proc.returncode, "\n".join(output_lines)
        except Exception as e:
            err = f"Execution error running {cmd[0]}: {e}"
            if log_to_cli:
                self.log_cli(err)
            self.log(err, level="ERROR", channel=log_channel)
            return 1, err

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
        return self._run_command_streaming(cmd, env=env, timeout=timeout, log_to_cli=True, log_channel="host_fs")

    def run_probe_rs(
        self,
        args: List[str],
        env: Optional[Dict[str, str]] = None,
        timeout: Optional[float] = None,
    ) -> Tuple[int, str]:
        """Subcommand wrapper for probe-rs, streaming colorized output in real time."""
        probe_bin = shutil.which("probe-rs") or "probe-rs"
        cmd = [probe_bin] + args
        return self._run_command_streaming(cmd, env=env, timeout=timeout, log_to_cli=True, log_channel="probe-rs")

    def detect_chip(self) -> str:
        """Detect the target chip name from ELF metadata using host_cli.

        Explicitly raises RuntimeError if program metadata is missing or target is unsupported.
        """
        if self.chip:
            return self.chip

        if not self.elf_path or not self.elf_path.exists():
            raise RuntimeError(
                f"Target ELF '{self.elf_path}' not found; cannot detect program metadata"
            )

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

            ret, out = self._run_command_streaming(cmd, log_to_cli=True, log_channel="flash")
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
            ret, out = self.run_probe_rs(cmd_args)
            if ret == 0:
                self.is_flashed = True
                self.status_message = "Flashed & Running"
                self.log("Flash download succeeded via probe-rs", level="INFO")
                return True
            else:
                self.log(f"probe-rs download failed (exit code {ret}):\n{out}", level="WARN")
                self.status_message = f"Flash failed: {out[:100]}"
                return False

    def stop_rtt_streaming(self) -> None:
        """Terminate active RTT streaming process to release the USB debug probe."""
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

        def _stream_worker() -> None:
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
                            if "INFO" in stripped or "WARN" in stripped or "ERROR" in stripped or "DEBUG" in stripped or "TRACE" in stripped:
                                self.log(stripped, channel="target")
                            else:
                                self.log_cli(stripped)
            except Exception as e:
                self.log(f"RTT stream error: {e}", level="WARN")
            finally:
                self.is_connected = False
                self.log("RTT stream disconnected", level="INFO")

        self._streaming_thread = threading.Thread(target=_stream_worker, daemon=True)
        self._streaming_thread.start()

    def execute_cli(self, command: str) -> str:
        """Send command to the interactive CLI shell."""
        self.log_cli(f"shell> {command}", is_input=True)
        # If RTT process is actively connected and taking stdin:
        if self.rtt_process and self.rtt_process.stdin and self.rtt_process.poll() is None:
            try:
                self.rtt_process.stdin.write(command + "\n")
                self.rtt_process.stdin.flush()
                return f"Dispatched: {command}"
            except Exception as e:
                err = f"Failed to send to RTT: {e}"
                self.log_cli(err)
                return err

        # Standby/offline shell response for help or simulated dispatch
        if command.strip().lower() == "help":
            help_text = (
                "Commands:\n"
                "  battery  Battery commands (battery status, battery test-alert)\n"
                "  thermal  Thermal commands (thermal status, thermal mcu)\n"
                "  motor    Motor commands (motor speed, motor rpm-speed, motor stop, motor current, motor rpm, motor calibrate, motor safety-bypass)\n"
                "  sensor   Sensor commands (sensor status, sensor cal_near, sensor cal_far, sensor cal_xtalk)\n"
                "  fs       Filesystem commands (fs format, fs ls)\n"
                "  system   System commands (system activity)\n"
                "  i2c      I2C commands (i2c scan)\n"
                "  gpio     GPIO commands (gpio status, gpio read, gpio set)\n"
                "  led      LED commands (led off, led green, led blue, led yellow, led orange, led blink-four, led blink-slow, led brightness)\n"
                "  core     Core monitor commands (core status, core crash)"
            )
            for line in help_text.splitlines():
                self.log_cli(line)
            return help_text

        output = f"[host_cli] Executed: '{command}' (mock/standby mode)"
        self.log_cli(output)
        return output

    def execute_fs(self, command_or_subcommand: str, args: Optional[List[str]] = None) -> str:
        """Execute host_fs utility command by invoking cargo run."""
        args = args or []
        tokens = command_or_subcommand.strip().split()
        if tokens:
            subcmd = tokens[0]
            extra_args = tokens[1:] + args
        else:
            subcmd = "ls"
            extra_args = args

        self.log_cli(f"fs> {subcmd} {' '.join(extra_args)}", is_input=True)

        fs_args: List[str] = []
        if self.elf_path and self.elf_path.exists():
            fs_args.extend(["--elf", str(self.elf_path)])
        fs_args.append(subcmd)
        fs_args.extend(extra_args)

        ret, out = self.run_host_fs(fs_args)
        out = out.strip() or f"Completed {subcmd} (exit code {ret})"
        return out

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
        }

    def stop(self) -> None:
        """Terminate active RTT processes and background tasks."""
        if self.rtt_process and self.rtt_process.poll() is None:
            try:
                self.rtt_process.terminate()
                self.rtt_process.wait(timeout=2)
            except Exception:
                self.rtt_process.kill()
        self.log("Daemon stopped")
