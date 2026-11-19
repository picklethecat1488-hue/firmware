"""Full benchtop electrical bringup and watchdog reboot tests for Carrier Board 2.0 (`carrier_board*`)."""

import re

from device_tests.client import AsyncHostSessionClient
from device_tests.manual import ManualTestHelper
from device_tests.model import ScenarioTier, device_test


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.BRINGUP,),
    target_binary=None,
    manual=True,
)
async def test_power_rails_and_protection(
    host: AsyncHostSessionClient,
    manual: ManualTestHelper,
) -> None:
    """Verify BQ24074 power-path regulation, 3.3V system buck, and 1.1V MCX N947 core supply before flashing."""
    del host
    result = await manual.verify(
        procedure=(
            "Apply 5.0V current-limited bench power (500 mA limit) to USB_VBUS (J1 Pin A4/B9, "
            "BQ24074 EN1=LOW/EN2=HIGH) and probe TP1=5.00V+/-0.25V, TP2=3.30V+/-0.05V, "
            "TP3=1.10V+/-0.03V, and TP4=3.70V-4.20V with a DMM."
        ),
        prompt="Confirm all rails settle within +/-3% tolerance with quiescent draw < 45 mA and zero thermal hotspots.",
    )
    assert result.passed


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.BRINGUP,),
    target_binary="carrier_board_shell",
    manual=True,
)
async def test_swd_attach_and_ftdi_console_banner(
    host: AsyncHostSessionClient,
    manual: ManualTestHelper,
) -> None:
    """Verify Cortex-M33 SWD attachment (`0x6BA02477`) and FTDI FT232RNQ 1 Mb/s UART console banner."""
    meta = await host.fs("query_metadata", ["--json"])
    assert "carrier_board" in meta, meta

    result = await manual.verify(
        procedure=(
            "Connect FTDI FT232RNQ USB-C console port (1,000,000 baud 8N1 on FC1_TXD/P0_2, FC1_RXD/P0_3) "
            "and press SW_RESET."
        ),
        prompt="Confirm CARRIER BOARD 2.0 ASCII banner and `shell>` prompt render cleanly without framing errors.",
    )
    assert result.passed


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.BRINGUP,),
    target_binary="carrier_board_shell",
)
async def test_hardware_watchdog_reset_recovery(host: AsyncHostSessionClient) -> None:
    """Trigger controlled WWDT0 timeout (`sys watchdog-test`), reboot, and verify CMC `LAST_RESET: WATCHDOG`."""
    out = await host.cli("sys watchdog-test", timeout=10.0)
    await host.reboot()
    assert re.search(r"LAST_RESET:\s*WATCHDOG.*BOOT:\s*OK", out), out
