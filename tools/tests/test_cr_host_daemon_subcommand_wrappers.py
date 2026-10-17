#!/usr/bin/env python3
"""Regression unit test for Host Daemon code review findings:

1. CR-01b411f5: Subcommand wrappers for host_cli, host_fs, and probe-rs with real-time colorized streaming.
2. CR-6783fe41: Strict chip detection without silent fallback; errors out if target lacks program metadata.
3. CR-bfc3c4eb: Target flashing via cargo run wrapper to remove direct probe-rs execution dependency.
4. CR-97bc2d55: host_fs execution via cargo run invocation.
"""

from pathlib import Path
import subprocess
import sys
from unittest.mock import MagicMock, patch
import pytest

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.host_daemon.runner import DaemonRunner


def test_cr_01b411f5_subcommand_wrappers_exist_and_stream():
    """Verify DaemonRunner provides subcommand wrappers for host_cli, host_fs, and probe-rs with streaming."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
    )

    # Invariant: Method wrappers exist on DaemonRunner
    assert hasattr(runner, "run_host_cli"), "DaemonRunner must implement run_host_cli"
    assert hasattr(runner, "run_host_fs"), "DaemonRunner must implement run_host_fs"
    assert hasattr(runner, "run_probe_rs"), "DaemonRunner must implement run_probe_rs"

    # Test real-time streaming helper pipes colorized lines immediately to log_cli and log
    lines_emitted = [
        "\x1b[1;32mCompiling\x1b[0m cat_detector v0.1.0\n",
        "\x1b[36mFinished\x1b[0m release profile\n",
    ]

    mock_proc = MagicMock()
    mock_proc.stdout.readline.side_effect = lines_emitted + [""]
    mock_proc.wait.return_value = 0
    mock_proc.returncode = 0

    with patch("subprocess.Popen", return_value=mock_proc):
        ret, full_output = runner._run_command_streaming(
            ["cargo", "run"],
            log_to_cli=True,
            log_channel="test_channel",
        )

        assert ret == 0
        assert "Compiling" in full_output
        assert "Finished" in full_output

        # Verify lines are in cli_outputs and logs
        cli_texts = [entry["text"] for entry in runner.cli_outputs]
        assert any("\x1b[1;32mCompiling\x1b[0m" in t for t in cli_texts)
        assert any("\x1b[36mFinished\x1b[0m" in t for t in cli_texts)

        log_messages = [entry["message"] for entry in runner.logs]
        assert any("Compiling" in m for m in log_messages)


def test_cr_6783fe41_detect_chip_strict_error_without_fallback(tmp_path: Path):
    """Verify detect_chip errors out when ELF lacks metadata and does not silently fall back to RP2040."""
    fake_elf = tmp_path / "dummy_firmware"
    fake_elf.write_bytes(b"\x7fELF\x01\x01\x01\x00")

    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
    )
    runner.elf_path = fake_elf
    runner.chip = None

    # Mock subprocess.run returning non-zero (or empty output) to simulate missing metadata
    mock_proc = MagicMock()
    mock_proc.returncode = 1
    mock_proc.stdout = ""
    mock_proc.stderr = "Error: no project metadata found in ELF"

    with patch("subprocess.run", return_value=mock_proc):
        with pytest.raises(RuntimeError) as exc_info:
            runner.detect_chip()

        assert "does not contain required program metadata for host_daemon" in str(exc_info.value)
        assert runner.chip is None or runner.chip != "RP2040", "Must not fallback to RP2040"


def test_cr_bfc3c4eb_flash_target_invokes_cargo_run():
    """Verify flash_target invokes cargo run to flash target and remove standalone probe-rs dependency."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
    )
    fake_elf = REPO_ROOT / "target" / "thumbv6m-none-eabi" / "debug" / "cat_detector_shell"
    runner.elf_path = fake_elf

    with patch.object(Path, "exists", return_value=True):
        with patch.object(runner, "_run_command_streaming", return_value=(0, "Flashed successfully")) as mock_stream:
            success = runner.flash_target(use_cargo_run=True)
            assert success is True
            assert runner.is_flashed is True
            assert runner.status_message == "Flashed & Running"

            mock_stream.assert_called_once()
            called_cmd = mock_stream.call_args[0][0]
            assert "cargo" in called_cmd
            assert "run" in called_cmd
            assert "--target" in called_cmd
            assert "thumbv6m-none-eabi" in called_cmd
            assert "cat_detector" in called_cmd


def test_cr_97bc2d55_execute_fs_invokes_cargo_run_host_fs():
    """Verify execute_fs calls host_fs by invoking cargo run."""
    runner = DaemonRunner(
        repo_root=REPO_ROOT,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
    )
    fake_elf = REPO_ROOT / "target" / "thumbv6m-none-eabi" / "debug" / "cat_detector_shell"
    runner.elf_path = fake_elf

    with patch.object(Path, "exists", return_value=True):
        with patch.object(runner, "run_host_fs", return_value=(0, "flash_log.bin  4096 bytes")) as mock_run_fs:
            out = runner.execute_fs("ls")
            assert "flash_log.bin" in out
            mock_run_fs.assert_called_once()
            args = mock_run_fs.call_args[0][0]
            assert "--elf" in args
            assert "ls" in args
