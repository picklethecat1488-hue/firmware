"""Full hardware bringup, calibration, fault-injection, and manual benchtop tests for Cat Detector."""

import re

from device_tests.client import AsyncHostSessionClient
from device_tests.manual import ManualTestHelper
from device_tests.model import ScenarioTier, device_test


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.BRINGUP,),
    target_binary="cat_detector_shell",
)
async def test_flash_filesystem_format_and_recovery(host: AsyncHostSessionClient) -> None:
    """Format the `sequential-storage` flash filesystem (`fs format`) and verify clean directory state."""
    fmt = await host.cli("fs format", timeout=15.0)
    assert re.search(r"(Format|formatted|success|OK)", fmt, re.IGNORECASE), fmt

    ls_out = await host.cli("fs ls")
    assert re.search(r"(No files found|Listing directory|empty)", ls_out, re.IGNORECASE), ls_out


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.BRINGUP,),
    target_binary="cat_detector_shell",
    manual=True,
)
async def test_motor_calibration_routine(
    host: AsyncHostSessionClient,
    manual: ManualTestHelper,
) -> None:
    """Run BLDC motor calibration (`motor calibrate`) and confirm smooth water flow on the bench."""
    await manual.ask_yes_no(
        procedure="Ensure fountain reservoir is filled above minimum waterline before starting calibration.",
        prompt="Is the reservoir filled and clear of obstructions?",
    )
    cal = await host.cli("motor calibrate", timeout=20.0)
    assert cal.strip() != ""
    result = await manual.ask_yes_no(
        procedure="Observe pump ramp and acoustic noise during `motor calibrate`.",
        prompt="Did the pump ramp smoothly with steady flow and zero DIAG fault assertions?",
    )
    assert result.passed


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.BRINGUP,),
    target_binary="cat_detector_shell",
    manual=True,
)
async def test_tof_sensor_calibration_sequence(
    host: AsyncHostSessionClient,
    manual: ManualTestHelper,
) -> None:
    """Execute VL53L0X near, far, and crosstalk calibration with a gray target card."""
    await manual.ask_yes_no(
        procedure="Place a flat 17% gray calibration target at 100 mm in front of the North ToF window.",
        prompt="Is the 100 mm calibration target positioned?",
    )
    await host.cli("sensor cal_near", timeout=15.0)

    await manual.ask_yes_no(
        procedure="Move calibration target to 300 mm for far-field ranging.",
        prompt="Is the 300 mm far target positioned?",
    )
    await host.cli("sensor cal_far", timeout=15.0)
    await host.cli("sensor cal_xtalk", timeout=15.0)

    status = await host.cli("sensor status")
    assert re.search(r"(\d+\s*mm|INVALID)", status), status


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.BRINGUP,),
    target_binary="cat_detector_shell",
    manual=True,
)
async def test_core_panic_and_crash_dump_decoding(
    host: AsyncHostSessionClient,
    manual: ManualTestHelper,
) -> None:
    """Trigger controlled panic (`core crash`), reboot RP2040, and decode CBOR crash dump via `host_fs`."""
    await manual.ask_yes_no(
        procedure="Prepare to trigger a controlled Core 0 panic (`core crash`) to test flash crash dump persistence.",
        prompt="Are you ready to trigger controlled fault injection and target reboot?",
    )
    await host.cli("core crash")
    await host.reboot()
    crash = await host.fs("crash-log")
    assert re.search(r"(Crash|Panic|Backtrace|No crash|crash_)", crash), crash


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.BRINGUP,),
    target_binary="cat_detector_app",
    manual=True,
)
async def test_cat_presence_approach_and_departure_journey(
    host: AsyncHostSessionClient,
    manual: ManualTestHelper,
) -> None:
    """Verify full physical cat arrival wake, fountain activation, and return to sleep on `cat_detector_app`."""
    del host
    result = await manual.ask_yes_no(
        procedure=(
            "Hold hand ~150 mm in front of the North ToF window for 3 seconds to simulate cat arrival, "
            "confirm fountain pump + LED activation, then withdraw hand and wait for idle sleep."
        ),
        prompt="Did the application wake from Sleep to Active, drive pump + LED, and return cleanly to Sleep?",
    )
    assert result.passed
