#!/usr/bin/env python3
"""Interactive System Shock 2 - Xerxes Host Daemon Workstation CLI.

Builds target firmware and host tools, flashes hardware devices, streams RTT defmt
logging and CLI shell interactions, and serves an interactive web workstation for
executing host_cli and host_fs subcommands.

Usage:
    PROBE_RS_SPEED=4000 python tools/host_daemon.py --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release
    python tools/host_daemon.py cli motor speed 100
    python tools/host_daemon.py fs ls
    python tools/host_daemon.py fs cat metadata
    python tools/host_daemon.py status
    python tools/host_daemon.py stop
"""

from pathlib import Path
import sys

# Ensure tools and tools/host_daemon are on sys.path
_tools_dir = Path(__file__).resolve().parent
_daemon_dir = _tools_dir / "host_daemon"
_dashboard_dir = _tools_dir / "dashboard"

for p in (str(_tools_dir), str(_daemon_dir), str(_dashboard_dir)):
    if p not in sys.path:
        sys.path.insert(0, p)

from host_daemon.cli import main

if __name__ == "__main__":
    sys.exit(main())
