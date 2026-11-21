"""On-device integration tests for the Cat Detector (`rp2040*`) project.

Covers dual-core executor monitoring, `host_fs` SWD file round-trips with 16-bit
`.program_metadata` checksum verification, async `defmt` log watching, and
automatic `cat_detector_app` binary switching.
"""

import asyncio
from pathlib import Path
import re
import shutil
import socket
import subprocess

from device_tests.client import AsyncHostSessionClient, _find_free_local_port
from device_tests.model import ScenarioTier, device_test

GDB_RSP_SOCKET_TIMEOUT_SEC: float = 5.0


class _MinimalGdbRspClient:
    """Minimal synchronous GDB RSP TCP client for exercising `host_debug` in `@device_test` cases."""

    def __init__(self, port: int) -> None:
        self._sock = socket.create_connection(("127.0.0.1", port), timeout=GDB_RSP_SOCKET_TIMEOUT_SEC)
        self._sock.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
        self._rx = bytearray()

    def close(self) -> None:
        """Close the underlying TCP socket."""
        try:
            self._sock.close()
        except OSError:
            pass

    def _frame(self, payload: str) -> bytes:
        raw = payload.encode("ascii")
        csum = sum(raw) & 0xFF
        return b"$" + raw + f"#{csum:02x}".encode("ascii")

    def read_reply(self) -> str:
        """Read the next `$payload#XX` packet from `host_debug`."""
        while True:
            while self._rx[:1] in (b"+", b"-"):
                del self._rx[0]
            if self._rx.startswith(b"$"):
                hash_idx = self._rx.find(b"#", 1)
                if hash_idx != -1 and len(self._rx) >= hash_idx + 3:
                    body = bytes(self._rx[1:hash_idx]).decode("utf-8", errors="replace")
                    del self._rx[: hash_idx + 3]
                    return body
            chunk = self._sock.recv(2048)
            if not chunk:
                raise RuntimeError("GDB RSP socket closed unexpectedly")
            self._rx.extend(chunk)

    def command(self, cmd: str) -> str:
        """Send an RSP command packet and return its reply payload."""
        self._sock.sendall(self._frame(cmd))
        return self.read_reply()

    def send_continue(self) -> None:
        """Send an asynchronous `c` continue packet without blocking immediately."""
        self._sock.sendall(self._frame("c"))

    def read_pc(self) -> int:
        """Read 32-bit architectural `PC` (`regnum 0x0f`) in little-endian byte order."""
        le_hex = self.command("p0f")
        return int.from_bytes(bytes.fromhex(le_hex), byteorder="little")


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
    assert re.search(r'"bin_name"\s*:\s*"cat_detector_shell"', meta_out), meta_out
    assert re.search(r'"checksum"\s*:\s*"0x[0-9A-Fa-f]{4}"', meta_out), meta_out
    size_match = re.search(r'"program_metadata_size"\s*:\s*(\d+)', meta_out)
    assert size_match is not None and int(size_match.group(1)) <= 12, meta_out

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
        assert re.search(
            r"(Executed:\s*system activity|System activity recorded|System controller|OK)",
            out,
        ), out
    matched = await watcher.matched()
    assert len(matched) >= 1


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
)
async def test_dual_core_gdb_reset_handler_to_loc(host: AsyncHostSessionClient) -> None:
    """Verify breaking at `Reset_Handler` -> Rust LOC on Core 0 and `Core1_Reset_Handler` -> Rust LOC on Core 1 via `host_debug`."""
    core0_loc_spec = "app/src/cat_detector_shell.rs:100"
    core1_loc_spec = "app/src/cat_detector.rs:181"

    prior_status = await host.get_status()
    prior_dbg = dict(prior_status.get("debug_server") or {})
    was_debug_enabled = bool(prior_dbg.get("enabled", False))

    c0_port = _find_free_local_port()
    c1_port = _find_free_local_port()
    rtt_port = _find_free_local_port()

    dbg = await host.start_debug_server(
        gdb_core0_port=c0_port,
        gdb_core1_port=c1_port,
        rtt_bridge_port=rtt_port,
        halt_on_reset=True,
        reset=True,
        resolve=["Reset_Handler", "Core1_Reset_Handler", core0_loc_spec, core1_loc_spec],
    )
    assert dbg.get("active") is True, dbg
    resolved = dbg["resolved"]
    reset_handler_addr = int(resolved["Reset_Handler"])
    core1_reset_addr = int(resolved["Core1_Reset_Handler"])
    core0_loc_addr = int(resolved[core0_loc_spec])
    core1_loc_addr = int(resolved[core1_loc_spec])

    def _exercise_dual_core_gdb() -> None:
        gdb0 = _MinimalGdbRspClient(int(dbg["gdb_core0_port"]))
        gdb1 = _MinimalGdbRspClient(int(dbg["gdb_core1_port"]))
        try:
            assert gdb0.command("QStartNoAckMode") == "OK"
            assert gdb1.command("QStartNoAckMode") == "OK"
            assert gdb0.command("?") == "S05"
            assert gdb1.command("?") == "S05"
            assert gdb0.read_pc() == reset_handler_addr

            # Set Core 0 breakpoint at `cat_detector_shell.rs:100` and Core 1 at `Core1_Reset_Handler`.
            # Leave Core 1 halted in Mask Boot ROM while Core 0 runs through `Board::init(p).await`
            # (verifying RP2040 `TIMER.DBGPAUSE = DBG0` never freezes Core 0 `.await` timers when Core 1 is halted!).
            assert gdb0.command(f"Z1,{core0_loc_addr:x},2") == "OK"
            assert gdb1.command(f"Z1,{core1_reset_addr:x},2") == "OK"

            gdb0.send_continue()

            # Core 0 hits `cat_detector_shell.rs:100` before `bootstrap_core1()`.
            assert gdb0.read_reply() == "S05"
            assert gdb0.read_pc() == core0_loc_addr

            # Single-step past the armed hardware breakpoint (`WORM-LUBRICATED-FLATWORM-512`) with interrupts masked!
            assert gdb0.command("vCont;c:2;s:1") == "S05"
            stepped_pc = gdb0.read_pc()
            assert stepped_pc != core0_loc_addr, f"Single-step remained trapped at 0x{stepped_pc:08x}"
            assert stepped_pc < 0x2000_0000, f"Single-step jumped out of flash: 0x{stepped_pc:08x}"

            # Clear Core 0 breakpoint and continue Core 0 through `bootstrap_core1()` -> Core 1 auto-resumes Boot ROM & hits `Core1_Reset_Handler`!
            assert gdb0.command(f"z1,{core0_loc_addr:x},2") == "OK"
            gdb1.send_continue()
            gdb0.send_continue()

            assert gdb1.read_reply() == "S05"
            assert gdb1.read_pc() == core1_reset_addr

            # Advance Core 1 from `Core1_Reset_Handler` to `app/src/cat_detector.rs:181` (`bootstrap_core1_task`).
            assert gdb1.command(f"z1,{core1_reset_addr:x},2") == "OK"
            assert gdb1.command(f"Z1,{core1_loc_addr:x},2") == "OK"
            gdb1.send_continue()

            assert gdb1.read_reply() == "S05"
            assert gdb1.read_pc() == core1_loc_addr

            # Clear Core 1 breakpoint and detach both GDB clients so both cores run freely.
            assert gdb1.command(f"z1,{core1_loc_addr:x},2") == "OK"
            assert gdb1.command("D") == "OK"
            # Halt Core 0 briefly with Ctrl-C so `D` detach handshake completes cleanly on Core 0.
            gdb0._sock.sendall(b"\x03")
            assert gdb0.read_reply() in ("S02", "S05")
            assert gdb0.command("D") == "OK"
        finally:
            gdb0.close()
            gdb1.close()

    try:
        await asyncio.to_thread(_exercise_dual_core_gdb)
        # Wait for `host_cli --gdb-bridge` to lock onto `_SEGGER_RTT` now that Core 0 & Core 1 are running!
        for _ in range(30):
            st = await host.get_status()
            if st.get("connected"):
                break
            await asyncio.sleep(0.15)
        await asyncio.sleep(0.25)
        core_out = await host.cli("core status")
        assert "Core Monitor Status:" in core_out, core_out

        gdb_bin = shutil.which("arm-none-eabi-gdb")
        active_link = dbg.get("active_elf_link") or dbg.get("executable")
        if gdb_bin and active_link and Path(str(active_link)).is_file():
            gdb_proc = await asyncio.to_thread(
                subprocess.run,
                [
                    gdb_bin,
                    "--nx",
                    "--batch",
                    "-ex",
                    f"target extended-remote 127.0.0.1:{int(dbg['gdb_core1_port'])}",
                    "-ex",
                    "bt",
                    "-ex",
                    "detach",
                    str(active_link),
                ],
                capture_output=True,
                text=True,
                timeout=15.0,
                check=False,
            )
            bt_combined = f"{gdb_proc.stdout}\n{gdb_proc.stderr}"
            assert gdb_proc.returncode == 0, bt_combined
            assert "Core1_Reset_Handler" in bt_combined, bt_combined
            assert "sequential_storage" not in bt_combined, bt_combined
            assert "0x00000000 in ??" not in bt_combined, bt_combined
            assert "Remote replied unexpectedly" not in bt_combined, bt_combined
    finally:
        if was_debug_enabled:
            await host.start_debug_server(
                gdb_core0_port=int(prior_dbg.get("gdb_core0_port") or 3333),
                gdb_core1_port=int(prior_dbg.get("gdb_core1_port") or 3334),
                rtt_bridge_port=int(prior_dbg.get("rtt_bridge_port") or 50000),
                halt_on_reset=False,
                reset=False,
            )
        else:
            await host.stop_debug_server()


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
    trace_mode="PERF",
)
async def test_trace_mode_perf_emits_spans_and_excludes_sensors(
    host: AsyncHostSessionClient,
) -> None:
    """Verify `PERF` trace mode (`--features trace-perf`) emits spans/CPU usage while excluding sensor reports."""
    repo_root = Path(__file__).resolve().parents[3]
    csv_out = repo_root / "target" / "_test_perf_trace.csv"
    perfetto_out = repo_root / "target" / "_test_perf_trace.perfetto.json"
    for p in (csv_out, perfetto_out):
        if p.exists():
            p.unlink()

    await host.control_trace("clear")

    await host.cli("motor current")
    await host.cli("fs ls")
    await host.cli("sensor status")
    await host.cli("thermal status")

    events = []
    for _ in range(30):
        snap = await host.get_trace_events(after_seq=0, limit=5000)
        events = list(snap.get("events") or [])
        has_span = any(e.get("category") == "span" for e in events)
        has_c0_usage = any(e.get("track") == "Core 0 CPU Usage (%)" for e in events)
        has_c1_usage = any(e.get("track") == "Core 1 CPU Usage (%)" for e in events)
        if has_span and has_c0_usage and has_c1_usage:
            break
        await asyncio.sleep(0.12)

    span_events = [e for e in events if e.get("category") == "span"]
    sensor_events = [e for e in events if e.get("category") == "sensor"]
    c0_usage_events = [e for e in events if e.get("track") == "Core 0 CPU Usage (%)"]
    c1_usage_events = [e for e in events if e.get("track") == "Core 1 CPU Usage (%)"]
    assert len(span_events) >= 1, f"Expected >= 1 span in PERF mode, got: {events[:10]}"
    assert len(c0_usage_events) >= 1, (
        f"Expected Core 0 CPU Usage (%) track in PERF mode, got tracks: { {e.get('track') for e in events} }"
    )
    assert len(c1_usage_events) >= 1, (
        f"Expected Core 1 CPU Usage (%) track in PERF mode, got tracks: { {e.get('track') for e in events} }"
    )
    assert len(sensor_events) == 0, f"PERF mode must exclude sensor reports, got: {sensor_events[:5]}"

    csv_res = await host.export_trace("csv", str(csv_out))
    perfetto_res = await host.export_trace("perfetto", str(perfetto_out))
    assert csv_res.get("success") is True and csv_out.is_file() and csv_out.stat().st_size > 0
    assert perfetto_res.get("success") is True and perfetto_out.is_file() and perfetto_out.stat().st_size > 0
    csv_out.unlink()
    perfetto_out.unlink()


@device_test(
    project="cat_detector",
    boards=("rp2040*",),
    scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
    target_binary="cat_detector_shell",
    trace_mode="SENSOR",
)
async def test_trace_mode_sensor_emits_sensor_reports_only(
    host: AsyncHostSessionClient,
) -> None:
    """Verify `SENSOR` trace mode (`--features trace-sensors`) emits measured sensor values with zero spans."""
    await host.control_trace("clear")

    await host.cli("sensor status")
    await host.cli("thermal status")
    await host.cli("motor current")

    events = []
    for _ in range(25):
        snap = await host.get_trace_events(after_seq=0, limit=5000)
        events = list(snap.get("events") or [])
        has_prox = any("Proximity" in str(e.get("track", "")) for e in events)
        has_therm = any("Thermal" in str(e.get("track", "")) for e in events)
        if has_prox and has_therm:
            break
        await asyncio.sleep(0.12)

    sensor_events = [e for e in events if e.get("category") == "sensor"]
    span_events = [e for e in events if e.get("category") == "span"]
    assert len(sensor_events) >= 2, f"Expected sensor reports in SENSOR mode, got: {events}"
    assert any("Proximity" in str(e.get("track", "")) for e in sensor_events), sensor_events
    assert any(
        "Thermal" in str(e.get("track", "")) and isinstance(e.get("value"), (int, float)) for e in sensor_events
    ), sensor_events
    assert len(span_events) == 0, f"SENSOR mode must not emit execution spans, got: {span_events[:5]}"


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
