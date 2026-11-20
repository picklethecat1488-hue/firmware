//! Strongly typed target communication interface and availability report structures
//! shared across SWD/USB debug probes (`probe-rs`), USB-UART bridges, and BLE links.

/// Physical or wireless communication transport kind connecting the host workstation to a target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetInterfaceKind {
    /// SWD/JTAG USB debug probe enumerated via `probe-rs` (`CMSIS-DAP`, `J-Link`, `ST-Link`).
    SwdUsb,
    /// Dedicated USB-UART bridge (`FTDI`, `CDC-ACM`).
    UsbUart,
    /// Bluetooth Low Energy GATT link (`NINA-B312`).
    Ble,
}

impl TargetInterfaceKind {
    /// Canonical wire identifier string (`"swd_usb"`, `"usb_uart"`, `"ble"`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SwdUsb => "swd_usb",
            Self::UsbUart => "usb_uart",
            Self::Ble => "ble",
        }
    }
}

/// Structured availability state of a target communication interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceAvailability {
    /// Interface hardware is enumerated and can be opened exclusively by the host.
    Available,
    /// Interface hardware is enumerated on the bus, but another process holds the exclusive claim.
    InUse,
    /// No matching interface hardware is attached to the host.
    Disconnected,
    /// Interface opened, but a transient wire/bus I/O timeout occurred while talking to the target.
    TransientIo,
    /// Target responded, but `.program_metadata` was erased or failed CRC-16 validation.
    InvalidMetadata,
}

impl InterfaceAvailability {
    /// Canonical wire identifier string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::InUse => "in_use",
            Self::Disconnected => "disconnected",
            Self::TransientIo => "transient_io",
            Self::InvalidMetadata => "invalid_metadata",
        }
    }

    /// Return `true` if the condition is transient (e.g. another short-lived tool releasing the lock)
    /// and merits bounded retry by `host.py`.
    pub const fn is_retryable(self) -> bool {
        matches!(self, Self::InUse | Self::TransientIo)
    }
}

/// Summary of an enumerated USB debug probe or target endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumeratedInterfaceDevice {
    /// Human-readable product/probe identifier.
    pub identifier: String,
    /// Optional USB `VID:PID` hexadecimal pair (e.g. `"1366:0105"`).
    pub vid_pid: Option<String>,
    /// Optional device serial number or address.
    pub serial_number: Option<String>,
}

/// Structured interface status report serialized by `host_fs probe-status --json` and `query_metadata --json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceStatusReport {
    /// Transport type (`SwdUsb`, `UsbUart`, `Ble`).
    pub interface: TargetInterfaceKind,
    /// Determined availability state (`Available`, `InUse`, `Disconnected`, `TransientIo`, `InvalidMetadata`).
    pub availability: InterfaceAvailability,
    /// Enumerated interface endpoints visible to the OS.
    pub devices: Vec<EnumeratedInterfaceDevice>,
    /// Optional diagnostic detail when `availability != Available`.
    pub error: Option<String>,
}

impl InterfaceStatusReport {
    /// Serialize the status report as a single-line JSON object.
    pub fn to_json(&self) -> String {
        let devices_val: Vec<serde_json::Value> = self
            .devices
            .iter()
            .map(|d| {
                serde_json::json!({
                    "identifier": d.identifier,
                    "vid_pid": d.vid_pid,
                    "serial_number": d.serial_number,
                })
            })
            .collect();
        serde_json::json!({
            "connected": matches!(self.availability, InterfaceAvailability::Available | InterfaceAvailability::InvalidMetadata),
            "interface": self.interface.as_str(),
            "availability": self.availability.as_str(),
            "in_use": self.availability == InterfaceAvailability::InUse,
            "retryable": self.availability.is_retryable(),
            "device_count": self.devices.len(),
            "devices": devices_val,
            "error": self.error,
        })
        .to_string()
    }
}
