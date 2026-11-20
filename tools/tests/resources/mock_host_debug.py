"""Mock `host_debug` executable fixture recording CLI argv and emitting structured `ready`/`stopped` JSON events."""

import json
import os
from pathlib import Path
import sys


def main() -> None:
    """Record `sys.argv[1:]` to `XERXES_MOCK_DEBUG_ARGV_LOG` (if set) and block on stdin until EOF."""
    argv_log = os.environ.get("XERXES_MOCK_DEBUG_ARGV_LOG")
    if argv_log:
        Path(argv_log).write_text(json.dumps(sys.argv[1:]), encoding="utf-8")

    c0 = int(sys.argv[sys.argv.index("--gdb-core0-port") + 1])
    c1 = int(sys.argv[sys.argv.index("--gdb-core1-port") + 1])
    rtt = int(sys.argv[sys.argv.index("--rtt-bridge-port") + 1])
    halted = "--halt-on-reset" in sys.argv
    print(
        json.dumps(
            {
                "event": "ready",
                "chip": "rp2040",
                "cores": 2,
                "gdb_core0_port": c0,
                "gdb_core1_port": c1,
                "rtt_bridge_port": rtt,
                "halted_on_reset": halted,
            }
        ),
        flush=True,
    )
    sys.stdin.read()
    print(json.dumps({"event": "stopped"}), flush=True)


if __name__ == "__main__":
    main()
