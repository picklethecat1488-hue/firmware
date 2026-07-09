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

try:
    import serial

    SERIAL_AVAILABLE = True
except ImportError:
    SERIAL_AVAILABLE = False


def print_banner():
    banner = """
============================================================
              BRINGUP CHECKLIST ASSISTANT                  
============================================================
    """
    print(banner)


def check_serial_port(port):
    if not SERIAL_AVAILABLE:
        print("Warning: 'pyserial' package is not installed. Serial communication will be disabled.")
        return None
    try:
        ser = serial.Serial(port, baudrate=115200, timeout=1.0)
        ser.close()
        return port
    except Exception as e:
        print(f"Warning: Could not open serial port '{port}': {e}")
        return None


def run_serial_command(ser, command):
    print(f"Sending command '{command}' over serial...")
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
            output.append(line)
            print(f"  [device] {line.strip()}")
            if "Command succeeded" in line or "Command failed" in line or "shell>" in line:
                # Read any remaining bytes
                time.sleep(0.1)
                while ser.in_waiting:
                    line = ser.readline().decode("utf-8", errors="replace")
                    output.append(line)
                    print(f"  [device] {line.strip()}")
                break
        else:
            time.sleep(0.05)

    return "".join(output)


def run_host_command(command, variables):
    # Format the command with project variables
    formatted_cmd = command.format(**variables)
    print(f"Executing host command: {formatted_cmd}")
    try:
        result = subprocess.run(formatted_cmd, shell=True, text=True, capture_output=True, check=False)
        print("--- STDOUT ---")
        print(result.stdout)
        if result.stderr:
            print("--- STDERR ---")
            print(result.stderr)
        print(f"Exit code: {result.returncode}")
        return result.returncode == 0, result.stdout + "\n" + result.stderr
    except Exception as e:
        print(f"Error executing command: {e}")
        return False, str(e)


def parse_value(val):
    val = val.strip()
    if not val:
        return ""
    # Strip quotes if any
    if (val.startswith('"') and val.endswith('"')) or (val.startswith("'") and val.endswith("'")):
        val = val[1:-1]
    # Replace escaped newlines if any
    val = val.replace("\\n", "\n")
    return val


def parse_yaml(yaml_text):
    """Parse YAML text using a lightweight, dependency-free parser.

    Supports simple key-values and lists of key-value maps (such as our bringup checklist).
    """
    result = {}
    current_list = None
    current_item = None
    in_literal_block = False
    literal_key = None
    literal_indent = 0
    literal_lines = []

    lines = yaml_text.splitlines()
    for line in lines:
        # Handle literal block values
        if in_literal_block:
            # Check indentation to determine if we are still in the block
            if line.strip() == "":
                literal_lines.append("")
                continue
            indent = len(line) - len(line.lstrip(" "))
            if indent > literal_indent:
                literal_lines.append(line[literal_indent:])
                continue
            else:
                # End of literal block
                block_val = "\n".join(literal_lines).strip()
                if current_item is not None:
                    current_item[literal_key] = block_val
                else:
                    result[literal_key] = block_val
                in_literal_block = False
                literal_lines = []
                literal_key = None

        # Strip comments
        if "#" in line:
            line = line.split("#", 1)[0]

        stripped = line.strip()
        if not stripped:
            continue

        indent = len(line) - len(line.lstrip(" "))

        # Check for list item indicator
        if stripped.startswith("-"):
            item_text = stripped[1:].strip()
            if current_list is not None:
                current_item = {}
                current_list.append(current_item)
                if item_text:
                    # It might be inline "- key: value"
                    key_val = re.match(r"^([^:]+):\s*(.*)$", item_text)
                    if key_val:
                        key, val = key_val.groups()
                        key = key.strip()
                        if val.strip() == "|":
                            in_literal_block = True
                            literal_key = key
                            literal_indent = indent + 2
                        else:
                            current_item[key] = parse_value(val)
            continue

        # Check for key-value pair
        key_val = re.match(r"^([^:]+):\s*(.*)$", stripped)
        if key_val:
            key, val = key_val.groups()
            key = key.strip()

            # Handle literal block starting indicator
            if val.strip() == "|":
                in_literal_block = True
                literal_key = key
                literal_indent = indent + 2
                continue

            val = parse_value(val)

            if indent == 0:
                if key == "steps":
                    current_list = []
                    result["steps"] = current_list
                else:
                    result[key] = val
                    current_list = None
                    current_item = None
            elif current_item is not None:
                current_item[key] = val

    # If file ends while still in literal block
    if in_literal_block and literal_key:
        block_val = "\n".join(literal_lines).strip()
        if current_item is not None:
            current_item[literal_key] = block_val
        else:
            result[literal_key] = block_val

    return result


def load_config(file_path):
    with open(file_path, "r") as f:
        content = f.read()

    # Try importing PyYAML first
    try:
        import yaml

        return yaml.safe_load(content)
    except ImportError:
        # Fall back to custom YAML parser
        return parse_yaml(content)


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

    args = parser.parse_args()

    print_banner()

    if not os.path.exists(args.config):
        print(f"Error: Configuration file '{args.config}' not found.")
        sys.exit(1)

    try:
        config = load_config(args.config)
    except Exception as e:
        print(f"Error reading configuration file: {e}")
        sys.exit(1)

    print(f"Loaded bringup checklist for project: {config.get('project_name', 'Unknown')}")
    print(f"Target chip: {config.get('device_chip', 'Unknown')}")

    # Establish serial if requested
    ser = None
    if args.port:
        port = check_serial_port(args.port)
        if port:
            try:
                ser = serial.Serial(port, baudrate=args.baud, timeout=1.0)
                print(f"Successfully connected to serial port: {port}")
            except Exception as e:
                print(f"Failed to open serial port {port}: {e}")
        else:
            print("Running in interactive manual shell mode (no serial connection).")
    else:
        print("No serial port specified. Running in interactive manual mode.")

    variables = {
        "device_chip": config.get("device_chip", "RP2040"),
        "flash_addr": config.get("flash_addr", "0x101C0000"),
        "flash_size": str(config.get("flash_size", 262144)),
    }

    steps = config.get("steps", [])
    results = []

    print(f"\nFound {len(steps)} bringup steps. Starting checklist...")
    print("Press Ctrl+C to abort and write partial report.\n")

    try:
        for idx, step in enumerate(steps):
            print("\n------------------------------------------------------------")
            print(f"STEP {idx + 1}/{len(steps)}: {step['name']} (ID: {step['id']})")
            print(f"Type: {step['type'].upper()}")
            print(f"Description: {step.get('description', '')}")

            status = "skipped"
            output_log = ""
            notes = ""
            matched_line = ""
            auto_pass = None

            if step["type"] == "shell_command":
                cmd = step["command"]
                expected_regex = step.get("expected_regex", "")
                print(f"Command to run on device: {cmd}")
                if expected_regex:
                    print(f"Expected output pattern (regex): {expected_regex}")

                if ser:
                    input("Press Enter to send the command to the device...")
                    output_log = run_serial_command(ser, cmd)
                    if expected_regex:
                        has_match, matched_line = check_regex(output_log, expected_regex)
                        if has_match:
                            print(f"\n[AUTO-VERIFICATION] Match found: {matched_line}")
                            auto_pass = True
                        else:
                            print(
                                f"\n[AUTO-VERIFICATION] WARNING: Pattern '{expected_regex}' not found in device output."
                            )
                            auto_pass = False
                else:
                    print("\n[ACTION REQUIRED]")
                    print(f"  Please open your device serial terminal and execute: {cmd}")
                    if expected_regex:
                        print(f"  Confirm the output matches pattern: {expected_regex}")
                    input("\nPress Enter when done to record the result...")

            elif step["type"] == "host_command":
                cmd = step["command"]
                expected_regex = step.get("expected_regex", "")
                print(f"Host Command: {cmd}")
                confirm = input("Execute this command now? (Y/n): ").strip().lower()
                if confirm in ("", "y", "yes"):
                    success, output_log = run_host_command(cmd, variables)
                    if success:
                        print("Command executed successfully.")
                        auto_pass = True
                        if expected_regex:
                            has_match, matched_line = check_regex(output_log, expected_regex)
                            if has_match:
                                print(f"\n[AUTO-VERIFICATION] Match found: {matched_line}")
                                auto_pass = True
                            else:
                                print(
                                    f"\n[AUTO-VERIFICATION] WARNING: Pattern '{expected_regex}' not found in command output."
                                )
                                auto_pass = False
                    else:
                        print("Command execution failed.")
                        auto_pass = False
                else:
                    print("Skipping command execution.")

            elif step["type"] == "interactive":
                print("\n[PROCEDURE]")
                print(step.get("procedure", ""))
                input("\nPerform the steps above, then press Enter to continue...")

            elif step["type"] == "manual":
                print("\n[MANUAL TEST]")
                print(f"Procedure: {step.get('description', '')}")
                if step.get("expected"):
                    print(f"Expected outcome: {step['expected']}")
                input("\nVerify the expected outcome above, then press Enter to continue...")

            # Ask for status
            default_status = "passed"
            if auto_pass is True:
                default_status = "passed"
            elif auto_pass is False:
                default_status = "failed"

            status_prompt = f"\nSelect status - [P]assed, [F]ailed, [S]kipped (default {default_status.upper()}): "

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

            print(f"Result recorded: {status.upper()}")

    except KeyboardInterrupt:
        print("\n\nChecklist run interrupted by user. Saving partial report...")
    finally:
        if ser:
            ser.close()

    # Generate markdown report
    passed_count = sum(1 for r in results if r["status"] == "passed")
    failed_count = sum(1 for r in results if r["status"] == "failed")
    skipped_count = sum(1 for r in results if r["status"] == "skipped")
    total_completed = len(results)

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
        print(f"\nBringup report successfully generated at: {args.output}")
    except Exception as e:
        print(f"Error writing report file: {e}")


if __name__ == "__main__":
    main()
