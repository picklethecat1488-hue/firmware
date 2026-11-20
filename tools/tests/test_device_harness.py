"""Behavioral unit and integration tests for the async on-device pytest test harness."""

import asyncio
from collections import deque
from datetime import datetime, timezone
import os
from pathlib import Path
import subprocess
import sys
import threading
import time
from typing import Any, Deque, Dict, List, Optional
import xml.etree.ElementTree as ET

import pytest

WORKSPACE_ROOT = Path(__file__).resolve().parents[2]
TOOLS_DIR = WORKSPACE_ROOT / "tools"
DASHBOARD_DIR = TOOLS_DIR / "dashboard"
for p in (str(WORKSPACE_ROOT), str(TOOLS_DIR), str(DASHBOARD_DIR)):
    if p not in sys.path:
        sys.path.insert(0, p)

from device_tests.client import AsyncHostSessionClient, _http_json_sync
from device_tests.manual import ManualPromptAction, ManualTestHelper
from device_tests.model import (
    BoardType,
    DetectedBoard,
    ScenarioTier,
    TargetBinarySpec,
    check_regex,
    discover_app_binaries,
    infer_scenario_from_script,
    matches_board_patterns,
    parse_query_metadata_output,
    resolve_target_binary,
)
from host_daemon.model.manual_prompt import ManualPromptState
from host_daemon.model.metadata import get_cargo_target_directory, validate_metadata_matches_target
from host_daemon.provider.daemon.server import HostDaemonServer


class SimulatedTargetRunner:
    """Stateful simulated `DaemonRunner` exercising `HostDaemonServer` HTTP routes end-to-end."""

    def __init__(
        self,
        repo_root: Path,
        *,
        connected_board_type: Optional[str] = "rp2040_flash_2mb",
        connected_chip: str = "rp2040",
        initial_bin: str = "cat_detector_shell",
        initial_flashed: bool = False,
        metadata_matches_target: bool = True,
    ) -> None:
        """Initialize simulated target state, virtual flash map, and monotonic log streams."""
        self.repo_root = repo_root
        self.connected_board_type = connected_board_type
        self.connected_chip = connected_chip
        self.target = "thumbv6m-none-eabi"
        self.package = "cat_detector"
        self.bin_name = initial_bin
        self.release = False
        self.speed = 1000
        self.chip: Optional[str] = connected_chip
        self.package_selection_mode = False
        self.show_system_logs = False
        self.active_mode = "CLI"
        self.is_flashed = initial_flashed
        self.is_connected = initial_flashed
        self.metadata_matches_target = metadata_matches_target
        self.status_message = "Ready"
        self.elf_path: Optional[Path] = None

        self._lock = threading.Lock()
        self.logs: Deque[Dict[str, Any]] = deque(maxlen=2000)
        self.cli_outputs: Deque[Dict[str, Any]] = deque(maxlen=2000)
        self.fs_outputs: Deque[Dict[str, Any]] = deque(maxlen=2000)
        self._log_seq = 0
        self.command_history: List[str] = []
        self.executed_cli_commands: List[str] = []
        self.sync_history: List[str] = []
        self.reboot_count: int = 0
        self.stop_called_count: int = 0
        self.virtual_fs: Dict[str, str] = {}
        self.manual_prompt = ManualPromptState()
        self.peak_manual_heartbeats: int = 0

        self.sync_status: Dict[str, Any] = {
            "active": False,
            "step": "complete" if initial_flashed else "idle",
            "step_index": 4 if initial_flashed else 0,
            "total_steps": 4,
            "progress": 100 if initial_flashed else 0,
            "message": "Ready",
            "target": self.target,
            "package": self.package,
            "bin": self.bin_name,
            "profile": "debug",
            "speed": self.speed,
            "flashed_commit": "deadbeef",
            "error": None,
            "console": [],
        }

    def log(self, message: str, level: str = "INFO", channel: str = "target") -> None:
        """Append a structured log entry to the thread-safe buffer."""
        with self._lock:
            self._log_seq += 1
            self.logs.append(
                {
                    "id": self._log_seq,
                    "timestamp": datetime.now(timezone.utc).isoformat(),
                    "level": level,
                    "channel": channel,
                    "message": message,
                }
            )

    def update_manual_prompt(self, payload: Dict[str, Any]) -> Dict[str, Any]:
        """Record periodic manual prompt heartbeat from `device_tests`."""
        with self._lock:
            self.manual_prompt.apply_update(payload)
            self.peak_manual_heartbeats = max(self.peak_manual_heartbeats, self.manual_prompt.heartbeat_count)
            return self.manual_prompt.to_dict()

    def get_status(self) -> Dict[str, Any]:
        """Return daemon status payload."""
        return {
            "status": "running",
            "target": self.target,
            "package": self.package,
            "bin": self.bin_name,
            "profile": "release" if self.release else "debug",
            "elf": None,
            "chip": self.chip,
            "speed": self.speed,
            "connected": self.is_connected,
            "flashed": self.is_flashed,
            "mode": self.active_mode,
            "message": self.status_message,
            "manual_prompt": self.manual_prompt.to_dict(),
        }

    def query_target_metadata(self) -> Dict[str, Any]:
        """Simulate `/api/metadata` query and target ELF metadata match check on `host.py`."""
        if self.connected_board_type is None:
            return {"connected": False, "matches_target": False, "error": "No debug probes connected"}
        dev_crc = "0x9A3F" if self.metadata_matches_target else "0x0001"
        return {
            "connected": True,
            "board_type": self.connected_board_type,
            "chip": self.connected_chip,
            "source": "program_metadata",
            "checksum": dev_crc,
            "elf_checksum": "0x9A3F",
            "matches_target": self.metadata_matches_target,
        }

    def update_config(
        self,
        target: Optional[str] = None,
        package: Optional[str] = None,
        bin_name: Optional[str] = None,
        release: Optional[bool] = None,
        speed: Optional[int] = None,
        chip: Optional[str] = None,
        show_system_logs: Optional[bool] = None,
    ) -> None:
        """Update target configuration fields prior to synchronization."""
        del show_system_logs
        if target:
            self.target = target
        if package:
            self.package = package
        if bin_name:
            self.bin_name = bin_name
        if release is not None:
            self.release = release
        if speed:
            self.speed = speed
        if chip:
            self.chip = chip

    def sync(self) -> Dict[str, Any]:
        """Simulate firmware build, probe-rs flash, RTT attach, and boot `defmt` log emission."""
        self.is_flashed = True
        self.is_connected = True
        self.metadata_matches_target = True
        self.sync_history.append(self.bin_name)
        with self._lock:
            self.sync_status.update(
                {
                    "active": False,
                    "step": "complete",
                    "step_index": 4,
                    "progress": 100,
                    "message": "Target synchronized & RTT connected successfully.",
                    "package": self.package,
                    "bin": self.bin_name,
                    "target": self.target,
                    "profile": "release" if self.release else "debug",
                    "error": None,
                }
            )
        self.log(f"INFO [boot] Booted {self.bin_name} -> SystemStatus::Active -> SystemStatus::Sleep")
        return dict(self.sync_status)

    def sync_async(self) -> None:
        """Execute simulated sync immediately for deterministic test timing."""
        self.sync()

    def cancel_sync(self) -> Dict[str, Any]:
        """Cancel active sync."""
        return {"success": True, "step": "cancelled"}

    def set_mode(self, mode: str) -> None:
        """Switch workstation mode."""
        self.active_mode = mode

    def reset_target(self) -> Dict[str, Any]:
        """Simulate hardware reset and boot log."""
        self.log(f"INFO [boot] Hardware reset completed ({self.bin_name})")
        return {"success": True, "probe_reset": True, "message": "Target reset completed."}

    def reboot_target(self) -> Dict[str, Any]:
        """Simulate hardware reboot via `/api/reboot`."""
        self.reboot_count += 1
        self.log(f"INFO [boot] Hardware reboot completed ({self.bin_name})")
        return {"success": True, "probe_reset": True, "message": "Target reboot completed."}

    def stop(self) -> None:
        """Record daemon stop requests."""
        self.stop_called_count += 1

    def execute_cli(self, command: str, wait_for_output: bool = False, timeout: float = 5.0) -> str:
        """Simulate realistic RP2040 Cat Detector and MCXN947 Carrier Board CLI responses."""
        del wait_for_output, timeout
        cmd = command.strip()
        self.executed_cli_commands.append(cmd)
        self.command_history.append(cmd)

        if cmd == "fs format":
            self.virtual_fs.clear()
            return "Formatting filesystem partition...\nFormatting successful!"
        if cmd == "battery status":
            return "Battery Status:\n  Voltage: 3980 mV\n  SoC: 87%"
        if cmd == "battery test-alert":
            return "Pin asserted (low): true. Pin deasserted (high): true. Pre-clear alerts: voltage=true, soc=false"
        if cmd == "i2c scan":
            return "Scanning I2C bus...\n0x14 (LP5009), 0x30 (VL53L0X), 0x36 (MAX17048)"
        if cmd == "sensor status":
            return "north: 125 mm | east: 180 mm | west: INVALID"
        if cmd == "thermal status":
            return "Direct thermal reading (ThermalController): 26.500 C"
        if cmd == "thermal mcu":
            return "Direct system temperature reading (RP2040): 28.125 C"
        if cmd == "fs ls":
            if not self.virtual_fs:
                return "Listing directory...\nNo files found (directory empty)."
            return "Listing directory...\nFilename\n" + "\n".join(sorted(self.virtual_fs.keys()))
        if cmd == "led test":
            return "LP5009 RGB 9-channel pattern sweep: PASS"
        if cmd.startswith("led brightness"):
            return "LED brightness set to 50%"
        if cmd.startswith("led"):
            self.log(f"INFO [led] {cmd}")
            return "LED pattern set successfully"
        if cmd == "motor current":
            return "Motor current: 0 mA"
        if cmd.startswith("motor speed"):
            self.log(f"INFO [motor] {cmd}")
            return "Motor current: 485 mA"
        if cmd == "motor rpm":
            return "Motor speed: 1250 RPM"
        if cmd == "motor stop":
            return "Executed: motor stop"
        if cmd == "gpio status":
            return "GPIO Pin 11: OUT LOW | GPIO Pin 14: IN HIGH"
        if cmd == "core status":
            return "Core Monitor Status:\nCore  Status  Last Progress  Stuck  Panicked  Timeout\n0     OK      1020           false  false     10000"
        if cmd == "system activity":
            self.log("INFO [shell] command execution succeeded (system activity)")
            return "Executed: system activity"
        if cmd == "core crash":
            self.virtual_fs["crash_0.cbor"] = "CrashLog: Panicked at explicit core crash"
            return "Triggering controlled Crash"

        # MCXN947 Carrier Board commands
        if cmd == "sys info":
            return "MCU: NXP MCXN947 @ 150 MHz | Core0: OK | Core1: OK"
        if cmd == "i3c scan":
            return "I3C0 Bus: ENTDAA DAA OK (0x44)"
        if cmd == "audio status":
            return "Audio Subsystem: I2S: READY"
        if cmd == "audio chime":
            return "Chime synthesis -> PLAYBACK: COMPLETE"
        if cmd == "ble info":
            return "Radio: u-blox NINA-B312 FW 5.0 | MAC: D4:CA:6E:11:22:33"
        if cmd == "pmic status":
            return "Charger: BQ24074 OK | SOC: 92% | VBAT: 4065 mV"
        if cmd == "touch status":
            return "Capacitive Touch: IQS7222A: READY (ATI locked)"
        if cmd == "flash probe":
            return "FlexSPI Port A: Winbond W25N01GV (128 MB SLC NAND)"
        if cmd.startswith("flash test"):
            return "Sector 256 R/W CRC32 -> VERIFY: PASS"
        if cmd == "expansion status":
            return "Modular Slot: EXPANSION: NONE (ERR_EXPANSION_NOT_FOUND isolated)"
        if cmd.startswith("ipc ping"):
            return "SRAMX Mailbox Ping: 10/10 OK (avg 14 us)"
        if cmd == "sys status":
            return "Hardening: MPU: ENABLED | STACK_GUARD: OK (MSPLIM/PSPLIM active)"
        if cmd == "sys watchdog-test":
            return "CMC Reset Cause -> LAST_RESET: WATCHDOG | BOOT: OK"

        return f"Executed: {cmd}"

    def execute_fs(self, command_or_subcommand: str, args: Optional[List[str]] = None) -> str:
        """Simulate `host_fs` subcommands (`query_metadata`, `cp`, `ls`, `cat`, `rm`, `crash-log`)."""
        tokens = command_or_subcommand.strip().split() + [str(a) for a in (args or [])]
        subcmd = tokens[0] if tokens else "ls"
        rest = tokens[1:]

        if subcmd == "query_metadata":
            if self.connected_board_type is None:
                return "Error: No debug probes connected"
            crc = "0x9A3F" if self.metadata_matches_target else "0x0001"
            if "--json" in rest:
                return (
                    f'{{"checksum":"{crc}","chip":"{self.connected_chip}",'
                    f'"board_type":"{self.connected_board_type}",'
                    f'"program_metadata_board_type":"{self.connected_board_type}",'
                    f'"source":"program_metadata"}}'
                )
            return f"checksum: {crc}\nchip: {self.connected_chip}\nboard_type: {self.connected_board_type}\nsource: program_metadata"

        if subcmd == "cp" and len(rest) == 2:
            src, dst = rest
            if dst.startswith("dev:"):
                dev_key = dst[len("dev:") :].lstrip("/")
                src_path = Path(src) if Path(src).is_absolute() else (self.repo_root / src)
                self.virtual_fs[dev_key] = src_path.read_text(encoding="utf-8")
                return f"Copied {src} -> {dst}"
            if src.startswith("dev:"):
                dev_key = src[len("dev:") :].lstrip("/")
                dst_path = Path(dst) if Path(dst).is_absolute() else (self.repo_root / dst)
                dst_path.parent.mkdir(parents=True, exist_ok=True)
                dst_path.write_text(self.virtual_fs.get(dev_key, ""), encoding="utf-8")
                return f"Copied {src} -> {dst}"

        if subcmd == "ls":
            return "\n".join(sorted(self.virtual_fs.keys())) or "(empty)"
        if subcmd == "cat" and rest:
            return self.virtual_fs.get(rest[0].lstrip("/"), "")
        if subcmd == "rm" and rest:
            self.virtual_fs.pop(rest[0].lstrip("/"), None)
            return f"Removed {rest[0]}"
        if subcmd == "crash-log":
            return "Decoded CrashDump #0: Backtrace at core crash"

        return f"Completed {subcmd}"


def _start_ephemeral_daemon(runner: SimulatedTargetRunner) -> tuple[HostDaemonServer, int]:
    """Bind a `HostDaemonServer` on an ephemeral port in a daemon thread."""
    server = HostDaemonServer("127.0.0.1", 0, runner)  # type: ignore[arg-type]
    port = int(server.server_address[1])
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    return server, port


def _run_device_pytest(port: int, extra_args: List[str]) -> subprocess.CompletedProcess[str]:
    """Invoke `pytest` against `tools/device_tests/suites` pointing at `127.0.0.1:<port>`."""
    env = dict(os.environ)
    env["PYTHONPATH"] = os.pathsep.join([str(TOOLS_DIR), str(DASHBOARD_DIR), str(WORKSPACE_ROOT)])
    cmd = [
        sys.executable,
        "-m",
        "pytest",
        "-o",
        "addopts=",
        "-v",
        str(TOOLS_DIR / "device_tests" / "suites"),
        "--host-addr",
        "127.0.0.1",
        "--host-port",
        str(port),
        *extra_args,
    ]
    return subprocess.run(cmd, cwd=str(WORKSPACE_ROOT), env=env, capture_output=True, text=True, check=False)


def test_board_glob_matching_and_host_metadata_validation() -> None:
    """Verify glob `board_type` matching, `query_metadata` parsing, and `host.py` target ELF validation."""
    assert matches_board_patterns("rp2040_flash_2mb", [BoardType.RP2040_FLASH_2MB])
    assert matches_board_patterns("rp2040_flash_2mb", ["rp2040*"])
    assert matches_board_patterns("rp2040_flash_4mb", ["rp2040_flash_*"])
    assert matches_board_patterns("carrier_board", ["carrier_board*"])
    assert matches_board_patterns("carrier_board", ["*"])
    assert not matches_board_patterns("rp2040_flash_2mb", [BoardType.CARRIER_BOARD])
    assert not matches_board_patterns("carrier_board", ["rp2040*"])

    parsed_json = parse_query_metadata_output(
        '{"checksum":"0x9A3F","board_type":"rp2040_flash_2mb","chip":"rp2040","source":"program_metadata"}'
    )
    assert parsed_json is not None
    assert parsed_json.board_type == "rp2040_flash_2mb"
    assert parsed_json.chip == "rp2040"
    assert parsed_json.checksum == "0x9A3F"

    parsed_kv = parse_query_metadata_output(
        "checksum: 0x4411\nchip: MCXN947\nboard_type: carrier_board\nsource: metadata_partition\n"
    )
    assert parsed_kv is not None
    assert parsed_kv.board_type == "carrier_board"
    assert parsed_kv.source == "metadata_partition"

    assert parse_query_metadata_output("Error: No debug probes connected") is None
    assert parse_query_metadata_output('{"board_type":"rp2040_flash_2mb"}') is None
    assert DetectedBoard.from_metadata_response({"board_type": "rp2040_flash_2mb", "chip": ""}) is None

    with pytest.raises(ValueError, match="unset"):
        TargetBinarySpec()
    with pytest.raises(ValueError, match="Unknown target binary"):
        resolve_target_binary("nonexistent_firmware_bin", repo_root=WORKSPACE_ROOT)

    app_bins = discover_app_binaries(WORKSPACE_ROOT)
    assert set(app_bins.keys()) >= {"cat_detector_shell", "cat_detector_app", "carrier_board_shell"}
    assert app_bins["carrier_board_shell"].chip == "MCXN947"

    cargo_target_dir = get_cargo_target_directory(WORKSPACE_ROOT)
    assert cargo_target_dir.name == "target"

    target_spec = {
        "checksum": "0x9A3F",
        "chip": "rp2040",
        "board_type": "rp2040_flash_2mb",
        "program_metadata_board_type": "rp2040_flash_2mb",
        "flash_start": 0x10000000,
        "flash_write_size": 1,
        "flash_erase_size": 4096,
        "stack_scan_limit": 8192,
        "partitions": [{"kind": 0, "name": "fs", "address": 0x101B0000, "size": 196608}],
    }
    assert validate_metadata_matches_target(dict(target_spec), target_spec)
    # Provisioned override in `metadata` partition still matches target ELF if `.program_metadata` matches
    provisioned_dev = dict(target_spec, board_type="rp2040_flash_4mb", source="metadata_partition")
    assert validate_metadata_matches_target(provisioned_dev, target_spec)
    # 16-bit checksum mismatch must reject target match and force re-sync
    corrupted_dev = dict(target_spec, checksum="0xFFFF")
    assert not validate_metadata_matches_target(corrupted_dev, target_spec)


def test_no_device_detected_skips_all_device_suites() -> None:
    """Verify all device tests skip cleanly when `query_metadata` detects no attached probe."""
    runner = SimulatedTargetRunner(WORKSPACE_ROOT, connected_board_type=None)
    server, port = _start_ephemeral_daemon(runner)
    try:
        res = _run_device_pytest(port, ["-m", "smoke or integration"])
        assert res.returncode == 0, f"Expected clean skip exit 0, got {res.returncode}:\n{res.stdout}\n{res.stderr}"
        assert "failed" not in res.stdout.lower()
        assert "skipped" in res.stdout.lower()
        assert runner.sync_history == [], "Must not attempt firmware flash when no device is attached"
    finally:
        server.shutdown()
        server.server_close()


def test_rp2040_smoke_integration_project_filter_and_binary_sync() -> None:
    """Verify RP2040 top-to-bottom ordering, `--project` isolation, and binary sync."""
    runner = SimulatedTargetRunner(WORKSPACE_ROOT, connected_board_type="rp2040_flash_2mb")
    server, port = _start_ephemeral_daemon(runner)
    try:
        smoke_res = _run_device_pytest(port, ["--project", "cat_detector", "-m", "smoke"])
        assert smoke_res.returncode == 0, f"Smoke failed:\n{smoke_res.stdout}\n{smoke_res.stderr}"
        assert "7 passed" in smoke_res.stdout
        assert "deselected" in smoke_res.stdout
        assert runner.sync_history == ["cat_detector_shell"]

        expected_cmd_sequence = [
            "battery status",
            "battery test-alert",
            "sensor status",
            "thermal status",
            "fs ls",
            "led green",
            "motor current",
        ]
        positions = [runner.executed_cli_commands.index(c) for c in expected_cmd_sequence]
        assert positions == sorted(positions), f"Commands executed out of top-to-bottom order: {positions}"

        runner.executed_cli_commands.clear()
        integ_res = _run_device_pytest(port, ["--project", "cat_detector", "-m", "integration"])
        assert integ_res.returncode == 0, f"Integration failed:\n{integ_res.stdout}\n{integ_res.stderr}"
        assert "4 passed" in integ_res.stdout
        assert "battery status" not in runner.executed_cli_commands
        assert runner.sync_history == ["cat_detector_shell", "cat_detector_app", "cat_detector_shell"]
    finally:
        server.shutdown()
        server.server_close()


def test_carrier_board_smoke_and_integration_and_reporting(tmp_path: Path) -> None:
    """Verify Carrier Board 2.0 smoke + integration suites, Markdown report, and JUnit XML properties."""
    runner = SimulatedTargetRunner(
        WORKSPACE_ROOT,
        connected_board_type="carrier_board",
        connected_chip="MCXN947",
    )
    server, port = _start_ephemeral_daemon(runner)
    report_md = tmp_path / "carrier_report.md"
    junit_xml = tmp_path / "carrier_junit.xml"
    try:
        res = _run_device_pytest(
            port,
            [
                "-m",
                "smoke or integration",
                "--report-md",
                str(report_md),
                "--junitxml",
                str(junit_xml),
            ],
        )
        assert res.returncode == 0, f"Carrier Board suite failed:\n{res.stdout}\n{res.stderr}"
        assert "11 passed" in res.stdout
        assert runner.sync_history == ["carrier_board_shell"]

        md_text = report_md.read_text(encoding="utf-8")
        assert "# On-Device Verification Report" in md_text
        assert "`carrier_board_shell`" in md_text
        assert "LP5009" in md_text

        tree = ET.parse(junit_xml)
        prop_names = {el.attrib.get("name") for el in tree.findall(".//property")}
        assert "description" in prop_names
        assert "target_binary" in prop_names
    finally:
        server.shutdown()
        server.server_close()


@pytest.mark.asyncio
async def test_async_client_reboot_checksum_resync_and_manual_heartbeats() -> None:
    """Verify `AsyncHostSessionClient` string outputs, `/api/reboot`, checksum re-sync, and manual heartbeats."""
    runner = SimulatedTargetRunner(
        WORKSPACE_ROOT,
        connected_board_type="rp2040_flash_2mb",
        initial_bin="cat_detector_shell",
        initial_flashed=True,
        metadata_matches_target=False,
    )
    server, port = _start_ephemeral_daemon(runner)
    try:
        client = await AsyncHostSessionClient.connect(WORKSPACE_ROOT, host="127.0.0.1", port=port)
        assert client.owns_daemon is False

        # Even though daemon reports `flashed=True`, `matches_target=False` MUST trigger a fresh sync!
        sync_res = await client.ensure_target_binary("cat_detector_shell")
        assert sync_res is not None and sync_res.synced is True
        assert runner.sync_history == ["cat_detector_shell"]

        # `cli` and `fs` return plain strings
        bat_str = await client.cli("battery status")
        assert isinstance(bat_str, str)
        assert "3980 mV" in bat_str

        reboot_res = await client.reboot()
        assert reboot_res.get("success") is True
        assert runner.reboot_count == 1

        # Verify `ManualTestHelper` supports ask_yes_no, read_measurement, and cancel with comment
        def _delayed_operator_input(_prompt: str) -> str:
            time.sleep(0.14)
            return "yes"

        helper = ManualTestHelper(
            host=client,
            test_name="test_motor_calibration_routine",
            project="cat_detector",
            manual_enabled=True,
            heartbeat_interval=0.04,
            prompt_reader=_delayed_operator_input,
        )
        manual_res = await helper.ask_yes_no(
            procedure="Fill reservoir above min waterline.",
            prompt="Did pump ramp smoothly?",
        )
        assert manual_res.action == ManualPromptAction.YES
        assert manual_res.passed is True
        assert runner.peak_manual_heartbeats >= 2
        assert runner.manual_prompt.active is False

        meas_helper = ManualTestHelper(
            host=client,
            test_name="test_power_rails_and_protection",
            project="carrier_board",
            manual_enabled=True,
            heartbeat_interval=0.04,
            prompt_reader=lambda _p: "5.02 V",
        )
        meas_res = await meas_helper.read_measurement(
            procedure="Probe TP1 with DMM.",
            prompt="Measure voltage on TP1",
            unit="V",
        )
        assert meas_res.action == ManualPromptAction.MEASUREMENT
        assert meas_res.measurement == pytest.approx(5.02)

        cancel_helper = ManualTestHelper(
            host=client,
            test_name="test_power_rails_and_protection",
            project="carrier_board",
            manual_enabled=True,
            heartbeat_interval=0.04,
            prompt_reader=lambda _p: "cancel Bench DMM probe lead unplugged",
        )
        with pytest.raises(pytest.skip.Exception, match="Bench DMM probe lead unplugged"):
            await cancel_helper.ask_yes_no(
                procedure="Probe TP2.",
                prompt="Is rail clean?",
            )

        async def _delayed_log_emitter() -> None:
            await asyncio.sleep(0.08)
            runner.log("INFO [sensor] Async proximity wake threshold crossed (88 mm)")

        async with client.watch_logs(r"proximity wake threshold crossed \(\d+ mm\)", timeout=3.0) as watcher:
            emit_task = asyncio.create_task(_delayed_log_emitter())
            await emit_task
        matched_lines = await watcher.matched()
        assert len(matched_lines) == 1
        assert check_regex(matched_lines[0], r"88 mm")[0]

        await client.ensure_target_binary("cat_detector_app")
        assert runner.bin_name == "cat_detector_app"
        await client.close()
        assert runner.stop_called_count == 0
        assert runner.bin_name == "cat_detector_shell"
    finally:
        server.shutdown()
        server.server_close()


def test_host_test_optional_script_and_scenario_inference(tmp_path: Path) -> None:
    """Verify `host.py test` accepts optional script paths and infers the scenario automatically."""
    from host_daemon.cli import main as host_cli_main

    assert (
        infer_scenario_from_script("tools/device_tests/suites/test_cat_detector_bringup.py", WORKSPACE_ROOT)
        == "bringup"
    )
    assert (
        infer_scenario_from_script("tools/device_tests/suites/test_carrier_board_bringup.py", WORKSPACE_ROOT)
        == "bringup"
    )
    assert (
        infer_scenario_from_script(
            "tools/device_tests/suites/test_cat_detector_integration.py::test_gpio_matrix_and_dual_core_monitor",
            WORKSPACE_ROOT,
        )
        == "integration or bringup"
    )

    smoke_script = tmp_path / "custom_battery_smoke_suite.py"
    smoke_script.write_text(
        "import re\n"
        "from device_tests.client import AsyncHostSessionClient\n"
        "from device_tests.model import ScenarioTier, device_test\n\n"
        "@device_test(\n"
        "    project='cat_detector',\n"
        "    boards=('rp2040*',),\n"
        "    scenarios=(ScenarioTier.SMOKE,),\n"
        "    target_binary='cat_detector_shell',\n"
        ")\n"
        "async def test_custom_smoke(host: AsyncHostSessionClient) -> None:\n"
        "    out = await host.cli('battery status')\n"
        "    assert re.search(r'Voltage:', out), out\n",
        encoding="utf-8",
    )
    assert infer_scenario_from_script(smoke_script, WORKSPACE_ROOT) == "smoke"

    unknown_script = tmp_path / "plain_helper.py"
    unknown_script.write_text("async def helper_fn() -> None:\n    return None\n", encoding="utf-8")
    with pytest.raises(ValueError, match="Could not infer scenario tier"):
        infer_scenario_from_script(unknown_script, WORKSPACE_ROOT)

    runner = SimulatedTargetRunner(WORKSPACE_ROOT, connected_board_type="rp2040_flash_2mb")
    server, port = _start_ephemeral_daemon(runner)
    try:
        rc = host_cli_main(["--port", str(port), "test", "--script", str(smoke_script), "--host-port", str(port)])
        assert rc == 0
        assert "battery status" in runner.executed_cli_commands
    finally:
        server.shutdown()
        server.server_close()


def test_host_test_mode_ui_scan_filter_run_and_manual_measurement_responses() -> None:
    """Verify `host.py` Test Mode UI APIs (`/api/tests*` & `/api/manual_prompt/respond`) end-to-end."""
    runner = SimulatedTargetRunner(
        WORKSPACE_ROOT,
        connected_board_type="carrier_board",
        connected_chip="MCXN947",
    )
    server, port = _start_ephemeral_daemon(runner)
    base_url = f"http://127.0.0.1:{port}"
    try:
        mode_res = _http_json_sync(f"{base_url}/api/mode", {"mode": "TEST"})
        assert mode_res["success"] is True
        assert runner.active_mode == "TEST"

        scan_res = _http_json_sync(
            f"{base_url}/api/tests/scan",
            {
                "project": "carrier_board",
                "scenario": "bringup",
                "manual": "manual",
                "probe_board": True,
            },
        )
        assert scan_res["success"] is True
        assert scan_res["detected_board"]["board_type"] == "carrier_board"
        node_ids = [item["node_id"] for item in scan_res["tests"]]
        rails_node = "tools/device_tests/suites/test_carrier_board_bringup.py::test_power_rails_and_protection"
        assert rails_node in node_ids

        # Execute `test_power_rails_and_protection` via Test Mode UI and feed measurements over HTTP!
        run_res = _http_json_sync(
            f"{base_url}/api/tests/run",
            {"node_ids": [rails_node], "enable_manual": True},
        )
        assert run_res["success"] is True

        scripted_measurements = {
            "TP1": "5.01",
            "TP2": "3.31",
            "TP3": "1.10",
            "TP4": "3.95",
            "quiescent": "22.4",
        }
        answered_prompt_ids: set[str] = set()
        deadline = time.monotonic() + 25.0
        final_status: Dict[str, Any] = {}

        while time.monotonic() < deadline:
            st = _http_json_sync(f"{base_url}/api/tests/status")
            final_status = st
            prompt_info = st.get("manual_prompt") or {}
            if prompt_info.get("active") and prompt_info.get("prompt_id") not in answered_prompt_ids:
                pid = str(prompt_info["prompt_id"])
                answered_prompt_ids.add(pid)
                if prompt_info.get("kind") == "measurement":
                    prompt_text = str(prompt_info.get("prompt", ""))
                    chosen = next(
                        (val for key, val in scripted_measurements.items() if key.lower() in prompt_text.lower()),
                        "5.00",
                    )
                    _http_json_sync(
                        f"{base_url}/api/manual_prompt/respond",
                        {"prompt_id": pid, "action": "measurement", "value": chosen},
                    )
                else:
                    _http_json_sync(
                        f"{base_url}/api/manual_prompt/respond",
                        {"prompt_id": pid, "action": "yes"},
                    )
            if not st.get("active") and st.get("step") in ("complete", "failed", "cancelled"):
                break
            time.sleep(0.08)

        assert final_status.get("step") == "complete", f"Unexpected final status: {final_status}"
        assert final_status.get("passed") == 1
        assert final_status["results_map"][rails_node]["status"] == "PASSED"
        assert len(answered_prompt_ids) == 6
    finally:
        server.shutdown()
        server.server_close()


def test_worm_translucent_planarian_439_all_binaries_excludes_other_board_types() -> None:
    """Verify 'All Binaries' excludes tests and binary facets targeting a different board type."""
    # 1. Connected rp2040_flash_2mb board with binary='all' and board='*' -> must exclude carrier_board* tests
    rp_runner = SimulatedTargetRunner(
        WORKSPACE_ROOT,
        connected_board_type="rp2040_flash_2mb",
        connected_chip="rp2040",
        initial_bin="cat_detector_shell",
    )
    rp_server, rp_port = _start_ephemeral_daemon(rp_runner)
    try:
        rp_scan = _http_json_sync(
            f"http://127.0.0.1:{rp_port}/api/tests/scan",
            {"project": "all", "scenario": "all", "board": "*", "binary": "all", "probe_board": True},
        )
        assert rp_scan["success"] is True
        rp_projects = {item["project"] for item in rp_scan["tests"]}
        assert rp_projects == {"cat_detector"}, f"Unexpected non-RP2040 projects in All Binaries: {rp_projects}"
        assert "carrier_board_shell" not in rp_scan["facets"]["binaries"]
        assert "cat_detector_shell" in rp_scan["facets"]["binaries"]
        assert "cat_detector_app" in rp_scan["facets"]["binaries"]
    finally:
        rp_server.shutdown()
        rp_server.server_close()

    # 2. Offline probe (connected_board_type=None) configured for carrier_board_shell -> excludes rp2040* tests
    cb_runner = SimulatedTargetRunner(
        WORKSPACE_ROOT,
        connected_board_type=None,
        connected_chip="MCXN947",
        initial_bin="carrier_board_shell",
    )
    cb_server, cb_port = _start_ephemeral_daemon(cb_runner)
    try:
        cb_scan = _http_json_sync(
            f"http://127.0.0.1:{cb_port}/api/tests/scan",
            {"project": "all", "scenario": "all", "board": "auto", "binary": "all", "probe_board": True},
        )
        assert cb_scan["success"] is True
        cb_projects = {item["project"] for item in cb_scan["tests"]}
        assert cb_projects == {"carrier_board"}, f"Unexpected non-carrier_board projects in All Binaries: {cb_projects}"
        assert "cat_detector_shell" not in cb_scan["facets"]["binaries"]
        assert "carrier_board_shell" in cb_scan["facets"]["binaries"]
    finally:
        cb_server.shutdown()
        cb_server.server_close()


def test_worm_rooted_fireworm_570_test_execution_logs_attached_in_test_mode_only(tmp_path: Path) -> None:
    """Verify `/api/target_bug` attaches test execution logs ONLY when filing bugs in `TEST` mode."""
    (tmp_path / "feedback").mkdir(parents=True, exist_ok=True)
    (tmp_path / "attachments").mkdir(parents=True, exist_ok=True)
    runner = SimulatedTargetRunner(
        tmp_path,
        connected_board_type="rp2040_flash_2mb",
        connected_chip="rp2040",
    )
    server, port = _start_ephemeral_daemon(runner)
    base_url = f"http://127.0.0.1:{port}"
    try:
        from host_daemon.provider.daemon.server import _get_test_manager

        tm = _get_test_manager(runner)
        tm._append_console("tools/device_tests/suites/test_cat_detector_smoke.py::test_battery PASSED")

        # 1. Filing bug in CLI mode -> must NOT include target_test_execution_logs_*.log
        runner.active_mode = "CLI"
        cli_bug = _http_json_sync(
            f"{base_url}/api/target_bug",
            {"title": "CLI mode bug check", "mode": "CLI", "attach_system_logs": True},
        )
        assert cli_bug["success"] is True
        cli_att_dir = tmp_path / "attachments" / cli_bug["worm_id"]
        assert not list(cli_att_dir.glob("target_test_execution_logs_*.log"))

        # 2. Filing bug in TEST mode -> MUST include target_test_execution_logs_*.log with pytest output
        runner.active_mode = "TEST"
        test_bug = _http_json_sync(
            f"{base_url}/api/target_bug",
            {"title": "Test mode bug check", "mode": "TEST", "attach_system_logs": True},
        )
        assert test_bug["success"] is True
        test_att_dir = tmp_path / "attachments" / test_bug["worm_id"]
        exec_logs = list(test_att_dir.glob("target_test_execution_logs_*.log"))
        assert len(exec_logs) == 1
        assert "test_battery PASSED" in exec_logs[0].read_text(encoding="utf-8")
    finally:
        server.shutdown()
        server.server_close()


def test_worm_silken_lumbricus_589_progress_and_target_lifecycle_status(monkeypatch: pytest.MonkeyPatch) -> None:
    """Verify `/api/tests/status` reports `progress_pct` and real-time target flashing, mode switch, reboot, and test execution states."""
    from host_daemon.provider.daemon.runner import DaemonRunner

    runner = DaemonRunner(
        repo_root=WORKSPACE_ROOT,
        package="cat_detector",
        bin_name="cat_detector_shell",
        target="thumbv6m-none-eabi",
        chip="rp2040",
    )
    monkeypatch.setattr(runner, "start_rtt_streaming", lambda: None)
    monkeypatch.setattr(runner, "stop_rtt_streaming", lambda: None)
    monkeypatch.setattr(runner, "run_probe_rs", lambda _args, **_kw: (0, "Reset complete"))

    server, port = _start_ephemeral_daemon(runner)
    base_url = f"http://127.0.0.1:{port}"
    try:
        # 1. Idle baseline status
        idle_st = _http_json_sync(f"{base_url}/api/tests/status")
        assert idle_st["progress_pct"] == 0
        assert idle_st["target_activity"]["phase"] == "idle"

        # 2. Simulate active test run (1 of 4 completed, 2nd running)
        tm = runner.test_manager
        with tm._lock:
            tm._active = True
            tm._step = tm._step.__class__("running")
            tm._total = 4
            tm._completed = 1
            tm._passed = 1
            tm._ordered_node_ids = [
                "suite.py::test_one",
                "suite.py::test_two",
                "suite.py::test_three",
                "suite.py::test_four",
            ]
            tm._current_test = "suite.py::test_two"
            tm._results_by_node_id = {
                "suite.py::test_one": {"node_id": "suite.py::test_one", "name": "test_one", "status": "PASSED"},
                "suite.py::test_two": {"node_id": "suite.py::test_two", "name": "test_two", "status": "RUNNING"},
            }

        exec_st = _http_json_sync(f"{base_url}/api/tests/status")
        assert 25 < exec_st["progress_pct"] < 60
        assert exec_st["target_activity"]["phase"] == "executing_test"
        assert "test_two" in exec_st["target_activity"]["headline"]

        # 3. Target flashing phase while test run is active
        runner._update_sync("flashing_target", 3, 75, "Flashing target device via probe-rs...", active=True)
        flash_st = _http_json_sync(f"{base_url}/api/tests/status")
        assert flash_st["target_activity"]["phase"] == "flashing"
        assert flash_st["target_activity"]["sync_active"] is True
        assert flash_st["target_activity"]["sync_progress"] == 75
        assert "Flashing" in flash_st["target_activity"]["headline"]
        runner._update_sync("complete", 4, 100, "Target synchronized & RTT connected successfully.", active=False)

        # 4. Target mode switch phase
        runner.set_mode("FS")
        mode_st = _http_json_sync(f"{base_url}/api/tests/status")
        assert mode_st["target_activity"]["phase"] == "mode_switch"
        assert mode_st["target_activity"]["mode"] == "FS"
        assert mode_st["target_activity"]["mode_switch_count"] >= 1
        assert "FS" in mode_st["target_activity"]["headline"]

        # 5. Target reboot phase
        runner.reboot_target()
        reboot_st = _http_json_sync(f"{base_url}/api/tests/status")
        assert reboot_st["target_activity"]["phase"] == "rebooting"
        assert reboot_st["target_activity"]["reboot_count"] == 1
        assert "reboot" in reboot_st["target_activity"]["headline"].lower()

        # 6. Suite completion -> 100%
        with tm._lock:
            tm._active = False
            tm._step = tm._step.__class__("complete")
            tm._completed = 4
            tm._passed = 4
            tm._current_test = None
        # Clear recent lifecycle window so completion state takes precedence
        runner._lifecycle_updated_at = 0.0
        done_st = _http_json_sync(f"{base_url}/api/tests/status")
        assert done_st["progress_pct"] == 100
        assert done_st["target_activity"]["phase"] == "complete"
    finally:
        server.shutdown()
        server.server_close()


def _count_white_pixels_in_png_region(
    png_path: Path,
    *,
    x_min_frac: float,
    x_max_frac: float,
    y_min_frac: float,
    y_max_frac: float,
) -> int:
    """Decode an RGB/RGBA PNG using pure stdlib `zlib`/`struct` and count `#ebebeb+` white pixels in fractional `[x, y]` bounds."""
    import struct
    import zlib

    raw = png_path.read_bytes()
    assert raw[:8] == b"\x89PNG\r\n\x1a\n"
    pos = 8
    width = height = color_type = 0
    idat_parts: List[bytes] = []
    while pos < len(raw):
        (length,) = struct.unpack(">I", raw[pos : pos + 4])
        chunk_type = raw[pos + 4 : pos + 8]
        chunk_data = raw[pos + 8 : pos + 8 + length]
        pos += 12 + length
        if chunk_type == b"IHDR":
            width, height, bit_depth, color_type, _, _, _ = struct.unpack(">IIBBBBB", chunk_data)
            assert bit_depth == 8 and color_type in (2, 6)
        elif chunk_type == b"IDAT":
            idat_parts.append(chunk_data)
        elif chunk_type == b"IEND":
            break

    x_min = int(width * x_min_frac)
    x_max = int(width * x_max_frac)
    y_min = int(height * y_min_frac)
    y_max = int(height * y_max_frac)

    bpp = 4 if color_type == 6 else 3
    stride = width * bpp
    decompressed = zlib.decompress(b"".join(idat_parts))
    prev_row = bytearray(stride)
    white_count = 0
    offset = 0
    for y in range(height):
        ftype = decompressed[offset]
        offset += 1
        cur = bytearray(decompressed[offset : offset + stride])
        offset += stride
        if ftype == 1:  # Sub
            for i in range(bpp, stride):
                cur[i] = (cur[i] + cur[i - bpp]) & 0xFF
        elif ftype == 2:  # Up
            for i in range(stride):
                cur[i] = (cur[i] + prev_row[i]) & 0xFF
        elif ftype == 3:  # Average
            for i in range(stride):
                left = cur[i - bpp] if i >= bpp else 0
                cur[i] = (cur[i] + ((left + prev_row[i]) >> 1)) & 0xFF
        elif ftype == 4:  # Paeth
            for i in range(stride):
                a = cur[i - bpp] if i >= bpp else 0
                b = prev_row[i]
                c = prev_row[i - bpp] if i >= bpp else 0
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                pr = a if (pa <= pb and pa <= pc) else (b if pb <= pc else c)
                cur[i] = (cur[i] + pr) & 0xFF
        prev_row = cur
        if y_min <= y <= y_max:
            for x in range(max(0, x_min), min(width, x_max + 1)):
                px = x * bpp
                if cur[px] >= 235 and cur[px + 1] >= 235 and cur[px + 2] >= 235:
                    white_count += 1
    return white_count


def test_worm_inching_millipede_969_no_white_scrollbar_gutter_artifact(tmp_path: Path) -> None:
    """Verify the rendered `#testStackList` scrollbar edge in Headless Chrome has zero bright-white gutter pixels."""
    import shutil

    chrome_candidates = [
        Path("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"),
        *(Path(p) for name in ("google-chrome", "chromium", "chromium-browser") if (p := shutil.which(name))),
    ]
    chrome_bin = next((c for c in chrome_candidates if c.is_file()), None)
    if chrome_bin is None:
        pytest.skip("Headless Chrome/Chromium not installed in this environment")

    runner = SimulatedTargetRunner(
        WORKSPACE_ROOT,
        connected_board_type="rp2040_flash_2mb",
        connected_chip="rp2040",
        initial_bin="cat_detector_shell",
    )
    runner.active_mode = "TEST"
    server, port = _start_ephemeral_daemon(runner)
    try:
        shot_path = tmp_path / "test_stack_render.png"
        subprocess.run(
            [
                str(chrome_bin),
                "--headless=new",
                "--no-sandbox",
                "--disable-gpu",
                "--window-size=1440,900",
                "--virtual-time-budget=2500",
                f"--screenshot={shot_path}",
                f"http://127.0.0.1:{port}/",
            ],
            check=True,
            capture_output=True,
            timeout=25.0,
        )
        assert shot_path.is_file()
        # Right edge of left-panel `#testStackList` (around x=660..715, y=260..860 on a 1440x900 50% split)
        white_pixels = _count_white_pixels_in_png_region(
            shot_path,
            x_min_frac=0.44,
            x_max_frac=0.51,
            y_min_frac=0.28,
            y_max_frac=0.94,
        )
        assert white_pixels == 0, (
            f"Found {white_pixels} bright white pixels along the right edge of #testStackList "
            "(light-mode scrollbar gutter artifact from WORM-INCHING-MILLIPEDE-969)"
        )
    finally:
        server.shutdown()
        server.server_close()


def test_worm_annelidic_bristle_968_host_test_cli_keyword_filter(capsys: pytest.CaptureFixture[str]) -> None:
    """Verify `host.py test -k <expr>` filters both `--list-tests` discovery and live `pytest -k` execution."""
    import json

    from host_daemon import cli as host_cli

    # 1. `--list-tests -k` supports pytest-style boolean keyword expressions
    rc_list = host_cli.main(["test", "--list-tests", "--json", "-k", "tof and not calibration"])
    assert rc_list == 0
    listed = json.loads(capsys.readouterr().out)
    listed_names = [entry["name"] for entry in listed]
    assert listed_names == ["test_tof_proximity_array_and_i2c_bus"]

    # 2. `host.py test -k` forwards `-k` to pytest and runs ONLY the matching test
    runner = SimulatedTargetRunner(
        WORKSPACE_ROOT,
        connected_board_type="rp2040_flash_2mb",
        connected_chip="rp2040",
        initial_bin="cat_detector_shell",
    )
    server, port = _start_ephemeral_daemon(runner)
    try:
        rc_run = host_cli.main(
            [
                "test",
                "--project",
                "cat_detector",
                "--host-port",
                str(port),
                "-k",
                "battery_fuel_gauge",
            ]
        )
        assert rc_run == 0
        assert runner.executed_cli_commands == ["battery status"]
    finally:
        server.shutdown()
        server.server_close()


def test_worm_loamy_clitellum_820_concurrent_probe_lock_and_timestamped_cli_test_logs(
    monkeypatch: pytest.MonkeyPatch,
    tmp_path: Path,
) -> None:
    """Verify concurrent metadata probes never abort `sensor status` CLI reads and test execution logs include device-timestamped CLI I/O."""
    import re
    import subprocess as sp

    from host_daemon.provider.daemon.runner import DaemonRunner

    # Part 1: Verify concurrent `query_target_metadata()` cannot kill `rtt_process` mid-`execute_cli("sensor status")`
    runner = DaemonRunner(
        repo_root=WORKSPACE_ROOT,
        package="cat_detector",
        bin_name="cat_detector_shell",
        target="thumbv6m-none-eabi",
        chip="rp2040",
    )
    fake_elf = tmp_path / "cat_detector_shell"
    fake_elf.write_bytes(b"\x7fELF")
    runner.elf_path = fake_elf
    runner.is_flashed = True

    mock_rtt_script = (
        "import os, queue, sys, threading, time\n"
        "q = queue.Queue()\n"
        "def _stdin_reader():\n"
        "    for line in sys.stdin:\n"
        "        q.put(line.strip())\n"
        "    os._exit(0)\n"
        "threading.Thread(target=_stdin_reader, daemon=True).start()\n"
        "print('RTT Session Started', flush=True)\n"
        "print('0.023200 INFO [platform::core_monitor] MPU stack guard configured', flush=True)\n"
        "while True:\n"
        "    cmd = q.get()\n"
        "    if cmd == 'sensor status':\n"
        "        time.sleep(0.25)\n"
        "        print('0.714500 INFO [controller::shell_controller] received command Sensor', flush=True)\n"
        "        print('north                                   east                                    west', flush=True)\n"
        "        print('1098mm                                  INVALID                                 546mm', flush=True)\n"
        "        print('0.714900 INFO [controller::shell_controller] command execution succeeded', flush=True)\n"
        "        print('\\x1b[2Kshell> ', flush=True)\n"
    )
    rtt_proc = sp.Popen(
        [sys.executable, "-u", "-c", mock_rtt_script],
        stdin=sp.PIPE,
        stdout=sp.PIPE,
        stderr=sp.STDOUT,
        text=True,
        bufsize=1,
    )
    runner.rtt_process = rtt_proc
    runner.is_connected = True

    def _pump_stdout() -> None:
        assert rtt_proc.stdout is not None
        for raw in iter(rtt_proc.stdout.readline, ""):
            s = raw.rstrip("\r\n")
            if not s:
                continue
            if "INFO" in s:
                runner.log(s, channel="target")
            elif "\x1b[2K" in s:
                with runner._lock:
                    runner._cli_clear_seq += 1
            else:
                runner.log_cli(s)

    pump_thread = threading.Thread(target=_pump_stdout, daemon=True)
    pump_thread.start()
    time.sleep(0.08)

    monkeypatch.setattr(
        "host_daemon.provider.daemon.runner.probe_attached_target_metadata",
        lambda *_a, **_kw: {
            "connected": True,
            "board_type": "rp2040_flash_2mb",
            "chip": "rp2040",
            "source": "program_metadata",
            "checksum": "0x70D6",
            "elf_checksum": "0x70D6",
            "matches_target": True,
        },
    )
    # Prevent start_rtt_streaming from spawning cargo/host_cli in this unit test
    monkeypatch.setattr(runner, "start_rtt_streaming", lambda: None)

    concurrent_meta_results: List[Dict[str, Any]] = []

    def _concurrent_metadata_probe() -> None:
        time.sleep(0.05)
        concurrent_meta_results.append(runner.query_target_metadata())

    probe_thread = threading.Thread(target=_concurrent_metadata_probe, daemon=True)
    probe_thread.start()
    try:
        sensor_out = runner.execute_cli("sensor status", wait_for_output=True, timeout=2.5)
        probe_thread.join(timeout=3.0)
        assert "Dispatched:" not in sensor_out, f"Concurrent metadata probe aborted sensor status: {sensor_out!r}"
        assert "1098mm" in sensor_out and "546mm" in sensor_out
        assert concurrent_meta_results and concurrent_meta_results[0]["connected"] is True
    finally:
        rtt_proc.terminate()
        rtt_proc.wait(timeout=2.0)

    # Part 2: Verify Test Execution Logs include CLI commands & responses with device timestamps
    sim_runner = SimulatedTargetRunner(
        WORKSPACE_ROOT,
        connected_board_type="rp2040_flash_2mb",
        connected_chip="rp2040",
        initial_bin="cat_detector_shell",
    )
    sim_runner.active_mode = "TEST"
    server, port = _start_ephemeral_daemon(sim_runner)
    base_url = f"http://127.0.0.1:{port}"
    tof_node = "tools/device_tests/suites/test_cat_detector_smoke.py::test_tof_proximity_array_and_i2c_bus"
    try:
        run_res = _http_json_sync(
            f"{base_url}/api/tests/run",
            {"node_ids": [tof_node], "board": "auto", "enable_manual": False},
        )
        assert run_res["success"] is True

        deadline = time.monotonic() + 25.0
        final_st: Dict[str, Any] = {}
        while time.monotonic() < deadline:
            st = _http_json_sync(f"{base_url}/api/tests/status")
            final_st = st
            if not st.get("active") and st.get("step") in ("complete", "failed", "cancelled"):
                break
            time.sleep(0.08)

        assert final_st.get("step") == "complete", f"Unexpected test run state: {final_st}"
        joined_console = "\n".join(final_st.get("console") or [])
        assert re.search(r"\[\d+\.\d+s\]\s+shell>\s+i2c scan", joined_console), joined_console
        assert re.search(r"\[\d+\.\d+s\]\s+.*0x14", joined_console), joined_console
        assert re.search(r"\[\d+\.\d+s\]\s+shell>\s+sensor status", joined_console), joined_console
        assert re.search(r"\[\d+\.\d+s\]\s+.*north:\s*125 mm", joined_console), joined_console
    finally:
        server.shutdown()
        server.server_close()


def test_host_test_stress_runs_and_async_cli_prompt_completion(
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
    tmp_path: Path,
) -> None:
    """Verify `--stress-runs` executes N sequential passes, rejects invalid counts, and guards slow async CLI reads & transient USB metadata probes."""
    import subprocess as sp

    from host_daemon import cli as host_cli
    from host_daemon.model import metadata as host_meta
    from host_daemon.provider.daemon.runner import DaemonRunner

    # 1. Invalid `--stress-runs 0` exits cleanly with error code 2
    rc_bad = host_cli.main(["test", "--stress-runs", "0"])
    assert rc_bad == 2
    assert "--stress-runs must be >= 1" in capsys.readouterr().err

    # 2. Both `host.py test --stress-runs 3` and `host.py --stress-runs 2 test` execute N passes
    sim_runner = SimulatedTargetRunner(
        WORKSPACE_ROOT,
        connected_board_type="rp2040_flash_2mb",
        connected_chip="rp2040",
        initial_bin="cat_detector_shell",
    )
    server, port = _start_ephemeral_daemon(sim_runner)
    try:
        rc_sub = host_cli.main(
            [
                "test",
                "--project",
                "cat_detector",
                "--host-port",
                str(port),
                "-k",
                "battery_fuel_gauge",
                "--stress-runs",
                "3",
            ]
        )
        assert rc_sub == 0
        out_sub = capsys.readouterr().out
        assert "Stress Run [1/3]" in out_sub
        assert "Stress Run [3/3]" in out_sub
        assert "3/3" in out_sub
        assert sim_runner.executed_cli_commands == [
            "battery status",
            "battery status",
            "battery status",
        ]

        sim_runner.executed_cli_commands.clear()
        rc_top = host_cli.main(
            [
                "--stress-runs",
                "2",
                "test",
                "--project",
                "cat_detector",
                "--host-port",
                str(port),
                "-k",
                "battery_fuel_gauge",
            ]
        )
        assert rc_top == 0
        assert sim_runner.executed_cli_commands == ["battery status", "battery status"]
        capsys.readouterr()
    finally:
        server.shutdown()
        server.server_close()

    # 3. Empty `\x1b[2K` (`cli.set_prompt("")` at async command start) must NOT abort a 450ms command at 250ms
    runner = DaemonRunner(
        repo_root=WORKSPACE_ROOT,
        package="cat_detector",
        bin_name="cat_detector_shell",
        target="thumbv6m-none-eabi",
        chip="rp2040",
    )
    fake_cli = tmp_path / "host_cli_mock"
    fake_elf = tmp_path / "cat_detector_shell"
    fake_elf.write_bytes(b"\x7fELF")
    mock_rtt_py = tmp_path / "mock_rtt.py"
    mock_rtt_py.write_text(
        "import os, queue, sys, threading, time\n"
        "q = queue.Queue()\n"
        "def _reader():\n"
        "    for line in sys.stdin:\n"
        "        q.put(line.strip())\n"
        "    os._exit(0)\n"
        "threading.Thread(target=_reader, daemon=True).start()\n"
        "print('RTT Session Started', flush=True)\n"
        "while True:\n"
        "    cmd = q.get()\n"
        "    if cmd == 'thermal status':\n"
        "        print('\\x1b[2K', flush=True)\n"
        "        time.sleep(0.45)\n"
        "        print('Thermal: 24.8 C', flush=True)\n"
        "        print('0.950000 INFO [controller::shell_controller] command execution succeeded', flush=True)\n"
        "        print('\\x1b[2Kshell> ', flush=True)\n",
        encoding="utf-8",
    )
    fake_cli.write_text(
        f"#!/bin/sh\nexec '{sys.executable}' -u '{mock_rtt_py}'\n",
        encoding="utf-8",
    )
    fake_cli.chmod(0o755)
    runner.host_cli_path = fake_cli
    runner.elf_path = fake_elf
    runner.start_rtt_streaming()
    try:
        therm_out = runner.execute_cli("thermal status", wait_for_output=True, timeout=3.0)
        assert "Thermal: 24.8 C" in therm_out, f"Async CLI read aborted prematurely on empty \\x1b[2K: {therm_out!r}"
    finally:
        runner.stop_rtt_streaming()

    # 4. `run_host_fs_query_metadata(..., elf_only=False)` retries transient USB probe timeout errors
    probe_attempts: List[int] = []

    def _flaky_probe_run(cmd: List[str], **_kwargs: Any) -> sp.CompletedProcess[str]:
        probe_attempts.append(len(probe_attempts) + 1)
        if len(probe_attempts) == 1:
            return sp.CompletedProcess(cmd, 1, stdout="", stderr='Error: USB(Custom("bulk write timed out"))')
        return sp.CompletedProcess(
            cmd,
            0,
            stdout='{"board_type":"rp2040_flash_2mb","chip":"rp2040","checksum":"0x70D6","source":"program_metadata"}\n',
            stderr="",
        )

    monkeypatch.setattr(host_meta.subprocess, "run", _flaky_probe_run)
    monkeypatch.setattr(host_meta, "_resolve_host_fs_cmd_prefix", lambda *_a, **_kw: ["mock_host_fs"])
    monkeypatch.setattr(host_meta, "METADATA_PROBE_RETRY_BACKOFF_SEC", 0.01, raising=False)
    rc_meta, out_meta = host_meta.run_host_fs_query_metadata(WORKSPACE_ROOT, fake_elf, elf_only=False)
    assert rc_meta == 0
    assert len(probe_attempts) == 2
    assert '"rp2040_flash_2mb"' in out_meta


def test_symbolic_test_debugging_dap_breakpoint_and_vscode_config(
    capsys: pytest.CaptureFixture[str],
) -> None:
    """Verify `host.py test --debug` hits symbolic breakpoints via DAP and `.vscode/` configs resolve cleanly."""
    import json
    import socket

    from device_tests.client import _find_free_local_port
    from host_daemon import cli as host_cli

    # 1. Invalid `--debug-port 0` fails cleanly with exit status 2
    rc_bad_port = host_cli.main(["test", "--debug", "--debug-port", "0"])
    assert rc_bad_port == 2
    assert "--debug-port must be between" in capsys.readouterr().err

    # 2. Validate `.vscode/{launch,settings,tasks,extensions}.json` integrity & task cross-references
    vscode_dir = WORKSPACE_ROOT / ".vscode"
    launch_cfg = json.loads((vscode_dir / "launch.json").read_text(encoding="utf-8"))
    settings_cfg = json.loads((vscode_dir / "settings.json").read_text(encoding="utf-8"))
    tasks_cfg = json.loads((vscode_dir / "tasks.json").read_text(encoding="utf-8"))
    ext_cfg = json.loads((vscode_dir / "extensions.json").read_text(encoding="utf-8"))

    assert "rust-analyzer.cargo.target" not in settings_cfg, (
        "rust-analyzer.cargo.target must not be locked to an MCU triple so host #[test] CodeLens debugging works"
    )
    pytest_args = settings_cfg.get("python.testing.pytestArgs", [])
    assert "-o" in pytest_args and any(str(a).startswith("addopts=") for a in pytest_args)
    assert "tools/device_tests/suites" in pytest_args

    defined_tasks = {t["label"] for t in tasks_cfg.get("tasks", [])}
    config_names = {c["name"] for c in launch_cfg.get("configurations", [])}
    for cfg in launch_cfg.get("configurations", []):
        pre_task = cfg.get("preLaunchTask")
        if pre_task:
            assert pre_task in defined_tasks, f"Missing preLaunchTask {pre_task!r} in .vscode/tasks.json"
    for comp in launch_cfg.get("compounds", []):
        for ref_name in comp.get("configurations", []):
            assert ref_name in config_names, f"Compound {comp['name']!r} references missing config {ref_name!r}"
    assert "ms-python.debugpy" in ext_cfg.get("recommendations", [])
    assert "vadimcn.vscode-lldb" in ext_cfg.get("recommendations", [])

    # 3. End-to-end symbolic breakpoint test over live Debug Adapter Protocol (`debugpy` + `host.py test --debug`)
    smoke_file = (WORKSPACE_ROOT / "tools" / "device_tests" / "suites" / "test_cat_detector_smoke.py").resolve()
    smoke_lines = smoke_file.read_text(encoding="utf-8").splitlines()
    bp_line = next(idx for idx, line in enumerate(smoke_lines, start=1) if 'await host.cli("battery status")' in line)

    sim_runner = SimulatedTargetRunner(
        WORKSPACE_ROOT,
        connected_board_type="rp2040_flash_2mb",
        connected_chip="rp2040",
        initial_bin="cat_detector_shell",
    )
    server, daemon_port = _start_ephemeral_daemon(sim_runner)
    debug_port = _find_free_local_port("127.0.0.1")
    run_rc: List[int] = []

    def _run_debug_cli() -> None:
        rc = host_cli.main(
            [
                "test",
                "--project",
                "cat_detector",
                "--host-port",
                str(daemon_port),
                "-k",
                "battery_fuel_gauge",
                "--debug",
                "--debug-port",
                str(debug_port),
            ]
        )
        run_rc.append(rc)

    def _send_dap(sock: socket.socket, seq: int, command: str, arguments: Dict[str, Any]) -> None:
        body = json.dumps({"seq": seq, "type": "request", "command": command, "arguments": arguments}).encode("utf-8")
        header = f"Content-Length: {len(body)}\r\n\r\n".encode("ascii")
        sock.sendall(header + body)

    def _recv_dap(sock: socket.socket, buf: bytearray) -> Dict[str, Any]:
        while True:
            sep_idx = buf.find(b"\r\n\r\n")
            if sep_idx != -1:
                hdr_text = bytes(buf[:sep_idx]).decode("ascii")
                content_len = 0
                for hdr_line in hdr_text.split("\r\n"):
                    if hdr_line.lower().startswith("content-length:"):
                        content_len = int(hdr_line.split(":", 1)[1].strip())
                total_needed = sep_idx + 4 + content_len
                if len(buf) >= total_needed:
                    payload_bytes = bytes(buf[sep_idx + 4 : total_needed])
                    del buf[:total_needed]
                    return json.loads(payload_bytes.decode("utf-8"))
            chunk = sock.recv(4096)
            if not chunk:
                raise EOFError("DAP socket closed before complete message")
            buf.extend(chunk)

    cli_thread = threading.Thread(target=_run_debug_cli, daemon=True)
    cli_thread.start()
    try:
        dap_sock: Optional[socket.socket] = None
        deadline = time.monotonic() + 12.0
        while time.monotonic() < deadline:
            cand = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            cand.settimeout(1.0)
            try:
                cand.connect(("127.0.0.1", debug_port))
                dap_sock = cand
                break
            except OSError:
                cand.close()
                time.sleep(0.08)

        assert dap_sock is not None, f"Timed out waiting for debugpy listener on 127.0.0.1:{debug_port}"
        dap_sock.settimeout(15.0)
        rx_buf = bytearray()
        try:
            _send_dap(
                dap_sock,
                1,
                "initialize",
                {
                    "adapterID": "python",
                    "linesStartAt1": True,
                    "columnsStartAt1": True,
                    "pathFormat": "path",
                },
            )
            _send_dap(dap_sock, 2, "attach", {"justMyCode": False})
            initialized_seen = False
            while not initialized_seen:
                msg = _recv_dap(dap_sock, rx_buf)
                if msg.get("type") == "event" and msg.get("event") == "initialized":
                    initialized_seen = True

            _send_dap(
                dap_sock,
                3,
                "setBreakpoints",
                {
                    "source": {"path": str(smoke_file)},
                    "breakpoints": [{"line": bp_line}],
                },
            )
            _send_dap(dap_sock, 4, "configurationDone", {})

            stopped_thread_id: Optional[int] = None
            while stopped_thread_id is None:
                msg = _recv_dap(dap_sock, rx_buf)
                if msg.get("type") == "event" and msg.get("event") == "stopped":
                    if msg.get("body", {}).get("reason") == "breakpoint":
                        stopped_thread_id = int(msg["body"]["threadId"])

            _send_dap(
                dap_sock,
                5,
                "stackTrace",
                {"threadId": stopped_thread_id, "startFrame": 0, "levels": 5},
            )
            top_frame: Optional[Dict[str, Any]] = None
            while top_frame is None:
                msg = _recv_dap(dap_sock, rx_buf)
                if msg.get("type") == "response" and msg.get("command") == "stackTrace":
                    frames = msg.get("body", {}).get("stackFrames", [])
                    assert frames, f"Expected symbolic stack frames at breakpoint, got: {msg}"
                    top_frame = frames[0]

            assert top_frame["name"] == "test_battery_fuel_gauge_status"
            assert int(top_frame["line"]) == bp_line

            _send_dap(dap_sock, 6, "continue", {"threadId": stopped_thread_id})
            _send_dap(dap_sock, 7, "disconnect", {"terminateDebuggee": False})
        finally:
            dap_sock.close()

        cli_thread.join(timeout=15.0)
        assert run_rc == [0], f"Expected debugged test run to exit 0, got {run_rc}"
        assert sim_runner.executed_cli_commands == ["battery status"]
    finally:
        server.shutdown()
        server.server_close()
