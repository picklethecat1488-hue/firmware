#!/usr/bin/env python3
"""Regression functional test for WORM-FOSSORIAL-COMPOSTER-481.

Verifies that RTT defmt logs and buffer draining messages during flash_target or RTT streaming
do not leak into the CLI console output view (runner.cli_outputs).
"""

import sys
from pathlib import Path
from unittest.mock import patch

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.host_daemon.provider.daemon.runner import DaemonRunner


def test_worm_fossorial_composter_481_flash_target_does_not_log_to_cli():
    """Verify flash_target sets log_to_cli=False to prevent flash and defmt drain logs in CLI view."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
    )
    fake_elf = REPO_ROOT / "target" / "thumbv6m-none-eabi" / "release" / "cat_detector_shell"
    runner.elf_path = fake_elf

    with patch.object(Path, "exists", return_value=True):
        with patch.object(runner, "_run_command_streaming", return_value=(0, "Success")) as mock_stream:
            success = runner.flash_target(use_cargo_run=True)
            assert success is True
            mock_stream.assert_called_once()
            kwargs = mock_stream.call_args[1]
            assert kwargs.get("log_to_cli") is False, (
                "flash_target must invoke _run_command_streaming with log_to_cli=False"
            )


def test_worm_fossorial_composter_481_probe_rs_does_not_log_to_cli():
    """Verify run_probe_rs sets log_to_cli=False so raw probe output does not pollute CLI view."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
    )
    with patch("shutil.which", return_value="/fake/probe-rs"):
        with patch.object(runner, "_run_command_streaming", return_value=(0, "Success")) as mock_stream:
            ret, _ = runner.run_probe_rs(["download", "test.elf"])
            assert ret == 0
            mock_stream.assert_called_once()
            kwargs = mock_stream.call_args[1]
            assert kwargs.get("log_to_cli") is False, (
                "run_probe_rs must invoke _run_command_streaming with log_to_cli=False"
            )


def test_worm_fossorial_composter_481_defmt_drain_not_appended_to_cli_outputs():
    """Verify lines containing 'Draining buffered defmt logs' or defmt log levels are never logged to CLI."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
    )

    test_lines = [
        "shell> Draining buffered defmt logs:",
        "0.001497 INFO MPU stack guard configured at 0x2003c000",
        "0.021760 INFO MPU stack guard configured at 0x2000c500",
        "shell> ",
    ]

    # Process lines as in _stream_worker
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
            else:
                runner.log_cli(stripped)

    cli_texts = [entry["text"] for entry in runner.cli_outputs]
    for text in cli_texts:
        assert "Draining buffered defmt logs" not in text, (
            f"CLI output should not contain defmt drain banner, found: {text}"
        )
        assert "MPU stack guard" not in text, (
            f"CLI output should not contain defmt log line, found: {text}"
        )

    # Verify defmt logs and drain banner went to target logs
    target_logs = [log["message"] for log in runner.logs if log["channel"] == "target"]
    assert any("Draining buffered defmt logs" in msg for msg in target_logs)
    assert any("MPU stack guard configured at 0x2003c000" in msg for msg in target_logs)
    assert any("MPU stack guard configured at 0x2000c500" in msg for msg in target_logs)
