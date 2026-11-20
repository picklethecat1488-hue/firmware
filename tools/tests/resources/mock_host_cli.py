"""Mock `host_cli` executable fixture recording CLI argv and emitting `session_started` + `shell_idle` JSON events."""

import json
import os
from pathlib import Path
import sys


def main() -> None:
    """Record `sys.argv[1:]` to `XERXES_MOCK_CLI_ARGV_LOG` (if set) and hold RTT stream open until stdin EOF."""
    argv_log = os.environ.get("XERXES_MOCK_CLI_ARGV_LOG")
    if argv_log:
        Path(argv_log).write_text(json.dumps(sys.argv[1:]), encoding="utf-8")

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
    print(json.dumps({"event": "shell_idle", "epoch": 1}), flush=True)
    sys.stdin.read()


if __name__ == "__main__":
    main()
