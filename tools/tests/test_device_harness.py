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
