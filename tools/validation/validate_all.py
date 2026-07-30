#!/usr/bin/env python
"""
Validate All.

Runs all codebase validation checks (formatting, Clippy clippy host/target checks,
and custom AST validation scripts) in parallel using a thread pool executor.
Uses a single global Halo spinner that dynamically updates its text with the last
active validation task to output text.
"""

import json
import os
import subprocess
import sys
import threading
from concurrent.futures import ThreadPoolExecutor, as_completed

# Import original Halo spinner
import halo

# Ensure workspace root is in path
workspace_root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
sys.path.insert(0, workspace_root)

# Import the validator functions
from tools.validation.validate_tracing import validate_tracing
from tools.validation.validate_multicore_support import validate_multicore_support
from tools.validation.validate_debug_derive import validate_debug_derive

# Globals for spinner control
spinner = None
spinner_lock = threading.Lock()


def log_status(msg, name):
    """Safely log status updates without spinner collision."""
    with spinner_lock:
        if spinner:
            spinner.stop()
            stdout_proxy.original.write(msg + "\n")
            stdout_proxy.original.flush()
            spinner.text = f"Validating: {name}"
            spinner.start()
        else:
            stdout_proxy.original.write(msg + "\n")
            stdout_proxy.original.flush()


class ThreadLocalStdout:
    """Thread-local proxy for stdout/stderr to prefix and stream outputs in parallel."""

    def __init__(self, original):
        """Initialize the thread-local proxy with the original stream."""
        self.original = original
        self.local = threading.local()

    def write(self, data):
        """Write data to local buffer and print line-by-line with prefixing."""
        # Store in thread-local buffer if initialized for file writing
        if hasattr(self.local, "buffer"):
            self.local.buffer.append(data)

        if hasattr(self.local, "name") and self.local.name:
            if not hasattr(self.local, "line_buffer"):
                self.local.line_buffer = []

            for char in data:
                if char == "\n":
                    line_content = "".join(self.local.line_buffer)
                    self.local.line_buffer = []
                    log_status(f"[{self.local.name}] {line_content}", self.local.name)
                else:
                    self.local.line_buffer.append(char)
        else:
            self.original.write(data)

    def flush(self):
        """Flush the active stream."""
        self.original.flush()

    def __getattr__(self, name):
        """Delegate all other attributes and methods to the original stream."""
        return getattr(self.original, name)


# Replace global stdout/stderr with proxies
stdout_proxy = ThreadLocalStdout(sys.stdout)
stderr_proxy = ThreadLocalStdout(sys.stderr)
sys.stdout = stdout_proxy
sys.stderr = stderr_proxy


def get_exclude_args():
    """Extract Cargo package exclude flags for host-only tools."""
    try:
        res = subprocess.run(
            ["cargo", "metadata", "--format-version", "1"],
            capture_output=True,
            text=True,
            check=True,
            cwd=workspace_root,
        )
        metadata = json.loads(res.stdout)
        exclude_pkgs = []
        for pkg in metadata.get("packages", []):
            manifest_path = pkg.get("manifest_path", "")
            if "/tools/" in manifest_path or "/host/" in manifest_path:
                exclude_pkgs.extend(["--exclude", pkg["name"]])
        return exclude_pkgs
    except Exception as e:
        print(f"Failed to fetch cargo metadata: {e}", file=sys.stderr)
        return []


def run_cmd(name, cmd, output_file=None):
    """Run an external shell command and stream its outputs in parallel."""
    stdout_proxy.local.name = name
    stderr_proxy.local.name = name
    stdout_proxy.local.buffer = []

    aligned_name = f"{name:<27}"
    log_status(f"✅ {aligned_name} - Started", name)

    proc = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        cwd=workspace_root,
        bufsize=1,
    )

    # Read output line-by-line as it compiles/runs
    if proc.stdout:
        for line in proc.stdout:
            sys.stdout.write(line)
            sys.stdout.flush()

    proc.wait()

    # Flush any remaining text in the buffer
    if hasattr(stdout_proxy.local, "line_buffer") and stdout_proxy.local.line_buffer:
        line_content = "".join(stdout_proxy.local.line_buffer)
        stdout_proxy.local.line_buffer = []
        log_status(f"[{name}] {line_content}", name)

    captured_data = "".join(stdout_proxy.local.buffer)

    if proc.returncode == 0:
        log_status(f"✅ {aligned_name} - Passed", name)
    else:
        log_status(f"❌ {aligned_name} - Failed", name)

    # Clear thread local references
    del stdout_proxy.local.name
    del stderr_proxy.local.name
    del stdout_proxy.local.buffer

    if output_file and output_file != "/dev/stdout":
        full_output = f"=== {name} ===\n" + captured_data
        with open(output_file, "a") as f:
            f.write(full_output + "\n")

    return name, proc.returncode == 0


def run_python_validator(name, func, output_file=None):
    """Run an imported Python validator function in real-time with prefixing."""
    stdout_proxy.local.name = name
    stderr_proxy.local.name = name
    stdout_proxy.local.buffer = []

    aligned_name = f"{name:<27}"
    log_status(f"✅ {aligned_name} - Started", name)

    try:
        exit_code = func()
        success = exit_code == 0
    except Exception as e:
        import traceback

        traceback.print_exc()
        success = False

    # Flush any remaining text in the buffer
    if hasattr(stdout_proxy.local, "line_buffer") and stdout_proxy.local.line_buffer:
        line_content = "".join(stdout_proxy.local.line_buffer)
        stdout_proxy.local.line_buffer = []
        log_status(f"[{name}] {line_content}", name)

    captured_data = "".join(stdout_proxy.local.buffer)

    if success:
        log_status(f"✅ {aligned_name} - Passed", name)
    else:
        log_status(f"❌ {aligned_name} - Failed", name)

    # Clear thread local references
    del stdout_proxy.local.name
    del stderr_proxy.local.name
    del stdout_proxy.local.buffer

    if output_file and output_file != "/dev/stdout":
        full_output = f"=== {name} ===\n" + captured_data
        with open(output_file, "a") as f:
            f.write(full_output + "\n")

    return name, success


def main():
    global spinner
    output_file = sys.argv[1] if len(sys.argv) > 1 else "/dev/stdout"
    if output_file != "/dev/stdout":
        os.makedirs(os.path.dirname(os.path.abspath(output_file)), exist_ok=True)
        with open(output_file, "w") as f:
            f.write("")

    exclude_args = get_exclude_args()

    tasks = [
        (
            "Cargo Format",
            run_cmd,
            (["cargo", "fmt", "--all", "--check"], output_file),
        ),
        (
            "Cargo Clippy (Host)",
            run_cmd,
            (
                ["cargo", "clippy", "--all-targets", "--color", "never", "--", "-D", "warnings"],
                output_file,
            ),
        ),
        (
            "Cargo Clippy (Target MCU)",
            run_cmd,
            (
                ["cargo", "clippy", "--workspace"]
                + exclude_args
                + [
                    "--lib",
                    "--bins",
                    "--target",
                    "thumbv6m-none-eabi",
                    "--color",
                    "never",
                    "--",
                    "-D",
                    "warnings",
                ],
                output_file,
            ),
        ),
        (
            "Tracing Hierarchy Validator",
            run_python_validator,
            (validate_tracing, output_file),
        ),
        (
            "Multicore Support Validator",
            run_python_validator,
            (validate_multicore_support, output_file),
        ),
        (
            "Debug Derive Validator",
            run_python_validator,
            (validate_debug_derive, output_file),
        ),
        (
            "Python Lint (Ruff)",
            run_cmd,
            (["ruff", "check", "tools/validation/", "tools/helpers/"], output_file),
        ),
        (
            "Python Format (Ruff)",
            run_cmd,
            (
                ["ruff", "format", "--check", "tools/validation/", "tools/helpers/"],
                output_file,
            ),
        ),
    ]

    all_passed = True
    failed_checks = []

    spinner = halo.Halo(text="Validating...", spinner="dots", enabled=sys.stdout.isatty())
    spinner.start()

    with ThreadPoolExecutor(max_workers=len(tasks)) as executor:
        futures = [executor.submit(task_fn, name, *args) for name, task_fn, args in tasks]
        for future in as_completed(futures):
            try:
                name, success = future.result()
                if not success:
                    all_passed = False
                    failed_checks.append(name)
            except Exception as e:
                print(f"Task raised exception: {e}", file=sys.stderr)
                all_passed = False

    if all_passed:
        spinner.succeed("All validation checks PASSED!")
        sys.exit(0)
    else:
        spinner.fail(f"Validation FAILED for the following checks: {', '.join(failed_checks)}")
        sys.exit(1)


if __name__ == "__main__":
    main()
