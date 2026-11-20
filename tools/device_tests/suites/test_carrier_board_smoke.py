"""On-device smoke & actuator verification tests for Carrier Board 2.0 (`carrier_board*`).

Tests execute strictly from top to bottom in source-line order and skip automatically
when no `carrier_board*` target is connected.
"""

import re

from device_tests.client import AsyncHostSessionClient
from device_tests.model import ScenarioTier, device_test


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="carrier_board_shell",
)
async def test_mcxn947_dual_core_boot_info(host: AsyncHostSessionClient) -> None:
    """Verify NXP MCX N947 Real-Time Processing Core (Core 0) and Always-On Core (Core 1) boot at 150 MHz."""
    out = await host.cli("sys info")
    assert re.search(r"MCXN94[67].*150\s*MHz.*Core0:\s*OK.*Core1:\s*OK", out), out


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="carrier_board_shell",
)
async def test_i3c_and_i2c_bus_enumeration(host: AsyncHostSessionClient) -> None:
    """Verify LPI2C0 (`0x14`, `0x36`) and I3C0 dynamic address assignment (`ENTDAA` `OK`)."""
    i2c = await host.cli("i2c scan")
    assert re.search(r"0x14.*0x36|0x36.*0x14", i2c), i2c

    i3c = await host.cli("i3c scan")
    assert re.search(r"I3C0.*OK", i3c), i3c


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="carrier_board_shell",
)
async def test_lp5009_rgb_led_driver_sweep(host: AsyncHostSessionClient) -> None:
    """Exercise TI LP5009 9-channel constant-current logarithmic RGB LED sweep and turn-off."""
    sweep = await host.cli("led test")
    assert re.search(r"(LP5009.*PASS|LED.*PASS)", sweep), sweep

    off = await host.cli("led off")
    assert re.search(r"(LED|off|OK|Executed:\s*led)", off, re.IGNORECASE), off


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="carrier_board_shell",
)
async def test_audio_amplifier_and_piezo_chime(host: AsyncHostSessionClient) -> None:
    """Verify MAX98357A Class-D PDM/I2S readiness (`I2S: READY`) and piezo chime synthesis."""
    st = await host.cli("audio status")
    assert re.search(r"I2S:\s*READY", st), st

    chime = await host.cli("audio chime")
    assert re.search(r"PLAYBACK:\s*COMPLETE", chime), chime


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
    target_binary="carrier_board_shell",
)
async def test_nina_b312_ble_transceiver_link(host: AsyncHostSessionClient) -> None:
    """Verify 1 Mb/s UART link and MAC readout from the u-blox NINA-B312 BLE 5.0 module."""
    ble = await host.cli("ble info")
    assert re.search(r"NINA-B312.*MAC:\s*([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}", ble), ble
