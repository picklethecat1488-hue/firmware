"""Mock plain-text RTT session simulating a 250ms `sensor status` read under concurrent metadata probes."""

import os
import queue
import sys
import threading
import time


def main() -> None:
    """Stream mock RTT console lines and respond to `sensor status` on stdin."""
    cmd_queue: "queue.Queue[str]" = queue.Queue()

    def _stdin_reader() -> None:
        for line in sys.stdin:
            cmd_queue.put(line.strip())
        os._exit(0)

    threading.Thread(target=_stdin_reader, daemon=True).start()
    print("RTT Session Started", flush=True)
    print("0.023200 INFO [platform::core_monitor] MPU stack guard configured", flush=True)
    while True:
        cmd = cmd_queue.get()
        if cmd == "sensor status":
            time.sleep(0.25)
            print("0.714500 INFO [controller::shell_controller] received command Sensor", flush=True)
            print("north                                   east                                    west", flush=True)
            print("1098mm                                  INVALID                                 546mm", flush=True)
            print("0.714900 INFO [controller::shell_controller] command execution succeeded", flush=True)
            print("\x1b[2Kshell> ", flush=True)


if __name__ == "__main__":
    main()
