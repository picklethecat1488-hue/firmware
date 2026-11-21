"""Mock `host_cli` executable fixture recording CLI argv and emitting `session_started` + `shell_idle` JSON events."""

import json
import os
from pathlib import Path
import sys


def main() -> None:
    """Record `sys.argv[1:]` to `XERXES_MOCK_CLI_ARGV_LOG` (if set) and serve `boot-status` or RTT JSON events."""
    cli_args = sys.argv[1:]
    argv_log = os.environ.get("XERXES_MOCK_CLI_ARGV_LOG")
    if argv_log:
        Path(argv_log).write_text(json.dumps(cli_args), encoding="utf-8")

    omit_shell_idle = os.environ.get("XERXES_MOCK_OMIT_SHELL_IDLE") == "1"
    if "boot-status" in cli_args or "--boot-status" in cli_args:
        report = {
            "chip": "rp2040",
            "state": "ready",
            "boot_ready": True,
            "shell_idle": not omit_shell_idle,
            "epoch": 0 if omit_shell_idle else 1,
        }
        if "--json" in cli_args:
            print(json.dumps(report), flush=True)
        else:
            for k, v in report.items():
                print(f"{k}: {v}", flush=True)
        return

    print(
        json.dumps(
            {
                "event": "session_started",
                "chip": "rp2040",
                "defmt": True,
                "cli_up": True,
                "cli_down": True,
            }
        ),
        flush=True,
    )
    print(
        json.dumps(
            {
                "event": "boot_status",
                "state": "ready",
                "boot_ready": True,
                "shell_idle": not omit_shell_idle,
                "epoch": 0 if omit_shell_idle else 1,
            }
        ),
        flush=True,
    )
    if not omit_shell_idle:
        print(json.dumps({"event": "shell_idle", "epoch": 1}), flush=True)
    sys.stdin.read()


if __name__ == "__main__":
    main()
