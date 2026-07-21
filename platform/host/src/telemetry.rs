use embedded_storage_async::nor_flash::MultiwriteNorFlash;
use model::telemetry::TelemetryRecord;
use std::cmp;

/// Helper utility to hash or pad a string filename into a 32-byte key
/// used by the sequential-storage map.
fn string_to_key(name: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    let bytes = name.as_bytes();
    let len = cmp::min(bytes.len(), 32);
    key[..len].copy_from_slice(&bytes[..len]);
    key
}

/// Unified trait representing a host-side telemetry parser.
pub trait TelemetryParser {
    /// Deserializes a raw CBOR payload (array of timestamp and TelemetryRecord) back into its parts.
    fn deserialize_raw_cbor(&self, payload: &[u8]) -> Option<(u64, TelemetryRecord)> {
        let mut decoder = minicbor::Decoder::new(payload);
        let _array_len = decoder.array().ok()?;
        let ts = decoder.u64().ok()?;
        let record = decoder.decode().ok()?;
        Some((ts, record))
    }

    /// Converts a TelemetryRecord into a list of Perfetto JSON events.
    fn record_to_perfetto_events(&self, rec: &TelemetryRecord, ts: f64) -> Vec<serde_json::Value>;
}

fn make_telemetry_event(name: &str, ts: f64, args: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "cat": "telemetry",
        "name": name,
        "ph": "i",
        "pid": 1,
        "ts": ts,
        "args": args
    })
}

/// Helper function to build Perfetto JSON events from a TelemetryRecord.
fn telemetry_record_to_perfetto_json(rec: &TelemetryRecord, ts: f64) -> Vec<serde_json::Value> {
    let mut events = Vec::new();
    match rec {
        TelemetryRecord::Battery(b) => match b {
            model::types::BatteryStatus::VolTempState(vol, temp, state, active_locks) => {
                let state_str = match state {
                    model::types::BatteryState::Ok => "Ok",
                    model::types::BatteryState::Low => "Low",
                    model::types::BatteryState::Charging => "Charging",
                    model::types::BatteryState::Critical => "Critical",
                };
                events.push(make_telemetry_event(
                    "Battery Voltage (mV)",
                    ts,
                    serde_json::json!({ "value": vol }),
                ));
                events.push(make_telemetry_event(
                    "Battery Temp (mC)",
                    ts,
                    serde_json::json!({ "value": temp }),
                ));
                events.push(make_telemetry_event(
                    "Battery State Change",
                    ts,
                    serde_json::json!({ "state": state_str, "active_locks": active_locks }),
                ));
            }
        },
        TelemetryRecord::Motor(m) => match m {
            model::types::MotorStatus::Brake => {
                events.push(make_telemetry_event(
                    "Motor Speed",
                    ts,
                    serde_json::json!({ "value": 0 }),
                ));
            }
            model::types::MotorStatus::Running(speed) => {
                events.push(make_telemetry_event(
                    "Motor Speed",
                    ts,
                    serde_json::json!({ "value": speed.get() }),
                ));
            }
        },
        TelemetryRecord::Thermal(t) => match t {
            model::types::ThermalStatus::TempOverheating(temp, overheating) => {
                events.push(make_telemetry_event(
                    "MCU Temperature (mC)",
                    ts,
                    serde_json::json!({ "value": temp }),
                ));
                if *overheating {
                    events.push(make_telemetry_event(
                        "Overheating Alarm",
                        ts,
                        serde_json::json!({ "overheating": true }),
                    ));
                }
            }
        },
        TelemetryRecord::System(s) => {
            let cmd_str = match s {
                model::types::SystemStatus::PowerDown => "PowerDown",
                model::types::SystemStatus::Active => "Active",
                model::types::SystemStatus::Sleep => "Sleep",
            };
            events.push(make_telemetry_event(
                "System Command",
                ts,
                serde_json::json!({ "cmd": cmd_str }),
            ));
        }
        TelemetryRecord::FuelGauge(fg) => match fg {
            model::types::FuelGaugeTelemetry::VolSoc(vol, soc) => {
                events.push(make_telemetry_event(
                    "FuelGauge Voltage (mV)",
                    ts,
                    serde_json::json!({ "value": vol }),
                ));
                events.push(make_telemetry_event(
                    "Battery SoC (%)",
                    ts,
                    serde_json::json!({ "value": soc }),
                ));
            }
        },
        TelemetryRecord::Proximity(p) => match p {
            model::types::ProximityTelemetry::InRange(dir, d) => {
                let dir_str = match dir {
                    model::types::Direction::North => "North",
                    model::types::Direction::East => "East",
                    model::types::Direction::West => "West",
                };
                events.push(make_telemetry_event(
                    &format!("Proximity ({})", dir_str),
                    ts,
                    serde_json::json!({ "value": d, "in_range": true }),
                ));
            }
            model::types::ProximityTelemetry::OutRange(dir, d) => {
                let dir_str = match dir {
                    model::types::Direction::North => "North",
                    model::types::Direction::East => "East",
                    model::types::Direction::West => "West",
                };
                events.push(make_telemetry_event(
                    &format!("Proximity ({})", dir_str),
                    ts,
                    serde_json::json!({ "value": d, "in_range": false }),
                ));
            }
        },
        TelemetryRecord::Led(led) => {
            let led_str = match led {
                model::types::SystemLedState::Off => "Off",
                model::types::SystemLedState::SolidGreen => "SolidGreen",
                model::types::SystemLedState::SolidBlue => "SolidBlue",
                model::types::SystemLedState::SolidYellow => "SolidYellow",
                model::types::SystemLedState::SolidOrange => "SolidOrange",
                model::types::SystemLedState::BlinksRedFourTimes => "BlinksRedFourTimes",
                model::types::SystemLedState::BlinksRedOncePerThirtySeconds => {
                    "BlinksRedOncePerThirtySeconds"
                }
            };
            events.push(make_telemetry_event(
                "LED Change",
                ts,
                serde_json::json!({ "led": led_str }),
            ));
        }
        TelemetryRecord::Gesture(g) => {
            let g_str = match g {
                model::types::Gesture::DualLongPress => "DualLongPress",
            };
            events.push(make_telemetry_event(
                "Gesture Action",
                ts,
                serde_json::json!({ "gesture": g_str }),
            ));
        }
        TelemetryRecord::FlashTelemetry(ft) => {
            events.push(make_telemetry_event("Flash Erase Duration (ms)", ts, serde_json::json!({ "value": ft.duration_ms, "sector": ft.sector, "erase_count": ft.erase_count })));
        }
        TelemetryRecord::ChargerState(state) => {
            let ch_str = match state {
                model::types::ChargeState::Charging => "Charging",
                model::types::ChargeState::DoneOrStandbyOrUnplugged => "DoneOrStandbyOrUnplugged",
                model::types::ChargeState::RecoverableFault => "RecoverableFault",
                model::types::ChargeState::NonRecoverableFault => "NonRecoverableFault",
            };
            events.push(make_telemetry_event(
                "Charger State",
                ts,
                serde_json::json!({ "state": ch_str }),
            ));
        }
        TelemetryRecord::PeripheralError(err) => {
            let err_str = match err {
                model::types::PeripheralError::DeviceNotFound => "DeviceNotFound",
                model::types::PeripheralError::InvalidConfiguration => "InvalidConfiguration",
                model::types::PeripheralError::NotImplemented => "NotImplemented",
                model::types::PeripheralError::DeviceNotAvailable => "DeviceNotAvailable",
                model::types::PeripheralError::Unknown => "Unknown",
                model::types::PeripheralError::PinError => "PinError",
                model::types::PeripheralError::I2CBusError(_, _) => "I2CBusError",
                model::types::PeripheralError::I2CArbitrationLoss(_, _) => "I2CArbitrationLoss",
                model::types::PeripheralError::I2COverrun(_, _) => "I2COverrun",
                model::types::PeripheralError::I2CNackAddress(_, _) => "I2CNackAddress",
                model::types::PeripheralError::I2CNackData(_, _) => "I2CNackData",
                model::types::PeripheralError::I2CNackUnknown(_, _) => "I2CNackUnknown",
                model::types::PeripheralError::I2COther(_, _) => "I2COther",
                model::types::PeripheralError::I2CUnknown(_, _) => "I2CUnknown",
            };
            events.push(make_telemetry_event(
                "Peripheral Error",
                ts,
                serde_json::json!({ "error": err_str }),
            ));
        }
        TelemetryRecord::Boot(reason) => {
            let reason_str = match reason {
                model::types::BootReason::PowerOn => "PowerOn",
                model::types::BootReason::Watchdog => "Watchdog",
                model::types::BootReason::SoftwareReset => "SoftwareReset",
                model::types::BootReason::Unknown => "Unknown",
            };
            events.push(make_telemetry_event(
                "System Boot",
                ts,
                serde_json::json!({ "reason": reason_str }),
            ));
        }
        TelemetryRecord::PeriodicInterval(device, interval) => {
            let device_str = match device {
                model::types::Device::Motor => "Motor",
                model::types::Device::Sensors => "Sensors",
                model::types::Device::Led => "Led",
                model::types::Device::Battery => "Battery",
                model::types::Device::Thermal => "Thermal",
            };
            let interval_str = match interval {
                model::types::PeriodicInterval::None => "None".to_string(),
                model::types::PeriodicInterval::UpdateMs(ms) => format!("UpdateMs({})", ms),
            };
            events.push(make_telemetry_event(
                "Periodic Interval Update",
                ts,
                serde_json::json!({ "device": device_str, "interval": interval_str }),
            ));
        }
    }
    events
}

/// Telemetry parser configured for RTT trace logs.
pub struct TraceTelemetryParser {
    tid: u64,
}

impl TraceTelemetryParser {
    /// Creates a new TraceTelemetryParser with a custom target tid.
    pub fn new(tid: u64) -> Self {
        Self { tid }
    }

    /// Parses structured telemetry record RTT log output into a list of Perfetto JSON events.
    pub fn parse_log(&self, msg: &str, ts: f64) -> Option<Vec<serde_json::Value>> {
        // Check for raw byte array logging prefix
        if msg.starts_with("Device Telemetry: ") {
            let array_str = msg.strip_prefix("Device Telemetry: ")?.trim();
            if !array_str.starts_with('[') || !array_str.ends_with(']') {
                return None;
            }
            let content = &array_str[1..array_str.len() - 1];
            let mut bytes = Vec::new();
            for part in content.split(',') {
                let trimmed = part.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let byte = trimmed.parse::<u8>().ok()?;
                bytes.push(byte);
            }

            let (_record_ts, record) = self.deserialize_raw_cbor(&bytes)?;
            return Some(self.record_to_perfetto_events(&record, ts));
        }
        None
    }
}

impl TelemetryParser for TraceTelemetryParser {
    fn record_to_perfetto_events(&self, rec: &TelemetryRecord, ts: f64) -> Vec<serde_json::Value> {
        let mut events = telemetry_record_to_perfetto_json(rec, ts);
        for event in &mut events {
            if let Some(obj) = event.as_object_mut() {
                obj.insert("tid".to_string(), serde_json::json!(self.tid));
            }
        }
        events
    }
}

/// Telemetry parser configured for flash storage extraction.
pub struct FlashTelemetryParser {
    tid: u64,
}

impl FlashTelemetryParser {
    /// Creates a new FlashTelemetryParser with a custom target tid.
    pub fn new(tid: u64) -> Self {
        Self { tid }
    }

    /// Reads and parses all telemetry records from flash storage.
    pub async fn read_records<F>(
        &self,
        flash: &mut F,
        flash_range: std::ops::Range<u32>,
        cache: &mut sequential_storage::cache::NoCache,
        buf: &mut [u8],
        max_records: usize,
    ) -> Result<Vec<(u64, TelemetryRecord)>, String>
    where
        F: MultiwriteNorFlash,
        F::Error: std::fmt::Debug,
    {
        let key = string_to_key("telemetry.rrd");

        let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
            flash,
            flash_range.clone(),
            cache,
            buf,
            &key,
        )
        .await
        .map_err(|e| format!("Failed to fetch telemetry.rrd: {:?}", e))?;

        let content = match res {
            Some(content) => content,
            None => return Ok(Vec::new()),
        };

        if content.len() < 12 {
            return Err(format!(
                "Telemetry file is too short ({} bytes)",
                content.len()
            ));
        }

        let mut header_bytes = [0u8; 12];
        header_bytes[..content.len()].copy_from_slice(content);

        let len = header_bytes[0] as usize;
        if len == 0 || len > 11 {
            return Err("Invalid telemetry header length".to_string());
        }

        let mut decoder = minicbor::Decoder::new(&header_bytes[1..1 + len]);
        let array_len = decoder
            .array()
            .map_err(|e| format!("Failed to decode header array: {:?}", e))?;
        if array_len != Some(2) {
            return Err("Invalid telemetry header format".to_string());
        }

        let count = decoder
            .u32()
            .map_err(|e| format!("Failed to decode count: {:?}", e))? as usize;
        let next_idx = decoder
            .u32()
            .map_err(|e| format!("Failed to decode next_idx: {:?}", e))?
            as usize;

        let mut records = Vec::new();
        let mut current_chunk_idx = None;
        let mut current_chunk_data = [0u8; model::telemetry::CHUNK_FILE_SIZE];

        let total_iterations = if count < max_records {
            count
        } else {
            max_records
        };
        for i in 0..total_iterations {
            let idx = if count < max_records {
                i
            } else {
                (next_idx + i) % max_records
            };
            let chunk_idx = idx / model::telemetry::CHUNK_SIZE;
            let slot_idx = idx % model::telemetry::CHUNK_SIZE;
            if current_chunk_idx != Some(chunk_idx) {
                let name = model::telemetry::chunk_name(chunk_idx);
                let chunk_key = string_to_key(name);

                let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                    flash,
                    flash_range.clone(),
                    cache,
                    buf,
                    &chunk_key,
                )
                .await
                .map_err(|e| format!("Failed to read telemetry chunk {}: {:?}", chunk_idx, e))?;

                match res {
                    Some(bytes) => {
                        current_chunk_data.fill(0);
                        let len = std::cmp::min(bytes.len(), current_chunk_data.len());
                        current_chunk_data[..len].copy_from_slice(&bytes[..len]);
                        current_chunk_idx = Some(chunk_idx);
                    }
                    None => {
                        return Err(format!("Telemetry chunk {} not found", chunk_idx));
                    }
                }
            }

            let offset = slot_idx * 20;
            if offset + 20 <= current_chunk_data.len() {
                let slot: &[u8; 20] = current_chunk_data[offset..offset + 20].try_into().unwrap();
                if let Some((ts, rec)) = TelemetryRecord::deserialize(slot) {
                    records.push((ts, rec));
                }
            }
        }

        Ok(records)
    }
}

impl TelemetryParser for FlashTelemetryParser {
    fn record_to_perfetto_events(&self, rec: &TelemetryRecord, ts: f64) -> Vec<serde_json::Value> {
        let mut events = telemetry_record_to_perfetto_json(rec, ts);
        for event in &mut events {
            if let Some(obj) = event.as_object_mut() {
                obj.insert("tid".to_string(), serde_json::json!(self.tid));
            }
        }
        events
    }
}
