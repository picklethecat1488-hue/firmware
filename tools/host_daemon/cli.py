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
from typing import Any, Dict, List, Optional
import urllib.error
import urllib.request

from .runner import DaemonRunner
from .server import HostDaemonServer

LOCK_FILE: Path = Path("/tmp/xerxes_host_daemon.lock")


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


def send_daemon_request(path: str, payload: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
    """Send an HTTP request to an active running host daemon."""
    lock = get_lock_data()
    if not lock:
        raise RuntimeError("No active host daemon detected (lock file /tmp/xerxes_host_daemon.lock not found).")

    pid = lock.get("pid", 0)
    port = lock.get("port", 8899)
    host = lock.get("host", "127.0.0.1")

    if not is_pid_alive(pid) and not is_port_in_use(port, host):
        remove_lock_file()
        raise RuntimeError(f"Host daemon (PID {pid}) is not running.")

    url = f"http://{host}:{port}{path}"
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
        help="Desktop window display environment.",
    )
    parser.add_argument(
        "--detach",
        action="store_true",
        help="Detach daemon from current terminal and run in background.",
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
            print("======================================================================")
            print("  ⚡ SYSTEM SHOCK 2 // XERXES HOST DAEMON STATUS")
            print("======================================================================")
            for k, v in res.items():
                print(f"  {k:<15}: {v}")
            return 0
        except Exception as e:
            print(f"Host daemon is not running ({e}).")
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
            print(f"Host daemon is already running (PID {pid} on port {port}).")
            print(f"Workstation URL: http://{args.host}:{port}")
            return 0
        remove_lock_file()

    # Detach if requested
    if args.detach:
        cmd = [sys.executable, str(Path(__file__).resolve().parent.parent / "host_daemon.py")]
        for a in sys.argv[1:]:
            if a != "--detach":
                cmd.append(a)
        subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
        print("Host daemon spawned in background.")
        return 0

    print("======================================================================")
    print("  ⚡ SYSTEM SHOCK 2 // XERXES HOST DAEMON WORKSTATION")
    print("======================================================================")
    print(f"  Target:    {args.target}")
    print(f"  Package:   {args.package} ({args.bin})")
    print(f"  Profile:   {'release' if args.release else 'debug'}")
    print(f"  Speed:     {args.speed} kHz")
    print(f"  Port:      {args.port}")

    runner = DaemonRunner(
        repo_root=repo_root,
        target=args.target,
        package=args.package,
        bin_name=args.bin,
        release=args.release,
        speed=args.speed,
        chip=args.chip,
    )

    # 1. Compile target binary
    try:
        runner.build_target()
    except Exception as e:
        print(f"Error compiling target firmware: {e}", file=sys.stderr)
        return 1

    # 2. Compile host tools
    try:
        runner.build_host_tools()
    except Exception as e:
        print(f"Error compiling host tools: {e}", file=sys.stderr)
        return 1

    # 3. Flash target (gracefully falls back if no probe attached)
    runner.flash_target()

    # 4. Start RTT streaming
    runner.start_rtt_streaming()

    # 5. Start HTTP server
    try:
        server = HostDaemonServer(args.host, args.port, runner)
    except OSError as e:
        print(f"Failed to bind port {args.port}: {e}", file=sys.stderr)
        return 1

    write_lock_file(
        {
            "pid": os.getpid(),
            "port": args.port,
            "host": args.host,
            "target": args.target,
            "package": args.package,
            "bin": args.bin,
            "profile": "release" if args.release else "debug",
            "url": f"http://{args.host}:{args.port}",
        }
    )

    print(f"\n  Workstation URL: http://{args.host}:{args.port}")
    print("  Press Ctrl+C to stop daemon.\n")

    server_thread = threading.Thread(target=server.serve_forever, daemon=True)
    server_thread.start()

    # Launch desktop UI window if not disabled
    if not args.no_browser:
        try:
            from dashboard.cli import launch_browser

            def _window_closed() -> None:
                runner.stop()
                server.shutdown()

            launch_browser(
                f"http://{args.host}:{args.port}",
                target=args.browser,
                window_title=f"Xerxes Host Daemon // {args.package}:{args.bin}",
                on_closed=_window_closed,
            )
        except Exception:
            pass

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
