"""Tests for Bringup checklist configuration loading, parsing, and execution."""

import os
from pathlib import Path
import subprocess
import sys
import tempfile
from unittest.mock import MagicMock

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

from host_daemon.model.bringup import (
    BringupConfig,
    BringupStep,
    BringupStepStatus,
    BringupStepType,
    check_regex,
    load_bringup_yaml,
    parse_cargo_target,
)
from host_daemon.provider.bringup.bringup_manager import BringupManager


def test_load_config():
    """Verify that load_bringup_yaml successfully parses YAML configuration files."""
    yaml_text = """
project_name: "Test Project"
device_chip: RP2040
flash_addr: "0x10000000"
steps:
  - id: 1
    name: "Check Power"
    type: manual
  - id: 2
    name: "Check Clock"
    type: host_command
    command: "ping"
"""
    with tempfile.NamedTemporaryFile("w", suffix=".yaml", delete=False) as f:
        f.write(yaml_text)
        temp_name = f.name

    try:
        res = load_bringup_yaml(temp_name)
        assert res.project_name == "Test Project"
        assert res.device_chip == "RP2040"
        assert res.flash_addr == "0x10000000"
        assert len(res.steps) == 2
        assert res.steps[0].id == "1"
        assert res.steps[0].name == "Check Power"
        assert res.steps[0].type == BringupStepType.MANUAL
        assert res.steps[1].command == "ping"
        assert res.steps[1].type == BringupStepType.HOST_COMMAND
    finally:
        os.remove(temp_name)


def test_parse_cargo_target():
    """Verify that parse_cargo_target extracts the correct ELF path from cargo run commands."""
    cmd = "cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release"
    assert parse_cargo_target(cmd) == "target/thumbv6m-none-eabi/release/cat_detector_shell"

    cmd_debug = "cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell"
    assert parse_cargo_target(cmd_debug) == "target/thumbv6m-none-eabi/debug/cat_detector_shell"

    cmd_no_target = "cargo run --package cat_detector --bin cat_detector_shell --release"
    assert parse_cargo_target(cmd_no_target) == "target/release/cat_detector_shell"


def test_check_regex():
    """Verify that check_regex correctly matches substrings and regular expression patterns."""
    text = "Line 1: Device boot completed\nLine 2: Direct proximity readings: North = 100 mm\nLine 3: OK"

    # Simple substring / regex match
    matched, line = check_regex(text, r"North = \d+ mm")
    assert matched
    assert line == "Line 2: Direct proximity readings: North = 100 mm"

    # Multicharacter ranges
    matched, line = check_regex(text, r"North = (9[0-9]|10[0-9]) mm")
    assert matched
    assert line == "Line 2: Direct proximity readings: North = 100 mm"

    # Non-matching pattern
    matched, line = check_regex(text, r"South = \d+ mm")
    assert not matched
    assert line == ""


def test_run_host_command_formatting():
    """Verify that host command strings are correctly formatted with project config variables."""
    cmd_template = "probe-rs read --chip {device_chip} b8 {flash_addr} {flash_size} --output flash_dump.bin"
    variables = {"device_chip": "RP2040", "flash_addr": "0x101C0000", "flash_size": "262144"}
    formatted = cmd_template.format(**variables)
    assert formatted == "probe-rs read --chip RP2040 b8 0x101C0000 262144 --output flash_dump.bin"


def test_parse_cargo_target_custom():
    """Verify parse_cargo_target works with custom binary targets."""
    cmd_bin = "cargo run --bin custom_mcu_binary"
    assert parse_cargo_target(cmd_bin) == "target/debug/custom_mcu_binary"

    cmd_bin_release = "cargo run --bin custom_mcu_binary --release"
    assert parse_cargo_target(cmd_bin_release) == "target/release/custom_mcu_binary"


def test_bringup_manager_lifecycle_and_execution(monkeypatch, tmp_path):
    """Verify that BringupManager correctly executes host commands and manages step lifecycle."""
    yaml_text = """
project_name: "Lifecycle Test"
device_chip: RP2040
flash_addr: "0x10000000"
steps:
  - id: 1
    name: "Check Clock"
    type: host_command
    command: "echo 'Clock frequency: 125 MHz'"
    expected_regex: 'Clock frequency: \\d+ MHz'
"""
    yaml_file = tmp_path / "test_lifecycle.yaml"
    yaml_file.write_text(yaml_text, encoding="utf-8")

    manager = BringupManager(repo_root=tmp_path)
    cfg = manager.load_script(yaml_file)
    assert cfg.project_name == "Lifecycle Test"
    assert manager.status == "idle"

    # Step through
    mock_run = MagicMock()
    mock_run.return_value.returncode = 0
    mock_run.return_value.stdout = "Clock frequency: 125 MHz\n"
    mock_run.return_value.stderr = ""
    monkeypatch.setattr(subprocess, "run", mock_run)

    step_res = manager.step()
    assert step_res["success"] is True
    assert step_res["status"] == "waiting_prompt"
    assert step_res["prompt"]["auto_pass"] is True
    assert step_res["prompt"]["matched_line"] == "Clock frequency: 125 MHz"

    # Respond to prompt with 'pass'
    resp_res = manager.respond(action="pass")
    assert resp_res["success"] is True
    assert resp_res["step"]["status"] == "passed"
    assert manager.status == "complete"

    # Verify report generation
    report = manager.generate_report()
    assert "# Bringup Verification Report: Lifecycle Test" in report
    assert "Check Clock" in report
