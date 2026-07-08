use crate::flash::EitherFlash;
use crate::string_to_key;
use std::fs::File;
use std::io::{self, Write};

pub async fn run(
    flash: &mut EitherFlash,
    flash_range: std::ops::Range<u32>,
    cache: &mut sequential_storage::cache::NoCache,
    spinner: &indicatif::ProgressBar,
    out_csv: &str,
    buf: &mut [u8],
) -> io::Result<()> {
    spinner.set_message("Fetching telemetry.rrd from filesystem...");
    let key = string_to_key("telemetry.rrd");

    let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
        flash,
        flash_range.clone(),
        cache,
        buf,
        &key,
    )
    .await;

    match res {
        Ok(Some(content)) => {
            if content.len() < 12 {
                spinner.finish_and_clear();
                eprintln!(
                    "Error: Telemetry file is too short ({} bytes)",
                    content.len()
                );
                std::process::exit(1);
            }

            spinner.set_message("Parsing CBOR telemetry records...");
            let len = content[0] as usize;
            let mut count = 0;
            let mut next_idx = 0;
            if len > 0 && len <= 11 && len < content.len() {
                let payload = &content[1..1 + len];
                let mut decoder = minicbor::Decoder::new(payload);
                if let Ok(_array_len) = decoder.array() {
                    if let Ok(c) = decoder.u32() {
                        if let Ok(n) = decoder.u32() {
                            count = c as usize;
                            next_idx = n as usize;
                        }
                    }
                }
            } else {
                eprintln!(
                    "Warning: Telemetry file header is invalid or uninitialized (length byte {}). Exporting 0 records.",
                    len
                );
            }

            let max_records = 1024;

            if count > max_records || next_idx > max_records {
                spinner.finish_and_clear();
                eprintln!("Error: Invalid header count/next_idx in telemetry file");
                std::process::exit(1);
            }

            let mut records = Vec::new();

            // New chunked file format
            let mut current_chunk_idx = None;
            let mut current_chunk_data = vec![0u8; model::telemetry::CHUNK_FILE_SIZE];

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
                    .await;

                    match res {
                        Ok(Some(bytes)) => {
                            current_chunk_data.fill(0);
                            let len = std::cmp::min(bytes.len(), current_chunk_data.len());
                            current_chunk_data[..len].copy_from_slice(&bytes[..len]);
                            current_chunk_idx = Some(chunk_idx);
                        }
                        _ => {
                            spinner.finish_and_clear();
                            eprintln!("Error: Failed to read telemetry chunk {}", chunk_idx);
                            std::process::exit(1);
                        }
                    }
                }

                let offset = slot_idx * 20;
                if offset + 20 <= current_chunk_data.len() {
                    let slot: &[u8; 20] =
                        current_chunk_data[offset..offset + 20].try_into().unwrap();
                    if let Some((ts, rec)) = model::telemetry::TelemetryRecord::deserialize(slot) {
                        records.push((ts, rec));
                    }
                }
            }

            spinner.set_message(format!("Writing records to {}...", out_csv));
            let mut csv_file = File::create(out_csv)?;
            writeln!(csv_file, "timestamp_us,record_type,val1,val2,val3,val4")?;

            for (ts, rec) in records {
                match rec {
                    model::telemetry::TelemetryRecord::Battery(b) => match b {
                        model::types::BatteryStatus::VolTempState(vol, temp, state) => {
                            writeln!(csv_file, "{},Battery,{},{},{:?},", ts, vol, temp, state)?;
                        }
                    },
                    model::telemetry::TelemetryRecord::Motor(m) => match m {
                        model::types::MotorStatus::Brake => {
                            writeln!(csv_file, "{},Motor,0,false,25000,", ts)?;
                        }
                        model::types::MotorStatus::Running(speed) => {
                            writeln!(csv_file, "{},Motor,{},true,25000,", ts, speed)?;
                        }
                    },
                    model::telemetry::TelemetryRecord::Thermal(t) => match t {
                        model::types::ThermalStatus::TempOverheating(temp, overheating) => {
                            writeln!(csv_file, "{},Thermal,{},{},,", ts, temp, overheating)?;
                        }
                    },
                    model::telemetry::TelemetryRecord::System(s) => {
                        writeln!(csv_file, "{},System,{:?},,,", ts, s)?;
                    }
                    model::telemetry::TelemetryRecord::FuelGauge(fg) => match fg {
                        model::types::FuelGaugeTelemetry::VolSoc(vol, soc) => {
                            writeln!(csv_file, "{},FuelGauge,{},{},,", ts, vol, soc)?;
                        }
                    },
                    model::telemetry::TelemetryRecord::Proximity(p) => match p {
                        model::types::ProximityTelemetry::InRange(dir, d) => {
                            writeln!(csv_file, "{},Proximity,InRange,{},{:?},", ts, d, dir)?;
                        }
                        model::types::ProximityTelemetry::OutRange(dir, d) => {
                            writeln!(csv_file, "{},Proximity,OutRange,{},{:?},", ts, d, dir)?;
                        }
                    },
                    model::telemetry::TelemetryRecord::Led(led) => {
                        writeln!(csv_file, "{},Led,{:?},,,", ts, led)?;
                    }
                    model::telemetry::TelemetryRecord::Gesture(g) => {
                        writeln!(csv_file, "{},Gesture,{:?},,,", ts, g)?;
                    }
                    model::telemetry::TelemetryRecord::FlashTelemetry(ft) => {
                        writeln!(
                            csv_file,
                            "{},FlashTelemetry,{},{},{},",
                            ts, ft.sector, ft.duration_ms, ft.erase_count
                        )?;
                    }
                    model::telemetry::TelemetryRecord::ChargerState(state) => {
                        writeln!(csv_file, "{},ChargerState,{:?},,,", ts, state)?;
                    }
                    model::telemetry::TelemetryRecord::PeripheralError(state) => {
                        writeln!(csv_file, "{},PeripheralError,{:?},,,", ts, state)?;
                    }
                    model::telemetry::TelemetryRecord::Boot(reason) => {
                        writeln!(csv_file, "{},Boot,{:?},,,", ts, reason)?;
                    }
                }
            }

            spinner.finish_with_message(format!(
                "Successfully exported telemetry records to {}",
                out_csv
            ));
        }
        Ok(None) => {
            spinner.finish_and_clear();
            eprintln!("File not found: telemetry.rrd");
            std::process::exit(1);
        }
        Err(e) => {
            spinner.finish_and_clear();
            eprintln!("Error reading file: {:?}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}
