"""Asynchronous pytest hardware-in-the-loop test harness built on top of tools/host.py."""

from device_tests.client import AsyncHostSessionClient, LogStreamWatcher, SyncOutcome
from device_tests.manual import ManualPromptAction, ManualPromptResult, ManualTestHelper
from device_tests.model import (
    BoardType,
    DetectedBoard,
    DeviceTestMetadata,
    ScenarioTier,
    TargetBinarySpec,
    check_regex,
    device_test,
    discover_app_binaries,
    discover_device_tests,
    get_device_test_metadata,
    infer_scenario_from_script,
    load_device_test_file,
    matches_board_patterns,
    parse_query_metadata_output,
    resolve_target_binary,
)

__all__ = [
    "AsyncHostSessionClient",
    "BoardType",
    "DetectedBoard",
    "DeviceTestMetadata",
    "LogStreamWatcher",
    "ManualPromptAction",
    "ManualPromptResult",
    "ManualTestHelper",
    "ScenarioTier",
    "SyncOutcome",
    "TargetBinarySpec",
    "check_regex",
    "device_test",
    "discover_app_binaries",
    "discover_device_tests",
    "get_device_test_metadata",
    "infer_scenario_from_script",
    "load_device_test_file",
    "matches_board_patterns",
    "parse_query_metadata_output",
    "resolve_target_binary",
]
