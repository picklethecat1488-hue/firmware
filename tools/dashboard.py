#!/usr/bin/env python3
"""Interactive System Shock 2 - Xerxes VCS Dashboard, Code Review, and Bug Report Workstation CLI.

Launches a unified local browser-based workstation with a System Shock 2 - Xerxes sci-fi HUD theme,
providing Smartlog ancestor DAG tree navigation, staged/unstaged/untracked file management,
commit split/combine, merge conflict resolution, interactive line-by-line Code Review,
and integrated Bug Tracker, all served under a single endpoint.

Usage:
    python tools/dashboard.py
    python tools/dashboard.py --port 8777
    python tools/dashboard.py --list
    python tools/dashboard.py --branch main
    python tools/dashboard.py --goto 75d5f92
    python tools/dashboard.py --bugs
    python tools/dashboard.py --add-bug "Motor overrun in tick loop" --severity HIGH --category CONTROLLER
    python tools/dashboard.py --resolve-bug BUG-001 --notes "Adjusted PID timing constants"
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
    handle_cli_commit,
    launch_browser,
    main,
    parse_arguments,
    print_cli_bugs,
    print_cli_reviews,
    print_cli_smartlog,
)

__all__ = [
    "handle_cli_commit",
    "launch_browser",
    "main",
    "parse_arguments",
    "print_cli_bugs",
    "print_cli_reviews",
    "print_cli_smartlog",
]

if __name__ == "__main__":
    main()
