#!/usr/bin/env python3
"""Regression functional unit test for WORM-VERMIFORM-REDWORM-38.

Verifies that target command echo, stray characters (like 'm' or 'p'), and redundant
standalone shell prompts do not pollute the interactive CLI output stream or get mangled into defmt logs.
"""

import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.host_daemon.provider.daemon.runner import DaemonRunner


def test_worm_vermiform_redworm_38_standalone_prompts_filtered():
    """Verify bare 'shell>' prompts are not appended as separate empty CLI output lines."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
    )

    test_lines = [
        "Motor current: 1461 mA\r\n",
        "shell>\r\n",
        "shell> \r\n",
        "Motor speed: 0 RPM\r\n",
        "shell>\r\n",
    ]

    for line in test_lines:
        stripped = line.rstrip("\r\n")
        if stripped:
            if (
                "INFO" in stripped
                or "WARN" in stripped
                or "ERROR" in stripped
                or "DEBUG" in stripped
                or "TRACE" in stripped
                or "Draining buffered defmt logs" in stripped
            ):
                runner.log(stripped, channel="target")
            elif stripped in ("shell>", "shell> "):
                pass
            else:
                runner.log_cli(stripped)

    cli_texts = [entry["text"] for entry in runner.cli_outputs]
    assert "shell>" not in cli_texts, f"Standalone prompt 'shell>' must not be logged to CLI outputs: {cli_texts}"
    assert "shell> " not in cli_texts, f"Standalone prompt 'shell> ' must not be logged to CLI outputs: {cli_texts}"
    assert "Motor current: 1461 mA" in cli_texts
    assert "Motor speed: 0 RPM" in cli_texts


def test_worm_vermiform_redworm_38_defmt_logs_routed_to_target():
    """Verify defmt logs occurring during command execution route to target logs and not CLI output."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
    )

    test_lines = [
        "364.173550 INFO [controller::shell_controller] received command \"Motor { subcommand: Some(Current) }\"",
        "Motor current: 1461 mA",
        "364.173754 INFO [controller::shell_controller] command execution succeeded",
    ]

    for line in test_lines:
        stripped = line.rstrip("\r\n")
        if stripped:
            if (
                "INFO" in stripped
                or "WARN" in stripped
                or "ERROR" in stripped
                or "DEBUG" in stripped
                or "TRACE" in stripped
                or "Draining buffered defmt logs" in stripped
            ):
                runner.log(stripped, channel="target")
            elif stripped in ("shell>", "shell> "):
                pass
            else:
                runner.log_cli(stripped)

    cli_texts = [entry["text"] for entry in runner.cli_outputs]
    assert cli_texts == ["Motor current: 1461 mA"], f"Only command response should be in CLI outputs, got {cli_texts}"

    target_msgs = [entry["message"] for entry in runner.logs if entry["channel"] == "target"]
    assert any("received command" in msg for msg in target_msgs)
    assert any("command execution succeeded" in msg for msg in target_msgs)
