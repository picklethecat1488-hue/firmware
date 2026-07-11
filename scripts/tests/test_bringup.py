import sys
import os
import tempfile

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import bringup


def test_load_config():
    """Verify that load_config successfully parses YAML configuration files."""
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
    type: host
    command: "ping"
"""
    with tempfile.NamedTemporaryFile("w", suffix=".yaml", delete=False) as f:
        f.write(yaml_text)
        temp_name = f.name

    try:
        res = bringup.load_config(temp_name)
        assert res["project_name"] == "Test Project"
        assert res["device_chip"] == "RP2040"
        assert res["flash_addr"] == "0x10000000"
        assert "steps" in res
        assert len(res["steps"]) == 2
        assert res["steps"][0]["id"] == 1
        assert res["steps"][0]["name"] == "Check Power"
        assert res["steps"][1]["command"] == "ping"
    finally:
        os.remove(temp_name)


def test_parse_cargo_target():
    """Verify that parse_cargo_target extracts the correct ELF path from cargo run commands."""
    cmd = "cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release"
    assert bringup.parse_cargo_target(cmd) == "target/thumbv6m-none-eabi/release/cat_detector_shell"

    cmd_debug = "cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell"
    assert bringup.parse_cargo_target(cmd_debug) == "target/thumbv6m-none-eabi/debug/cat_detector_shell"

    cmd_no_target = "cargo run --package cat_detector --bin cat_detector_shell --release"
    assert bringup.parse_cargo_target(cmd_no_target) == "target/release/cat_detector_shell"


def test_check_regex():
    """Verify that check_regex correctly matches substrings and regular expression patterns."""
    text = "Line 1: Device boot completed\nLine 2: Direct proximity readings: North = 100 mm\nLine 3: OK"

    # Simple substring / regex match
    matched, line = bringup.check_regex(text, r"North = \d+ mm")
    assert matched
    assert line == "Line 2: Direct proximity readings: North = 100 mm"

    # Multicharacter ranges
    matched, line = bringup.check_regex(text, r"North = (9[0-9]|10[0-9]) mm")
    assert matched
    assert line == "Line 2: Direct proximity readings: North = 100 mm"

    # Non-matching pattern
    matched, line = bringup.check_regex(text, r"South = \d+ mm")
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
    assert bringup.parse_cargo_target(cmd_bin) == "target/debug/custom_mcu_binary"

    cmd_bin_release = "cargo run --bin custom_mcu_binary --release"
    assert bringup.parse_cargo_target(cmd_bin_release) == "target/release/custom_mcu_binary"
