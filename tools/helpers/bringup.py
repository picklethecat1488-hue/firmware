#!/usr/bin/env python3
"""Target-agnostic bringup checklist assistant script.

Walks the user through the specified project bringup checklist, executes host
commands, interacts with the device shell (optional), and generates a markdown report.
"""

import argparse
import datetime
import os
import re
import subprocess
import sys
import time

import serial
import yaml
from colorama import init, Fore, Style
from halo import Halo

init()


def print_success(msg):
    print(f"{Fore.GREEN}✔ {msg}{Style.RESET_ALL}")


def print_error(msg):
    print(f"{Fore.RED}✘ {msg}{Style.RESET_ALL}")


def print_warning(msg):
    print(f"{Fore.YELLOW}⚠ {msg}{Style.RESET_ALL}")


def print_info(msg):
    print(f"{Fore.BLUE}ℹ {msg}{Style.RESET_ALL}")


def print_banner():
    banner = f"""
{Fore.CYAN}============================================================
              BRINGUP CHECKLIST ASSISTANT                  
============================================================{Style.RESET_ALL}
    """
    print(banner)


def check_serial_port(port):
    try:
        ser = serial.Serial(port, baudrate=115200, timeout=1.0)
        ser.close()
        return port
    except Exception as e:
        print_warning(f"Could not open serial port '{port}': {e}")
        return None


def run_serial_command(ser, command):
    spinner = Halo(text=f"Sending command '{command}' over serial...", spinner="dots")
    spinner.start()

    # Clear buffers
    ser.reset_input_buffer()
    ser.reset_output_buffer()

    # Send newline to clear any prompt debris
    ser.write(b"\r\n")
    time.sleep(0.05)
    ser.reset_input_buffer()

    # Send command
    ser.write(f"{command}\r\n".encode("utf-8"))

    # Read response
    output = []
    start_time = time.time()
    # Wait up to 5 seconds for response
    while time.time() - start_time < 5.0:
        if ser.in_waiting:
            line = ser.readline().decode("utf-8", errors="replace")
            if not line and getattr(ser, "proc", None) and ser.proc.poll() is not None:
                break
            output.append(line)
            spinner.text = f"[device] {line.strip()}"
            if "Command succeeded" in line or "Command failed" in line or "shell>" in line:
                # Read any remaining bytes
                time.sleep(0.1)
                while ser.in_waiting:
                    line = ser.readline().decode("utf-8", errors="replace")
                    if not line and getattr(ser, "proc", None) and ser.proc.poll() is not None:
                        break
                    output.append(line)
                    spinner.text = f"[device] {line.strip()}"
                break
        else:
            time.sleep(0.05)

    spinner.succeed("Response received from device")

    return "".join(output)


def run_host_command(command, variables):
    # Format the command with project variables
    formatted_cmd = command.format(**variables)
    spinner = Halo(text=f"Running host command: {formatted_cmd}", spinner="dots")
    spinner.start()
    try:
        result = subprocess.run(formatted_cmd, shell=True, text=True, capture_output=True, check=False)
        stdout_stderr = result.stdout + "\n" + result.stderr
        if result.returncode == 0:
            spinner.succeed("Host command executed successfully")
        else:
            spinner.fail("Host command execution failed")
        print("--- STDOUT ---")
        print(result.stdout)
        if result.stderr:
            print("--- STDERR ---")
            print(result.stderr)
        if result.returncode != 0:
            if "probe-rs" in formatted_cmd:
                print(f"\n{Fore.CYAN}[TROUBLESHOOTING TIP]{Style.RESET_ALL}")
                print("  If probe-rs failed to connect, please ensure:")
                print(
                    "  1. The target device is awake (not in low-power sleep). Wave your hand in front of the proximity sensors or reset the device."
                )
                print(
                    "  2. The active RTT connection has fully closed (the script waited 1 second, but the OS may need more time)."
                )
                print("  3. No other debugger/RTT client is running in another terminal window.")
        return result.returncode == 0, stdout_stderr
    except Exception as e:
        spinner.fail(f"Error executing command: {e}")
        return False, str(e)


def prompt_user_with_interactive_shell(prompt_text, ser=None):
    if not ser or getattr(ser, "channel", "cli") == "defmt":
        return input(prompt_text)

    actual_prompt = f"{Fore.GREEN}{prompt_text}{Style.RESET_ALL} (or type command first): "

    while True:
        choice = input(actual_prompt).strip()
        if not choice:
            return ""

        print(f"\n--- Running device command: '{choice}' ---")
        output = run_serial_command(ser, choice)
        print("--- End ---\n")


def load_config(file_path):
    """Load and parse the YAML config file."""
    with open(file_path, "r") as f:
        return yaml.safe_load(f)


def check_regex(output_log, regex_str):
    if not regex_str or not output_log:
        return False, ""
    try:
        pattern = re.compile(regex_str)
        for line in output_log.splitlines():
            if pattern.search(line):
                return True, line.strip()
    except Exception as e:
        print(f"Error compiling or searching regex '{regex_str}': {e}")
    return False, ""


def stream_process_output(ser):
    """Spawns a background daemon thread to stream target output to a log file."""
    import threading

    def read_thread():
        try:
            with open("device_logs.log", "a", encoding="utf-8") as f:
                f.write(f"\n--- Logging Session Started: {time.strftime('%Y-%m-%d %H:%M:%S')} ---\n")
                while True:
                    line = ser.stdout.readline()
                    if not line:
                        break
                    text = line.decode("utf-8", errors="replace")
                    f.write(text)
                    f.flush()
        except Exception:
            pass

    t = threading.Thread(target=read_thread, daemon=True)
    t.start()


def parse_cargo_target(cargo_target_str, default_bin="cat_detector_shell"):
    """Parse a cargo run target command/argument string and return the path to the built ELF."""
    # Split the cargo target string into arguments
    args = cargo_target_str.split()
    target = None
    package = None
    bin_name = None
    is_release = False

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "--target" and i + 1 < len(args):
            target = args[i + 1]
            i += 2
        elif arg == "--package" and i + 1 < len(args):
            package = args[i + 1]
            i += 2
        elif arg == "--bin" and i + 1 < len(args):
            bin_name = args[i + 1]
            i += 2
        elif arg == "--release":
            is_release = True
            i += 1
        else:
            i += 1

    if not bin_name:
        bin_name = default_bin

    profile = "release" if is_release else "debug"
    if target:
        return f"target/{target}/{profile}/{bin_name}"
    else:
        return f"target/{profile}/{bin_name}"


class RttProcessWrapper:
    """Spawns host_cli as a subprocess and redirects stdin/stdout to mimic a serial port."""

    def __init__(self, elf_path, channel="cli"):
        """Initialize the host_cli process connected to target RTT CLI channel."""
        self.channel = channel
        self.read_buf = b""
        print_info("Building host_cli...")
        # Build host_cli first to avoid compilation delay during run
        subprocess.run(["cargo", "build", "--package", "host_cli", "--bin", "host_cli"], capture_output=True)

        print_info(f"Spawning host_cli subprocess for ELF: {elf_path} (channel={channel})...")
        self.proc = subprocess.Popen(
            [
                "cargo",
                "run",
                "--package",
                "host_cli",
                "--bin",
                "host_cli",
                "--",
                "--elf",
                elf_path,
                "--channel",
                channel,
            ],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            bufsize=0,
        )
        # Give it a second to connect
        time.sleep(2.0)

        # Check if it failed immediately
        if self.proc.poll() is not None:
            if self.proc.stderr is not None:
                stderr_out = self.proc.stderr.read().decode("utf-8", errors="replace")
            else:
                stderr_out = "Unknown error"
            raise RuntimeError(f"host_cli failed to start: {stderr_out}")

        if self.proc.stdin is None or self.proc.stdout is None or self.proc.stderr is None:
            raise RuntimeError("Subprocess pipes failed to initialize.")

        self.stdin = self.proc.stdin
        self.stdout = self.proc.stdout
        self.stderr = self.proc.stderr

    def write(self, data):
        """Write command data bytes to the host_cli process input."""
        self.stdin.write(data)
        self.stdin.flush()

    def readline(self):
        """Read a response line bytes from the host_cli process output without blocking on partial lines."""
        import select
        import os

        # Check if we already have a newline in our buffer
        if b"\n" in self.read_buf:
            line, self.read_buf = self.read_buf.split(b"\n", 1)
            return line + b"\n"

        # Otherwise, read more bytes from stdout
        ready, _, _ = select.select([self.stdout], [], [], 0.05)
        if ready:
            try:
                # Read whatever is available in the OS pipe buffer
                chunk = os.read(self.stdout.fileno(), 4096)
                if not chunk:
                    # EOF
                    if self.read_buf:
                        ret = self.read_buf
                        self.read_buf = b""
                        return ret
                    return b""
                self.read_buf += chunk
            except Exception:
                pass

        if b"\n" in self.read_buf:
            line, self.read_buf = self.read_buf.split(b"\n", 1)
            return line + b"\n"

        ret = self.read_buf
        self.read_buf = b""
        return ret

    @property
    def in_waiting(self):
        """Returns 1 if data is available to be read from process stdout, 0 otherwise."""
        if self.read_buf:
            return 1
        import select

        ready, _, _ = select.select([self.stdout], [], [], 0.05)
        return 1 if ready else 0

    def reset_input_buffer(self):
        """Drains any available bytes from host_cli stdout buffer."""
        import select
        import os

        self.read_buf = b""
        while True:
            ready, _, _ = select.select([self.stdout], [], [], 0.01)
            if ready:
                try:
                    chunk = os.read(self.stdout.fileno(), 4096)
                    if not chunk:
                        break
                except Exception:
                    break
            else:
                break

    def reset_output_buffer(self):
        """Mock method satisfying serial buffer control interface."""
        pass

    def close(self):
        """Gracefully terminates host_cli subprocess."""
        self.proc.terminate()
        self.proc.wait()


def main():
    parser = argparse.ArgumentParser(description="Bringup checklist assistant for firmware projects.")
    parser.add_argument(
        "--config",
        default="projects/cat_detector_bringup.yaml",
        help="Path to the bringup configuration YAML file.",
    )
    parser.add_argument(
        "--port",
        help="Serial port for target device shell (e.g. /dev/tty.usbmodem101).",
    )
    parser.add_argument("--baud", type=int, default=115200, help="Serial port baud rate.")
    parser.add_argument(
        "--output",
        default="bringup_report.md",
        help="Path to write the bringup markdown report.",
    )
    parser.add_argument(
        "--rtt-elf",
        default=None,
        help="Path to target ELF binary to run shell via RTT host_cli.",
    )
    parser.add_argument(
        "--manual",
        action="store_true",
        help="Force manual interactive shell mode (no serial/RTT connection).",
    )

    args = parser.parse_args()

    print_banner()

    if not os.path.exists(args.config):
        print_error(f"Configuration file '{args.config}' not found.")
        sys.exit(1)

    try:
        config = load_config(args.config)
    except Exception as e:
        print_error(f"Error reading configuration file: {e}")
        sys.exit(1)

    # Determine default RTT ELF path from config if not explicitly provided
    shell_cargo_target = config.get(
        "cargo_target",
        "cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release",
    )
    app_cargo_target = config.get(
        "app_cargo_target",
        "cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_app --release",
    )

    shell_elf = parse_cargo_target(shell_cargo_target, default_bin="cat_detector_shell")
    app_elf = parse_cargo_target(app_cargo_target, default_bin="cat_detector_app")
    device_chip = config.get("device_chip", "RP2040")

    if args.rtt_elf is None:
        args.rtt_elf = shell_elf

    print_info(f"Loaded bringup checklist for project: {config.get('project_name', 'Unknown')}")
    print_info(f"Target chip: {config.get('device_chip', 'Unknown')}")

    # Establish serial or RTT if requested (defaulting to RTT)
    ser = None
    if args.manual:
        print_info("Forced manual mode. Running in interactive manual mode (no serial/RTT connection).")
    elif args.port:
        port = check_serial_port(args.port)
        if port:
            spinner = Halo(text=f"Connecting to serial port: {port}", spinner="dots")
            spinner.start()
            try:
                ser = serial.Serial(port, baudrate=args.baud, timeout=1.0)
                spinner.succeed(f"Connected to serial port: {port}")
            except Exception as e:
                spinner.fail(f"Failed to open serial port {port}: {e}")
        else:
            print_info("Running in interactive manual shell mode (no serial connection).")
    else:
        # Default to RTT mode
        print_info(f"Automatic Startup: Building and flashing target binary ({args.rtt_elf})...")
        matching_cargo_target = shell_cargo_target if "shell" in args.rtt_elf else app_cargo_target
        build_cmd = matching_cargo_target.split()
        if "run" in build_cmd:
            build_cmd[build_cmd.index("run")] = "build"
        subprocess.run(build_cmd, capture_output=True)
        try:
            subprocess.run(["probe-rs", "download", "--chip", device_chip, args.rtt_elf], check=True)
            subprocess.run(["probe-rs", "reset", "--chip", device_chip], check=True)
            print_success("Successfully flashed target device.")
        except Exception as e:
            print_error(f"Failed to flash target device: {e}")

        try:
            ser = RttProcessWrapper(args.rtt_elf)
            print_success("Successfully connected to device shell via RTT (host_cli)")
        except Exception as e:
            print_error(f"Failed to connect via RTT: {e}")
            print_info("Falling back to interactive manual mode.")

    variables = {
        "device_chip": config.get("device_chip", "RP2040"),
        "flash_addr": config.get("flash_addr", "0x101C0000"),
        "flash_size": str(config.get("flash_size", 262144)),
        "shell_elf": shell_elf,
        "app_elf": app_elf,
    }

    steps = config.get("steps", [])
    results = []

    print(f"\nFound {len(steps)} bringup steps. Starting checklist...")
    print("Press Ctrl+C to abort and write partial report.\n")

    try:
        current_app = "shell"
        for idx, step in enumerate(steps):
            print("\n" + "=" * 60)
            print(f"{Fore.CYAN}STEP {idx + 1}/{len(steps)}: {step['name']} (ID: {step['id']}){Style.RESET_ALL}")
            print(f"{Fore.MAGENTA}Type:{Style.RESET_ALL} {step['type'].upper()}")
            if step.get("description"):
                print(f"{Fore.MAGENTA}Description:{Style.RESET_ALL} {step.get('description', '')}")

            # If this is a host command, release the debug probe by closing RTT connection
            if step["type"] == "host_command" and ser:
                print_info("Closing active RTT/serial connection to release USB debug probe...")
                ser.close()
                ser = None
                time.sleep(2.0)

            # Automatic app flashing / transition handling
            if not args.manual and "flash_before" in step:
                target_type = step["flash_before"]
                if target_type == "app" and current_app == "shell":
                    print_info(f"Automatic Transition: Flashing the main app ({app_elf}) for low-power tests...")
                    if ser:
                        ser.close()
                        ser = None
                        time.sleep(2.0)
                    build_cmd = app_cargo_target.split()
                    if "run" in build_cmd:
                        build_cmd[build_cmd.index("run")] = "build"
                    subprocess.run(build_cmd, capture_output=True)
                    subprocess.run(
                        [
                            "probe-rs",
                            "download",
                            "--chip",
                            device_chip,
                            app_elf,
                        ],
                        check=True,
                    )
                    subprocess.run(
                        [
                            "probe-rs",
                            "reset",
                            "--chip",
                            device_chip,
                        ],
                        check=True,
                    )
                    ser = RttProcessWrapper(app_elf, channel="defmt")
                    stream_process_output(ser)
                    current_app = "app"
                elif target_type == "shell" and current_app == "app":
                    print_info(
                        f"Automatic Transition: Flashing bringup shell ({shell_elf}) for diagnostic CLI command tests..."
                    )
                    if ser:
                        ser.close()
                        ser = None
                        time.sleep(2.0)
                    build_cmd = shell_cargo_target.split()
                    if "run" in build_cmd:
                        build_cmd[build_cmd.index("run")] = "build"
                    subprocess.run(build_cmd, capture_output=True)
                    subprocess.run(
                        [
                            "probe-rs",
                            "download",
                            "--chip",
                            device_chip,
                            shell_elf,
                        ],
                        check=True,
                    )
                    subprocess.run(
                        [
                            "probe-rs",
                            "reset",
                            "--chip",
                            device_chip,
                        ],
                        check=True,
                    )
                    ser = RttProcessWrapper(shell_elf, channel="cli")
                    current_app = "shell"

            # Ensure RTT connection is active if needed for this step type
            if step["type"] in ("shell_command", "interactive", "manual"):
                if not ser and not args.manual and not args.port:
                    rtt_binary = app_elf if current_app == "app" else shell_elf
                    print_info(f"Re-opening RTT connection to target ({rtt_binary})...")
                    try:
                        ser = RttProcessWrapper(rtt_binary, channel="defmt" if current_app == "app" else "cli")
                        if current_app == "app":
                            stream_process_output(ser)
                        print_success("Successfully reconnected to device shell via RTT.")
                    except Exception as e:
                        print_error(f"Failed to reconnect via RTT: {e}")

            status = "skipped"
            output_log = ""
            notes = ""
            matched_line = ""
            auto_pass = None

            if step["type"] == "shell_command":
                cmd = step["command"]
                expected_regex = step.get("expected_regex", "")
                print(f"{Fore.YELLOW}Command to run on device:{Style.RESET_ALL} {cmd}")

                if ser:
                    prompt_user_with_interactive_shell("Press Enter to send", ser)
                    output_log = run_serial_command(ser, cmd)
                    if expected_regex:
                        has_match, matched_line = check_regex(output_log, expected_regex)
                        if has_match:
                            print_success(f"Match found: {matched_line}")
                            auto_pass = True
                        else:
                            print_warning(f"Pattern '{expected_regex}' not found in device output.")
                            if output_log:
                                print(f"{Fore.RED}--- Actual Device Output ---{Style.RESET_ALL}")
                                print(output_log.strip())
                                print(f"{Fore.RED}----------------------------{Style.RESET_ALL}")
                            auto_pass = False

                    # Gracefully close connection if step specifies target reboot
                    if step.get("reboot", False):
                        print_info("Step specified target reboot. Closing connection to allow clean startup...")
                        ser.close()
                        ser = None
                        time.sleep(3.0)
                else:
                    print(f"\n{Fore.RED}[ACTION REQUIRED]{Style.RESET_ALL}")
                    print(f"  Please open your device serial terminal and execute: {cmd}")
                    if expected_regex:
                        print(f"  Confirm the output matches pattern: {expected_regex}")
                    input("\nPress Enter when done to record the result...")

            elif step["type"] == "host_command":
                cmd = step["command"]
                expected_regex = step.get("expected_regex", "")
                print(f"{Fore.YELLOW}Host Command:{Style.RESET_ALL} {cmd}")
                confirm_prompt = f"{Fore.GREEN}Execute this command now? (Y/n): {Style.RESET_ALL}"
                confirm = input(confirm_prompt).strip().lower()
                if confirm in ("", "y", "yes"):
                    success, output_log = run_host_command(cmd, variables)
                    if success:
                        auto_pass = True
                        if expected_regex:
                            has_match, matched_line = check_regex(output_log, expected_regex)
                            if has_match:
                                print_success(f"Match found: {matched_line}")
                                auto_pass = True
                            else:
                                print_warning(f"Pattern '{expected_regex}' not found in command output.")
                                auto_pass = False
                    else:
                        auto_pass = False
                else:
                    print_info("Skipping command execution.")

            elif step["type"] == "interactive":
                print(f"\n{Fore.YELLOW}[PROCEDURE]{Style.RESET_ALL}")
                print(step.get("procedure", ""))
                prompt_user_with_interactive_shell("\nPress Enter to continue", ser)

            elif step["type"] == "manual":
                print(f"\n{Fore.YELLOW}[MANUAL TEST]{Style.RESET_ALL}")
                print(f"Procedure: {step.get('description', '')}")
                if step.get("expected"):
                    print(f"Expected outcome: {step['expected']}")
                prompt_user_with_interactive_shell("\nPress Enter to continue", ser)

            # Ask for status
            default_status = "passed"
            if auto_pass is True:
                default_status = "passed"
            elif auto_pass is False:
                default_status = "failed"

            color_map = {"passed": Fore.GREEN, "failed": Fore.RED, "skipped": Fore.YELLOW}
            colored_default = f"{color_map[default_status]}{default_status.upper()}{Style.RESET_ALL}"
            status_prompt = f"\nSelect status - [P]assed, [F]ailed, [S]kipped (default {colored_default}): "

            while True:
                choice = input(status_prompt).strip().lower()
                if choice == "":
                    status = default_status
                    break
                elif choice in ("p", "pass", "passed"):
                    status = "passed"
                    break
                elif choice in ("f", "fail", "failed"):
                    status = "failed"
                    break
                elif choice in ("s", "skip", "skipped"):
                    status = "skipped"
                    break

            notes = input("Add any notes/observations (optional): ").strip()

            results.append(
                {
                    "id": step["id"],
                    "name": step["name"],
                    "type": step["type"],
                    "status": status,
                    "output_log": output_log,
                    "matched_line": matched_line,
                    "notes": notes,
                    "timestamp": datetime.datetime.now().isoformat(),
                }
            )

            color_map = {"passed": Fore.GREEN, "failed": Fore.RED, "skipped": Fore.YELLOW}
            print(f"Result recorded: {color_map[status]}{status.upper()}{Style.RESET_ALL}")

    except KeyboardInterrupt:
        print_warning("\n\nChecklist run interrupted by user. Saving partial report...")
    finally:
        if ser:
            ser.close()

    # Generate markdown report
    passed_count = sum(1 for r in results if r["status"] == "passed")
    failed_count = sum(1 for r in results if r["status"] == "failed")
    skipped_count = sum(1 for r in results if r["status"] == "skipped")
    total_completed = len(results)

    # Output colorful CLI summary
    print("\n" + "=" * 60)
    print_info("BRINGUP RUN COMPLETED SUMMARY:")
    print_success(f"Passed: {passed_count}")
    print_error(f"Failed: {failed_count}")
    print_warning(f"Skipped: {skipped_count}")
    print(f"Total: {total_completed} / {len(steps)}")
    print("=" * 60)

    report_content = []
    report_content.append(f"# Bringup Verification Report: {config.get('project_name', 'Unknown')}")
    report_content.append(f"**Date:** {datetime.date.today().isoformat()}")
    report_content.append(f"**Device Chip:** {config.get('device_chip', 'Unknown')}")
    report_content.append(f"**Flash Address:** {config.get('flash_addr', 'Unknown')}")
    report_content.append("\n## Summary")
    report_content.append(f"- **Total Steps Run:** {total_completed} / {len(steps)}")
    report_content.append(f"- **Passed:** {passed_count}")
    report_content.append(f"- **Failed:** {failed_count}")
    report_content.append(f"- **Skipped:** {skipped_count}")

    report_content.append("\n## Detailed Results")

    for r in results:
        status_emoji = "✅" if r["status"] == "passed" else "❌" if r["status"] == "failed" else "⚠️"
        report_content.append(f"\n### {status_emoji} Step {r['id']}: {r['name']}")
        report_content.append(f"- **Type:** {r['type']}")
        report_content.append(f"- **Status:** {r['status'].upper()}")
        report_content.append(f"- **Time:** {r['timestamp']}")
        if r.get("matched_line"):
            report_content.append(f"- **Actual Matching Output Line:** `{r['matched_line']}`")
        if r["notes"]:
            report_content.append(f"- **Notes:** {r['notes']}")
        if r["output_log"]:
            report_content.append("- **Command Output:**")
            report_content.append("  ```text")
            for line in r["output_log"].split("\n"):
                report_content.append(f"  {line}")
            report_content.append("  ```")

    try:
        with open(args.output, "w") as f:
            f.write("\n".join(report_content))
        print_success(f"Bringup report successfully generated at: {args.output}")
    except Exception as e:
        print_error(f"Error writing report file: {e}")


if __name__ == "__main__":
    main()
