"""Behavioral unit and integration tests for the async on-device pytest test harness."""

import asyncio
from collections import deque
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import shutil
import socket
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
TEST_RESOURCES_DIR = Path(__file__).resolve().parent / "resources"
for p in (str(WORKSPACE_ROOT), str(TOOLS_DIR), str(DASHBOARD_DIR)):
    if p not in sys.path:
        sys.path.insert(0, p)


def _copy_resource_tree(resource_rel_dir: str, dest_dir: Path) -> Path:
    """Copy a directory fixture from `tools/tests/resources/<resource_rel_dir>` into `dest_dir`."""
    src_dir = TEST_RESOURCES_DIR / resource_rel_dir
    shutil.copytree(src_dir, dest_dir, dirs_exist_ok=True)
    return dest_dir


def _make_resource_launcher(
    dest_exe: Path,
    resource_rel_path: str,
    *,
    env_overrides: Optional[Dict[str, str]] = None,
    forward_args: bool = True,
) -> Path:
    """Create an executable shell wrapper invoking `tools/tests/resources/<resource_rel_path>`."""
    script_path = (TEST_RESOURCES_DIR / resource_rel_path).resolve()
    if script_path.suffix == ".sh":
        shutil.copy2(script_path, dest_exe)
        dest_exe.chmod(0o755)
        return dest_exe

    lines = ["#!/bin/sh"]
    for key, val in (env_overrides or {}).items():
        lines.append(f"export {key}='{val}'")
    tail = ' "$@"' if forward_args else ""
    lines.append(f"exec '{sys.executable}' -u '{script_path}'{tail}")
    dest_exe.write_text("\n".join(lines) + "\n", encoding="utf-8")
    dest_exe.chmod(0o755)
    return dest_exe


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


SIM_RESET_HANDLER_ADDR: int = 0x100001C0
SIM_CORE1_RESET_HANDLER_ADDR: int = 0x10022C0A
SIM_LOC_BASE_ADDR: int = 0x10004100


class _SimulatedDualCoreGdbServer:
    """In-process dual-core GDB RSP stub for exercising `@device_test` GDB flows without USB hardware."""

    def __init__(self, core0_port: int, core1_port: int, *, halt_on_reset: bool) -> None:
        self._stop_event = threading.Event()
        self._core1_spawned = threading.Event()
        if not halt_on_reset:
            self._core1_spawned.set()
        self._sock0 = self._bind_listener(core0_port)
        self._sock1 = self._bind_listener(core1_port)
        self.core0_port = int(self._sock0.getsockname()[1])
        self.core1_port = int(self._sock1.getsockname()[1])
        self._t0 = threading.Thread(target=self._serve_core, args=(self._sock0, 0, SIM_RESET_HANDLER_ADDR), daemon=True)
        self._t1 = threading.Thread(target=self._serve_core, args=(self._sock1, 1, 0), daemon=True)
        self._t0.start()
        self._t1.start()

    @staticmethod
    def _bind_listener(port: int) -> socket.socket:
        srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        srv.bind(("127.0.0.1", port))
        srv.listen(2)
        srv.settimeout(0.15)
        return srv

    def _serve_core(self, listener: socket.socket, core_idx: int, initial_pc: int) -> None:
        while not self._stop_event.is_set():
            try:
                conn, _ = listener.accept()
            except socket.timeout:
                continue
            except OSError:
                return
            with conn:
                conn.settimeout(0.15)
                no_ack = False
                pc = initial_pc
                breakpoints: set[int] = set()
                rx = bytearray()

                def _send(body: str) -> None:
                    raw = body.encode("ascii")
                    csum = sum(raw) & 0xFF
                    prefix = b"" if no_ack else b"+"
                    conn.sendall(prefix + b"$" + raw + f"#{csum:02x}".encode("ascii"))

                detached = False
                while not self._stop_event.is_set() and not detached:
                    try:
                        chunk = conn.recv(1024)
                    except socket.timeout:
                        continue
                    except OSError:
                        break
                    if not chunk:
                        break
                    rx.extend(chunk)
                    while rx and not detached:
                        while rx[:1] in (b"+", b"-"):
                            del rx[0]
                        if not rx:
                            break
                        if rx[:1] == b"\x03":
                            del rx[0]
                            _send("S02")
                            continue
                        if rx.startswith(b"$"):
                            hash_idx = rx.find(b"#", 1)
                            if hash_idx == -1 or len(rx) < hash_idx + 3:
                                break
                            cmd = bytes(rx[1:hash_idx]).decode("ascii", errors="replace")
                            del rx[: hash_idx + 3]
                            if cmd == "QStartNoAckMode":
                                _send("OK")
                                no_ack = True
                            elif cmd == "?":
                                _send("S05")
                            elif cmd == "p0f":
                                _send(pc.to_bytes(4, "little").hex())
                            elif cmd.startswith("Z1,"):
                                breakpoints.add(int(cmd.split(",")[1], 16))
                                _send("OK")
                            elif cmd.startswith("z1,"):
                                breakpoints.discard(int(cmd.split(",")[1], 16))
                                _send("OK")
                            elif cmd == "D":
                                _send("OK")
                                detached = True
                            elif cmd in ("s", "vCont;s", "vCont;s:1;c", "vCont;c:2;s:1"):
                                pc = (pc + 2) & ~1
                                _send("S05")
                            elif cmd == "c":
                                if core_idx == 1:
                                    self._core1_spawned.wait(timeout=4.0)
                                candidates = sorted(bp for bp in breakpoints if bp != pc)
                                if candidates:
                                    pc = candidates[0]
                                    _send("S05")
                                elif core_idx == 0:
                                    self._core1_spawned.set()
                            else:
                                _send("")
                        else:
                            del rx[0]
            return

    def close(self) -> None:
        """Stop listener threads and close bound sockets."""
        self._stop_event.set()
        self._core1_spawned.set()
        for srv in (self._sock0, self._sock1):
            try:
                srv.close()
            except OSError:
                pass


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
        self.trace_mode = "NONE"
        self._trace_ts_us = 1_000_000
        from host_daemon.provider.daemon.trace_store import TraceSessionStore

        self.trace_store = TraceSessionStore()
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
        self.enable_debug_server = False
        self.halt_on_reset = False
        self.gdb_core0_port = 3333
        self.gdb_core1_port = 3334
        self.rtt_bridge_port = 50000
        self._sim_gdb: Optional[_SimulatedDualCoreGdbServer] = None

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
            "trace_mode": self.trace_mode,
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
            "trace_mode": self.trace_mode,
            "elf_trace_mode": self.trace_mode,
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
        trace_mode: Optional[str] = None,
        enable_debug_server: Optional[bool] = None,
        halt_on_reset: Optional[bool] = None,
        gdb_core0_port: Optional[int] = None,
        gdb_core1_port: Optional[int] = None,
        rtt_bridge_port: Optional[int] = None,
        probe_selector: Optional[str] = None,
    ) -> None:
        """Update target configuration fields prior to synchronization."""
        del show_system_logs, probe_selector
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
        if trace_mode is not None:
            normalized_tm = str(trace_mode).strip().upper()
            if normalized_tm != self.trace_mode:
                self.trace_store.clear()
            self.trace_mode = normalized_tm
        if enable_debug_server is not None:
            self.enable_debug_server = bool(enable_debug_server)
        if halt_on_reset is not None:
            self.halt_on_reset = bool(halt_on_reset)
        if gdb_core0_port is not None:
            self.gdb_core0_port = int(gdb_core0_port)
        if gdb_core1_port is not None:
            self.gdb_core1_port = int(gdb_core1_port)
        if rtt_bridge_port is not None:
            self.rtt_bridge_port = int(rtt_bridge_port)

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
        if self._sim_gdb is not None:
            self._sim_gdb.close()
            self._sim_gdb = None

    def get_debug_status(self) -> Dict[str, Any]:
        """Return simulated `host_debug` status payload."""
        return {
            "enabled": self.enable_debug_server,
            "active": self._sim_gdb is not None,
            "halt_on_reset": self.halt_on_reset,
            "gdb_core0_port": self.gdb_core0_port,
            "gdb_core1_port": self.gdb_core1_port,
            "rtt_bridge_port": self.rtt_bridge_port,
            "probe": None,
            "cores": 2,
            "halted_on_reset": self.halt_on_reset,
            "gdb_core0_connected": False,
            "gdb_core1_connected": False,
        }

    def enable_live_debug_server(
        self,
        *,
        gdb_core0_port: Optional[int] = None,
        gdb_core1_port: Optional[int] = None,
        rtt_bridge_port: Optional[int] = None,
        probe_selector: Optional[str] = None,
        halt_on_reset: Optional[bool] = None,
        reset: bool = False,
        resolve: Optional[List[str]] = None,
    ) -> Dict[str, Any]:
        """Start an in-process `_SimulatedDualCoreGdbServer` and return resolved symbol addresses."""
        del probe_selector, reset
        if self._sim_gdb is not None:
            self._sim_gdb.close()
            self._sim_gdb = None
        self.update_config(
            enable_debug_server=True,
            halt_on_reset=halt_on_reset,
            gdb_core0_port=gdb_core0_port,
            gdb_core1_port=gdb_core1_port,
            rtt_bridge_port=rtt_bridge_port,
        )
        self._sim_gdb = _SimulatedDualCoreGdbServer(
            self.gdb_core0_port,
            self.gdb_core1_port,
            halt_on_reset=self.halt_on_reset,
        )
        self.gdb_core0_port = self._sim_gdb.core0_port
        self.gdb_core1_port = self._sim_gdb.core1_port
        status = self.get_debug_status()
        if resolve:
            resolved_map: Dict[str, int] = {}
            next_loc = SIM_LOC_BASE_ADDR
            for spec in resolve:
                if spec == "Reset_Handler":
                    resolved_map[spec] = SIM_RESET_HANDLER_ADDR
                elif spec == "Core1_Reset_Handler":
                    resolved_map[spec] = SIM_CORE1_RESET_HANDLER_ADDR
                else:
                    resolved_map[spec] = next_loc
                    next_loc += 0x100
            status["resolved"] = resolved_map
        return status

    def disable_live_debug_server(self) -> Dict[str, Any]:
        """Stop any active `_SimulatedDualCoreGdbServer` and restore connected state."""
        if self._sim_gdb is not None:
            self._sim_gdb.close()
            self._sim_gdb = None
        self.enable_debug_server = False
        self.halt_on_reset = False
        self.is_connected = True
        return self.get_debug_status()

    def execute_cli(self, command: str, wait_for_output: bool = False, timeout: float = 5.0) -> str:
        """Simulate realistic RP2040 Cat Detector and MCXN947 Carrier Board CLI responses."""
        del wait_for_output, timeout
        cmd = command.strip()
        self.executed_cli_commands.append(cmd)
        self.command_history.append(cmd)
        self._emit_simulated_trace_for_cli(cmd)

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

    def _emit_simulated_trace_for_cli(self, cmd: str) -> None:
        """Emit realistic simulated `PERF` spans or `SENSOR` reports matching firmware trace features."""
        if self.trace_mode == "PERF":
            if cmd.startswith(("battery", "motor", "fs", "led", "core", "system")):
                self._trace_ts_us += 10_000
                span_id = f"sim_{self._trace_ts_us}"
                span_name = (
                    "max17048::read_voltage_mv"
                    if cmd.startswith("battery")
                    else "ina219::read_current_ma"
                    if cmd.startswith("motor")
                    else f"shell::{cmd.split()[0]}"
                )
                self.trace_store.ingest_event(
                    {
                        "ts_us": self._trace_ts_us,
                        "track": "CPU Core 0",
                        "ph": "B",
                        "category": "span",
                        "name": span_name,
                        "span_id": span_id,
                        "module": "controller",
                        "args": {"core": 0},
                    }
                )
                self._trace_ts_us += 2_400
                self.trace_store.ingest_event(
                    {
                        "ts_us": self._trace_ts_us,
                        "track": "CPU Core 0",
                        "ph": "E",
                        "category": "span",
                        "name": span_name,
                        "span_id": span_id,
                        "module": "controller",
                        "args": {"core": 0},
                    }
                )
                self.trace_store.ingest_event(
                    {
                        "ts_us": self._trace_ts_us,
                        "track": "Core 0 CPU Usage",
                        "ph": "C",
                        "category": "cpu_usage",
                        "name": "Core 0 CPU Usage (%)",
                        "value": 34.5,
                        "unit": "%",
                        "args": {"core": 0},
                    }
                )
        elif self.trace_mode == "SENSOR":
            if cmd == "sensor status":
                for direction, val, valid in [("North", 125.0, True), ("East", 180.0, True), ("West", None, False)]:
                    self._trace_ts_us += 1_500
                    self.trace_store.ingest_event(
                        {
                            "ts_us": self._trace_ts_us,
                            "track": f"Proximity ({direction})",
                            "ph": "C" if valid else "i",
                            "category": "sensor",
                            "name": f"Proximity ({direction})" if valid else f"Proximity ({direction}) Invalid",
                            "value": val,
                            "unit": "mm",
                            "args": {
                                "record_type": "Proximity",
                                "direction": direction,
                                "valid": valid,
                                "value": int(val) if val is not None else None,
                                "csv": [
                                    "Valid" if valid else "Invalid",
                                    str(int(val)) if val is not None else "",
                                    direction,
                                    "",
                                ],
                            },
                        }
                    )
            elif cmd in ("thermal status", "thermal mcu"):
                self._trace_ts_us += 1_500
                temp_mc = 26_500.0 if cmd == "thermal status" else 28_125.0
                self.trace_store.ingest_event(
                    {
                        "ts_us": self._trace_ts_us,
                        "track": "MCU / Board Thermal",
                        "ph": "C",
                        "category": "sensor",
                        "name": "MCU Temperature (mC)",
                        "value": temp_mc,
                        "unit": "mC",
                        "args": {
                            "record_type": "Thermal",
                            "value": int(temp_mc),
                            "overheating": False,
                            "csv": [str(int(temp_mc)), "false", "", ""],
                        },
                    }
                )

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
                    f'"trace_mode":"{self.trace_mode}",'
                    f'"source":"program_metadata"}}'
                )
            return (
                f"checksum: {crc}\nchip: {self.connected_chip}\n"
                f"board_type: {self.connected_board_type}\ntrace_mode: {self.trace_mode}\nsource: program_metadata"
            )

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
        assert "7 passed" in integ_res.stdout
        assert "battery status" not in runner.executed_cli_commands
        assert runner.sync_history == [
            "cat_detector_shell",
            "cat_detector_shell",
            "cat_detector_shell",
            "cat_detector_app",
            "cat_detector_shell",
        ]
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
    shutil.copy2(TEST_RESOURCES_DIR / "suites" / "custom_battery_smoke_suite.py", smoke_script)
    assert infer_scenario_from_script(smoke_script, WORKSPACE_ROOT) == "smoke"

    unknown_script = tmp_path / "plain_helper.py"
    shutil.copy2(TEST_RESOURCES_DIR / "suites" / "plain_helper.py", unknown_script)
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
    import re
    import shutil
    import urllib.request

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
        base_url = f"http://127.0.0.1:{port}"
        with urllib.request.urlopen(f"{base_url}/", timeout=5.0) as resp:
            raw_html = resp.read().decode("utf-8")
        scan_res = _http_json_sync(
            f"{base_url}/api/tests/scan",
            {"project": "cat_detector", "scenario": "all"},
        )
        scanned_tests = list(scan_res.get("tests") or [])
        assert len(scanned_tests) >= 5

        cards_html = "\n".join(
            f'<div class="test-stack-node" style="padding:10px 12px; min-height:54px;">'
            f'<div style="font-weight:bold; color:var(--xerxes-cyan);">{t["name"]} #{idx}</div>'
            f'<div style="color:var(--xerxes-text-muted); font-size:9px;">{t.get("description", "")}</div>'
            f"</div>"
            for idx in range(3)
            for t in scanned_tests
        )
        rendered_html = raw_html.replace(
            'id="logPanel" style="display: none;"',
            'id="logPanel" style="display: flex; flex: 0 0 50%;"',
        ).replace(
            '<div id="testStackList" class="test-stack-list"></div>',
            f'<div id="testStackList" class="test-stack-list">{cards_html}</div>',
        )
        rendered_html = re.sub(r'<script\s+type="module"[^>]*></script>', "", rendered_html)
        rendered_html = rendered_html.replace("<head>", f'<head><base href="{base_url}/">', 1)

        snapshot_html = tmp_path / "test_stack_snapshot.html"
        snapshot_html.write_text(rendered_html, encoding="utf-8")
        shot_path = tmp_path / "test_stack_render.png"
        chrome_profile = tmp_path / "chrome_profile"
        chrome_profile.mkdir(parents=True, exist_ok=True)
        chrome_env = dict(os.environ)
        chrome_env["DBUS_SESSION_BUS_ADDRESS"] = "/dev/null"

        subprocess.run(
            [
                str(chrome_bin),
                "--headless=new",
                "--no-sandbox",
                "--disable-dev-shm-usage",
                "--disable-gpu",
                "--no-first-run",
                "--no-default-browser-check",
                "--disable-extensions",
                "--disable-background-networking",
                "--disable-sync",
                f"--user-data-dir={chrome_profile}",
                "--window-size=1440,900",
                f"--screenshot={shot_path}",
                snapshot_html.resolve().as_uri(),
            ],
            env=chrome_env,
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

    rtt_proc = sp.Popen(
        [sys.executable, "-u", str(TEST_RESOURCES_DIR / "mock_rtt_sensor_cli.py")],
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
    monkeypatch.setattr(runner, "start_rtt_streaming", lambda *_a, **_kw: None)

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
    fake_cli = _make_resource_launcher(
        tmp_path / "host_cli_mock",
        "mock_rtt_thermal_slow.py",
        forward_args=False,
    )
    fake_elf = tmp_path / "cat_detector_shell"
    fake_elf.write_bytes(b"\x7fELF")
    runner.host_cli_path = fake_cli
    runner.elf_path = fake_elf
    runner.start_rtt_streaming()
    try:
        therm_out = runner.execute_cli("thermal status", wait_for_output=True, timeout=3.0)
        assert "Thermal: 24.8 C" in therm_out, f"Async CLI read aborted prematurely on empty \\x1b[2K: {therm_out!r}"
    finally:
        runner.stop_rtt_streaming()

    # 4. `run_host_fs_query_metadata(..., elf_only=False)` retries structured `in_use` / `transient_io` USB status
    probe_attempts: List[int] = []

    def _flaky_probe_run(cmd: List[str], **_kwargs: Any) -> sp.CompletedProcess[str]:
        probe_attempts.append(len(probe_attempts) + 1)
        if len(probe_attempts) == 1:
            return sp.CompletedProcess(
                cmd,
                1,
                stdout='{"connected":false,"interface":"swd_usb","availability":"in_use","in_use":true,"retryable":true,"error":"USB probe in use"}\n',
                stderr="",
            )
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
    assert settings_cfg.get("OcpCadViewer.advanced.watchByDefault") is False, (
        "OcpCadViewer wildcard debug tracker must be disabled by default in firmware to prevent GDB 'from' errors"
    )
    assert settings_cfg.get("OcpCadViewer.advanced.autostart") is False
    pytest_args = settings_cfg.get("python.testing.pytestArgs", [])
    assert "-o" in pytest_args and any(str(a).startswith("addopts=") for a in pytest_args)
    assert "tools/device_tests/suites" in pytest_args

    defined_tasks = {t["label"] for t in tasks_cfg.get("tasks", [])}
    config_names = {c["name"] for c in launch_cfg.get("configurations", [])}
    assert not (vscode_dir / "openocd_custom.cfg").exists()
    assert "Host Daemon: Attach Core 0 (GDB)" in config_names
    assert "Host Daemon: Attach Core 1 (GDB)" in config_names
    assert "probe-rs (Secondary): Debug Shell (RP2040)" in config_names
    for cfg in launch_cfg.get("configurations", []):
        assert cfg.get("servertype") != "openocd", f"Legacy OpenOCD config must be removed: {cfg['name']}"
        if cfg.get("type") == "cortex-debug":
            assert cfg.get("overrideRestartCommands") == [
                "monitor reset halt",
                "maintenance flush register-cache",
            ], f"cortex-debug config {cfg['name']!r} must flush GDB register cache after monitor reset halt"
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


def test_worm_loamy_clitellum_820_post_host_fs_delayed_rtt_boot_and_json_events(
    tmp_path: Path,
) -> None:
    """Verify `start_rtt_streaming()` awaits structured JSON boot/prompt readiness after `host_fs` and never drops `system activity`."""
    import json

    from host_daemon.provider.daemon.runner import DaemonRunner

    runner = DaemonRunner(
        repo_root=WORKSPACE_ROOT,
        package="cat_detector",
        bin_name="cat_detector_shell",
        target="thumbv6m-none-eabi",
        chip="rp2040",
    )
    fake_elf = tmp_path / "cat_detector_shell"
    fake_elf.write_bytes(b"\x7fELF")
    fake_cli = _make_resource_launcher(
        tmp_path / "host_cli_json_mock",
        "mock_rtt_delayed_boot.py",
        forward_args=True,
    )

    runner.host_cli_path = fake_cli
    runner.elf_path = fake_elf
    runner.start_rtt_streaming()
    try:
        assert runner.is_connected is True
        out = runner.execute_cli("system activity", wait_for_output=True, timeout=2.0)
        assert "Dispatched:" not in out, f"Premature CLI dispatch before RTT boot readiness: {out!r}"
        assert "System activity recorded." in out
    finally:
        runner.stop_rtt_streaming()


def test_host_py_bug_cli_online_and_offline_reporting(
    tmp_path: Path,
    capsys: pytest.CaptureFixture[str],
) -> None:
    """Verify `host.py bug` / `report-bug` files worm reports both via live daemon and offline fallback."""
    import json

    from host_daemon import cli as host_cli_mod
    from host_daemon.provider.daemon.server import _get_test_manager

    # 1. Online mode: live ephemeral daemon with command history, RTT logs, test console logs, and file attachment
    online_root = tmp_path / "online_repo"
    (online_root / "feedback").mkdir(parents=True, exist_ok=True)
    (online_root / "attachments").mkdir(parents=True, exist_ok=True)
    screenshot_file = tmp_path / "scope_trace.png"
    screenshot_file.write_bytes(b"\x89PNG\r\n\x1a\nFAKEPNG")

    runner = SimulatedTargetRunner(
        online_root,
        connected_board_type="rp2040_flash_2mb",
        connected_chip="rp2040",
    )
    runner.package = "cat_detector"
    runner.bin_name = "cat_detector_shell"
    runner.command_history = ["motor speed 45", "sensor status"]
    runner.logs.append({"time": "12:00:00", "channel": "rtt", "level": "ERROR", "message": "I2C bus NACK on 0x29"})
    tm = _get_test_manager(runner)
    tm._append_console("test_tof_proximity_array FAILED: NACK")

    server, port = _start_ephemeral_daemon(runner)
    try:
        rc = host_cli_mod.main(
            [
                "--host",
                "127.0.0.1",
                "bug",
                "ToF sensor NACK under motor load",
                "--severity",
                "high",
                "--category",
                "driver",
                "--component",
                "vl53l0x",
                "--description",
                "ToF sensor returns I2C NACK after motor speed 45.",
                "--expected",
                "Valid millimeter distance reading.",
                "--actual",
                "I2C NACK 0x29.",
                "--attach-test-logs",
                "--attach-crash-log",
                "--attach",
                str(screenshot_file),
                "--host-port",
                str(port),
                "--json",
            ]
        )
        assert rc == 0
        online_res = json.loads(capsys.readouterr().out)
        assert online_res["success"] is True
        assert online_res["source"] == "daemon"
        assert online_res["attachments_count"] == 4

        online_md = Path(online_res["file_path"])
        assert online_md.is_file()
        md_text = online_md.read_text(encoding="utf-8")
        assert "ToF sensor NACK under motor load" in md_text
        assert "Execute command 'motor speed 45'" in md_text
        assert "Execute command 'sensor status'" in md_text
        assert "test_tof_proximity_array FAILED" in md_text

        att_dir = online_root / "attachments" / online_res["worm_id"]
        assert (att_dir / "scope_trace.png").read_bytes() == b"\x89PNG\r\n\x1a\nFAKEPNG"
    finally:
        server.shutdown()
        server.server_close()

    # 2. Offline mode (`report-bug` alias) when no daemon is running
    offline_root = tmp_path / "offline_repo"
    (offline_root / "feedback").mkdir(parents=True, exist_ok=True)
    (offline_root / "attachments").mkdir(parents=True, exist_ok=True)
    log_capture = tmp_path / "bench_capture.log"
    log_capture.write_text("UART0 framing error at 115200 baud\n", encoding="utf-8")

    rc_off = host_cli_mod.main(
        [
            "report-bug",
            "--title",
            "UART framing glitch on cold boot",
            "--severity",
            "MEDIUM",
            "--category",
            "PLATFORM",
            "--step",
            "1. Power-cycle bench supply",
            "--step",
            "2. Capture UART0 TX during first 10 ms",
            "--attach",
            str(log_capture),
            "--no-system-logs",
            "--repo-root",
            str(offline_root),
            "--json",
        ]
    )
    assert rc_off == 0
    offline_res = json.loads(capsys.readouterr().out)
    assert offline_res["success"] is True
    assert offline_res["source"] == "offline"
    assert offline_res["attachments_count"] == 1
    offline_md = Path(offline_res["file_path"]).read_text(encoding="utf-8")
    assert "1. 1." not in offline_md
    assert "Power-cycle bench supply" in offline_md

    # 3. Sad cases: missing title -> exit 2, missing attachment file -> exit 1
    assert host_cli_mod.main(["bug", "--repo-root", str(offline_root)]) == 2
    capsys.readouterr()
    assert (
        host_cli_mod.main(
            [
                "bug",
                "Missing file check",
                "--attach",
                str(tmp_path / "does_not_exist.png"),
                "--repo-root",
                str(offline_root),
            ]
        )
        == 1
    )
    capsys.readouterr()


def test_host_py_debug_server_cli_and_single_reset_sync(
    tmp_path: Path,
    capsys: pytest.CaptureFixture[str],
) -> None:
    """Verify `DaemonRunner.sync()` with `enable_debug_server=True` performs a single probe reset and `host.py debug` controls it."""
    import json

    from host_daemon import cli as host_cli_mod
    from host_daemon.provider.daemon.runner import DaemonRunner

    fake_elf = tmp_path / "cat_detector_shell"
    fake_elf.write_bytes(b"\x7fELF")

    # Mock `host_debug`, `host_cli`, and `host_fs` from `tools/tests/resources/`
    dbg_argv_log = tmp_path / "host_debug_argv.json"
    fake_dbg = _make_resource_launcher(
        tmp_path / "host_debug_mock",
        "mock_host_debug.py",
        env_overrides={"XERXES_MOCK_DEBUG_ARGV_LOG": str(dbg_argv_log)},
    )
    cli_argv_log = tmp_path / "host_cli_argv.json"
    fake_cli = _make_resource_launcher(
        tmp_path / "host_cli_mock",
        "mock_host_cli.py",
        env_overrides={"XERXES_MOCK_CLI_ARGV_LOG": str(cli_argv_log)},
    )
    fake_fs = _make_resource_launcher(tmp_path / "host_fs_mock", "mock_host_fs_noop.sh")

    runner = DaemonRunner(
        repo_root=tmp_path,
        package="cat_detector",
        bin_name="cat_detector_shell",
        target="thumbv6m-none-eabi",
        chip="rp2040",
        enable_debug_server=True,
        halt_on_reset=True,
        gdb_core0_port=4333,
        gdb_core1_port=4334,
        rtt_bridge_port=51000,
    )
    runner.elf_path = fake_elf
    runner.host_cli_path = fake_cli
    runner.host_fs_path = fake_fs
    runner.host_debug_path = fake_dbg

    flash_cargo_flags: List[bool] = []
    runner.build_target = lambda: fake_elf  # type: ignore[method-assign]
    runner.build_host_tools = lambda: (fake_cli, fake_fs)  # type: ignore[method-assign]

    def _fake_flash(use_cargo_run: bool = True) -> bool:
        flash_cargo_flags.append(use_cargo_run)
        runner.is_flashed = True
        return True

    runner.flash_target = _fake_flash  # type: ignore[method-assign]

    server, port = _start_ephemeral_daemon(runner)
    try:
        sync_res = runner.sync()
        assert sync_res["step"] == "complete"
        assert flash_cargo_flags == [False], "Sync with debug server must use probe-rs download (zero extra reset)"

        dbg_args = json.loads(dbg_argv_log.read_text(encoding="utf-8"))
        assert "--halt-on-reset" in dbg_args
        assert dbg_args[dbg_args.index("--gdb-core0-port") + 1] == "4333"
        assert dbg_args[dbg_args.index("--gdb-core1-port") + 1] == "4334"

        for _ in range(40):
            if cli_argv_log.is_file() and cli_argv_log.stat().st_size > 0:
                break
            time.sleep(0.05)
        cli_args = json.loads(cli_argv_log.read_text(encoding="utf-8"))
        assert "--gdb-bridge" in cli_args
        assert "127.0.0.1:51000" in cli_args
        assert "--reset" not in cli_args, "host_cli must NOT issue a second reset when bridged to host_debug"

        # Exercise `host.py debug status`, `stop`, and `start` CLI subcommands
        assert host_cli_mod.main(["debug", "status", "--host-port", str(port), "--json"]) == 0
        st_json = json.loads(capsys.readouterr().out)
        assert st_json["active"] is True
        assert st_json["gdb_core0_port"] == 4333
        assert st_json["gdb_core1_port"] == 4334

        assert host_cli_mod.main(["debug", "stop", "--host-port", str(port), "--json"]) == 0
        stop_json = json.loads(capsys.readouterr().out)
        assert stop_json["active"] is False

        assert (
            host_cli_mod.main(
                [
                    "debug",
                    "start",
                    "--gdb-core0-port",
                    "4433",
                    "--gdb-core1-port",
                    "4434",
                    "--no-halt-on-reset",
                    "--host-port",
                    str(port),
                    "--json",
                ]
            )
            == 0
        )
        start_json = json.loads(capsys.readouterr().out)
        assert start_json["active"] is True
        assert start_json["gdb_core0_port"] == 4433
        assert start_json["gdb_core1_port"] == 4434
    finally:
        runner.stop()
        server.shutdown()
        server.server_close()


def test_worm_segmented_chrysalis_575_debug_release_symbols_and_active_target_elf(
    tmp_path: Path,
    capsys: pytest.CaptureFixture[str],
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """Verify debug & release ELF symbol sync, `.program_metadata` profile validation, `host.py debug target-info/sync-symbols`, and `.vscode/launch.json`."""
    import json

    from host_daemon import cli as host_cli_mod
    from host_daemon.model import metadata as host_meta
    from host_daemon.provider.daemon.config import save_last_session
    from host_daemon.provider.daemon.runner import DaemonRunner

    # 1. `validate_metadata_matches_target` must reject `debug` vs `release` (`release_build` bool) or `shell` vs `app` mismatches
    base_meta = {
        "checksum": "0x1234",
        "chip": "rp2040",
        "board_type": "rp2040_flash_2mb",
        "program_metadata_board_type": "rp2040_flash_2mb",
        "bin_name": "cat_detector_shell",
        "release_build": True,
    }
    assert host_meta.validate_metadata_matches_target(base_meta, dict(base_meta)) is True
    debug_meta = {**base_meta, "release_build": False}
    assert host_meta.validate_metadata_matches_target(base_meta, debug_meta) is False
    app_meta = {**base_meta, "bin_name": "cat_detector_app"}
    assert host_meta.validate_metadata_matches_target(base_meta, app_meta) is False

    # 2. Fake workspace from `tools/tests/resources/fake_workspace` with both `debug` and `release` ELFs
    fake_repo = _copy_resource_tree("fake_workspace", tmp_path / "repo")
    target_dir = fake_repo / "target"
    dbg_elf = target_dir / "thumbv6m-none-eabi" / "debug" / "cat_detector_shell"
    rel_elf = target_dir / "thumbv6m-none-eabi" / "release" / "cat_detector_shell"
    cb_rel_elf = target_dir / "thumbv8m.main-none-eabihf" / "release" / "carrier_board_shell"
    for p, tag in ((dbg_elf, b"DBG_ELF"), (rel_elf, b"REL_ELF"), (cb_rel_elf, b"CB_REL_ELF")):
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_bytes(b"\x7fELF" + tag)

    monkeypatch.setattr(host_meta, "get_cargo_target_directory", lambda _r: target_dir)

    runner = DaemonRunner(
        repo_root=fake_repo,
        package="cat_detector",
        bin_name="cat_detector_shell",
        target="thumbv6m-none-eabi",
        chip="rp2040",
        release=False,
    )
    runner.update_config(release=False)
    active_link = target_dir / "active_target.elf"
    assert active_link.is_symlink() or active_link.is_file()
    assert active_link.resolve() == dbg_elf.resolve()
    dbg_status_0 = runner.get_debug_status()
    assert dbg_status_0["profile"] == "debug"
    assert dbg_status_0["device"] == "RP2040"
    assert Path(dbg_status_0["executable"]).resolve() == dbg_elf.resolve()

    # Switch to `release` mode (exact scenario of WORM-SEGMENTED-CHRYSALIS-575!)
    runner.update_config(release=True)
    assert active_link.resolve() == rel_elf.resolve()
    dbg_status_rel = runner.get_debug_status()
    assert dbg_status_rel["profile"] == "release"
    assert dbg_status_rel["device"] == "RP2040"
    assert Path(dbg_status_rel["executable"]).resolve() == rel_elf.resolve()

    # Switch to `carrier_board_shell` (`MCXN947` in `release` mode) via live daemon + `host.py debug` CLI
    server, port = _start_ephemeral_daemon(runner)
    try:
        runner.update_config(
            package="cat_detector",
            bin_name="carrier_board_shell",
            target="thumbv8m.main-none-eabihf",
            chip="MCXN947",
            release=True,
        )
        assert (
            host_cli_mod.main(
                [
                    "debug",
                    "target-info",
                    "--host-port",
                    str(port),
                    "--json",
                ]
            )
            == 0
        )
        ti = json.loads(capsys.readouterr().out)
        assert ti["device"] == "MCXN947"
        assert ti["chip"] == "MCXN947"
        assert ti["profile"] == "release"
        assert ti["bin"] == "carrier_board_shell"
        assert Path(ti["executable"]).resolve() == cb_rel_elf.resolve()

        # Break the symlink and confirm `host.py debug sync-symbols` restores it from `host.py`!
        active_link.unlink()
        assert (
            host_cli_mod.main(
                [
                    "debug",
                    "sync-symbols",
                    "--host-port",
                    str(port),
                    "--json",
                ]
            )
            == 0
        )
        sync_out = json.loads(capsys.readouterr().out)
        assert Path(sync_out["active_elf_link"]).resolve() == cb_rel_elf.resolve()
        assert active_link.resolve() == cb_rel_elf.resolve()
    finally:
        runner.stop()
        server.shutdown()
        server.server_close()

    # Offline fallback when `host.py` is stopped: `host.py debug sync-symbols --repo-root <fake_repo>` reads `host_session.json`
    save_last_session(
        fake_repo,
        {
            "package": "cat_detector",
            "bin": "cat_detector_shell",
            "release": True,
            "target": "thumbv6m-none-eabi",
            "chip": "rp2040",
        },
    )
    active_link.unlink()
    assert (
        host_cli_mod.main(
            [
                "debug",
                "sync-symbols",
                "--repo-root",
                str(fake_repo),
                "--json",
            ]
        )
        == 0
    )
    off_sync = json.loads(capsys.readouterr().out)
    assert off_sync["profile"] == "release"
    assert off_sync["device"] == "RP2040"
    assert active_link.resolve() == rel_elf.resolve()

    # 3. Verify `.vscode/launch.json` & `.vscode/tasks.json` never hardcode `debug/<bin>` on Host Daemon GDB configs
    vscode_dir = WORKSPACE_ROOT / ".vscode"
    launch_cfg = json.loads((vscode_dir / "launch.json").read_text(encoding="utf-8"))
    tasks_cfg = json.loads((vscode_dir / "tasks.json").read_text(encoding="utf-8"))
    cortex_cfgs = [c for c in launch_cfg.get("configurations", []) if c.get("type") == "cortex-debug"]
    assert len(cortex_cfgs) >= 2
    for c in cortex_cfgs:
        assert c.get("executable") == "${workspaceFolder}/target/active_target.elf", (
            f"cortex-debug config {c['name']!r} must read target/active_target.elf synced by host.py"
        )
        assert c.get("preLaunchTask") == "host: sync active target symbols"
    for c in launch_cfg.get("configurations", []):
        raw_c = json.dumps(c)
        if "carrier_board_shell" in raw_c:
            assert "thumbv8m.main-none-eabihf" in raw_c and "thumbv6m-none-eabi" not in raw_c
    for t in tasks_cfg.get("tasks", []):
        raw_t = json.dumps(t)
        if "carrier_board_shell" in raw_t:
            assert "thumbv8m.main-none-eabihf" in raw_t and "thumbv6m-none-eabi" not in raw_t


def test_host_py_test_explicit_debug_server_and_no_debug_server_flags(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """Verify `host.py test --[no-]debug-server` flags and multi-transport `SWD_USB` / `USB_UART` / `BLE` availability checks."""
    from device_tests.model import DetectedBoard
    from host_daemon import cli as host_cli_mod

    parsed_on = host_cli_mod.parse_arguments(["test", "--debug-server", "--stress-runs", "10"])
    assert parsed_on.test_debug_server is True
    assert parsed_on.test_stress_runs == 10

    parsed_off = host_cli_mod.parse_arguments(["test", "--no-debug-server", "--stress-runs", "10"])
    assert parsed_off.test_debug_server is False
    assert parsed_off.test_stress_runs == 10

    captured_cmds: List[List[str]] = []

    class _FakeCompleted:
        returncode = 0

    monkeypatch.setattr(
        host_cli_mod,
        "send_daemon_request",
        lambda *_a, **_kw: (_ for _ in ()).throw(RuntimeError("offline")),
    )
    import device_tests.client as client_mod

    monkeypatch.setattr(
        client_mod,
        "query_metadata_standalone",
        lambda _root: DetectedBoard(board_type="rp2040_flash_2mb", chip="rp2040"),
    )
    monkeypatch.setattr(
        host_cli_mod.subprocess,
        "run",
        lambda cmd, **_kw: (captured_cmds.append(list(cmd)), _FakeCompleted())[1],
    )

    assert host_cli_mod.main(["test", "--debug-server"]) == 0
    assert "--debug-server" in captured_cmds[-1]
    assert "--no-debug-server" not in captured_cmds[-1]

    assert host_cli_mod.main(["test", "--no-debug-server"]) == 0
    assert "--no-debug-server" in captured_cmds[-1]
    assert "--debug-server" not in captured_cmds[-1]

    from host_daemon.model.transport import (
        InterfaceAvailability,
        TargetInterfaceType,
        parse_interface_status_output,
        query_ble_interface_status,
        query_usb_uart_port_status,
    )
    from host_daemon.provider.daemon.config import has_explicit_target_args

    assert has_explicit_target_args(["--no-detach", "--no-browser", "--no-debug-server"]) is False

    # Structured `SWD_USB` JSON status parsing (`in_use` vs `disconnected` + `XERXES_REQUIRE_CONNECTED_TARGET`)
    swd_busy = parse_interface_status_output(
        '{"interface":"swd_usb","availability":"in_use","in_use":true,"device_count":1,"devices":[{"identifier":"J-Link"}]}'
    )
    assert swd_busy is not None
    assert swd_busy.interface == TargetInterfaceType.SWD_USB
    assert swd_busy.availability == InterfaceAvailability.IN_USE
    assert swd_busy.in_use is True
    assert swd_busy.is_retryable is True

    swd_disc = parse_interface_status_output(
        '{"interface":"swd_usb","availability":"disconnected","in_use":false,"device_count":0,"devices":[]}'
    )
    assert swd_disc is not None
    assert swd_disc.availability == InterfaceAvailability.DISCONNECTED
    monkeypatch.delenv("XERXES_REQUIRE_CONNECTED_TARGET", raising=False)
    assert swd_disc.is_retryable is False
    monkeypatch.setenv("XERXES_REQUIRE_CONNECTED_TARGET", "1")
    assert swd_disc.is_retryable is True

    # `USB_UART` non-blocking POSIX lock check (`DISCONNECTED`, `AVAILABLE`, `IN_USE`)
    missing_uart = query_usb_uart_port_status(tmp_path / "cu.usbserial-missing")
    assert missing_uart.availability == InterfaceAvailability.DISCONNECTED
    fake_uart = tmp_path / "cu.usbserial-mock"
    fake_uart.write_bytes(b"")
    avail_uart = query_usb_uart_port_status(fake_uart)
    assert avail_uart.availability == InterfaceAvailability.AVAILABLE
    if os.name != "nt":
        import fcntl

        with fake_uart.open("r+b") as locked_fh:
            fcntl.flock(locked_fh.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
            busy_uart = query_usb_uart_port_status(fake_uart)
            assert busy_uart.availability == InterfaceAvailability.IN_USE
            assert busy_uart.in_use is True

    # `BLE` GATT lease status (`DISCONNECTED`, `AVAILABLE`, `IN_USE`)
    assert query_ble_interface_status(None).availability == InterfaceAvailability.DISCONNECTED
    assert (
        query_ble_interface_status(
            "AA:BB:CC:11:22:33",
            lease_probe=lambda _a: {"visible": True, "claimed_by_other": False, "name": "Xerxes-NINA"},
        ).availability
        == InterfaceAvailability.AVAILABLE
    )
    ble_busy = query_ble_interface_status(
        "AA:BB:CC:11:22:33",
        lease_probe=lambda _a: {"visible": True, "claimed_by_other": True, "name": "Xerxes-NINA"},
    )
    assert ble_busy.availability == InterfaceAvailability.IN_USE
    assert ble_busy.in_use is True


def test_collect_cli_output_ignores_pre_completion_shell_idle_in_json_mode(tmp_path: Path) -> None:
    """Verify early/reconnect `shell_idle` events before `command_completed` never truncate CLI output."""
    from host_daemon.provider.daemon.runner import DaemonRunner

    runner = DaemonRunner(
        repo_root=tmp_path,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
        enable_debug_server=False,
    )
    fake_elf = tmp_path / "cat_detector_shell"
    fake_elf.write_bytes(b"\x7fELF")
    fake_cli = _make_resource_launcher(
        tmp_path / "host_cli",
        "mock_rtt_reconnect_idle.py",
        forward_args=False,
    )
    runner.host_cli_path = fake_cli
    runner.elf_path = fake_elf
    runner.start_rtt_streaming()
    try:
        out = runner.execute_cli("battery status", wait_for_output=True, timeout=3.0)
        assert "Battery Status:" in out
        assert "Voltage: 4022 mV" in out
    finally:
        runner.stop_rtt_streaming()


def test_trace_mode_cargo_features_and_external_viewer_mapping(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    """Verify `TraceMode` passes exact Cargo `--features` flags and maps `PERF`->Perfetto vs `SENSOR`/`AUDIO`->Rerun."""
    from host_daemon.model import metadata as meta_mod
    from host_daemon.model.trace_mode import (
        ExternalTraceViewer,
        TraceMode,
        get_cargo_feature_args,
        get_external_trace_viewer,
        normalize_trace_mode,
    )
    from host_daemon.provider.daemon.runner import DaemonRunner

    assert normalize_trace_mode(None) == TraceMode.NONE
    assert normalize_trace_mode("sensor") == TraceMode.SENSOR
    assert normalize_trace_mode("PERF") == TraceMode.PERF
    assert normalize_trace_mode("audio") == TraceMode.AUDIO
    with pytest.raises(ValueError):
        normalize_trace_mode("bogus")

    assert get_cargo_feature_args(TraceMode.NONE) == []
    assert get_cargo_feature_args(TraceMode.SENSOR) == ["--features", "trace-sensors"]
    assert get_cargo_feature_args(TraceMode.PERF) == ["--features", "trace-perf"]
    assert get_cargo_feature_args(TraceMode.AUDIO) == ["--features", "trace-audio"]

    assert get_external_trace_viewer(TraceMode.NONE) == ExternalTraceViewer.NONE
    assert get_external_trace_viewer(TraceMode.PERF) == ExternalTraceViewer.PERFETTO
    assert get_external_trace_viewer(TraceMode.SENSOR) == ExternalTraceViewer.RERUN
    assert get_external_trace_viewer(TraceMode.AUDIO) == ExternalTraceViewer.RERUN

    fake_target_dir = tmp_path / "target"
    elf_dir = fake_target_dir / "thumbv6m-none-eabi" / "release"
    elf_dir.mkdir(parents=True)
    fake_elf = elf_dir / "cat_detector_app"
    fake_elf.write_bytes(b"\x7fELF")
    monkeypatch.setattr(meta_mod, "get_cargo_target_directory", lambda _root: fake_target_dir)
    monkeypatch.setattr("host_daemon.provider.daemon.runner.get_cargo_target_directory", lambda _root: fake_target_dir)

    captured_cmds: List[List[str]] = []
    runner = DaemonRunner(
        repo_root=tmp_path,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_app",
        release=True,
        trace_mode="PERF",
    )
    monkeypatch.setattr(
        runner,
        "_run_command_streaming",
        lambda cmd, **_kw: (captured_cmds.append(list(cmd)) or 0, "ok"),
    )

    runner.build_target()
    assert captured_cmds[-1] == [
        "cargo",
        "build",
        "--target",
        "thumbv6m-none-eabi",
        "-p",
        "cat_detector",
        "--bin",
        "cat_detector_app",
        "--release",
        "--features",
        "trace-perf",
    ]

    runner.update_config(trace_mode="SENSOR")
    assert runner.get_status()["external_viewer"] == "rerun"
    runner.build_target()
    assert captured_cmds[-1][-2:] == ["--features", "trace-sensors"]

    runner.update_config(trace_mode="NONE")
    assert runner.get_status()["external_viewer"] == "none"
    runner.build_target()
    assert "--features" not in captured_cmds[-1]


def test_trace_session_store_rerun_bridge_and_http_cli_endpoints(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """Verify `TraceSessionStore`, `open_trace_in_rerun`, `/api/trace/*` HTTP routes, and `host.py trace` CLI."""
    from host_daemon import cli as host_cli_mod
    from host_daemon.provider.daemon import rerun_bridge
    from host_daemon.provider.daemon.runner import DaemonRunner
    from host_daemon.provider.daemon.server import HostDaemonServer

    class MockRerunRecorder:
        def __init__(self) -> None:
            self.inits: List[tuple[str, Optional[str]]] = []
            self.spawned: int = 0
            self.saved_paths: List[str] = []
            self.times: List[tuple[str, float]] = []
            self.logs: List[tuple[str, tuple[str, Any]]] = []

        def init(self, app_id: str, recording_id: Optional[str] = None) -> None:
            self.inits.append((app_id, recording_id))

        def spawn(self) -> None:
            self.spawned += 1

        def save(self, path: str) -> None:
            self.saved_paths.append(path)

        def set_time(self, timeline: str, *, duration: float) -> None:
            self.times.append((timeline, duration))

        def Scalars(self, val: float) -> tuple[str, float]:  # noqa: N802
            return ("Scalars", float(val))

        def TextDocument(self, text: str) -> tuple[str, str]:  # noqa: N802
            return ("TextDocument", str(text))

        def log(self, entity: str, payload: tuple[str, Any]) -> None:
            self.logs.append((entity, payload))

    runner = DaemonRunner(
        repo_root=tmp_path,
        target="thumbv6m-none-eabi",
        package="cat_detector",
        bin_name="cat_detector_shell",
        chip="rp2040",
        trace_mode="SENSOR",
    )
    # Ingest nested spans + sensor reports
    runner.trace_store.ingest_event(
        {
            "ts_us": 1_000_000,
            "track": "CPU Core 1",
            "ph": "B",
            "category": "span",
            "name": "motor_task",
            "span_id": "s1",
            "module": "controller::motor_controller",
            "args": {"core": 1},
        }
    )
    runner.trace_store.ingest_event(
        {
            "ts_us": 1_005_000,
            "track": "CPU Core 1",
            "ph": "B",
            "category": "span",
            "name": "motor_controller::update",
            "span_id": "s2",
            "parent_id": "s1",
            "module": "controller::motor_controller",
            "args": {"core": 1},
        }
    )
    runner.trace_store.ingest_event(
        {
            "ts_us": 1_041_000,
            "track": "CPU Core 1",
            "ph": "E",
            "category": "span",
            "name": "motor_controller::update",
            "span_id": "s2",
        }
    )
    runner.trace_store.ingest_event(
        {
            "ts_us": 1_050_000,
            "track": "CPU Core 1",
            "ph": "E",
            "category": "span",
            "name": "motor_task",
            "span_id": "s1",
        }
    )
    runner.trace_store.ingest_event(
        {
            "ts_us": 5_204_110,
            "track": "Proximity (North)",
            "ph": "C",
            "category": "sensor",
            "name": "Proximity (North)",
            "value": 118.0,
            "unit": "mm",
            "args": {
                "record_type": "Proximity",
                "direction": "North",
                "valid": True,
                "value": 118,
                "csv": ["Valid", "118", "North", ""],
            },
        }
    )
    runner.trace_store.ingest_event(
        {
            "ts_us": 5_250_000,
            "track": "MCU / Board Thermal",
            "ph": "C",
            "category": "sensor",
            "name": "MCU Temperature (mC)",
            "value": 25_000.0,
            "unit": "mC",
            "args": {
                "record_type": "Thermal",
                "value": 25_000,
                "overheating": False,
                "csv": ["25000", "false", "", ""],
            },
        }
    )
    runner.trace_store.ingest_event(
        {
            "ts_us": 5_300_000,
            "track": "Gesture & Touch",
            "ph": "i",
            "category": "sensor",
            "name": "Gesture::DualLongPress",
            "args": {
                "record_type": "Gesture",
                "gesture": "DualLongPress",
                "csv": ["DualLongPress", "", "", ""],
            },
        }
    )

    mock_rr = MockRerunRecorder()
    rr_res = rerun_bridge.open_trace_in_rerun(
        runner.trace_store.snapshot_all(),
        app_id="test_sensor_trace",
        spawn=True,
        rr_module=mock_rr,
    )
    assert rr_res["success"] is True
    assert rr_res["viewer"] == "rerun"
    assert mock_rr.spawned == 1
    logged_map = dict(mock_rr.logs)
    assert logged_map["proximity/north/distance"] == ("Scalars", 118.0)
    assert logged_map["proximity/north/state"] == ("TextDocument", "Valid")
    assert logged_map["thermal/temperature"] == ("Scalars", 25.0)
    assert logged_map["gesture"] == ("TextDocument", "DualLongPress")

    monkeypatch.setattr(
        "host_daemon.provider.daemon.runner.open_trace_in_rerun",
        lambda events, **kw: rerun_bridge.open_trace_in_rerun(events, rr_module=mock_rr, **kw),
    )

    server = HostDaemonServer("127.0.0.1", 0, runner)
    port = int(server.server_address[1])
    srv_thread = threading.Thread(target=server.serve_forever, daemon=True)
    srv_thread.start()
    try:
        # 1. Switch mode to TRACE
        mode_res = host_cli_mod.send_daemon_request("/api/mode", {"mode": "TRACE"}, port=port, host="127.0.0.1")
        assert mode_res == {"success": True, "mode": "TRACE"}
        assert runner.active_mode == "TRACE"

        # 2. Poll incremental events and verify span durations & depths
        ev_res = host_cli_mod.send_daemon_request("/api/trace/events?after_seq=0", None, port=port, host="127.0.0.1")
        assert ev_res["trace_mode"] == "SENSOR"
        assert ev_res["external_viewer"] == "rerun"
        assert ev_res["total_events"] == 5
        by_name = {e["name"]: e for e in ev_res["events"]}
        assert by_name["motor_task"]["dur_us"] == 50_000
        assert by_name["motor_task"]["depth"] == 0
        assert by_name["motor_controller::update"]["dur_us"] == 36_000
        assert by_name["motor_controller::update"]["depth"] == 1

        # 3. Export CSV & Perfetto JSON via CLI subcommand
        csv_dest = tmp_path / "exports" / "session_out.csv"
        assert (
            host_cli_mod.main(
                [
                    "--host",
                    "127.0.0.1",
                    "trace",
                    "export",
                    "--format",
                    "csv",
                    "--output",
                    str(csv_dest),
                    "--host-port",
                    str(port),
                ]
            )
            == 0
        )
        assert csv_dest.is_file()
        csv_text = csv_dest.read_text(encoding="utf-8")
        assert "5204110,Proximity (North),C,sensor,Proximity (North),,118,mm,Proximity,Valid,118,North," in csv_text

        json_dest = tmp_path / "exports" / "session_out.perfetto.json"
        assert (
            host_cli_mod.main(
                [
                    "--host",
                    "127.0.0.1",
                    "trace",
                    "export",
                    "--format",
                    "perfetto",
                    "--output",
                    str(json_dest),
                    "--host-port",
                    str(port),
                ]
            )
            == 0
        )
        perfetto_doc = json.loads(json_dest.read_text(encoding="utf-8"))
        assert perfetto_doc["displayTimeUnit"] == "ms"
        assert any(
            e.get("name") == "motor_controller::update" and e.get("dur") == 36_000 for e in perfetto_doc["traceEvents"]
        )

        # 4. Trigger Open in Rerun via CLI subcommand
        assert host_cli_mod.main(["--host", "127.0.0.1", "trace", "open-rerun", "--host-port", str(port)]) == 0
        assert mock_rr.spawned == 2

        # 5. Pause -> ingest ignored -> resume -> clear
        assert host_cli_mod.main(["--host", "127.0.0.1", "trace", "pause", "--host-port", str(port)]) == 0
        assert (
            runner.trace_store.ingest_event(
                {"ts_us": 6_000_000, "track": "Proximity (North)", "ph": "C", "value": 99.0}
            )
            is None
        )
        assert host_cli_mod.main(["--host", "127.0.0.1", "trace", "resume", "--host-port", str(port)]) == 0
        assert (
            runner.trace_store.ingest_event(
                {"ts_us": 6_000_000, "track": "Proximity (North)", "ph": "C", "value": 99.0}
            )
            is not None
        )
        assert host_cli_mod.main(["--host", "127.0.0.1", "trace", "clear", "--host-port", str(port)]) == 0
        assert len(runner.trace_store.snapshot_all()) == 0
    finally:
        server.shutdown()
        server.server_close()


def test_worm_moist_bristle_687_stdlib_sync_symbols_ephemeral_port_isolation_and_gdb_proxy(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """Verify `host.py debug sync-symbols` runs without `pytest`, ephemeral debug ports never pollute session defaults, and `gdb-proxy` tracks Target Selection ports."""
    import socket

    from host_daemon.model import metadata as host_meta
    from host_daemon.provider.daemon.config import load_last_session, save_last_session
    from host_daemon.provider.daemon.runner import DaemonRunner

    # 1. `tools/host.py debug sync-symbols` must succeed in a Python environment WITHOUT `pytest` installed!
    fake_repo = _copy_resource_tree("fake_workspace", tmp_path / "stdlib_repo")
    rel_elf = fake_repo / "target" / "thumbv6m-none-eabi" / "release" / "cat_detector_shell"
    rel_elf.parent.mkdir(parents=True, exist_ok=True)
    rel_elf.write_bytes(b"\x7fELF_REL")
    save_last_session(
        fake_repo,
        {
            "package": "cat_detector",
            "bin": "cat_detector_shell",
            "release": True,
            "target": "thumbv6m-none-eabi",
            "chip": "rp2040",
            "gdb_core0_port": 3333,
            "gdb_core1_port": 3334,
        },
    )

    no_pytest_code = (
        "import runpy, sys\n"
        "sys.modules['pytest'] = None\n"
        f"sys.argv = [{str(WORKSPACE_ROOT / 'tools' / 'host.py')!r}, 'debug', 'sync-symbols', '--repo-root', {str(fake_repo)!r}, '--json']\n"
        f"runpy.run_path({str(WORKSPACE_ROOT / 'tools' / 'host.py')!r}, run_name='__main__')\n"
    )
    proc_sync = subprocess.run(
        [sys.executable, "-c", no_pytest_code],
        cwd=str(WORKSPACE_ROOT),
        capture_output=True,
        text=True,
        timeout=15.0,
        check=False,
    )
    assert proc_sync.returncode == 0, f"sync-symbols failed without pytest:\n{proc_sync.stdout}\n{proc_sync.stderr}"
    assert "ModuleNotFoundError" not in proc_sync.stderr

    # 2. Ephemeral `enable_live_debug_server` / `disable_live_debug_server` ports must NOT clobber Target Selection ports (3333/3334)!
    monkeypatch.setattr(host_meta, "get_cargo_target_directory", lambda _r: fake_repo / "target")
    runner = DaemonRunner(
        repo_root=fake_repo,
        package="cat_detector",
        bin_name="cat_detector_shell",
        target="thumbv6m-none-eabi",
        chip="rp2040",
        release=True,
        gdb_core0_port=3333,
        gdb_core1_port=3334,
    )
    monkeypatch.setattr(runner, "start_debug_server", lambda **_kw: runner.get_debug_status())
    monkeypatch.setattr(runner, "stop_debug_server", lambda: None)
    monkeypatch.setattr(runner, "start_rtt_streaming", lambda **_kw: None)
    monkeypatch.setattr(runner, "stop_rtt_streaming", lambda: None)

    runner.enable_live_debug_server(gdb_core0_port=56947, gdb_core1_port=56948, rtt_bridge_port=56949)
    assert runner.gdb_core0_port == 56947
    runner.disable_live_debug_server()
    assert runner.gdb_core0_port == 3333
    assert runner.gdb_core1_port == 3334
    persisted = load_last_session(fake_repo)
    assert persisted is not None
    assert persisted["gdb_core0_port"] == 3333
    assert persisted["gdb_core1_port"] == 3334

    # 3. `.vscode/launch.json` must use `gdb-proxy` (zero stale 3333/3334 input prompts) and bridge dynamically to Target Selection ports!
    launch_text = (WORKSPACE_ROOT / ".vscode" / "launch.json").read_text(encoding="utf-8")
    assert "gdbCore0Port" not in launch_text
    assert "gdbCore1Port" not in launch_text
    assert "gdb-proxy --core 0" in launch_text
    assert "gdb-proxy --core 1" in launch_text

    rsp_listener = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    rsp_listener.bind(("127.0.0.1", 0))
    rsp_listener.listen(1)
    custom_c0_port = int(rsp_listener.getsockname()[1])

    def _echo_rsp_once() -> None:
        conn, _ = rsp_listener.accept()
        try:
            rx = conn.recv(64)
            if rx:
                conn.sendall(b"+$OK#9a")
        finally:
            conn.close()
            rsp_listener.close()

    threading.Thread(target=_echo_rsp_once, daemon=True).start()
    runner.update_config(enable_debug_server=True, gdb_core0_port=custom_c0_port, gdb_core1_port=custom_c0_port + 1)
    runner.debug_server_active = True
    server, daemon_port = _start_ephemeral_daemon(runner)
    try:
        proxy_proc = subprocess.run(
            [
                sys.executable,
                "-u",
                str(WORKSPACE_ROOT / "tools" / "host.py"),
                "debug",
                "gdb-proxy",
                "--core",
                "0",
                "--host-port",
                str(daemon_port),
            ],
            input=b"+$?#3f",
            capture_output=True,
            timeout=10.0,
            check=False,
        )
        assert proxy_proc.returncode == 0, proxy_proc.stderr.decode(errors="ignore")
        assert b"+$OK#9a" in proxy_proc.stdout
    finally:
        server.shutdown()
        server.server_close()
