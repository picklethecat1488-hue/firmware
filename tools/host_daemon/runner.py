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

        self.host_cli_path = cli_cand
        self.host_fs_path = fs_cand
        self.log(f"Host tools ready: cli={cli_cand}, fs={fs_cand}")
        return cli_cand, fs_cand

    def detect_chip(self) -> str:
        """Detect the target chip name from ELF metadata using host_cli."""
        if self.chip:
            return self.chip
        if self.host_cli_path and self.elf_path and self.host_cli_path.exists() and self.elf_path.exists():
            try:
                proc = subprocess.run(
                    [str(self.host_cli_path), "--elf", str(self.elf_path), "--print-chip"],
                    capture_output=True,
                    text=True,
                    timeout=5,
                )
                if proc.returncode == 0 and proc.stdout.strip():
                    self.chip = proc.stdout.strip()
                    return self.chip
            except Exception as e:
                self.log(f"Chip detection warning: {e}", level="WARN")

        self.chip = "RP2040"
        return self.chip

    def flash_target(self) -> bool:
        """Flash target device with probe-rs."""
        if not self.elf_path or not self.elf_path.exists():
            self.log("Cannot flash target: ELF not compiled", level="ERROR")
            return False

        chip = self.detect_chip()
        probe_rs = shutil.which("probe-rs")
        if not probe_rs:
            self.log("probe-rs not found in PATH; skipping hardware download", level="WARN")
            self.status_message = "Ready (probe-rs not in PATH)"
            return False

        cmd = [probe_rs, "download", "--chip", chip, "--speed", str(self.speed), str(self.elf_path)]
        self.log(f"Executing: {' '.join(cmd)}")
        try:
            proc = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
            if proc.returncode == 0:
                self.is_flashed = True
                self.status_message = "Flashed & Running"
                self.log("Flash download succeeded", level="INFO")
                return True
            else:
                self.log(f"probe-rs download failed:\n{proc.stderr}", level="WARN")
                self.status_message = f"Flash failed: {proc.stderr[:100]}"
                return False
        except Exception as e:
            self.log(f"probe-rs invocation error: {e}", level="WARN")
            self.status_message = f"Flash error: {e}"
            return False

    def start_rtt_streaming(self) -> None:
        """Start streaming RTT logs and CLI interactions in a background thread."""
        if not self.host_cli_path or not self.host_cli_path.exists() or not self.elf_path:
            return

        cmd = [
            str(self.host_cli_path),
            "--elf",
            str(self.elf_path),
            "--speed",
            str(self.speed),
            "--channel",
            "both",
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
                            if "INFO" in stripped or "WARN" in stripped or "ERROR" in stripped or "DEBUG" in stripped:
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

        # Host offline / one-shot fallback simulation
        output = f"[host_cli] Executed: '{command}' (mock/standby mode)"
        self.log_cli(output)
        return output

    def execute_fs(self, command_or_subcommand: str, args: Optional[List[str]] = None) -> str:
        """Execute host_fs utility command."""
        args = args or []
        tokens = command_or_subcommand.strip().split()
        if tokens:
            subcmd = tokens[0]
            extra_args = tokens[1:] + args
        else:
            subcmd = "ls"
            extra_args = args

        self.log_cli(f"fs> {subcmd} {' '.join(extra_args)}", is_input=True)

        if not self.host_fs_path or not self.host_fs_path.exists():
            res = "host_fs binary not available. Build host tools first."
            self.log_cli(res)
            return res

        cmd = [str(self.host_fs_path)]
        if self.elf_path and self.elf_path.exists():
            cmd.extend(["--elf", str(self.elf_path)])
        cmd.append(subcmd)
        cmd.extend(extra_args)

        try:
            proc = subprocess.run(cmd, cwd=self.repo_root, capture_output=True, text=True, timeout=15)
            out = proc.stdout if proc.stdout else proc.stderr
            out = out.strip() or f"Completed {subcmd} (exit code {proc.returncode})"
            self.log_cli(out)
            return out
        except Exception as e:
            err = f"host_fs execution failed: {e}"
            self.log_cli(err)
            return err

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
