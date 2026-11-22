//! Architecture-independent debug probe enumeration and session-attach retry frontend.

use std::thread;
use std::time::Duration;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
use probe_rs::probe::list::Lister;
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
use probe_rs::probe::DebugProbeSelector;
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
use probe_rs::{Permissions, Session};

/// Default maximum attempts when claiming and attaching a USB debug probe session.
pub const PROBE_ATTACH_MAX_ATTEMPTS: usize = 4;

/// Default delay in milliseconds between consecutive USB debug probe attach attempts.
pub const PROBE_ATTACH_RETRY_DELAY_MS: u64 = 250;

/// Execute `op` up to `max_attempts` times (minimum `1`), sleeping `retry_delay` between transient failures.
pub fn retry_probe_attach<T, F>(
    max_attempts: usize,
    retry_delay: Duration,
    mut op: F,
) -> Result<T, String>
where
    F: FnMut(usize) -> Result<T, String>,
{
    let total_attempts = max_attempts.max(1);
    let mut last_err = String::new();
    for attempt in 0..total_attempts {
        match op(attempt) {
            Ok(value) => return Ok(value),
            Err(err) => {
                last_err = err;
                if attempt + 1 < total_attempts && !retry_delay.is_zero() {
                    thread::sleep(retry_delay);
                }
            }
        }
    }
    Err(last_err)
}

/// Discover, open, configure speed, and attach a `probe-rs` [`Session`] for `chip` with frontend retries.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub fn open_probe_session_with_retry(
    chip: &str,
    probe_selector: Option<&str>,
    speed_khz: u32,
) -> Result<Session, String> {
    let selector = match probe_selector {
        Some(sel_str) => Some(
            DebugProbeSelector::try_from(sel_str)
                .map_err(|e| format!("Invalid --probe selector '{}': {}", sel_str, e))?,
        ),
        None => None,
    };

    retry_probe_attach(
        PROBE_ATTACH_MAX_ATTEMPTS,
        Duration::from_millis(PROBE_ATTACH_RETRY_DELAY_MS),
        |_attempt| {
            let lister = Lister::new();
            let probes = match selector {
                Some(ref sel) => lister.list(Some(sel)),
                None => lister.list_all(),
            };
            let probe_info = probes
                .first()
                .ok_or_else(|| "No debug probes connected".to_string())?;
            let mut probe = probe_info
                .open()
                .map_err(|e| format!("Failed to open debug probe: {:?}", e))?;
            let _ = probe.set_speed(speed_khz);
            probe
                .attach(chip, Permissions::default())
                .map_err(|e| format!("Failed to attach probe to '{}': {:?}", chip, e))
        },
    )
}
