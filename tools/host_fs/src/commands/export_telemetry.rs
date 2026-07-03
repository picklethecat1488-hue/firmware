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
) -> io::Result<()> {
    spinner.set_message("Fetching telemetry.rrd from filesystem...");
    let key = string_to_key("telemetry.rrd");
    let mut out_buf = vec![0u8; 1024 * 16]; // support up to 16KB files

    let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
        flash,
        flash_range,
        cache,
        &mut out_buf,
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
            if len == 0 || len > 11 {
                spinner.finish_and_clear();
                eprintln!("Error: Invalid telemetry header length byte ({})", len);
                std::process::exit(1);
            }

            let payload = &content[1..1 + len];
            let mut decoder = minicbor::Decoder::new(payload);
            let mut count = 0;
            let mut next_idx = 0;
            if let Ok(_array_len) = decoder.array() {
                if let Ok(c) = decoder.u32() {
                    if let Ok(n) = decoder.u32() {
                        count = c as usize;
                        next_idx = n as usize;
                    }
                }
            }

            let max_records = 45;
            if count > max_records || next_idx > max_records {
                spinner.finish_and_clear();
                eprintln!("Error: Invalid header count/next_idx in telemetry file");
                std::process::exit(1);
            }

            let mut records = Vec::new();

            let mut process_record = |offset: usize| {
                if offset + 20 <= content.len() {
                    let slot: &[u8; 20] = content[offset..offset + 20].try_into().unwrap();
                    if let Some((ts, rec)) = model::telemetry::TelemetryRecord::deserialize(slot) {
                        records.push((ts, rec));
                    }
                }
            };

            if count < max_records {
                for i in 0..count {
                    process_record(12 + i * 20);
                }
            } else {
                for i in 0..max_records {
                    let idx = (next_idx + i) % max_records;
                    process_record(12 + idx * 20);
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
                        model::types::ProximityTelemetry::InRange(d) => {
                            writeln!(csv_file, "{},Proximity,InRange,{},,", ts, d)?;
                        }
                        model::types::ProximityTelemetry::OutRange(d) => {
                            writeln!(csv_file, "{},Proximity,OutRange,{},,", ts, d)?;
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
