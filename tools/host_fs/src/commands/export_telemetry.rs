use crate::flash::EitherFlash;
use model::telemetry::TelemetryRecord;
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

    let max_records = cat_detector::MAX_RECORDS;
    let parser = tool_common::FlashTelemetryParser::new(999);
    let records = match parser
        .read_records(flash, flash_range, cache, buf, max_records)
        .await
    {
        Ok(recs) => recs,
        Err(e) => {
            spinner.finish_and_clear();
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    if out_csv.ends_with(".json") || out_csv.ends_with(".perfetto") {
        spinner.set_message(format!("Writing Perfetto JSON records to {}...", out_csv));
        if let Err(e) = write_perfetto_trace(out_csv, &records) {
            spinner.finish_and_clear();
            eprintln!("Error writing Perfetto trace: {:?}", e);
            std::process::exit(1);
        }
        spinner.finish_with_message(format!(
            "Successfully exported telemetry records in Perfetto format to {}",
            out_csv
        ));
    } else {
        spinner.set_message(format!("Writing CSV records to {}...", out_csv));
        if let Err(e) = write_csv_trace(out_csv, &records) {
            spinner.finish_and_clear();
            eprintln!("Error writing CSV: {:?}", e);
            std::process::exit(1);
        }
        spinner.finish_with_message(format!(
            "Successfully exported telemetry records to {}",
            out_csv
        ));
    }

    Ok(())
}

fn write_csv_trace(out_path: &str, records: &[(u64, TelemetryRecord)]) -> io::Result<()> {
    let mut csv_file = File::create(out_path)?;
    writeln!(csv_file, "timestamp_us,record_type,val1,val2,val3,val4")?;

    for &(ts, ref rec) in records {
        match rec {
            TelemetryRecord::Battery(b) => match b {
                model::types::BatteryStatus::VolTempState(vol, temp, state, active_locks) => {
                    writeln!(
                        csv_file,
                        "{},Battery,{},{},{:?},{}",
                        ts, vol, temp, state, active_locks
                    )?;
                }
            },
            TelemetryRecord::Motor(m) => match m {
                model::types::MotorStatus::Brake => {
                    writeln!(csv_file, "{},Motor,0,false,25000,", ts)?;
                }
                model::types::MotorStatus::Running(speed) => {
                    writeln!(csv_file, "{},Motor,{},true,25000,", ts, speed.get())?;
                }
            },
            TelemetryRecord::Thermal(t) => match t {
                model::types::ThermalStatus::TempOverheating(temp, overheating) => {
                    writeln!(csv_file, "{},Thermal,{},{},,", ts, temp, overheating)?;
                }
            },
            TelemetryRecord::System(s) => {
                writeln!(csv_file, "{},System,{:?},,,", ts, s)?;
            }
            TelemetryRecord::FuelGauge(fg) => match fg {
                model::types::FuelGaugeTelemetry::VolSoc(vol, soc) => {
                    writeln!(csv_file, "{},FuelGauge,{},{},,", ts, vol, soc)?;
                }
            },
            TelemetryRecord::Proximity(p) => match p {
                model::types::ProximityTelemetry::InRange(dir, d) => {
                    writeln!(csv_file, "{},Proximity,InRange,{},{:?},", ts, d, dir)?;
                }
                model::types::ProximityTelemetry::OutRange(dir, d) => {
                    writeln!(csv_file, "{},Proximity,OutRange,{},{:?},", ts, d, dir)?;
                }
            },
            TelemetryRecord::Led(led) => {
                writeln!(csv_file, "{},Led,{:?},,,", ts, led)?;
            }
            TelemetryRecord::Gesture(g) => {
                writeln!(csv_file, "{},Gesture,{:?},,,", ts, g)?;
            }
            TelemetryRecord::FlashTelemetry(ft) => {
                writeln!(
                    csv_file,
                    "{},FlashTelemetry,{},{},{},",
                    ts, ft.sector, ft.duration_ms, ft.erase_count
                )?;
            }
            TelemetryRecord::ChargerState(state) => {
                writeln!(csv_file, "{},ChargerState,{:?},,,", ts, state)?;
            }
            TelemetryRecord::PeripheralError(state) => match state {
                model::types::PeripheralError::I2CBusError(address, register) => {
                    writeln!(
                        csv_file,
                        "{},PeripheralError,I2CBusError,0x{:02X},0x{:02X},",
                        ts, address, register
                    )?;
                }
                model::types::PeripheralError::I2CArbitrationLoss(address, register) => {
                    writeln!(
                        csv_file,
                        "{},PeripheralError,I2CArbitrationLoss,0x{:02X},0x{:02X},",
                        ts, address, register
                    )?;
                }
                model::types::PeripheralError::I2COverrun(address, register) => {
                    writeln!(
                        csv_file,
                        "{},PeripheralError,I2COverrun,0x{:02X},0x{:02X},",
                        ts, address, register
                    )?;
                }
                model::types::PeripheralError::I2CNackAddress(address, register) => {
                    writeln!(
                        csv_file,
                        "{},PeripheralError,I2CNackAddress,0x{:02X},0x{:02X},",
                        ts, address, register
                    )?;
                }
                model::types::PeripheralError::I2CNackData(address, register) => {
                    writeln!(
                        csv_file,
                        "{},PeripheralError,I2CNackData,0x{:02X},0x{:02X},",
                        ts, address, register
                    )?;
                }
                model::types::PeripheralError::I2CNackUnknown(address, register) => {
                    writeln!(
                        csv_file,
                        "{},PeripheralError,I2CNackUnknown,0x{:02X},0x{:02X},",
                        ts, address, register
                    )?;
                }
                model::types::PeripheralError::I2COther(address, register) => {
                    writeln!(
                        csv_file,
                        "{},PeripheralError,I2COther,0x{:02X},0x{:02X},",
                        ts, address, register
                    )?;
                }
                model::types::PeripheralError::I2CUnknown(address, register) => {
                    writeln!(
                        csv_file,
                        "{},PeripheralError,I2CUnknown,0x{:02X},0x{:02X},",
                        ts, address, register
                    )?;
                }
                other => {
                    writeln!(csv_file, "{},PeripheralError,{:?},,,", ts, other)?;
                }
            },
            TelemetryRecord::Boot(reason) => {
                writeln!(csv_file, "{},Boot,{:?},,,", ts, reason)?;
            }
            TelemetryRecord::PeriodicInterval(device, interval) => {
                let interval_str = match interval {
                    model::types::PeriodicInterval::None => "None".to_string(),
                    model::types::PeriodicInterval::UpdateMs(ms) => ms.to_string(),
                };
                writeln!(
                    csv_file,
                    "{},PeriodicInterval,{:?},{},,",
                    ts, device, interval_str
                )?;
            }
        }
    }
    Ok(())
}

fn write_perfetto_trace(out_path: &str, records: &[(u64, TelemetryRecord)]) -> io::Result<()> {
    let mut file = File::create(out_path)?;
    let mut events = Vec::new();

    // metadata to name the track
    events.push(serde_json::json!({
        "name": "thread_name",
        "ph": "M",
        "pid": 1,
        "tid": 999,
        "args": {
            "name": "CPU Telemetry"
        }
    }));

    events.push(serde_json::json!({
        "name": "thread_sort_index",
        "ph": "M",
        "pid": 1,
        "tid": 999,
        "args": {
            "sort_index": 50
        }
    }));

    use tool_common::TelemetryParser;
    let parser = tool_common::FlashTelemetryParser::new(999);
    for &(ts_us, ref rec) in records {
        let ts = ts_us as f64;
        let record_events = parser.record_to_perfetto_events(rec, ts);
        events.extend(record_events);
    }

    serde_json::to_writer_pretty(&mut file, &events)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}
