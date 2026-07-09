import sys
import os

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import bringup


def test_parse_value():
    assert bringup.parse_value("   hello   ") == "hello"
    assert bringup.parse_value('"quoted"') == "quoted"
    assert bringup.parse_value("'single_quoted'") == "single_quoted"
    assert bringup.parse_value("line1\\nline2") == "line1\nline2"
    assert bringup.parse_value("") == ""


def test_parse_yaml_simple():
    yaml_text = """
project_name: "Test Project"
device_chip: RP2040
flash_addr: "0x10000000"
    """
    res = bringup.parse_yaml(yaml_text)
    assert res["project_name"] == "Test Project"
    assert res["device_chip"] == "RP2040"
    assert res["flash_addr"] == "0x10000000"


def test_parse_yaml_list():
    yaml_text = """
steps:
  - id: 1
    name: "Check Power"
    type: manual
  - id: 2
    name: "Check Clock"
    type: host
    command: "ping"
    """
    res = bringup.parse_yaml(yaml_text)
    assert "steps" in res
    assert len(res["steps"]) == 2
    assert res["steps"][0]["id"] == "1"
    assert res["steps"][0]["name"] == "Check Power"
    assert res["steps"][1]["command"] == "ping"
