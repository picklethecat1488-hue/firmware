"""Command-line interface and daemon management for Xerxes Host Daemon."""

import argparse
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import threading
import time
from typing import Any, Callable, Dict, List, Optional
import urllib.error
import urllib.request

from .runner import DaemonRunner
from .server import HostDaemonServer

_repo_root = Path(__file__).resolve().parent.parent.parent
if str(_repo_root) not in sys.path:
    sys.path.insert(0, str(_repo_root))
_tools_dir = _repo_root / "tools"
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

LOCK_FILE: Path = Path("/tmp/xerxes_host_daemon.lock")


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

    return parser.parse_args(args)


def main(argv: Optional[List[str]] = None) -> int:
    """Execute host daemon CLI commands or launch daemon server."""
    args = parse_arguments(argv)
    repo_root = Path(__file__).resolve().parent.parent.parent

    # Subcommand handling
    if args.subcommand == "cli":
        cmd_str = " ".join(args.command) if args.command else ""
        if not cmd_str:
            print("Error: Missing CLI command. Usage: host_daemon.py cli <command...>", file=sys.stderr)
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
                config_changed = (
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
                        print(f"Warning: Could not reconfigure running daemon ({e}). Restarting daemon...", file=sys.stderr)
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
            [sys.executable, str(Path(__file__).resolve().parent.parent / "host_daemon.py")]
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
    )

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
        }
    )

    # Trigger asynchronous target synchronization (compiles target, compiles host tools, flashes, connects RTT)
    runner.sync_async()

    print(f"➜ Host daemon running at {url} (PID {os.getpid()}).")
    print("  Press Ctrl+C to stop daemon.\n")

    server_thread = threading.Thread(target=server.serve_forever, daemon=True)
    server_thread.start()

    # Launch dedicated desktop application window if not disabled
    if not getattr(args, "no_browser", False):
        def _on_app_close() -> None:
            def _shutdown() -> None:
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
