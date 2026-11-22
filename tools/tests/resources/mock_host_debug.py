"""Mock `host_debug` executable fixture handling `serve`, `resolve`, and `probe-status` subcommands."""

import json
import os
from pathlib import Path
import sys


def main() -> None:
    """Record `sys.argv[1:]` to `XERXES_MOCK_DEBUG_ARGV_LOG` (if set) and simulate `host_debug` subcommands."""
    args = sys.argv[1:]
    argv_log = os.environ.get("XERXES_MOCK_DEBUG_ARGV_LOG")
    if argv_log:
        Path(argv_log).write_text(json.dumps(args), encoding="utf-8")

    subcmd = args[0] if args else "serve"
    if subcmd == "probe-status":
        print(
            json.dumps(
                {
                    "interface": "swd_usb",
                    "availability": "available",
                    "in_use": False,
                    "device_count": 1,
                    "devices": [{"identifier": "Mock Probe"}],
                    "error": None,
                }
            ),
            flush=True,
        )
        return

    if subcmd == "resolve":
        elf_idx = args.index("--elf")
        specs = args[elf_idx + 2 :]
        print(json.dumps({spec: 0x100001C0 + idx * 4 for idx, spec in enumerate(specs)}), flush=True)
        return

    assert subcmd == "serve", f"Expected host_debug subcommand 'serve', got {args!r}"
    c0 = int(args[args.index("--gdb-core0-port") + 1])
    c1 = int(args[args.index("--gdb-core1-port") + 1])
    rtt = int(args[args.index("--rtt-bridge-port") + 1])
    halted = "--halt-on-reset" in args
    log_file = Path(args[args.index("--log-file") + 1]) if "--log-file" in args else None
    ready_line = json.dumps(
        {
            "event": "ready",
            "chip": "rp2040",
            "cores": 2,
            "gdb_core0_port": c0,
            "gdb_core1_port": c1,
            "rtt_bridge_port": rtt,
            "halted_on_reset": halted,
        }
    )
    if log_file is not None:
        log_file.parent.mkdir(parents=True, exist_ok=True)
        with log_file.open("a", encoding="utf-8") as fh:
            fh.write(ready_line + "\n")
    print(ready_line, flush=True)
    sys.stdin.read()
    stopped_line = json.dumps({"event": "stopped"})
    if log_file is not None:
        with log_file.open("a", encoding="utf-8") as fh:
            fh.write(stopped_line + "\n")
    print(stopped_line, flush=True)


if __name__ == "__main__":
    main()
