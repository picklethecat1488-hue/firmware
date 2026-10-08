#!/usr/bin/env python3
"""Interactive System Shock 2 - Xerxes VCS Dashboard, Code Review, and Worm Report Workstation CLI.

Launches a unified local browser-based workstation with a System Shock 2 - Xerxes sci-fi HUD theme,
providing Smartlog ancestor DAG tree navigation, staged/unstaged/untracked file management,
commit split/combine, merge conflict resolution, interactive line-by-line Code Review,
and integrated Worm Tracker, all served under a single endpoint.

Usage:
    python tools/dashboard.py
    python tools/dashboard.py --port 8877
    python tools/dashboard.py --list
    python tools/dashboard.py --branch main
    python tools/dashboard.py --goto 75d5f92
    python tools/dashboard.py --worms
    python tools/dashboard.py --add-worm "Motor overrun in tick loop" --severity HIGH --category CONTROLLER
    python tools/dashboard.py --resolve-worm WORM-001 --notes "Adjusted PID timing constants"
    python tools/dashboard.py --reviews
    python tools/dashboard.py --add-comment "Verify DMA buffer alignment" --file app/src/main.rs --line 42
    python tools/dashboard.py --resolve-comment abc123
    python tools/dashboard.py --verdict APPROVED
"""

from pathlib import Path
import sys

# Add tools/dashboard and tools/ to sys.path so imports succeed in any environment
_dashboard_dir = Path(__file__).resolve().parent / "dashboard"
_tools_dir = Path(__file__).resolve().parent
if str(_dashboard_dir) not in sys.path:
    sys.path.insert(0, str(_dashboard_dir))
if str(_tools_dir) not in sys.path:
    sys.path.insert(0, str(_tools_dir))

from dashboard.cli import (  # noqa: E402
    check_existing_instance,
    get_dashboard_lock_file,
    handle_cli_commit,
    is_pid_alive,
    is_port_in_use,
    launch_browser,
    main,
    parse_arguments,
    print_cli_reviews,
    print_cli_smartlog,
    print_cli_worms,
)

__all__ = [
    "check_existing_instance",
    "get_dashboard_lock_file",
    "handle_cli_commit",
    "is_pid_alive",
    "is_port_in_use",
    "launch_browser",
    "main",
    "parse_arguments",
    "print_cli_reviews",
    "print_cli_smartlog",
    "print_cli_worms",
]

if __name__ == "__main__":
    main()
