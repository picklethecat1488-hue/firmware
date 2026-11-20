"""Multi-transport target interface availability probing (`SWD_USB`, `USB_UART`, `BLE`) for `host.py`.

Replaces fragile stderr substring matching with structured USB/OS interface availability checks:
- `SWD_USB`: Queries `host_fs probe-status --json` (using `probe_rs::probe::list::Lister` +
  non-attaching USB claim `DebugProbeInfo::open()`) or parses structured JSON availability envelopes
  returned by `host_fs query_metadata --json`.
- `USB_UART`: Checks OS device node presence (`/dev/cu.*`, `/dev/ttyACM*`, `COM*`) and performs a
  non-blocking POSIX `flock` lock probe (`O_NONBLOCK | O_NOCTTY`) to distinguish `DISCONNECTED` vs
  `IN_USE` vs `AVAILABLE` without toggling DTR/RTS reset lines.
- `BLE`: Queries a pluggable GATT adapter/peripheral connection lease provider (for `NINA-B312`
  wireless target sessions).
"""

from dataclasses import dataclass, field
from enum import Enum
import json
import os
from pathlib import Path
import subprocess
from typing import Any, Callable, Dict, List, Optional

DEFAULT_PROBE_STATUS_TIMEOUT_SEC: float = 10.0


class TargetInterfaceType(str, Enum):
    """Supported physical and wireless communication transports between `host.py` and a target."""

    SWD_USB = "swd_usb"
    USB_UART = "usb_uart"
    BLE = "ble"


class InterfaceAvailability(str, Enum):
    """Structured availability state of a target communication interface."""

    AVAILABLE = "available"
    IN_USE = "in_use"
    DISCONNECTED = "disconnected"
    TRANSIENT_IO = "transient_io"
    INVALID_METADATA = "invalid_metadata"


@dataclass(frozen=True)
class InterfaceStatus:
    """Structured status snapshot returned by a transport availability check."""

    interface: TargetInterfaceType
    availability: InterfaceAvailability
    devices: List[Dict[str, Any]] = field(default_factory=list)
    error: Optional[str] = None

    @property
    def in_use(self) -> bool:
        """Return True if the interface hardware is present on the host bus but locked by another process."""
        return self.availability == InterfaceAvailability.IN_USE

    @property
    def is_retryable(self) -> bool:
        """Return True if the failure is transient (`IN_USE` or `TRANSIENT_IO`, or required-target mode)."""
        if self.availability in (InterfaceAvailability.IN_USE, InterfaceAvailability.TRANSIENT_IO):
            return True
        if (
            self.availability == InterfaceAvailability.DISCONNECTED
            and os.environ.get("XERXES_REQUIRE_CONNECTED_TARGET") == "1"
        ):
            return True
        return False

    def to_dict(self) -> Dict[str, Any]:
        """Serialize interface status to a JSON-compatible dictionary."""
        return {
            "interface": self.interface.value,
            "availability": self.availability.value,
            "in_use": self.in_use,
            "retryable": self.is_retryable,
            "device_count": len(self.devices),
            "devices": list(self.devices),
            "error": self.error,
        }


def parse_interface_status_output(
    raw_output: str,
    *,
    default_interface: TargetInterfaceType = TargetInterfaceType.SWD_USB,
) -> Optional[InterfaceStatus]:
    """Parse a structured JSON `InterfaceStatusReport` line emitted by `host_fs`."""
    cleaned = raw_output.strip()
    if not cleaned:
        return None
    for line in reversed(cleaned.splitlines()):
        candidate = line.strip()
        if not (candidate.startswith("{") and candidate.endswith("}")):
            continue
        try:
            payload = json.loads(candidate)
        except json.JSONDecodeError:
            continue
        if not isinstance(payload, dict) or "availability" not in payload:
            continue
        raw_iface = str(payload.get("interface") or default_interface.value).strip().lower()
        raw_avail = str(payload.get("availability") or "").strip().lower()
        iface = (
            TargetInterfaceType(raw_iface) if raw_iface in {m.value for m in TargetInterfaceType} else default_interface
        )
        avail = (
            InterfaceAvailability(raw_avail)
            if raw_avail in {m.value for m in InterfaceAvailability}
            else InterfaceAvailability.TRANSIENT_IO
        )
        devices_raw = payload.get("devices")
        devices = list(devices_raw) if isinstance(devices_raw, list) else []
        err_val = payload.get("error")
        return InterfaceStatus(
            interface=iface,
            availability=avail,
            devices=devices,
            error=str(err_val) if err_val is not None else None,
        )
    return None


def query_swd_usb_probe_status(
    repo_root: Path,
    *,
    host_fs_cmd_prefix: List[str],
    probe_selector: Optional[str] = None,
    timeout: float = DEFAULT_PROBE_STATUS_TIMEOUT_SEC,
) -> InterfaceStatus:
    """Issue USB-side `host_fs probe-status --json` (`probe-rs` Lister + USB claim test) without resetting the MCU."""
    cmd = [*host_fs_cmd_prefix, "probe-status", "--json"]
    if probe_selector:
        cmd.extend(["--probe", probe_selector])

    env = dict(os.environ)
    env.pop("CARGO_BUILD_TARGET", None)
    env.pop("CARGO_ENCODED_RUSTFLAGS", None)

    try:
        proc = subprocess.run(
            cmd,
            cwd=str(repo_root),
            env=env,
            capture_output=True,
            text=True,
            timeout=timeout,
            check=False,
        )
    except (OSError, subprocess.SubprocessError) as exc:
        return InterfaceStatus(
            interface=TargetInterfaceType.SWD_USB,
            availability=InterfaceAvailability.TRANSIENT_IO,
            error=str(exc),
        )

    parsed = parse_interface_status_output(f"{proc.stdout}\n{proc.stderr}")
    if parsed is not None:
        return parsed
    return InterfaceStatus(
        interface=TargetInterfaceType.SWD_USB,
        availability=InterfaceAvailability.TRANSIENT_IO,
        error=(proc.stderr or proc.stdout).strip() or f"probe-status exited with {proc.returncode}",
    )


def query_usb_uart_port_status(serial_port: Path) -> InterfaceStatus:
    """Determine whether a USB-UART character device (`FTDI` / `CDC-ACM`) is disconnected, in use, or available."""
    port_path = Path(serial_port)
    if not port_path.exists():
        return InterfaceStatus(
            interface=TargetInterfaceType.USB_UART,
            availability=InterfaceAvailability.DISCONNECTED,
            error=f"Serial port '{port_path}' is not connected",
        )

    device_entry = [{"identifier": port_path.name, "path": str(port_path)}]
    open_flags = os.O_RDWR | getattr(os, "O_NONBLOCK", 0) | getattr(os, "O_NOCTTY", 0)
    try:
        fd = os.open(str(port_path), open_flags)
    except PermissionError as exc:
        return InterfaceStatus(
            interface=TargetInterfaceType.USB_UART,
            availability=InterfaceAvailability.IN_USE,
            devices=device_entry,
            error=f"Serial port '{port_path}' is busy or locked: {exc}",
        )
    except OSError as exc:
        return InterfaceStatus(
            interface=TargetInterfaceType.USB_UART,
            availability=InterfaceAvailability.IN_USE,
            devices=device_entry,
            error=f"Could not claim serial port '{port_path}': {exc}",
        )

    try:
        if os.name != "nt":
            import fcntl

            try:
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except OSError as exc:
                return InterfaceStatus(
                    interface=TargetInterfaceType.USB_UART,
                    availability=InterfaceAvailability.IN_USE,
                    devices=device_entry,
                    error=f"Serial port '{port_path}' holds an active advisory lock: {exc}",
                )
            finally:
                try:
                    fcntl.flock(fd, fcntl.LOCK_UN)
                except OSError:
                    pass
    finally:
        os.close(fd)

    return InterfaceStatus(
        interface=TargetInterfaceType.USB_UART,
        availability=InterfaceAvailability.AVAILABLE,
        devices=device_entry,
    )


def query_ble_interface_status(
    peripheral_address: Optional[str],
    *,
    lease_probe: Optional[Callable[[str], Dict[str, Any]]] = None,
) -> InterfaceStatus:
    """Determine BLE GATT target availability (`NINA-B312`) via an optional adapter lease callback."""
    cleaned_addr = str(peripheral_address or "").strip()
    if not cleaned_addr:
        return InterfaceStatus(
            interface=TargetInterfaceType.BLE,
            availability=InterfaceAvailability.DISCONNECTED,
            error="No BLE peripheral address configured",
        )

    if lease_probe is None:
        return InterfaceStatus(
            interface=TargetInterfaceType.BLE,
            availability=InterfaceAvailability.DISCONNECTED,
            error=f"No BLE adapter session registered for peripheral '{cleaned_addr}'",
        )

    try:
        snap = lease_probe(cleaned_addr)
    except OSError as exc:
        return InterfaceStatus(
            interface=TargetInterfaceType.BLE,
            availability=InterfaceAvailability.TRANSIENT_IO,
            error=str(exc),
        )

    if not snap.get("visible", False):
        return InterfaceStatus(
            interface=TargetInterfaceType.BLE,
            availability=InterfaceAvailability.DISCONNECTED,
            error=f"BLE peripheral '{cleaned_addr}' not advertising",
        )
    device_entry = [{"identifier": str(snap.get("name") or cleaned_addr), "address": cleaned_addr}]
    if snap.get("claimed_by_other", False):
        return InterfaceStatus(
            interface=TargetInterfaceType.BLE,
            availability=InterfaceAvailability.IN_USE,
            devices=device_entry,
            error=f"BLE peripheral '{cleaned_addr}' GATT link is already leased",
        )
    return InterfaceStatus(
        interface=TargetInterfaceType.BLE,
        availability=InterfaceAvailability.AVAILABLE,
        devices=device_entry,
    )


def query_target_interface_status(
    repo_root: Path,
    *,
    interface: TargetInterfaceType = TargetInterfaceType.SWD_USB,
    host_fs_cmd_prefix: Optional[List[str]] = None,
    probe_selector: Optional[str] = None,
    serial_port: Optional[Path] = None,
    ble_address: Optional[str] = None,
    ble_lease_probe: Optional[Callable[[str], Dict[str, Any]]] = None,
) -> InterfaceStatus:
    """Unified multi-transport entry point querying `SWD_USB`, `USB_UART`, or `BLE` availability."""
    if interface == TargetInterfaceType.SWD_USB:
        prefix = host_fs_cmd_prefix or ["cargo", "run", "--quiet", "--package", "host_fs", "--"]
        return query_swd_usb_probe_status(
            repo_root,
            host_fs_cmd_prefix=prefix,
            probe_selector=probe_selector,
        )
    if interface == TargetInterfaceType.USB_UART:
        if serial_port is None:
            return InterfaceStatus(
                interface=TargetInterfaceType.USB_UART,
                availability=InterfaceAvailability.DISCONNECTED,
                error="No USB-UART serial port specified",
            )
        return query_usb_uart_port_status(serial_port)
    return query_ble_interface_status(ble_address, lease_probe=ble_lease_probe)
