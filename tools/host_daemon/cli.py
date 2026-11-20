"""Command-line interface and daemon management for Xerxes Host Daemon."""

import argparse
import atexit
import json
import os
from pathlib import Path
import signal
import socket
import subprocess
import sys
import threading
import time
from typing import Any, Callable, Dict, List, Optional
import urllib.error
import urllib.request

from host_daemon.provider.daemon.config import (
    discover_target_packages,
    has_explicit_target_args,
    infer_arm_target,
    load_last_session,
    save_last_session,
)
from host_daemon.provider.daemon.runner import DaemonRunner
from host_daemon.provider.daemon.server import HostDaemonServer

_repo_root = Path(__file__).resolve().parent.parent.parent
if str(_repo_root) not in sys.path:
    sys.path.insert(0, str(_repo_root))
_tools_dir = _repo_root / "tools"
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

LOCK_FILE: Path = Path("/tmp/xerxes_host_daemon.lock")
DEFAULT_DAEMON_REQUEST_TIMEOUT_SEC: float = 45.0
DEFAULT_DEBUGPY_PORT: int = 5678
MIN_TCP_PORT: int = 1
MAX_TCP_PORT: int = 65535


def format_clickable_choice(label: str, key: str, action: str, color_ansi: str) -> str:
    """Format an interactive choice with ANSI colors and an OSC 8 terminal hyperlink.

    Args:
        label: Full option label text (e.g. 'Pass').
        key: Accelerator character to enclose in brackets (e.g. 'P').
        action: Semantic action keyword (e.g. 'pass').
        color_ansi: ANSI color escape sequence to style the bracketed accelerator.

    Returns:
        String with embedded OSC 8 hyperlink and ANSI styling.
    """
    return f"\033]8;;command:{action}\033\\{color_ansi}[{key}]{label[1:]}\033[0m\033]8;;\033\\"


def get_host_daemon_icon_path() -> Optional[Path]:
    """Resolve distinct host daemon application icon for native desktop window."""
    static_dir = Path(__file__).resolve().parents[1] / "js" / "static" / "daemon"
    if not static_dir.exists():
        return None

    if sys.platform == "darwin":
        icns = static_dir / "app.icns"
        if icns.exists():
            return icns

    if sys.platform in ("win32", "win64"):
        ico = static_dir / "favicon.ico"
        if ico.exists():
            return ico

    png = static_dir / "icon-512.png"
    if png.exists():
        return png

    fallback_ico = static_dir / "favicon.ico"
    if fallback_ico.exists():
        return fallback_ico

    return None


def launch_host_daemon_app(
    url: str,
    browser: str = "webview",
    on_close: Optional[Callable[[], None]] = None,
) -> bool:
    """Launch the host daemon in a dedicated native window using webview or eel."""
    if browser == "eel":
        try:
            try:
                from tools.dashboard.provider.eel.launcher import launch_eel
            except ImportError:
                from dashboard.provider.eel.launcher import launch_eel

            return launch_eel(url, size=(1400, 900), on_close=on_close)
        except Exception as e:
            print(f"[HostDaemon] Eel launch failed, falling back to webview: {e}", file=sys.stderr)

    try:
        try:
            from tools.dashboard.provider.webview.launcher import launch_webview
        except ImportError:
            from dashboard.provider.webview.launcher import launch_webview

        icon = get_host_daemon_icon_path()
        return launch_webview(
            url=url,
            title="⚡ SYSTEM SHOCK 2 // XERXES HOST DAEMON",
            size=(1400, 900),
            bg_color="#08161d",
            icon_path=icon,
            on_close=on_close,
        )
    except Exception as e:
        print(f"[HostDaemon] Native webview launch failed: {e}", file=sys.stderr)
        return False


def get_lock_data() -> Optional[Dict[str, Any]]:
    """Read host daemon lock file metadata if present."""
    if not LOCK_FILE.exists():
        return None
    try:
        return json.loads(LOCK_FILE.read_text(encoding="utf-8"))
    except Exception:
        return None


def write_lock_file(data: Dict[str, Any]) -> None:
    """Write lock file with daemon metadata."""
    try:
        LOCK_FILE.write_text(json.dumps(data, indent=2), encoding="utf-8")
    except Exception:
        pass


def remove_lock_file() -> None:
    """Remove lock file if present."""
    try:
        if LOCK_FILE.exists():
            LOCK_FILE.unlink()
    except Exception:
        pass


def is_pid_alive(pid: int) -> bool:
    """Check if process ID is alive."""
    if pid <= 0:
        return False
    try:
        os.kill(pid, 0)
        return True
    except (OSError, ProcessLookupError):
        return False


def is_port_in_use(port: int, host: str = "127.0.0.1") -> bool:
    """Check if local TCP port is in use."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.settimeout(0.5)
        return s.connect_ex((host, port)) == 0


def send_daemon_request(
    path: str,
    payload: Optional[Dict[str, Any]] = None,
    port: Optional[int] = None,
    host: Optional[str] = None,
) -> Dict[str, Any]:
    """Send an HTTP request to an active running host daemon."""
    lock = get_lock_data()
    req_port = port if port is not None else (lock.get("port", 8899) if lock else 8899)
    req_host = host if host is not None else (lock.get("host", "127.0.0.1") if lock else "127.0.0.1")
    pid = lock.get("pid", 0) if lock else 0

    if not lock and port is None and not is_port_in_use(req_port, req_host):
        raise RuntimeError("No active host daemon detected (lock file /tmp/xerxes_host_daemon.lock not found).")

    if lock and not is_pid_alive(pid) and not is_port_in_use(req_port, req_host):
        remove_lock_file()
        raise RuntimeError(f"Host daemon (PID {pid}) is not running.")

    url = f"http://{req_host}:{req_port}{path}"
    headers = {"Content-Type": "application/json"}
    data = json.dumps(payload).encode("utf-8") if payload is not None else None
    req = urllib.request.Request(url, data=data, headers=headers, method="POST" if payload is not None else "GET")

    try:
        with urllib.request.urlopen(req, timeout=DEFAULT_DAEMON_REQUEST_TIMEOUT_SEC) as resp:
            content = resp.read().decode("utf-8")
            return json.loads(content) if content else {}
    except urllib.error.URLError as e:
        raise RuntimeError(f"Failed to communicate with host daemon at {url}: {e}")


def parse_arguments(args: Optional[List[str]] = None) -> argparse.Namespace:
    """Parse command line arguments for host_daemon."""
    parser = argparse.ArgumentParser(
        description="Unified System Shock 2 - Xerxes Host Daemon Workstation",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument(
        "--target",
        type=str,
        default=None,
        help="Embedded target architecture triple (inferred from app/Cargo.toml when omitted).",
    )
    parser.add_argument(
        "--package",
        type=str,
        default=None,
        help="Cargo package name containing the firmware application (inferred from app/Cargo.toml when omitted).",
    )
    parser.add_argument(
        "--bin",
        type=str,
        default=None,
        help="Binary name to build and flash (inferred from app/Cargo.toml default-run when omitted).",
    )
    parser.add_argument(
        "--release",
        action="store_true",
        help="Build firmware and host tools with release profile optimization.",
    )
    parser.add_argument(
        "--speed",
        type=int,
        default=int(os.environ.get("PROBE_RS_SPEED", "1000")),
        help="Probe protocol speed in kHz.",
    )
    parser.add_argument(
        "--chip",
        type=str,
        default=None,
        help="Target chip name override (autodetected from ELF if omitted).",
    )
    parser.add_argument(
        "--port",
        type=int,
        default=8899,
        help="Local HTTP port for host daemon UI and REST API.",
    )
    parser.add_argument(
        "--host",
        type=str,
        default="127.0.0.1",
        help="Host network interface to bind.",
    )
    parser.add_argument(
        "--no-browser",
        "--no-app",
        action="store_true",
        dest="no_browser",
        help="Disable automatic workstation desktop window launch.",
    )
    parser.add_argument(
        "--browser",
        choices=["webview", "eel"],
        default="webview",
        help="Desktop window display environment ('webview' or 'eel'). Default is 'webview'.",
    )
    parser.add_argument(
        "--webview",
        dest="browser",
        action="store_const",
        const="webview",
        help="Target native OS webview window.",
    )
    parser.add_argument(
        "--eel",
        dest="browser",
        action="store_const",
        const="eel",
        help="Target standalone Chromium window via Eel.",
    )
    parser.add_argument(
        "--app",
        dest="browser",
        action="store_const",
        const="webview",
        help="Target native OS webview window.",
    )
    parser.add_argument(
        "--detach",
        action="store_true",
        help="Detach daemon from current terminal and run in background (default behavior).",
    )
    parser.add_argument(
        "--no-detach",
        "--foreground",
        dest="no_detach",
        action="store_true",
        help="Run workstation server in the foreground without detaching from terminal.",
    )

    parser.add_argument(
        "--restart",
        action="store_true",
        help="Restart existing host daemon if already running.",
    )
    parser.add_argument(
        "--show-system-logs",
        "--show-logs",
        dest="show_system_logs",
        action="store_true",
        default=False,
        help="Show system logging monitor (defmt/RTT) by default on startup in the host UI.",
    )
    parser.add_argument(
        "--stress-runs",
        dest="stress_runs",
        type=int,
        default=1,
        help="Number of sequential on-device test iterations to execute for `host.py test`.",
    )

    subparsers = parser.add_subparsers(dest="subcommand", help="Host daemon subcommands")

    # CLI forwarding
    cli_parser = subparsers.add_parser("cli", help="Forward command to active host daemon CLI")
    cli_parser.add_argument("command", nargs=argparse.REMAINDER, help="CLI command to execute")

    # FS forwarding
    fs_parser = subparsers.add_parser("fs", help="Forward command to active host daemon FS tool")
    fs_parser.add_argument("command", nargs=argparse.REMAINDER, help="FS subcommand and arguments (e.g. ls, cat ...)")

    # Status
    subparsers.add_parser("status", help="Show status of running host daemon")

    # Sync & Flash
    subparsers.add_parser("sync", help="Trigger target compilation, flash, and RTT sync on running daemon")
    subparsers.add_parser("flash", help="Trigger target compilation, flash, and RTT sync on running daemon")

    # Reset / Reboot / Metadata
    subparsers.add_parser("reset", help="Reset the target device via active host daemon")
    subparsers.add_parser("reboot", help="Reboot the target device and re-attach RTT via active host daemon")
    subparsers.add_parser("metadata", help="Query target .program_metadata and validate against target ELF")

    # Stop
    subparsers.add_parser("stop", help="Stop running host daemon")

    # On-device pytest test runner
    test_parser = subparsers.add_parser(
        "test",
        help="Auto-discover attached target board via host_fs and execute async pytest device suites",
    )
    test_parser.add_argument(
        "script_pos",
        nargs="?",
        metavar="script",
        default=None,
        help="Optional path (or pytest node ID) to a @device_test script to execute.",
    )
    test_parser.add_argument(
        "--script",
        "-s",
        type=str,
        default=None,
        help="Optional path (or pytest node ID) to a @device_test script; infers scenario when --scenario is omitted.",
    )
    test_parser.add_argument(
        "--project",
        "-p",
        type=str,
        default="all",
        help="Target firmware project filter matching @device_test(project=...) (e.g. cat_detector, carrier_board).",
    )
    test_parser.add_argument(
        "--scenario",
        "-m",
        type=str,
        default=None,
        help="Scenario expression: smoke, integration, bringup, default (smoke or integration), or all.",
    )
    test_parser.add_argument(
        "--board",
        "-b",
        type=str,
        default="auto",
        help="Target board_type or glob filter (e.g. auto, rp2040*, rp2040_flash_2mb, carrier_board*).",
    )
    test_parser.add_argument(
        "-k",
        "--keyword",
        "--filter",
        dest="keyword",
        type=str,
        default=None,
        help="Filter tests by name/keyword expression mirroring pytest -k (e.g. -k tof or -k 'tof and not calibration').",
    )
    test_parser.add_argument(
        "--manual",
        action="store_true",
        help="Enable operator prompts for @device_test(manual=True) benchtop steps.",
    )
    test_parser.add_argument(
        "--list-devices",
        action="store_true",
        help="Query connected target board via host_fs board-type and exit.",
    )
    test_parser.add_argument(
        "--list-tests",
        "--list",
        dest="list_tests",
        action="store_true",
        help="List registered @device_test cases with scenarios, boards, target_binary, and description.",
    )
    test_parser.add_argument(
        "--json",
        action="store_true",
        help="Format --list-devices or --list-tests output as JSON.",
    )
    test_parser.add_argument(
        "--report-md",
        type=str,
        default=None,
        help="Optional path to write Markdown verification report.",
    )
    test_parser.add_argument(
        "--junit-xml",
        type=str,
        default=None,
        help="Optional path to write JUnit XML verification report.",
    )
    test_parser.add_argument(
        "--host-port",
        type=int,
        default=None,
        help="Optional explicit TCP port of a running host.py daemon.",
    )
    test_parser.add_argument(
        "--stress-runs",
        dest="test_stress_runs",
        type=int,
        default=None,
        help="Execute the selected on-device test suite N times sequentially to stress-test target reliability.",
    )
    test_parser.add_argument(
        "--debug",
        action="store_true",
        help="Launch pytest under debugpy and wait for a VS Code symbolic debugger to attach.",
    )
    test_parser.add_argument(
        "--debug-port",
        type=int,
        default=DEFAULT_DEBUGPY_PORT,
        help="TCP loopback port for debugpy symbolic debugger attachment when --debug is enabled.",
    )

    return parser.parse_args(args)


def main(argv: Optional[List[str]] = None) -> int:
    """Execute host daemon CLI commands or launch daemon server."""
    args = parse_arguments(argv)
    repo_root = Path(__file__).resolve().parent.parent.parent

    raw_args = argv if argv is not None else sys.argv[1:]
    is_explicit = has_explicit_target_args(raw_args)
    package_selection_mode = not is_explicit and args.subcommand is None

    last_session = load_last_session(repo_root)
    if not is_explicit and last_session and args.subcommand is None:
        if "package" in last_session:
            args.package = last_session["package"]
        if "bin" in last_session:
            args.bin = last_session["bin"]
        if "release" in last_session:
            args.release = last_session["release"]
        if "target" in last_session:
            args.target = last_session["target"]
        if "speed" in last_session:
            args.speed = last_session["speed"]
        if "chip" in last_session:
            args.chip = last_session["chip"]
    elif is_explicit and args.subcommand is None:
        discovered_pkgs = discover_target_packages(repo_root)
        if not args.package and discovered_pkgs:
            args.package = str(discovered_pkgs[0]["package"])
        if not args.bin and discovered_pkgs:
            matched_pkg = next((p for p in discovered_pkgs if p["package"] == args.package), discovered_pkgs[0])
            args.bin = str(matched_pkg["default_bin"])
        if not args.target and args.package:
            args.target = infer_arm_target(args.package, args.bin, chip=args.chip, repo_root=repo_root)
        save_last_session(
            repo_root,
            {
                "package": args.package,
                "bin": args.bin,
                "release": args.release,
                "target": args.target,
                "speed": args.speed,
                "chip": args.chip,
            },
        )

    # Subcommand handling
    if args.subcommand == "cli":
        cmd_str = " ".join(args.command) if args.command else ""
        if not cmd_str:
            print("Error: Missing CLI command. Usage: host.py cli <command...>", file=sys.stderr)
            return 1
        try:
            res = send_daemon_request("/api/cli/execute", {"command": cmd_str, "wait_for_output": True})
            output = res.get("output", "")
            print(output)
            return 0
        except Exception as e:
            print(f"Error: {e}", file=sys.stderr)
            return 1

    if args.subcommand == "fs":
        cmd_str = " ".join(args.command) if args.command else "ls"
        try:
            res = send_daemon_request("/api/fs/execute", {"command": cmd_str})
            output = res.get("output", "")
            print(output)
            return 0
        except Exception as e:
            print(f"Error: {e}", file=sys.stderr)
            return 1

    if args.subcommand == "status":
        try:
            res = send_daemon_request("/api/status")
            print("Host daemon status:")
            for k, v in res.items():
                print(f"  {k:<15}: {v}")
            return 0
        except Exception as e:
            print(f"Host daemon is not running ({e}).")
            return 1

    if args.subcommand in ("sync", "flash"):
        try:
            res = send_daemon_request("/api/sync", {})
            print("Target synchronization and flash triggered.")
            return 0
        except Exception as e:
            print(f"Could not trigger sync: {e}", file=sys.stderr)
            return 1

    if args.subcommand == "reset":
        try:
            res = send_daemon_request("/api/reset", {})
            print(res.get("message", "Target reset completed."))
            return 0
        except Exception as e:
            print(f"Could not reset target: {e}", file=sys.stderr)
            return 1

    if args.subcommand == "reboot":
        try:
            res = send_daemon_request("/api/reboot", {})
            print(res.get("message", "Target reboot completed."))
            return 0
        except Exception as e:
            print(f"Could not reboot target: {e}", file=sys.stderr)
            return 1

    if args.subcommand == "metadata":
        try:
            res = send_daemon_request("/api/metadata")
            print(json.dumps(res, indent=2))
            return 0
        except Exception:
            from host_daemon.model.metadata import probe_attached_target_metadata

            res = probe_attached_target_metadata(repo_root)
            print(json.dumps(res, indent=2))
            return 0 if res.get("connected") else 1

    if args.subcommand == "stop":
        try:
            res = send_daemon_request("/api/stop", {})
            print(res.get("message", "Host daemon stopped."))
            remove_lock_file()
            return 0
        except Exception as e:
            print(f"Could not stop daemon: {e}", file=sys.stderr)
            return 1

    if args.subcommand == "test":
        from device_tests.client import query_metadata_standalone
        from device_tests.model import (
            DetectedBoard,
            discover_device_tests,
            infer_scenario_from_script,
            matches_board_patterns,
        )

        raw_test_stress = getattr(args, "test_stress_runs", None)
        stress_runs: int = int(raw_test_stress if raw_test_stress is not None else getattr(args, "stress_runs", 1))
        if stress_runs < 1:
            print(
                f"Error: --stress-runs must be >= 1 (got {stress_runs}).",
                file=sys.stderr,
            )
            return 2

        debug_enabled: bool = bool(getattr(args, "debug", False))
        debug_port: int = int(getattr(args, "debug_port", DEFAULT_DEBUGPY_PORT))
        if debug_enabled and not (MIN_TCP_PORT <= debug_port <= MAX_TCP_PORT):
            print(
                f"Error: --debug-port must be between {MIN_TCP_PORT} and {MAX_TCP_PORT} (got {debug_port}).",
                file=sys.stderr,
            )
            return 2

        target_script: Optional[str] = args.script or getattr(args, "script_pos", None)
        project_filter: str = (getattr(args, "project", None) or "all").strip().lower()
        keyword_expr: Optional[str] = (getattr(args, "keyword", None) or "").strip() or None
        resolved_pytest_target = str(repo_root / "tools" / "device_tests" / "suites")
        if target_script:
            file_part, sep, node_part = target_script.partition("::")
            candidate_file = Path(file_part) if Path(file_part).is_absolute() else (repo_root / file_part)
            if not candidate_file.exists():
                print(f"Error: Device test script not found: {candidate_file}", file=sys.stderr)
                return 1
            resolved_pytest_target = f"{candidate_file}{sep}{node_part}" if sep else str(candidate_file)

        if args.list_tests:
            entries = discover_device_tests(
                repo_root,
                project=project_filter,
                script=target_script,
                keyword=keyword_expr,
            )
            if args.json:
                print(json.dumps(entries, indent=2))
            else:
                for entry in entries:
                    bin_tag = entry["target_binary"] or "none"
                    scen_tag = ",".join(entry["scenarios"])
                    print(
                        f"  [{entry['order']:>2}] [{entry['project']}] {entry['name']:<44} ({scen_tag}) [bin={bin_tag}]"
                    )
                    if entry.get("description"):
                        print(f"       {entry['description']}")
            return 0

        detected_board = None
        try:
            meta_res = send_daemon_request(
                "/api/metadata",
                None,
                port=args.host_port,
                host=args.host,
            )
            if isinstance(meta_res, dict) and meta_res.get("connected") and meta_res.get("board_type"):
                detected_board = DetectedBoard.from_metadata_response(meta_res)
        except Exception:
            detected_board = query_metadata_standalone(repo_root)

        if args.list_devices:
            if args.json:
                print(
                    json.dumps(
                        {"detected": detected_board.to_dict() if detected_board else None},
                        indent=2,
                    )
                )
            elif detected_board is not None:
                print(
                    f"Connected Target Board: {detected_board.board_type} "
                    f"(chip={detected_board.chip}, source={detected_board.source})"
                )
            else:
                print("No physical target devices detected.")
            return 0

        if detected_board is None:
            if stress_runs > 1:
                print(
                    "Error: --stress-runs requires an attached physical target board, but none was detected.",
                    file=sys.stderr,
                )
                return 1
            print("ℹ️  No physical target device detected — skipping on-device tests.")
            return 0

        board_filter = (args.board or "auto").strip()
        if board_filter.lower() != "auto" and not matches_board_patterns(detected_board.board_type, [board_filter]):
            print(
                f"ℹ️  Connected board '{detected_board.board_type}' does not match "
                f"--board='{board_filter}' — skipping on-device tests."
            )
            return 0

        if args.scenario is not None and args.scenario.strip():
            raw_scenario = args.scenario.strip()
            if raw_scenario.lower() == "default":
                marker_expr = "smoke or integration"
            elif raw_scenario.lower() == "all":
                marker_expr = ""
            elif "," in raw_scenario:
                marker_expr = " or ".join(part.strip() for part in raw_scenario.split(",") if part.strip())
            else:
                marker_expr = raw_scenario
        elif target_script:
            try:
                marker_expr = infer_scenario_from_script(target_script, repo_root)
            except ValueError as exc:
                print(f"Error: {exc}", file=sys.stderr)
                return 1
        else:
            marker_expr = "smoke or integration"

        print(
            f"🔌 Discovered target board '{detected_board.board_type}' ({detected_board.chip}) — "
            f"running on-device pytest suites (scenario: {marker_expr or 'all'})..."
        )
        if debug_enabled:
            print(
                f"🐞 Waiting for VS Code symbolic debugger (debugpy) to attach on 127.0.0.1:{debug_port} ...\n"
                f"   Select 'Python: Attach to host.py test --debug (Port {debug_port})' in VS Code Run & Debug.",
                flush=True,
            )
            pytest_prefix: List[str] = [
                sys.executable,
                "-m",
                "debugpy",
                "--listen",
                f"127.0.0.1:{debug_port}",
                "--wait-for-client",
                "-m",
                "pytest",
            ]
        else:
            pytest_prefix = [sys.executable, "-m", "pytest"]

        pytest_cmd: List[str] = [
            *pytest_prefix,
            "-c",
            str(repo_root / "pyproject.toml"),
            "--rootdir",
            str(repo_root),
            "-o",
            "addopts=",
            "-v",
            "--maxfail=1",
        ]
        if debug_enabled:
            pytest_cmd.append("-s")
        target_file_resolved = Path(resolved_pytest_target.partition("::")[0]).resolve()
        if not target_file_resolved.is_relative_to((repo_root / "tools" / "device_tests").resolve()):
            pytest_cmd.extend(["-p", "device_tests.conftest"])
        pytest_cmd.extend(
            [
                resolved_pytest_target,
                "--project",
                project_filter,
                "--board",
                board_filter,
                "--host-addr",
                args.host,
            ]
        )
        if marker_expr:
            pytest_cmd.extend(["-m", marker_expr])
        if keyword_expr:
            pytest_cmd.extend(["-k", keyword_expr])
        if args.host_port is not None:
            pytest_cmd.extend(["--host-port", str(args.host_port)])
        if args.manual:
            pytest_cmd.append("--manual")
        if args.report_md:
            pytest_cmd.extend(["--report-md", args.report_md])
        if args.junit_xml:
            pytest_cmd.extend(["--junitxml", args.junit_xml])

        env = dict(os.environ)
        env["XERXES_REQUIRE_CONNECTED_TARGET"] = "1"
        py_paths = [
            str(repo_root / "tools"),
            str(repo_root / "tools" / "validation"),
            str(repo_root / "tools" / "helpers"),
            str(repo_root / "tools" / "dashboard"),
        ]
        if env.get("PYTHONPATH"):
            py_paths.append(env["PYTHONPATH"])
        env["PYTHONPATH"] = os.pathsep.join(py_paths)

        if stress_runs == 1:
            proc = subprocess.run(pytest_cmd, cwd=str(repo_root), env=env, check=False)
            return 0 if proc.returncode in (0, 5) else proc.returncode

        stress_start = time.monotonic()
        passed_runs = 0
        for run_idx in range(1, stress_runs + 1):
            print(
                f"\n{'━' * 70}\n"
                f"🔁 Stress Run [{run_idx}/{stress_runs}] — board '{detected_board.board_type}' ({detected_board.chip})\n"
                f"{'━' * 70}",
                flush=True,
            )
            iter_start = time.monotonic()
            proc = subprocess.run(pytest_cmd, cwd=str(repo_root), env=env, check=False)
            iter_elapsed = time.monotonic() - iter_start
            if proc.returncode not in (0, 5):
                total_elapsed = time.monotonic() - stress_start
                print(
                    f"\n❌ Stress Run [{run_idx}/{stress_runs}] FAILED after {iter_elapsed:.1f}s "
                    f"(exit code {proc.returncode}; {passed_runs}/{stress_runs} runs passed in {total_elapsed:.1f}s).",
                    file=sys.stderr,
                    flush=True,
                )
                return proc.returncode
            passed_runs += 1
            print(
                f"✅ Stress Run [{run_idx}/{stress_runs}] PASSED in {iter_elapsed:.1f}s.",
                flush=True,
            )

        total_elapsed = time.monotonic() - stress_start
        print(
            f"\n{'━' * 70}\n"
            f"✅ Stress Test Summary: {passed_runs}/{stress_runs} runs PASSED in {total_elapsed:.1f}s\n"
            f"{'━' * 70}",
            flush=True,
        )
        return 0

    # Check if daemon already running
    existing = get_lock_data()
    if existing:
        pid = existing.get("pid", 0)
        port = existing.get("port", args.port)
        if is_pid_alive(pid) or is_port_in_use(port, args.host):
            if getattr(args, "restart", False):
                print(f"Restarting host daemon (stopping existing PID {pid})...")
                try:
                    send_daemon_request("/api/stop", {}, port=port, host=args.host)
                except Exception:
                    pass
                deadline = time.time() + 3.0
                while time.time() < deadline and is_pid_alive(pid):
                    time.sleep(0.05)
                remove_lock_file()
            else:
                req_profile = "release" if args.release else "debug"
                config_changed = is_explicit and (
                    (args.bin and args.bin != existing.get("bin"))
                    or (args.package and args.package != existing.get("package"))
                    or (args.target and args.target != existing.get("target"))
                    or (req_profile != existing.get("profile"))
                    or (args.speed and args.speed != existing.get("speed"))
                )
                if config_changed:
                    print(f"➜ Host daemon is running (PID {pid} on port {port}).")
                    print(f"➜ Reconfiguring and flashing target: {args.package} ({args.bin}) [{req_profile}]...")
                    try:
                        send_daemon_request(
                            "/api/sync",
                            {
                                "target": args.target,
                                "package": args.package,
                                "bin": args.bin,
                                "release": args.release,
                                "speed": args.speed,
                                "chip": args.chip,
                            },
                            port=port,
                            host=args.host,
                        )
                        print("➜ Target synchronization and flash started.")
                        print(f"Workstation URL: http://{args.host}:{port}")
                        return 0
                    except Exception as e:
                        print(
                            f"Warning: Could not reconfigure running daemon ({e}). Restarting daemon...",
                            file=sys.stderr,
                        )
                        try:
                            send_daemon_request("/api/stop", {}, port=port, host=args.host)
                        except Exception:
                            pass
                        deadline = time.time() + 3.0
                        while time.time() < deadline and is_pid_alive(pid):
                            time.sleep(0.05)
                        remove_lock_file()
                else:
                    print(f"Host daemon is already running (PID {pid} on port {port}).")
                    print(f"Workstation URL: http://{args.host}:{port}")
                    return 0
        remove_lock_file()

    # Detach from terminal unless --no-detach / --foreground is requested
    if not getattr(args, "no_detach", False):
        log_dir = repo_root / "target" if (repo_root / "target").exists() else repo_root / "build"
        log_dir.mkdir(parents=True, exist_ok=True)
        log_path = log_dir / "host_daemon.log"
        log_file = open(log_path, "a", encoding="utf-8")

        child_args = (
            [sys.executable, str(Path(__file__).resolve().parent.parent / "host.py")]
            + [a for a in sys.argv[1:] if a not in ("--no-detach", "--foreground", "--detach")]
            + ["--no-detach"]
        )

        popen_kwargs: Dict[str, Any] = {
            "stdout": log_file,
            "stderr": log_file,
            "stdin": subprocess.DEVNULL,
        }
        if os.name == "posix":
            popen_kwargs["start_new_session"] = True
        elif sys.platform in ("win32", "win64"):
            detached_flag = getattr(subprocess, "DETACHED_PROCESS", 0x00000008)
            new_grp_flag = getattr(subprocess, "CREATE_NEW_PROCESS_GROUP", 0x00000200)
            popen_kwargs["creationflags"] = detached_flag | new_grp_flag

        proc = subprocess.Popen(child_args, **popen_kwargs)

        deadline = time.time() + 4.0
        while time.time() < deadline:
            if proc.poll() is not None:
                print("Error: Host daemon failed to start.", file=sys.stderr)
                return 1
            lock_data = get_lock_data()
            if lock_data and lock_data.get("pid") == proc.pid:
                break
            time.sleep(0.05)

        print(f"➜ Host daemon running at http://{args.host}:{args.port} (PID {proc.pid}).")
        print("➜ Detached from terminal.")
        return 0

    runner = DaemonRunner(
        repo_root=repo_root,
        target=args.target,
        package=args.package,
        bin_name=args.bin,
        release=args.release,
        speed=args.speed,
        chip=args.chip,
        package_selection_mode=package_selection_mode,
        show_system_logs=getattr(args, "show_system_logs", False),
    )
    atexit.register(runner.stop)
    atexit.register(remove_lock_file)

    def _sig_handler(signum: int, frame: Any) -> None:
        try:
            runner.stop()
        except Exception:
            pass
        remove_lock_file()
        sys.exit(0)

    try:
        signal.signal(signal.SIGTERM, _sig_handler)
        signal.signal(signal.SIGINT, _sig_handler)
    except Exception:
        pass

    # Start HTTP server first so workstation UI and /api/sync are immediately accessible
    try:
        server = HostDaemonServer(args.host, args.port, runner)
    except OSError as e:
        print(f"Failed to bind port {args.port}: {e}", file=sys.stderr)
        return 1

    url = f"http://{args.host}:{args.port}"
    write_lock_file(
        {
            "pid": os.getpid(),
            "port": args.port,
            "host": args.host,
            "target": args.target,
            "package": args.package,
            "bin": args.bin,
            "profile": "release" if args.release else "debug",
            "url": url,
            "browser": getattr(args, "browser", "webview"),
            "no_browser": getattr(args, "no_browser", False),
            "package_selection_mode": package_selection_mode,
        }
    )

    # Trigger asynchronous target synchronization unless awaiting package selection in UI
    if not package_selection_mode:
        runner.sync_async()
    else:
        print("➜ Target package selection mode: awaiting configuration modal confirmation in workstation.")

    print(f"➜ Host daemon running at {url} (PID {os.getpid()}).")
    print("  Press Ctrl+C to stop daemon.\n")

    server_thread = threading.Thread(target=server.serve_forever, daemon=True)
    server_thread.start()

    # Launch dedicated desktop application window if not disabled
    if not getattr(args, "no_browser", False):

        def _on_app_close() -> None:
            def _shutdown() -> None:
                try:
                    runner.stop()
                except Exception:
                    pass
                try:
                    server.shutdown()
                except Exception:
                    pass
                remove_lock_file()
                os._exit(0)

            threading.Thread(target=_shutdown, daemon=True).start()

        launch_host_daemon_app(
            url,
            browser=getattr(args, "browser", "webview"),
            on_close=_on_app_close,
        )

    try:
        while server_thread.is_alive():
            time.sleep(0.5)
    except KeyboardInterrupt:
        print("\nStopping host daemon...")
    finally:
        runner.stop()
        server.shutdown()
        remove_lock_file()

    return 0
