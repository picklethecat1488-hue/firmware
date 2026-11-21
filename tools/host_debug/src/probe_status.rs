//! Non-intrusive USB debug probe enumeration and exclusive-claim (`in_use`) status check via `probe-rs`.

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
use probe_rs::probe::list::Lister;
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
use probe_rs::probe::DebugProbeSelector;
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
use tool_common::{
    EnumeratedInterfaceDevice, InterfaceAvailability, InterfaceStatusReport, TargetInterfaceKind,
};

/// Query the OS USB bus via `probe-rs` (`Lister` + non-attaching `DebugProbeInfo::open()`) to determine
/// whether a debug probe is `Disconnected`, `InUse` (claimed by another process), or `Available`.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub fn inspect_swd_usb_probe_status(selector_str: Option<&str>) -> InterfaceStatusReport {
    let lister = Lister::new();
    let probes = match selector_str {
        Some(raw) => match DebugProbeSelector::try_from(raw) {
            Ok(sel) => lister.list(Some(&sel)),
            Err(err) => {
                return InterfaceStatusReport {
                    interface: TargetInterfaceKind::SwdUsb,
                    availability: InterfaceAvailability::Disconnected,
                    devices: Vec::new(),
                    error: Some(format!("Invalid probe selector '{}': {}", raw, err)),
                };
            }
        },
        None => lister.list_all(),
    };

    let devices: Vec<EnumeratedInterfaceDevice> = probes
        .iter()
        .map(|info| EnumeratedInterfaceDevice {
            identifier: info.identifier.clone(),
            vid_pid: Some(format!("{:04x}:{:04x}", info.vendor_id, info.product_id)),
            serial_number: info.serial_number.clone(),
        })
        .collect();

    let Some(first_probe) = probes.first() else {
        return InterfaceStatusReport {
            interface: TargetInterfaceKind::SwdUsb,
            availability: InterfaceAvailability::Disconnected,
            devices,
            error: Some("No debug probes connected".to_string()),
        };
    };

    // Perform a pure USB-side probe interface claim (`open()`) without attaching to SWD or resetting the MCU.
    // If the probe is visible on USB (`!probes.is_empty()`) yet `open()` fails, another process holds the probe!
    match first_probe.open() {
        Ok(probe_handle) => {
            drop(probe_handle);
            InterfaceStatusReport {
                interface: TargetInterfaceKind::SwdUsb,
                availability: InterfaceAvailability::Available,
                devices,
                error: None,
            }
        }
        Err(open_err) => InterfaceStatusReport {
            interface: TargetInterfaceKind::SwdUsb,
            availability: InterfaceAvailability::InUse,
            devices,
            error: Some(format!(
                "USB probe '{}' is currently in use: {:?}",
                first_probe.identifier, open_err
            )),
        },
    }
}

/// Execute the `probe-status` subcommand and print structured availability to stdout.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub fn run_probe_status(probe_selector: Option<&str>, json: bool) {
    let report = inspect_swd_usb_probe_status(probe_selector);
    if json {
        println!("{}", report.to_json());
    } else {
        println!("interface: {}", report.interface.as_str());
        println!("availability: {}", report.availability.as_str());
        println!("device_count: {}", report.devices.len());
        for (idx, dev) in report.devices.iter().enumerate() {
            println!(
                "device[{}]: {} (vid_pid={}, serial={})",
                idx,
                dev.identifier,
                dev.vid_pid.as_deref().unwrap_or("unknown"),
                dev.serial_number.as_deref().unwrap_or("none")
            );
        }
        if let Some(ref err) = report.error {
            println!("detail: {}", err);
        }
    }
}
