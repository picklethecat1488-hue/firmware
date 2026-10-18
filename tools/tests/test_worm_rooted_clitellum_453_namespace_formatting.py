#!/usr/bin/env python3
"""Regression functional test for WORM-ROOTED-CLITELLUM-453.

Ensures that host_cli parses target defmt function namespaces into clean crate and module
identifiers only, stripping compiler-generated impl block tokens (`{impl#...}`), closures,
async fn wrappers, and trailing function/method names:
  - `peripheral::tmc6300::{impl#7}::tick::{async_fn#0}` -> `peripheral::tmc6300`
  - `controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}` -> `controller::motor_controller`
  - `controller::shell_controller::log_command` -> `controller::shell_controller`

Rather than checking source code strings or AST tokens, this test executes the actual
compiled functional unit test in `tools/host_cli/tests/host_cli_tests.rs`.
"""

from pathlib import Path
import subprocess


def test_worm_rooted_clitellum_453_namespace_formatting_functional():
    """Verify target namespace formatting and log PC module reduction by executing the Rust functional unit test."""
    workspace_root = Path(__file__).resolve().parent.parent.parent
    cmd = [
        "cargo",
        "test",
        "--package",
        "host_cli",
        "--test",
        "host_cli_tests",
        "test_worm_rooted_clitellum_453_format_target_namespace",
        "--",
        "--nocapture",
    ]
    proc = subprocess.run(
        cmd,
        cwd=workspace_root,
        capture_output=True,
        text=True,
        check=False,
    )
    assert proc.returncode == 0, (
        f"cargo test failed with return code {proc.returncode}:\nSTDOUT:\n{proc.stdout}\nSTDERR:\n{proc.stderr}"
    )
    assert (
        "test test_worm_rooted_clitellum_453_format_target_namespace ... ok"
        in proc.stdout
    ), f"Expected test success in cargo test output, got:\n{proc.stdout}"
    assert "1 passed; 0 failed" in proc.stdout, (
        f"Expected 1 passed in cargo test output, got:\n{proc.stdout}"
    )
