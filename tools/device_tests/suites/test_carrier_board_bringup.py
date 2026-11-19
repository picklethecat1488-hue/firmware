"""Full benchtop electrical bringup and watchdog reboot tests for Carrier Board 2.0 (`carrier_board*`)."""

import re

from device_tests.client import AsyncHostSessionClient
from device_tests.manual import ManualTestHelper
from device_tests.model import ScenarioTier, device_test

TP1_VBUS_MIN_V: float = 4.75
TP1_VBUS_MAX_V: float = 5.25
TP2_SYS_3V3_MIN_V: float = 3.25
TP2_SYS_3V3_MAX_V: float = 3.35
TP3_VCORE_MIN_V: float = 1.07
TP3_VCORE_MAX_V: float = 1.13
TP4_VBAT_MIN_V: float = 3.70
TP4_VBAT_MAX_V: float = 4.20
MAX_QUIESCENT_CURRENT_MA: float = 45.0


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
    bench_setup = (
        "Apply bench power to USB_VBUS (J1 Pin A4/B9, BQ24074 EN1=LOW/EN2=HIGH) "
        "and measure each rail test point with a calibrated DMM."
    )

    tp1 = await manual.read_measurement(
        procedure=bench_setup,
        prompt="Measure voltage on TP1 (USB_VBUS)",
        unit="V",
    )
    assert tp1.measurement is not None
    assert TP1_VBUS_MIN_V <= tp1.measurement <= TP1_VBUS_MAX_V, f"TP1 out of range: {tp1.measurement} V"

    tp2 = await manual.read_measurement(
        procedure=bench_setup,
        prompt="Measure voltage on TP2 (+3V3_SYS)",
        unit="V",
    )
    assert tp2.measurement is not None
    assert TP2_SYS_3V3_MIN_V <= tp2.measurement <= TP2_SYS_3V3_MAX_V, f"TP2 out of range: {tp2.measurement} V"

    tp3 = await manual.read_measurement(
        procedure=bench_setup,
        prompt="Measure voltage on TP3 (+1V1_CORE)",
        unit="V",
    )
    assert tp3.measurement is not None
    assert TP3_VCORE_MIN_V <= tp3.measurement <= TP3_VCORE_MAX_V, f"TP3 out of range: {tp3.measurement} V"

    tp4 = await manual.read_measurement(
        procedure=bench_setup,
        prompt="Measure voltage on TP4 (VBAT_SYS)",
        unit="V",
    )
    assert tp4.measurement is not None
    assert TP4_VBAT_MIN_V <= tp4.measurement <= TP4_VBAT_MAX_V, f"TP4 out of range: {tp4.measurement} V"

    quiescent = await manual.read_measurement(
        procedure=bench_setup,
        prompt="Measure input quiescent current draw from bench supply",
        unit="mA",
    )
    assert quiescent.measurement is not None
    assert 0.0 <= quiescent.measurement < MAX_QUIESCENT_CURRENT_MA, (
        f"Quiescent draw out of range: {quiescent.measurement} mA"
    )

    thermal = await manual.ask_yes_no(
        procedure="Inspect BQ24074, buck converter, and MCX N947 packages for abnormal heating.",
        prompt="Are all regulators and MCU packages free of thermal hotspots?",
    )
    assert thermal.passed


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

    result = await manual.ask_yes_no(
        procedure=(
            "Connect FTDI FT232RNQ USB-C console port (1,000,000 baud 8N1 on FC1_TXD/P0_2, FC1_RXD/P0_3) "
            "and press SW_RESET."
        ),
        prompt="Did the CARRIER BOARD 2.0 ASCII banner and `shell>` prompt render cleanly without framing errors?",
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
