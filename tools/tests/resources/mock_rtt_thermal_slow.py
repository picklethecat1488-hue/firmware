r"""Mock JSON-events RTT session emitting an empty `\x1b[2K` prompt clear before a 450ms `thermal status` read."""

import json
import os
import queue
import sys
import threading
import time


def main() -> None:
    """Emit structured JSON RTT events for slow `thermal status` verification."""
    cmd_queue: "queue.Queue[str]" = queue.Queue()

    def _reader() -> None:
        for line in sys.stdin:
            cmd_queue.put(line.strip())
        os._exit(0)

    threading.Thread(target=_reader, daemon=True).start()
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
    while True:
        cmd = cmd_queue.get()
        if cmd == "thermal status":
            print("\x1b[2K", flush=True)
            time.sleep(0.45)
            print(json.dumps({"event": "cli_output", "text": "Thermal: 24.8 C"}), flush=True)
            print(
                json.dumps(
                    {
                        "event": "command_completed",
                        "success": True,
                        "error": None,
                        "device_ts": 0.95,
                    }
                ),
                flush=True,
            )
            print(json.dumps({"event": "shell_idle", "epoch": 2}), flush=True)


if __name__ == "__main__":
    main()
