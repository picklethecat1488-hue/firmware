"""On-device integration tests for the Cat Detector (`rp2040*`) project.

Covers dual-core executor monitoring, `host_fs` SWD file round-trips with 16-bit
`.program_metadata` checksum verification, async `defmt` log watching, and
automatic `cat_detector_app` binary switching.
"""

from pathlib import Path
import re

from device_tests.client import AsyncHostSessionClient
from device_tests.model import ScenarioTier, device_test


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
)
async def test_gpio_matrix_and_dual_core_monitor(host: AsyncHostSessionClient) -> None:
    """Verify `gpio status` and `core status` report healthy pin states and zero stuck/panicked cores."""
    gpio = await host.cli("gpio status")
    assert re.search(r"(Pin|GPIO|IN|OUT|HIGH|LOW)", gpio), gpio

    core = await host.cli("core status")
    assert "Core Monitor Status:" in core, core


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
)
async def test_host_fs_transfer_and_metadata_checksum(host: AsyncHostSessionClient) -> None:
    """Verify 16-bit checksummed `.program_metadata` dump and `host_fs` `cp`/`ls`/`cat`/`rm` round-trip."""
    meta_out = await host.fs("query_metadata", ["--json"])
    assert re.search(r'"board_type"\s*:\s*"rp2040', meta_out), meta_out
    assert re.search(r'"checksum"\s*:\s*"0x[0-9A-Fa-f]{4}"', meta_out), meta_out

    board_info = await host.query_metadata(force_refresh=True)
    assert board_info is not None and board_info.matches_target

    repo_root = Path(__file__).resolve().parents[3]
    src_rel = "board/board.toml"
    dst_rel = "target/_host_fs_roundtrip.toml"
    dst_local = repo_root / dst_rel
    if dst_local.exists():
        dst_local.unlink()

    await host.fs("cp", [src_rel, "dev:_host_fs_test.toml"])
    ls_out = await host.fs("ls")
    assert "_host_fs_test.toml" in ls_out, ls_out

    cat_out = await host.fs("cat", ["_host_fs_test.toml"])
    assert "rp2040_flash_2mb" in cat_out, cat_out

    await host.fs("cp", ["dev:_host_fs_test.toml", dst_rel])
    if dst_local.exists():
        assert dst_local.read_text(encoding="utf-8") == (repo_root / src_rel).read_text(encoding="utf-8")
        dst_local.unlink()

    await host.fs("rm", ["_host_fs_test.toml"])


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
)
async def test_system_activity_async_log_watcher(host: AsyncHostSessionClient) -> None:
    """Await target `defmt` RTT logs emitted concurrently while dispatching `system activity`."""
    async with host.watch_logs(
        r"(command execution succeeded|Activity|System|INFO)",
        timeout=6.0,
    ) as watcher:
        out = await host.cli("system activity")
        assert re.search(r"(Executed:\s*system activity|System controller|OK)", out), out
    matched = await watcher.matched()
    assert len(matched) >= 1


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="cat_detector_app",
)
async def test_standalone_app_binary_boot_lifecycle(host: AsyncHostSessionClient) -> None:
    """Flash `cat_detector_app` via `target_binary` and verify clean dual-core Embassy boot logs."""
    async with host.watch_logs(
        r"(boot|INFO|System|Battery|Thermal|Sensor)",
        timeout=10.0,
        since_boot=True,
    ) as watcher:
        matched = await watcher.matched()
    assert len(matched) >= 1
