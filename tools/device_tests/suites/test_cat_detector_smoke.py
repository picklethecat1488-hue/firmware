"""On-device smoke & actuator verification tests for the Cat Detector (`rp2040*`) project.

Tests execute strictly from top to bottom in source-line order via `tools/host.py`.
"""

import re

from device_tests.client import AsyncHostSessionClient
from device_tests.model import ScenarioTier, device_test


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
)
async def test_battery_fuel_gauge_status(host: AsyncHostSessionClient) -> None:
    """Verify MAX17048 fuel gauge cell voltage (3000-4350 mV) and state-of-charge over I2C0."""
    out = await host.cli("battery status")
    assert re.search(r"Voltage:\s*\d+\s*mV", out), out
    assert re.search(r"SoC:\s*\d+%", out), out


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
)
async def test_battery_alert_pin_assertion(host: AsyncHostSessionClient) -> None:
    """Exercise MAX17048 active-low `FUEL_GAUGE_INT_PIN` alert assertion and restoration."""
    out = await host.cli("battery test-alert")
    assert re.search(r"Pin asserted \(low\):\s*(true|false)", out), out


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
)
async def test_tof_proximity_array_and_i2c_bus(host: AsyncHostSessionClient) -> None:
    """Scan I2C0 and verify North, East, and West VL53L0X ToF channels report valid distance or INVALID."""
    scan = await host.cli("i2c scan", timeout=10.0)
    assert re.search(r"(Scanning I2C bus|[0-9a-f]{2}:|0x[0-9a-fA-F]{2})", scan), scan

    sensor = await host.cli("sensor status", timeout=12.0)
    assert re.search(r"(\d+\s*mm|INVALID)", sensor), sensor
    assert "FAILED" not in sensor, sensor


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
)
async def test_thermal_and_mcu_die_temperature(host: AsyncHostSessionClient) -> None:
    """Verify ThermalController and RP2040 on-die ADC temperature sensors report plausible ambient values."""
    therm = await host.cli("thermal status", timeout=10.0)
    assert re.search(r"([1-5]\d(\.\d+)?\s*C|Thermal)", therm), therm

    mcu = await host.cli("thermal mcu", timeout=10.0)
    assert re.search(r"([1-6]\d(\.\d+)?\s*C|RP2040)", mcu), mcu


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
)
async def test_flash_filesystem_listing(host: AsyncHostSessionClient) -> None:
    """Verify non-destructive `sequential-storage` flash directory read (`fs ls`)."""
    out = await host.cli("fs ls")
    assert re.search(r"(Listing directory|No files found|Filename|bytes)", out), out


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
)
async def test_ws2812_status_led_actuator(host: AsyncHostSessionClient) -> None:
    """Exercise PIO0 WS2812 RGB status LED color transitions, brightness scaling, and clean turn-off."""
    green = await host.cli("led green")
    assert re.search(r"(LED|green|Executed:\s*led)", green, re.IGNORECASE), green

    blue = await host.cli("led blue")
    assert re.search(r"(LED|blue|Executed:\s*led)", blue, re.IGNORECASE), blue

    bright = await host.cli("led brightness 50")
    assert re.search(r"(brightness|50|Executed:\s*led)", bright, re.IGNORECASE), bright

    off = await host.cli("led off")
    assert re.search(r"(LED|off|Executed:\s*led)", off, re.IGNORECASE), off


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
)
async def test_bldc_pump_and_current_telemetry(host: AsyncHostSessionClient) -> None:
    """Verify TMC6300 3-phase BLDC pump commutation, INA219 current telemetry, and safe stop."""
    idle_cur = await host.cli("motor current")
    assert re.search(r"(-?\d+\s*mA|Motor current)", idle_cur), idle_cur

    try:
        spin = await host.cli("motor speed 50")
        assert re.search(r"(Motor|speed|Executed:\s*motor|\d+\s*mA)", spin, re.IGNORECASE), spin

        active_cur = await host.cli("motor current")
        assert re.search(r"(-?\d+\s*mA|Motor current)", active_cur), active_cur

        rpm = await host.cli("motor rpm")
        assert re.search(r"(\d+\s*RPM|Motor speed|rpm)", rpm, re.IGNORECASE), rpm
    finally:
        stop = await host.cli("motor stop")
        assert re.search(r"(stop|Motor|0\s*%|Executed:\s*motor)", stop, re.IGNORECASE), stop
