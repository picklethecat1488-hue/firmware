"""Mock JSON-events RTT session simulating a 0.6s USB reconnect + 0.3s `Board::init()` boot delay."""

import json
import os
import queue
import sys
import threading
import time


def main() -> None:
    """Discard premature stdin prior to `session_started` and reply to `system activity` once booted."""
    assert "--json-events" in sys.argv, f"Expected --json-events in {sys.argv}"
    cmd_queue: "queue.Queue[str]" = queue.Queue()

    def _reader() -> None:
        for line in sys.stdin:
            cmd_queue.put(line.strip())
        os._exit(0)

    threading.Thread(target=_reader, daemon=True).start()
    time.sleep(0.60)
    # Hardware reset wipes any stdin commands sent prematurely before RTT session started.
    while not cmd_queue.empty():
        cmd_queue.get_nowait()

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
    time.sleep(0.30)
    print(
        json.dumps(
            {
                "event": "defmt",
                "level": "INFO",
                "module": "platform::core_monitor",
                "device_ts": 0.023,
                "message": "MPU stack guard configured",
                "formatted": "0.023000 INFO [platform::core_monitor] MPU stack guard configured",
            }
        ),
        flush=True,
    )
    print(json.dumps({"event": "shell_idle", "epoch": 1}), flush=True)
    while True:
        cmd = cmd_queue.get()
        if cmd == "system activity":
            print(
                json.dumps(
                    {
                        "event": "defmt",
                        "level": "INFO",
                        "module": "controller::shell_controller",
                        "device_ts": 0.412,
                        "message": "received command System",
                        "formatted": "0.412000 INFO [controller::shell_controller] received command System",
                    }
                ),
                flush=True,
            )
            print(json.dumps({"event": "cli_output", "text": "System activity recorded."}), flush=True)
            print(
                json.dumps(
                    {
                        "event": "command_completed",
                        "success": True,
                        "error": None,
                        "device_ts": 0.413,
                    }
                ),
                flush=True,
            )
            print(json.dumps({"event": "shell_idle", "epoch": 2}), flush=True)


if __name__ == "__main__":
    main()
