#!/usr/bin/env python3
"""Repeatable GDB MI2 reproduction and benchmark script for `tools/host.py`.

Exercises the exact VS Code Cortex-Debug workflow over `host.py debug gdb-proxy`:
1. Ensures `tools/host.py` is running with `--debug-server` on the requested binary and syncs `target/active_target.elf`.
2. Attaches `arm-none-eabi-gdb --nx --interpreter=mi2` via `python3 -u tools/host.py debug gdb-proxy --core 0`.
3. Sets a hardware breakpoint at `main` (resolved automatically from `#[cortex_m_rt::entry] fn main`).
4. Repeats `N` restart iterations (`monitor reset halt`, `maintenance flush register-cache`, `-exec-continue`),
   asserting automatic stop at `main` and executing `10` consecutive source-line single-steps (`-exec-next` or `-exec-step`).
"""

from __future__ import annotations

import argparse
from pathlib import Path
import queue
import re
import shutil
import subprocess
import sys
import threading
import time
from typing import Dict, List, Optional, Tuple

REPO_ROOT: Path = Path(__file__).resolve().parent.parent
DEFAULT_PACKAGE: str = "cat_detector"
DEFAULT_BIN: str = "cat_detector_shell"
DEFAULT_ITERATIONS: int = 3
DEFAULT_STEPS: int = 10
DEFAULT_CMD_TIMEOUT_SEC: float = 15.0
DEFAULT_LOG_REL_PATH: str = "target/logs/repro_gdb_break_and_step.log"
ENTRY_ATTR_MARKER: str = "#[cortex_m_rt::entry]"

STOPPED_FRAME_RE = re.compile(
    r'\*stopped,.*?reason="([^"]+)".*?frame=\{addr="([^"]+)",func="([^"]+)"'
    r'(?:,args=\[[^\]]*\])?(?:,file="([^"]+)")?(?:,fullname="([^"]+)")?(?:,line="(\d+)")?'
)


def resolve_main_breakpoint_spec(repo_root: Path, bin_name: str) -> str:
    """Resolve `<source_path>:<first_body_line>` for `#[cortex_m_rt::entry] fn main` of `bin_name`."""
    candidate = repo_root / "app" / "src" / f"{bin_name}.rs"
    if not candidate.is_file():
        return "main"
    lines = candidate.read_text(encoding="utf-8").splitlines()
    saw_entry = False
    for idx, raw_line in enumerate(lines, start=1):
        stripped = raw_line.strip()
        if ENTRY_ATTR_MARKER in stripped:
            saw_entry = True
            continue
        if saw_entry and stripped.startswith("fn main"):
            # Place breakpoint on the first executable statement inside `fn main()`.
            for body_idx in range(idx + 1, len(lines) + 1):
                body_stmt = lines[body_idx - 1].strip()
                if body_stmt and not body_stmt.startswith("//") and body_stmt != "{":
                    return f"{candidate}:{body_idx}"
            return f"{candidate}:{idx}"
    return "main"


class GdbMiSession:
    """Line-oriented GDB/MI2 subprocess session forwarding raw protocol traffic to a log file."""

    def __init__(self, gdb_bin: str, elf_path: Path, log_path: Path, repo_root: Path) -> None:
        """Spawn `arm-none-eabi-gdb --nx --interpreter=mi2` against `elf_path`."""
        log_path.parent.mkdir(parents=True, exist_ok=True)
        self._log_fh = log_path.open("a", encoding="utf-8")
        self._proc = subprocess.Popen(
            [gdb_bin, "-q", "--nx", "--interpreter=mi2", str(elf_path)],
            cwd=str(repo_root),
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
        )
        self._token = 0
        self._lines: "queue.Queue[str]" = queue.Queue()
        self._reader = threading.Thread(target=self._pump_stdout, daemon=True)
        self._reader.start()

    def _pump_stdout(self) -> None:
        assert self._proc.stdout is not None
        for raw in self._proc.stdout:
            line = raw.rstrip("\r\n")
            self._log_fh.write(f"<- {line}\n")
            self._log_fh.flush()
            self._lines.put(line)

    def send_mi(self, command: str, timeout: float = DEFAULT_CMD_TIMEOUT_SEC) -> Tuple[str, List[str]]:
        """Send a tokenized MI2 command and wait for its `<token>^(done|running|connected|error|exit)` record."""
        assert self._proc.stdin is not None
        self._token += 1
        tok = self._token
        wire = f"{tok}{command}"
        self._log_fh.write(f"-> {wire}\n")
        self._log_fh.flush()
        self._proc.stdin.write(wire + "\n")
        self._proc.stdin.flush()

        prefix = f"{tok}^"
        collected: List[str] = []
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            remaining = max(0.02, deadline - time.monotonic())
            try:
                line = self._lines.get(timeout=min(0.25, remaining))
            except queue.Empty:
                continue
            collected.append(line)
            if line.startswith(prefix):
                status = line[len(prefix) :].split(",", 1)[0]
                if status == "error":
                    raise RuntimeError(f"GDB MI command `{wire}` failed: {line}")
                return status, collected
        raise TimeoutError(f"Timed out after {timeout:.1f}s waiting for GDB MI reply to `{wire}`")

    def wait_for_stopped(self, pre_collected: List[str], timeout: float = DEFAULT_CMD_TIMEOUT_SEC) -> Dict[str, str]:
        """Wait for an asynchronous `*stopped,...` MI2 record and return parsed stop metadata."""
        for existing in pre_collected:
            parsed = self._parse_stopped_line(existing)
            if parsed is not None:
                return parsed

        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            remaining = max(0.02, deadline - time.monotonic())
            try:
                line = self._lines.get(timeout=min(0.25, remaining))
            except queue.Empty:
                continue
            parsed = self._parse_stopped_line(line)
            if parsed is not None:
                return parsed
        raise TimeoutError(f"Timed out after {timeout:.1f}s waiting for target `*stopped` event")

    @staticmethod
    def _parse_stopped_line(line: str) -> Optional[Dict[str, str]]:
        if not line.startswith("*stopped,"):
            return None
        match = STOPPED_FRAME_RE.search(line)
        if match is None:
            return {"reason": "unknown", "addr": "?", "func": "?", "file": "?", "line": "?", "raw": line}
        return {
            "reason": match.group(1) or "unknown",
            "addr": match.group(2) or "?",
            "func": match.group(3) or "?",
            "file": match.group(4) or "?",
            "line": match.group(6) or "?",
            "raw": line,
        }

    def close(self) -> None:
        """Detach cleanly from the target GDB proxy and close the MI2 subprocess."""
        if self._proc.poll() is None:
            try:
                self.send_mi("-break-delete", timeout=3.0)
                self.send_mi("-target-detach", timeout=3.0)
                self.send_mi("-gdb-exit", timeout=3.0)
            except (OSError, RuntimeError, TimeoutError):
                self._proc.terminate()
        self._log_fh.close()


def ensure_host_debug_ready(repo_root: Path, package: str, bin_name: str, release: bool, sync: bool) -> Path:
    """Ensure `host.py` is running with `--debug-server` active and return `target/active_target.elf`."""
    host_py = str(repo_root / "tools" / "host.py")
    profile_label = "release" if release else "debug"
    status_res = subprocess.run(
        [sys.executable, host_py, "debug", "status"],
        cwd=str(repo_root),
        capture_output=True,
        text=True,
        check=False,
    )
    already_matching = (
        not sync
        and status_res.returncode == 0
        and "ACTIVE" in status_res.stdout
        and f"{package} / {bin_name} [{profile_label}]" in status_res.stdout
    )
    if not already_matching:
        cmd = [
            sys.executable,
            host_py,
            "--package",
            package,
            "--bin",
            bin_name,
            "--debug-server",
            "--no-browser",
        ]
        if release:
            cmd.append("--release")
        subprocess.run(cmd, cwd=str(repo_root), check=True)
        if sync:
            subprocess.run([sys.executable, host_py, "sync"], cwd=str(repo_root), check=True)

        # Wait for any background sync / debug server startup to become ACTIVE.
        deadline = time.monotonic() + 45.0
        while time.monotonic() < deadline:
            res = subprocess.run(
                [sys.executable, host_py, "debug", "status"],
                cwd=str(repo_root),
                capture_output=True,
                text=True,
                check=False,
            )
            if res.returncode == 0 and "ACTIVE" in res.stdout:
                break
            time.sleep(0.5)
        else:
            subprocess.run([sys.executable, host_py, "debug", "start"], cwd=str(repo_root), check=True)

    subprocess.run(
        [sys.executable, host_py, "debug", "sync-symbols"],
        cwd=str(repo_root),
        stdout=subprocess.DEVNULL,
        check=True,
    )
    elf_path = repo_root / "target" / "active_target.elf"
    if not elf_path.exists():
        raise FileNotFoundError(f"Expected active target ELF symlink at {elf_path}")
    return elf_path


def build_arg_parser() -> argparse.ArgumentParser:
    """Build CLI argument parser for `repro_gdb_break_and_step.py`."""
    parser = argparse.ArgumentParser(
        description="Repeat VS Code GDB restart -> break at main -> 10x single-step over tools/host.py."
    )
    parser.add_argument("--package", default=DEFAULT_PACKAGE, help="Cargo firmware package name.")
    parser.add_argument("--bin", dest="bin_name", default=DEFAULT_BIN, help="Firmware binary target name.")
    parser.add_argument("--release", action="store_true", help="Build/debug release profile binary.")
    parser.add_argument("--sync", action="store_true", help="Force `host.py sync` re-flash before running.")
    parser.add_argument(
        "--iterations",
        type=int,
        default=DEFAULT_ITERATIONS,
        help=f"Number of restart + break-at-main + step cycles (default: {DEFAULT_ITERATIONS}).",
    )
    parser.add_argument(
        "--steps",
        type=int,
        default=DEFAULT_STEPS,
        help=f"Number of single-steps per iteration (default: {DEFAULT_STEPS}).",
    )
    parser.add_argument(
        "--step-over",
        action="store_true",
        help="Use `-exec-next` (step over) instead of `-exec-step` (single-step into; note `run_executor() -> !` diverges).",
    )
    parser.add_argument(
        "--breakpoint",
        default=None,
        help="Optional explicit GDB breakpoint location (defaults to `#[cortex_m_rt::entry] fn main`).",
    )
    parser.add_argument(
        "--log-file",
        default=DEFAULT_LOG_REL_PATH,
        help=f"Log file receiving verbose GDB MI2 packets (default: {DEFAULT_LOG_REL_PATH}).",
    )
    return parser


def main(argv: Optional[List[str]] = None) -> int:
    """Execute the repeatable `host.py` break-at-main + 10x single-step loop."""
    args = build_arg_parser().parse_args(argv)
    if args.iterations < 1 or args.steps < 1:
        print("Error: --iterations and --steps must be >= 1", file=sys.stderr)
        return 2

    gdb_bin = shutil.which("arm-none-eabi-gdb")
    if not gdb_bin:
        print("Error: `arm-none-eabi-gdb` not found on PATH", file=sys.stderr)
        return 2

    elf_path = ensure_host_debug_ready(
        REPO_ROOT,
        package=args.package,
        bin_name=args.bin_name,
        release=args.release,
        sync=args.sync,
    )
    bp_spec = args.breakpoint or resolve_main_breakpoint_spec(REPO_ROOT, args.bin_name)
    log_path = Path(args.log_file) if Path(args.log_file).is_absolute() else (REPO_ROOT / args.log_file)
    step_cmd = "-exec-next" if args.step_over else "-exec-step"

    print(
        f"Attaching GDB via host.py proxy -> ELF={elf_path.name}, breakpoint={bp_spec}, "
        f"iterations={args.iterations}, steps={args.steps} ({step_cmd}), log={log_path}"
    )

    session = GdbMiSession(gdb_bin, elf_path, log_path, REPO_ROOT)
    try:
        session.send_mi('-interpreter-exec console "set sysroot /"')
        session.send_mi('-interpreter-exec console "source .vscode/cortex_debug_init.gdb"')
        proxy_cmd = f"{sys.executable} -u tools/host.py debug gdb-proxy --core 0"
        session.send_mi(f"-target-select extended-remote | {proxy_cmd}")
        session.send_mi('-interpreter-exec console "monitor halt"')
        session.send_mi(f'-break-insert "{bp_spec}"')

        for iteration in range(1, args.iterations + 1):
            iter_start = time.monotonic()
            session.send_mi('-interpreter-exec console "monitor reset halt"')
            session.send_mi('-interpreter-exec console "maintenance flush register-cache"')
            _, pre_lines = session.send_mi("-exec-continue")
            bp_stop = session.wait_for_stopped(pre_lines)
            bp_elapsed_ms = (time.monotonic() - iter_start) * 1000.0
            if bp_stop["reason"] != "breakpoint-hit":
                raise RuntimeError(
                    f"Iteration {iteration}: expected `breakpoint-hit` at {bp_spec} after restart, got {bp_stop}"
                )
            print(
                f"\n[Iter {iteration}/{args.iterations}] Hit breakpoint `{bp_stop['func']}` "
                f"at {bp_stop['file']}:{bp_stop['line']} ({bp_stop['addr']}) in {bp_elapsed_ms:.0f} ms"
            )

            step_suite_start = time.monotonic()
            for step_idx in range(1, args.steps + 1):
                t0 = time.monotonic()
                _, step_lines = session.send_mi(step_cmd)
                stop_info = session.wait_for_stopped(step_lines)
                dt_ms = (time.monotonic() - t0) * 1000.0
                print(
                    f"  step {step_idx:2d}/{args.steps}: {dt_ms:6.1f} ms -> "
                    f"{stop_info['addr']} {stop_info['func']} ({stop_info['file']}:{stop_info['line']})"
                )
            total_step_s = time.monotonic() - step_suite_start
            print(
                f"[Iter {iteration}/{args.iterations}] Completed {args.steps} steps in {total_step_s:.2f}s"
            )
    finally:
        session.close()

    print(f"\nSUCCESS: All {args.iterations} restart + {args.steps}-step iterations passed (MI2 log: {log_path}).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
