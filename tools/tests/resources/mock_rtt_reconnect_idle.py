"""Mock JSON-events RTT session emitting pre-completion `shell_idle` epochs before `command_completed`."""

import json
import os
import queue
import sys
import threading
import time


def main() -> None:
    """Verify early/reconnect `shell_idle` events never truncate slow I2C CLI responses."""
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
        if cmd == "battery status":
            # Simulate both a duplicate reconnect epoch=1 and a pre-command idle epoch=2
            # followed by a 0.40s (> CLI_COMPLETION_SETTLE_SEC=0.25s) I2C transaction.
            print(json.dumps({"event": "shell_idle", "epoch": 1}), flush=True)
            print(json.dumps({"event": "shell_idle", "epoch": 2}), flush=True)
            time.sleep(0.40)
            print(json.dumps({"event": "cli_output", "text": "Battery Status:"}), flush=True)
            print(json.dumps({"event": "cli_output", "text": "  Voltage: 4022 mV"}), flush=True)
            print(
                json.dumps(
                    {
                        "event": "command_completed",
                        "success": True,
                        "error": None,
                        "device_ts": 0.85,
                    }
                ),
                flush=True,
            )
            print(json.dumps({"event": "shell_idle", "epoch": 3}), flush=True)


if __name__ == "__main__":
    main()
