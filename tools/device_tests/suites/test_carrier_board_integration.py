"""On-device integration tests for Carrier Board 2.0 (`carrier_board*`).

Covers BQ24074 + MAX17048 PMIC telemetry, IQS7222A capacitive touch ATI lock,
Winbond W25N01GV QSPI NAND R/W CRC32 verification, expansion card fault isolation,
SRAMX dual-core IPC latency, and ARMv8-M MPU + stack guard hardening.
"""

import re

from device_tests.client import AsyncHostSessionClient
from device_tests.model import ScenarioTier, device_test


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="carrier_board_shell",
)
async def test_pmic_charger_and_fuel_gauge(host: AsyncHostSessionClient) -> None:
    """Verify TI BQ24074 power-good status and MAX17048 ModelGauge SoC/voltage telemetry."""
    out = await host.cli("pmic status")
    assert re.search(r"BQ24074.*SOC:\s*\d+%.*VBAT:\s*\d+\s*mV", out), out


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="carrier_board_shell",
)
async def test_iqs7222a_touch_controller_baseline(host: AsyncHostSessionClient) -> None:
    """Verify Azoteq IQS7222A ProxFusion capacitive touch controller ATI calibration lock (`IQS7222A: READY`)."""
    out = await host.cli("touch status")
    assert re.search(r"IQS7222A:\s*READY", out), out


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="carrier_board_shell",
)
async def test_w25n01gv_qspi_nand_flash_rw(host: AsyncHostSessionClient) -> None:
    """Verify Winbond W25N01GV JEDEC ID over 100 MHz FlexSPI Port A and non-destructive sector R/W CRC32."""
    probe = await host.cli("flash probe")
    assert re.search(r"W25N01GV.*128\s*MB", probe), probe

    rw = await host.cli("flash test --sector 256", timeout=15.0)
    assert re.search(r"VERIFY:\s*PASS", rw), rw


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="carrier_board_shell",
)
async def test_expansion_card_fault_isolation(host: AsyncHostSessionClient) -> None:
    """Verify graceful fault isolation when optional expansion cards are populated or unpopulated."""
    out = await host.cli("expansion status")
    assert re.search(r"EXPANSION:\s*(NONE|CAMERA_OK|GNSS_OK|AUDIO_OK)", out), out


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="carrier_board_shell",
)
async def test_dual_core_sramx_ipc_ping(host: AsyncHostSessionClient) -> None:
    """Verify zero-loss 10-packet lock-free SRAMX SPSC mailbox round-trip between Core 0 and Core 1."""
    out = await host.cli("ipc ping --count 10")
    assert re.search(r"10/10\s*OK", out), out


@device_test(
    project="carrier_board",
    boards=("carrier_board*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="carrier_board_shell",
)
async def test_armv8m_mpu_and_stack_guard_status(host: AsyncHostSessionClient) -> None:
    """Verify ARMv8-M MPU Execute-Never (`XN`) regions and `MSPLIM`/`PSPLIM` hardware stack guards."""
    out = await host.cli("sys status")
    assert re.search(r"MPU:\s*ENABLED.*STACK_GUARD:\s*OK", out), out
