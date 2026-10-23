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

from host_daemon.provider.daemon.config import has_explicit_target_args, load_last_session, save_last_session
from host_daemon.provider.daemon.runner import DaemonRunner
from host_daemon.provider.daemon.server import HostDaemonServer

_repo_root = Path(__file__).resolve().parent.parent.parent
if str(_repo_root) not in sys.path:
    sys.path.insert(0, str(_repo_root))
_tools_dir = _repo_root / "tools"
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

LOCK_FILE: Path = Path("/tmp/xerxes_host_daemon.lock")


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
    static_dir = Path(__file__).resolve().parent / "static"
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

    if not lock and port is None:
        raise RuntimeError("No active host daemon detected (lock file /tmp/xerxes_host_daemon.lock not found).")

    if lock and not is_pid_alive(pid) and not is_port_in_use(req_port, req_host):
        remove_lock_file()
        raise RuntimeError(f"Host daemon (PID {pid}) is not running.")

    url = f"http://{req_host}:{req_port}{path}"
    headers = {"Content-Type": "application/json"}
    data = json.dumps(payload).encode("utf-8") if payload is not None else None
    req = urllib.request.Request(url, data=data, headers=headers, method="POST" if payload is not None else "GET")

    try:
        with urllib.request.urlopen(req, timeout=15) as resp:
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
        default="thumbv6m-none-eabi",
        help="Embedded target architecture triple.",
    )
    parser.add_argument(
        "--package",
        type=str,
        default="cat_detector",
        help="Cargo package name containing the firmware application.",
    )
    parser.add_argument(
        "--bin",
        type=str,
        default="cat_detector_shell",
        help="Binary name to build and flash.",
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

    # Reset
    subparsers.add_parser("reset", help="Reset the target device via active host daemon")

    # Stop
    subparsers.add_parser("stop", help="Stop running host daemon")

    # Bringup
    bringup_parser = subparsers.add_parser("bringup", help="Parse and run bringup verification scripts")
    bringup_parser.add_argument("--script", "-s", type=str, default=None, help="Path to bringup YAML script")
    bringup_parser.add_argument("--list", action="store_true", help="List discovered bringup scripts in repository")
    bringup_parser.add_argument("--status", action="store_true", help="Show current bringup checklist status")
    bringup_parser.add_argument("--run", action="store_true", help="Execute bringup checklist steps")
    bringup_parser.add_argument("--step", action="store_true", help="Execute single bringup step")
    bringup_parser.add_argument("--reset", action="store_true", help="Reset bringup checklist execution")
    bringup_parser.add_argument(
        "--output", "-o", type=str, default="bringup_report.md", help="Path to output markdown report"
    )
    bringup_parser.add_argument(
        "--manual", action="store_true", help="Run in manual prompt mode without hardware probe"
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
            res = send_daemon_request("/api/cli/execute", {"command": cmd_str})
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

    if args.subcommand == "stop":
        try:
            res = send_daemon_request("/api/stop", {})
            print(res.get("message", "Host daemon stopped."))
            remove_lock_file()
            return 0
        except Exception as e:
            print(f"Could not stop daemon: {e}", file=sys.stderr)
            return 1

    if args.subcommand == "bringup":
        from host_daemon.model.bringup import discover_bringup_scripts
        from host_daemon.provider.bringup.bringup_manager import BringupManager

        # 1. List scripts if requested
        if args.list:
            scripts = discover_bringup_scripts(repo_root)
            print("Available Bringup Checklist Scripts:")
            for s in scripts:
                print(f"  • {s['name']:<25} ({s['chip']}) - {s['path']} [{s['steps_count']} steps]")
            return 0

        # 2. Check if active host daemon can handle the request
        daemon_alive = False
        try:
            lock_data = get_lock_data()
            if lock_data and is_pid_alive(lock_data.get("pid", 0)):
                daemon_alive = True
        except Exception:
            daemon_alive = False

        if daemon_alive and not args.manual:
            try:
                if args.script:
                    send_daemon_request("/api/bringup/load", {"script_path": args.script})
                    print(f"Loaded bringup script '{args.script}' into host daemon.")

                if args.reset:
                    send_daemon_request("/api/bringup/reset", {})
                    print("Bringup checklist reset to beginning on host daemon.")

                if args.status:
                    st = send_daemon_request("/api/bringup/status")
                    print(f"Bringup Checklist Status: {st.get('script_name', 'None')} ({st.get('status', 'unknown')})")
                    print(f"  Current Step: {st.get('current_step_idx', 0) + 1} / {st.get('total_steps', 0)}")
                    summary = st.get("summary", {})
                    print(
                        f"  Passed: {summary.get('passed', 0)}, Failed: {summary.get('failed', 0)}, Skipped: {summary.get('skipped', 0)}, Pending: {summary.get('pending', 0)}"
                    )
                    return 0

                if args.step:
                    res = send_daemon_request("/api/bringup/step", {})
                    print(f"Executed step on host daemon: {res}")
                    return 0
            except Exception as e:
                print(f"Host daemon interaction failed: {e}. Falling back to standalone mode.")
                daemon_alive = False

        # 3. Standalone Bringup Execution
        script_path = args.script
        if not script_path:
            discovered = discover_bringup_scripts(repo_root)
            if discovered:
                script_path = discovered[0]["path"]
            else:
                print("Error: No bringup script specified and none found in repository.", file=sys.stderr)
                return 1

        resolved_script = repo_root / script_path if not Path(script_path).is_absolute() else Path(script_path)
        if not resolved_script.exists():
            print(f"Error: Bringup script not found: {resolved_script}", file=sys.stderr)
            return 1

        manager = BringupManager(repo_root=repo_root)
        try:
            cfg = manager.load_script(resolved_script)
        except Exception as e:
            print(f"Error loading bringup YAML: {e}", file=sys.stderr)
            return 1

        if args.status:
            print(f"Bringup Checklist: {cfg.project_name} ({cfg.device_chip})")
            print(f"  Total Steps: {len(cfg.steps)}")
            for s in cfg.steps:
                print(f"  [{s.id}] {s.name:<40} ({s.type.value}) - {s.status.value.upper()}")
            return 0

        # Execute checklist interactively
        print("=" * 60)
        print(f"BRINGUP CHECKLIST ASSISTANT: {cfg.project_name} ({cfg.device_chip})")
        rel_disp = (
            resolved_script.relative_to(repo_root) if resolved_script.is_relative_to(repo_root) else resolved_script
        )
        print(f"Script: {rel_disp}")
        print(f"Total Steps: {len(cfg.steps)}")
        print("=" * 60)

        for idx, step in enumerate(cfg.steps):
            print(f"\n--- STEP {idx + 1}/{len(cfg.steps)}: {step.name} (ID: {step.id}) [{step.type.value.upper()}] ---")
            if step.description:
                print(f"  Description: {step.description}")
            if step.procedure:
                print(f"  Procedure: {step.procedure}")
            if step.expected:
                print(f"  Expected: {step.expected}")
            if step.test_points:
                print("  Test Points:")
                for tp, val in step.test_points.items():
                    print(f"    • {tp}: {val}")
            if step.control_pins:
                print("  Control Pins:")
                for cp, val in step.control_pins.items():
                    print(f"    • {cp}: {val}")

            res = manager.step()
            prompt = res.get("prompt", {})
            auto_pass = prompt.get("auto_pass")
            def_status = prompt.get("default_status", "passed")

            if step.output_log:
                print(f"  Output:\n{step.output_log.strip()}")

            if auto_pass is True:
                print(f"  ✔ Check passed: {step.matched_line or 'OK'}")
            elif auto_pass is False:
                print(f"  ✘ Check failed (expected pattern not matched)")

            try:
                pass_choice = format_clickable_choice("Pass", "P", "pass", "\033[1;32m")
                fail_choice = format_clickable_choice("Fail", "F", "fail", "\033[1;31m")
                skip_choice = format_clickable_choice("Skip", "S", "skip", "\033[1;33m")
                prompt_str = f"Status {pass_choice}, {fail_choice}, {skip_choice} (default: {def_status.upper()}): "
                choice = input(prompt_str).strip().lower()
                if not choice:
                    choice = def_status
                notes = input("Notes (optional): ").strip()
                manager.respond(action=choice, notes=notes)
            except (KeyboardInterrupt, EOFError):
                print("\nChecklist run interrupted by user.")
                break

        # Generate markdown report
        report_path = repo_root / args.output if not Path(args.output).is_absolute() else Path(args.output)
        manager.generate_report(output_path=report_path)
        print(f"\nBringup report generated: {report_path}")
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
