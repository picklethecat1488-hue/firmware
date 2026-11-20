use host_cli::{dump_logs, stream_logs, DefmtLogSource};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

struct MockLogSource {
    data: Vec<u8>,
    read_index: usize,
}

impl DefmtLogSource for MockLogSource {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        if self.read_index >= self.data.len() {
            return Ok(0);
        }
        let chunk_size = std::cmp::min(buf.len(), self.data.len() - self.read_index);
        buf[..chunk_size]
            .copy_from_slice(&self.data[self.read_index..self.read_index + chunk_size]);
        self.read_index += chunk_size;
        Ok(chunk_size)
    }
}

#[test]
fn test_stream_logs_poll_loop() {
    let source = MockLogSource {
        data: vec![],
        read_index: 0,
    };

    let elf_candidates = [
        "target/thumbv6m-none-eabi/debug/cat_detector/app",
        "target/thumbv6m-none-eabi/release/cat_detector/app",
        "../../target/thumbv6m-none-eabi/debug/cat_detector/app",
    ];
    let mut elf_path = None;
    for &c in &elf_candidates {
        if PathBuf::from(c).is_file() {
            elf_path = Some(PathBuf::from(c));
            break;
        }
    }

    if let Some(path) = elf_path {
        let elf_data = std::fs::read(path).unwrap();
        if let Ok(Some(table)) = defmt_decoder::Table::parse(&elf_data) {
            let mut writer = Vec::new();
            let res = stream_logs(
                source,
                &table,
                &mut writer,
                Duration::from_millis(1),
                || true, // exit immediately
            );
            assert!(res.is_ok());
            assert!(writer.is_empty());
        }
    }
}

#[test]
fn test_cli_argument_validation() {
    let bin_path = env!("CARGO_BIN_EXE_host_cli");

    // 1. Non-existent ELF file (failing to load metadata / autodetect chip)
    let output = Command::new(bin_path)
        .arg("--elf")
        .arg("non_existent_file.elf")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Failed to read ELF file") || stderr.contains("error"));
}

#[test]
fn test_dump_logs_empty() {
    let source = MockLogSource {
        data: vec![],
        read_index: 0,
    };

    let elf_candidates = [
        "target/thumbv6m-none-eabi/debug/cat_detector/app",
        "target/thumbv6m-none-eabi/release/cat_detector/app",
        "../../target/thumbv6m-none-eabi/debug/cat_detector/app",
    ];
    let mut elf_path = None;
    for &c in &elf_candidates {
        if PathBuf::from(c).is_file() {
            elf_path = Some(PathBuf::from(c));
            break;
        }
    }

    if let Some(path) = elf_path {
        let elf_data = std::fs::read(path).unwrap();
        if let Ok(Some(table)) = defmt_decoder::Table::parse(&elf_data) {
            let mut writer = Vec::new();
            let res = dump_logs(source, &table, &mut writer);
            assert!(res.is_ok());
            assert!(writer.is_empty());
        }
    }
}

#[test]
fn test_cli_trace_argument() {
    let bin_path = env!("CARGO_BIN_EXE_host_cli");

    // Run host_cli with --trace and --duration options on non-existent ELF
    let output = Command::new(bin_path)
        .arg("--elf")
        .arg("non_existent_file.elf")
        .arg("--trace")
        .arg("my_test_trace.json")
        .arg("--duration")
        .arg("5")
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Failed to read ELF file"));
}

#[test]
fn test_cli_trace_argument_missing_duration() {
    let bin_path = env!("CARGO_BIN_EXE_host_cli");

    // Run host_cli with --trace option but missing --duration
    let output = Command::new(bin_path)
        .arg("--elf")
        .arg("non_existent_file.elf")
        .arg("--trace")
        .arg("my_test_trace.json")
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("duration is required"));
}

#[test]
fn test_post_process_trace_dynamic_grouping() {
    use std::io::Write;
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_post_process_trace.json");
    let path = file_path.to_str().unwrap();

    let mock_trace = r#"[
        {"cat": "device_span_enter", "ts": 100, "pid": 42, "tid": 1, "args": {"span_id": "1", "span_name": "sensor_task"}},
        {"cat": "device_span_enter", "ts": 105, "pid": 42, "tid": 1, "args": {"span_id": "2", "span_name": "read_distance"}},
        {"cat": "device_log", "ts": 108, "pid": 42, "tid": 1, "args": {"message": "read 120mm"}},
        {"cat": "device_span_exit", "ts": 110, "pid": 42, "tid": 1, "args": {"span_id": "2"}},
        {"cat": "device_span_exit", "ts": 120, "pid": 42, "tid": 1, "args": {"span_id": "1"}},
        {"cat": "device_span_enter", "ts": 200, "pid": 42, "tid": 1, "args": {"span_id": "3", "span_name": "motor_task"}},
        {"cat": "device_span_enter", "ts": 205, "pid": 42, "tid": 1, "args": {"span_id": "4", "span_name": "set_speed"}},
        {"cat": "device_span_exit", "ts": 210, "pid": 42, "tid": 1, "args": {"span_id": "4"}},
        {"cat": "device_span_exit", "ts": 220, "pid": 42, "tid": 1, "args": {"span_id": "3"}}
    ]"#;

    {
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(mock_trace.as_bytes()).unwrap();
    }

    // Run post processor
    host_cli::tracing::post_process_trace(path, None).unwrap();

    // Read and parse output
    let content = std::fs::read_to_string(path).unwrap();
    let events: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap();

    // Clean up temp file
    let _ = std::fs::remove_file(path);

    // We expect 13 events: 4 metadata prepended + 9 original events.
    assert_eq!(events.len(), 13);

    // Verify metadata events
    assert_eq!(
        events[0].get("name").and_then(|n| n.as_str()),
        Some("process_name")
    );
    assert_eq!(events[0].get("pid").and_then(|p| p.as_i64()), Some(42));

    assert_eq!(
        events[1].get("name").and_then(|n| n.as_str()),
        Some("thread_name")
    );
    assert_eq!(events[1].get("pid").and_then(|p| p.as_i64()), Some(42));
    assert_eq!(events[1].get("tid").and_then(|t| t.as_i64()), Some(1));

    assert_eq!(
        events[2].get("name").and_then(|n| n.as_str()),
        Some("thread_name")
    );
    assert_eq!(events[2].get("pid").and_then(|p| p.as_i64()), Some(42));
    assert_eq!(events[2].get("tid").and_then(|t| t.as_i64()), Some(2));

    assert_eq!(
        events[3].get("name").and_then(|n| n.as_str()),
        Some("thread_name")
    );
    assert_eq!(events[3].get("pid").and_then(|p| p.as_i64()), Some(42));
    assert_eq!(events[3].get("tid").and_then(|t| t.as_i64()), Some(3));

    for (i, ev) in events[4..].iter().enumerate() {
        assert_eq!(ev.get("pid").and_then(|p| p.as_i64()), Some(42));
        if i == 2 {
            assert_eq!(ev.get("tid").and_then(|t| t.as_i64()), Some(3));
            assert_eq!(ev.get("name").and_then(|n| n.as_str()), Some("read 120mm"));
        } else {
            assert_eq!(ev.get("tid").and_then(|t| t.as_i64()), Some(1));
        }
    }
}

#[test]
fn test_handle_tracing_line_parsing() {
    use std::sync::Arc;
    use std::sync::Mutex;
    use tracing::subscriber::set_default;
    use tracing_subscriber::layer::SubscriberExt;

    struct InterceptLayer {
        events: Arc<Mutex<Vec<(String, String, String)>>>,
    }

    impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for InterceptLayer {
        fn on_event(
            &self,
            event: &tracing::Event<'_>,
            _ctx: tracing_subscriber::layer::Context<'_, S>,
        ) {
            let metadata = event.metadata();
            let target = metadata.target().to_string();

            struct Visitor {
                span_name: String,
                span_id: String,
            }
            impl tracing::field::Visit for Visitor {
                fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
                    if field.name() == "span_name" {
                        self.span_name = value.to_string();
                    } else if field.name() == "span_id" {
                        self.span_id = value.to_string();
                    }
                }
                fn record_debug(
                    &mut self,
                    _field: &tracing::field::Field,
                    _value: &dyn std::fmt::Debug,
                ) {
                }
            }

            let mut visitor = Visitor {
                span_name: String::new(),
                span_id: String::new(),
            };
            event.record(&mut visitor);

            self.events
                .lock()
                .unwrap()
                .push((target, visitor.span_name, visitor.span_id));
        }
    }

    let events = Arc::new(Mutex::new(Vec::new()));
    let layer = InterceptLayer {
        events: Arc::clone(&events),
    };

    let subscriber = tracing_subscriber::registry().with(layer);
    let _guard = set_default(subscriber);

    // Test span_enter with ctx prefix and brackets containing colons
    assert_eq!(
        host_cli::tracing::handle_tracing_line("33.986126 TRACE [peripherals::tmc6300::{impl#2}::tick] ctx=000102030405060708090a0b0c0d0e0f:0001020304050607 parent=0000000000000000 span_enter: sensor_task", None, None),
        Ok(true)
    );

    assert_eq!(
        host_cli::tracing::handle_tracing_line(
            "platform::system::run_loop: cpu_idle_c0 span_exit: Core 0: CPU Idle Core 0",
            None,
            None
        ),
        Ok(true)
    );

    // Test span_exit without ctx prefix (raw hex id) and brackets containing colons
    assert_eq!(
        host_cli::tracing::handle_tracing_line("33.986126 TRACE [peripherals::tmc6300::{impl#2}::tick] 000102030405060708090a0b0c0d0e0f:0001020304050607 parent=0000000000000000 span_exit: sensor_task", None, None),
        Ok(true)
    );

    // Test with ANSI color escape codes to ensure they are stripped correctly
    assert_eq!(
        host_cli::tracing::handle_tracing_line("\u{1b}[2m33.986126\u{1b}[0m \u{1b}[2mTRACE\u{1b}[0m \u{1b}[2m[peripherals::tmc6300::{impl#2}::tick]\u{1b}[0m ctx=000102030405060708090a0b0c0d0e0f:0001020304050607 parent=0000000000000000 span_enter: sensor_task", None, None),
        Ok(true)
    );

    // Test normal non-tracing line
    assert_eq!(
        host_cli::tracing::handle_tracing_line("some normal log message", None, None),
        Ok(false)
    );

    let recorded = events.lock().unwrap();
    assert_eq!(recorded.len(), 4);

    assert_eq!(recorded[0].0, "device_span_enter");
    assert_eq!(recorded[0].1, "sensor_task");
    assert_eq!(recorded[0].2, "0001020304050607");

    assert_eq!(recorded[1].0, "device_span_exit");
    assert_eq!(recorded[1].1, "Core 0: CPU Idle Core 0");
    assert_eq!(recorded[1].2, "cpu_idle_c0");

    assert_eq!(recorded[2].0, "device_span_exit");
    assert_eq!(recorded[2].2, "0001020304050607");

    assert_eq!(recorded[3].0, "device_span_enter");
    assert_eq!(recorded[3].1, "sensor_task");
    assert_eq!(recorded[3].2, "0001020304050607");

    // Test direct ParsedTracingLine field accessors & line lifetime reference
    let raw_line = "33.986126 TRACE [module] ctx=000102030405060708090a0b0c0d0e0f parent=0000000000000000 span_enter: sensor_task";
    let parsed = host_cli::tracing::ParsedTracingLine::parse(raw_line).unwrap();
    assert_eq!(parsed.line(), raw_line);
    let (span_id, parent_id) = parsed.parse_ids();
    assert_eq!(span_id, "000102030405060708090a0b0c0d0e0f");
    assert_eq!(parent_id, "0000000000000000");
    assert_eq!(parsed.span_name(), "sensor_task");
    assert_eq!(parsed.is_enter(), true);
    assert_eq!(parsed.device_ts(), Some(33.986126 * 1_000_000.0));

    // Test custom ts= override parsing in ParsedTracingLine
    let ts_override_line = "33.986126 TRACE [module] ctx=000102030405060708090a0b0c0d0e0f parent=0000000000000000 span_enter: sensor_task ts=12345.678";
    let parsed = host_cli::tracing::ParsedTracingLine::parse(ts_override_line).unwrap();
    assert_eq!(parsed.span_name(), "sensor_task");
    assert_eq!(parsed.device_ts(), Some(12345.678));
    let (span_id, parent_id) = parsed.parse_ids();
    assert_eq!(span_id, "000102030405060708090a0b0c0d0e0f");
    assert_eq!(parent_id, "0000000000000000");

    let ts_override_exit = "33.986126 TRACE [module] 000102030405060708090a0b0c0d0e0f span_exit: sensor_task ts=12345.999";
    let parsed_exit = host_cli::tracing::ParsedTracingLine::parse(ts_override_exit).unwrap();
    assert_eq!(parsed_exit.span_name(), "sensor_task");
    assert_eq!(parsed_exit.device_ts(), Some(12345.999));
    let (span_id_exit, _) = parsed_exit.parse_ids();
    assert_eq!(span_id_exit, "000102030405060708090a0b0c0d0e0f");
}

#[test]
fn test_post_process_run_span_renaming() {
    use std::io::Write;
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_run_rename.json");
    let path = file_path.to_str().unwrap();

    let mock_trace = r#"[
        {"cat": "device_span_enter", "ts": 100, "pid": 42, "tid": 1, "args": {"span_id": "1", "span_name": "run", "module": "controller::system_controller"}},
        {"cat": "device_span_exit", "ts": 120, "pid": 42, "tid": 1, "args": {"span_id": "1"}}
    ]"#;

    {
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(mock_trace.as_bytes()).unwrap();
    }

    // Run post processor
    host_cli::tracing::post_process_trace(path, None).unwrap();

    // Read and parse output
    let content = std::fs::read_to_string(path).unwrap();
    let events: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap();

    // Clean up temp file
    let _ = std::fs::remove_file(path);

    // We expect 6 events (4 metadata prepended + 2 renamed events).
    assert_eq!(events.len(), 6);

    // Verify metadata events
    assert_eq!(
        events[0].get("name").and_then(|n| n.as_str()),
        Some("process_name")
    );
    assert_eq!(
        events[1].get("name").and_then(|n| n.as_str()),
        Some("thread_name")
    );
    assert_eq!(events[1].get("tid").and_then(|t| t.as_i64()), Some(1));
    assert_eq!(
        events[2].get("name").and_then(|n| n.as_str()),
        Some("thread_name")
    );
    assert_eq!(events[2].get("tid").and_then(|t| t.as_i64()), Some(2));
    assert_eq!(
        events[3].get("name").and_then(|n| n.as_str()),
        Some("thread_name")
    );
    assert_eq!(events[3].get("tid").and_then(|t| t.as_i64()), Some(3));

    assert_eq!(
        events[4].get("name").and_then(|n| n.as_str()),
        Some("system_controller")
    );
    assert_eq!(events[4].get("ph").and_then(|p| p.as_str()), Some("B"));
    assert_eq!(events[4].get("pid").and_then(|p| p.as_i64()), Some(42));
    assert_eq!(events[4].get("tid").and_then(|t| t.as_i64()), Some(1));

    assert_eq!(
        events[5].get("name").and_then(|n| n.as_str()),
        Some("system_controller")
    );
    assert_eq!(events[5].get("ph").and_then(|p| p.as_str()), Some("E"));
    assert_eq!(events[5].get("pid").and_then(|p| p.as_i64()), Some(42));
    assert_eq!(events[5].get("tid").and_then(|t| t.as_i64()), Some(1));
}

#[test]
fn test_post_process_trace_telemetry() {
    use model::telemetry::TelemetryRecord;
    use model::types::{MotorSpeed, MotorStatus, MotorTelemetry};
    use std::io::Write;

    let rec = TelemetryRecord::Motor(MotorStatus::Running(MotorTelemetry::new(
        MotorSpeed::new(75).unwrap(),
        2500,
        2500,
        150,
        310,
        102,
    )));
    let serialized = rec.serialize(1000);
    let len = serialized[0] as usize;
    let payload = &serialized[1..1 + len];
    let mut log_payload = String::new();
    for (i, &b) in payload.iter().enumerate() {
        if i > 0 {
            log_payload.push_str(", ");
        }
        log_payload.push_str(&format!("{}", b));
    }

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_telemetry_post_process.json");
    let path = file_path.to_str().unwrap();

    let mock_trace = format!(
        r#"[
            {{"cat": "device_log", "ts": 1000, "pid": 42, "tid": 1, "args": {{"message": "Device Telemetry: [{}]"}}}}
        ]"#,
        log_payload
    );

    {
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(mock_trace.as_bytes()).unwrap();
    }

    // Run post processor
    host_cli::tracing::post_process_trace(path, None).unwrap();

    // Read and parse output
    let content = std::fs::read_to_string(path).unwrap();
    let events: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap();

    // Clean up temp file
    let _ = std::fs::remove_file(path);

    // Print events
    for (i, ev) in events.iter().enumerate() {
        println!("Event {}: {}", i, ev);
    }

    // We expect 10 events (4 metadata + 6 parsed motor telemetry events)
    assert_eq!(events.len(), 10);
    assert_eq!(events[4]["name"], "Motor Speed");
    assert_eq!(events[4]["args"]["value"], 75);
    assert_eq!(events[5]["name"], "Motor Current (mA)");
    assert_eq!(events[5]["args"]["value"], 150);
    assert_eq!(events[6]["name"], "Motor Measured RPM");
    assert_eq!(events[6]["args"]["value"], 2500);
    assert_eq!(events[7]["name"], "Motor Estimated RPM");
    assert_eq!(events[7]["args"]["value"], 2500);
    assert_eq!(events[8]["name"], "Motor Modeled Peak Current (mA)");
    assert_eq!(events[8]["args"]["value"], 310);
    assert_eq!(events[9]["name"], "Motor Modeled Torque (uNm)");
    assert_eq!(events[9]["args"]["value"], 102);
}

#[test]
fn test_post_process_cpu_usage_step_chart() {
    use std::io::Write;
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_cpu_usage_step.json");
    let path = file_path.to_str().unwrap();

    let mock_trace = r#"[
        {"cat": "device_span_enter", "ts": 10000, "pid": 42, "tid": 1, "args": {"span_id": "cpu_idle_c0", "span_name": "Core 0: CPU Idle Core 0"}},
        {"cat": "device_span_exit", "ts": 15000, "pid": 42, "tid": 1, "args": {"span_id": "cpu_idle_c0"}}
    ]"#;

    {
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(mock_trace.as_bytes()).unwrap();
    }

    // Run post processor
    host_cli::tracing::post_process_trace(path, None).unwrap();

    // Read and parse output
    let content = std::fs::read_to_string(path).unwrap();
    let events: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap();

    // Clean up temp file
    let _ = std::fs::remove_file(path);

    // Let's filter out Core 0 CPU Usage events
    let usage_events: Vec<&serde_json::Value> = events
        .iter()
        .filter(|e| e.get("name").and_then(|n| n.as_str()) == Some("Core 0 CPU Usage (%)"))
        .collect();

    // We expect exactly 2 counter events: 0% at ts=10000, and 100% at ts=15000
    assert_eq!(usage_events.len(), 2);

    assert_eq!(
        usage_events[0].get("ts").and_then(|t| t.as_f64()),
        Some(10000.0)
    );
    assert_eq!(
        usage_events[0]
            .get("args")
            .and_then(|a| a.get("value"))
            .and_then(|v| v.as_i64()),
        Some(0)
    );

    assert_eq!(
        usage_events[1].get("ts").and_then(|t| t.as_f64()),
        Some(15000.0)
    );
    assert_eq!(
        usage_events[1]
            .get("args")
            .and_then(|a| a.get("value"))
            .and_then(|v| v.as_i64()),
        Some(100)
    );
}

#[test]
fn test_post_process_implicit_closure_on_data_loss() {
    use std::io::Write;
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_implicit_closure.json");
    let path = file_path.to_str().unwrap();

    // Mock trace with missing exit event for span_id "1" before span_id "2" enters with the same name.
    let mock_trace = r#"[
        {"cat": "device_span_enter", "ts": 100, "pid": 42, "tid": 1, "args": {"span_id": "1", "span_name": "sensor_task"}},
        {"cat": "device_span_enter", "ts": 105, "pid": 42, "tid": 1, "args": {"span_id": "2", "span_name": "sensor_task"}},
        {"cat": "device_span_exit", "ts": 120, "pid": 42, "tid": 1, "args": {"span_id": "2"}}
    ]"#;

    {
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(mock_trace.as_bytes()).unwrap();
    }

    // Run post processor
    host_cli::tracing::post_process_trace(path, None).unwrap();

    // Read and parse output
    let content = std::fs::read_to_string(path).unwrap();
    let events: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap();

    // Clean up temp file
    let _ = std::fs::remove_file(path);

    // We expect:
    // 4 metadata events (process_name, thread_name Core 0, thread_name Core 1, thread_name Device Logs)
    // + 1 enter event for span "1" (ts=100)
    // + 1 implicit exit event for span "1" (ts=105)
    // + 1 enter event for span "2" (ts=105)
    // + 1 exit event for span "2" (ts=120)
    // Total: 8 events
    assert_eq!(events.len(), 8);

    // Let's filter for device span events
    let device_events: Vec<&serde_json::Value> = events
        .iter()
        .filter(|e| e.get("cat").and_then(|c| c.as_str()) == Some("device"))
        .collect();

    assert_eq!(device_events.len(), 4);

    // 1. Enter span "1"
    assert_eq!(
        device_events[0].get("name").and_then(|n| n.as_str()),
        Some("sensor_task")
    );
    assert_eq!(
        device_events[0].get("ph").and_then(|p| p.as_str()),
        Some("B")
    );
    assert_eq!(
        device_events[0].get("ts").and_then(|t| t.as_f64()),
        Some(100.0)
    );
    assert_eq!(
        device_events[0].get("span_id").and_then(|s| s.as_str()),
        Some("1")
    );

    // 2. Implicit exit of span "1" at ts=105
    assert_eq!(
        device_events[1].get("name").and_then(|n| n.as_str()),
        Some("sensor_task")
    );
    assert_eq!(
        device_events[1].get("ph").and_then(|p| p.as_str()),
        Some("E")
    );
    assert_eq!(
        device_events[1].get("ts").and_then(|t| t.as_f64()),
        Some(105.0)
    );
    assert_eq!(
        device_events[1].get("span_id").and_then(|s| s.as_str()),
        Some("1")
    );
    assert_eq!(
        device_events[1]
            .get("args")
            .and_then(|a| a.get("implicit"))
            .and_then(|i| i.as_bool()),
        Some(true)
    );

    // 3. Enter span "2" at ts=105
    assert_eq!(
        device_events[2].get("name").and_then(|n| n.as_str()),
        Some("sensor_task")
    );
    assert_eq!(
        device_events[2].get("ph").and_then(|p| p.as_str()),
        Some("B")
    );
    assert_eq!(
        device_events[2].get("ts").and_then(|t| t.as_f64()),
        Some(105.0)
    );
    assert_eq!(
        device_events[2].get("span_id").and_then(|s| s.as_str()),
        Some("2")
    );

    // 4. Exit span "2" at ts=120
    assert_eq!(
        device_events[3].get("name").and_then(|n| n.as_str()),
        Some("sensor_task")
    );
    assert_eq!(
        device_events[3].get("ph").and_then(|p| p.as_str()),
        Some("E")
    );
    assert_eq!(
        device_events[3].get("ts").and_then(|t| t.as_f64()),
        Some(120.0)
    );
    assert_eq!(
        device_events[3].get("span_id").and_then(|s| s.as_str()),
        Some("2")
    );
}

#[test]
fn test_worm_rooted_clitellum_453_format_target_namespace() {
    use host_cli::format_target_namespace;

    // Test cases directly from WORM-ROOTED-CLITELLUM-453
    assert_eq!(
        format_target_namespace("peripheral::tmc6300::{impl#7}::tick::{async_fn#0}"),
        "peripheral::tmc6300"
    );
    assert_eq!(
        format_target_namespace(
            "controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}"
        ),
        "controller::motor_controller"
    );
    assert_eq!(
        format_target_namespace("controller::motor_controller::{impl#5}::update::{async_fn#0}"),
        "controller::motor_controller"
    );
    assert_eq!(
        format_target_namespace("controller::shell_controller::log_command"),
        "controller::shell_controller"
    );
    assert_eq!(
        format_target_namespace("controller::shell_controller::log_success"),
        "controller::shell_controller"
    );

    // Test bracketed forms from console output
    assert_eq!(
        format_target_namespace("[peripheral::tmc6300::{impl#7}::tick::{async_fn#0}]"),
        "peripheral::tmc6300"
    );
    assert_eq!(
        format_target_namespace("[controller::shell_controller::log_command]"),
        "controller::shell_controller"
    );

    // Test other peripheral and nested module cases
    assert_eq!(
        format_target_namespace("peripherals::tmc6300::{impl#2}::tick"),
        "peripherals::tmc6300"
    );
    assert_eq!(
        format_target_namespace("platform::io::flash::{impl#1}::read"),
        "platform::io::flash"
    );
    assert_eq!(
        format_target_namespace("platform::io::flash::read_data"),
        "platform::io::flash"
    );
    assert_eq!(
        format_target_namespace("controller::shell_controller::run_shell::{closure#0}"),
        "controller::shell_controller"
    );

    // Test binary crate / main / edge cases
    assert_eq!(
        format_target_namespace("cat_detector_app::main"),
        "cat_detector_app"
    );
    assert_eq!(
        format_target_namespace("controller::motor_controller"),
        "controller::motor_controller"
    );
    assert_eq!(format_target_namespace("cat_detector"), "cat_detector");
    assert_eq!(format_target_namespace(""), "");
    assert_eq!(format_target_namespace("   "), "");
}

#[test]
fn test_worm_slick_fireworm_677_dump_logs_no_unexpected_eof() {
    let source = MockLogSource {
        data: vec![],
        read_index: 0,
    };

    let elf_candidates = [
        "target/thumbv6m-none-eabi/debug/cat_detector/app",
        "target/thumbv6m-none-eabi/release/cat_detector/app",
        "target/thumbv6m-none-eabi/release/cat_detector_shell",
        "target/thumbv6m-none-eabi/debug/cat_detector_shell",
        "../../target/thumbv6m-none-eabi/debug/cat_detector/app",
    ];
    let mut elf_path = None;
    for &c in &elf_candidates {
        if PathBuf::from(c).is_file() {
            elf_path = Some(PathBuf::from(c));
            break;
        }
    }

    if let Some(path) = elf_path {
        let elf_data = std::fs::read(path).unwrap();
        if let Ok(Some(table)) = defmt_decoder::Table::parse(&elf_data) {
            let mut writer = Vec::new();
            let res = dump_logs(source, &table, &mut writer);
            assert!(res.is_ok());
        }
    }
}

#[test]
fn test_cancel_echo_full_match() {
    let mut sent = b"motor speed 30\r\n".to_vec();
    let rtt = b"motor speed 30\r\n";
    let output = host_cli::cancel_echo(rtt, &mut sent);
    assert!(output.is_empty(), "Echoed characters must be cancelled");
    assert!(sent.is_empty(), "Sent buffer should be emptied");
}

#[test]
fn test_cancel_echo_chunked() {
    let mut sent = b"motor rpm\r\n".to_vec();
    let chunk1 = b"motor rp";
    let chunk2 = b"m\r\n";

    let out1 = host_cli::cancel_echo(chunk1, &mut sent);
    assert!(out1.is_empty(), "First chunk must be cancelled");

    let out2 = host_cli::cancel_echo(chunk2, &mut sent);
    assert!(out2.is_empty(), "Second chunk (stray m) must be cancelled");
    assert!(sent.is_empty());
}

#[test]
fn test_cancel_echo_preserves_target_output() {
    let mut sent = b"motor current\r\n".to_vec();
    let rtt = b"motor current\r\nMotor current: 1461 mA\r\nshell> ";
    let output = host_cli::cancel_echo(rtt, &mut sent);
    assert_eq!(
        output, b"Motor current: 1461 mA\r\nshell> ",
        "Target command response must be preserved while echoed command is dropped"
    );
    assert!(sent.is_empty());
}

#[test]
fn test_structured_json_events_rtt_shell_idle_flag_and_cli_chunk_parser() {
    let mut parser = host_cli::CliChunkParser::new();
    let mut poller = host_cli::ShellIdlePoller::new();

    // 1. Encode/decode RTT control flags (`RTT_SHELL_IDLE_FLAG` + 16-bit wrapping epoch)
    let busy_flags = tool_common::encode_rtt_shell_flags(1, false, 0);
    assert_eq!(tool_common::decode_rtt_shell_flags(busy_flags), (false, 0));
    assert_eq!(poller.poll(busy_flags, true, true), None);

    let idle_epoch1 = tool_common::encode_rtt_shell_flags(1, true, 1);
    assert_eq!(tool_common::decode_rtt_shell_flags(idle_epoch1), (true, 1));
    // Must wait until both `cli_up` and `cli_down` ring buffers are empty before reporting idle!
    assert_eq!(poller.poll(idle_epoch1, false, true), None);
    assert_eq!(poller.poll(idle_epoch1, true, false), None);
    assert_eq!(poller.poll(idle_epoch1, true, true), Some(1));
    // Polling the same epoch again does not emit a duplicate idle event.
    assert_eq!(poller.poll(idle_epoch1, true, true), None);

    // 2. Empty `\x1b[2K\r` (`cli.set_prompt("")` when an async command begins) produces no events
    assert!(parser.push_bytes(b"\x1b[2K\r").is_empty());

    // 3. Partial line split across RTT chunks + `\x1b[2K\r\n<prompt>` followed by RTT idle flag epoch 2
    let ev1 = parser.push_bytes(b"Battery Status:\r\n  Voltage: 40");
    assert_eq!(
        ev1,
        vec![host_cli::HostCliEvent::CliOutput {
            text: "Battery Status:".to_string()
        }]
    );

    let ev2 = parser.push_bytes(b"22 mV\r\n\x1b[2K\r\nshell> ");
    assert_eq!(
        ev2,
        vec![host_cli::HostCliEvent::CliOutput {
            text: "  Voltage: 4022 mV".to_string()
        }]
    );

    let idle_epoch2 = tool_common::encode_rtt_shell_flags(1, true, 2);
    let epoch = poller
        .poll(idle_epoch2, true, true)
        .expect("epoch 2 must trigger ShellIdle");
    let idle_ev = parser.on_shell_idle(epoch);
    assert_eq!(idle_ev, host_cli::HostCliEvent::ShellIdle { epoch: 2 });
    let idle_json: serde_json::Value = serde_json::from_str(&idle_ev.to_json_line()).unwrap();
    assert_eq!(idle_json["event"], "shell_idle");
    assert_eq!(idle_json["epoch"], 2);

    // 4. Verify `detect_shell_completion_event` and JSON serialization
    let ok_ev = host_cli::detect_shell_completion_event(
        "0.412300 INFO command execution succeeded",
        Some(0.4123),
    )
    .expect("must detect success");
    let parsed_ok: serde_json::Value = serde_json::from_str(&ok_ev.to_json_line()).unwrap();
    assert_eq!(parsed_ok["event"], "command_completed");
    assert_eq!(parsed_ok["success"], true);

    let err_ev = host_cli::detect_shell_completion_event(
        "0.512000 ERROR command execution failed: Target out of range",
        Some(0.512),
    )
    .expect("must detect failure");
    let parsed_err: serde_json::Value = serde_json::from_str(&err_ev.to_json_line()).unwrap();
    assert_eq!(parsed_err["event"], "command_completed");
    assert_eq!(parsed_err["success"], false);
    assert_eq!(parsed_err["error"], "Target out of range");
    assert_eq!(
        host_cli::extract_defmt_level("0.512000 WARN battery low"),
        "WARN"
    );
}

#[test]
fn test_live_trace_stream_processor_perf_spans_sensor_reports_and_exports() {
    use host_cli::{
        export_trace_events_to_csv, export_trace_events_to_perfetto_json, HostCliEvent,
        LiveTraceStreamProcessor,
    };
    use model::telemetry::TelemetryRecord;
    use model::types::{
        Direction, Gesture, GestureSource, SensorReading, SensorTelemetry, ThermalStatus,
    };

    let mut proc = LiveTraceStreamProcessor::new();

    // 1. PERF execution spans on Core 0 and Core 1 + CPU Idle counter
    let mut emitted = Vec::new();
    emitted.extend(proc.process_defmt_frame(
        "1.122000 TRACE [controller::motor_controller] ctx=10 parent=0 span_enter: Core 1: motor_controller::update",
        "ctx=10 parent=0 span_enter: Core 1: motor_controller::update",
        Some("controller::motor_controller"),
        Some(1_122_000.0),
    ));
    emitted.extend(proc.process_defmt_frame(
        "1.158000 TRACE [controller::motor_controller] 10 parent=0 span_exit: Core 1: motor_controller::update",
        "10 parent=0 span_exit: Core 1: motor_controller::update",
        Some("controller::motor_controller"),
        Some(1_158_000.0),
    ));
    emitted.extend(proc.process_defmt_frame(
        "1.200000 TRACE [platform::system] ctx=cpu_idle_c0 parent=0 span_enter: Core 0: CPU Idle Core 0 ts=1200000",
        "ctx=cpu_idle_c0 parent=0 span_enter: Core 0: CPU Idle Core 0 ts=1200000",
        Some("platform::system"),
        Some(1_200_000.0),
    ));

    // 2. SENSOR CBOR Device Telemetry frames (Proximity North 118 mm, Thermal 25062 mC, Gesture DualLongPress)
    let format_device_telemetry = |ts_us: u64, rec: TelemetryRecord| -> String {
        let serialized = rec.serialize(ts_us);
        let len = serialized[0] as usize;
        let payload = &serialized[1..1 + len];
        let joined = payload
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("Device Telemetry: [{}]", joined)
    };

    let prox_msg = format_device_telemetry(
        5_204_110,
        TelemetryRecord::Proximity(SensorTelemetry::Status(
            Direction::North,
            SensorReading::Proximity(118),
        )),
    );
    emitted.extend(proc.process_defmt_frame(
        &format!("5.204110 TRACE {}", prox_msg),
        &prox_msg,
        Some("platform::tracing"),
        Some(5_204_110.0),
    ));

    let therm_msg = format_device_telemetry(
        5_150_880,
        TelemetryRecord::Thermal(ThermalStatus::TempOverheating(25_062, false)),
    );
    emitted.extend(proc.process_defmt_frame(
        &format!("5.150880 TRACE {}", therm_msg),
        &therm_msg,
        Some("platform::tracing"),
        Some(5_150_880.0),
    ));

    let gest_msg = format_device_telemetry(
        5_810_220,
        TelemetryRecord::Gesture(Gesture::DualLongPress(GestureSource::Proximity)),
    );
    emitted.extend(proc.process_defmt_frame(
        &format!("5.810220 TRACE {}", gest_msg),
        &gest_msg,
        Some("platform::tracing"),
        Some(5_810_220.0),
    ));

    // Verify span B/E events, CPU usage counter, and sensor value events
    assert!(emitted.iter().any(|ev| matches!(
        ev,
        HostCliEvent::Trace {
            ts_us: 1_122_000,
            track,
            ph,
            category,
            name,
            ..
        } if track == "CPU Core 1" && ph == "B" && category == "span" && name == "motor_controller::update"
    )));
    assert!(emitted.iter().any(|ev| matches!(
        ev,
        HostCliEvent::Trace {
            ts_us: 5_204_110,
            track,
            ph,
            category,
            value: Some(v),
            unit: Some(u),
            ..
        } if track == "Proximity (North)" && ph == "C" && category == "sensor" && (*v - 118.0).abs() < 1e-6 && u == "mm"
    )));
    assert!(emitted.iter().any(|ev| matches!(
        ev,
        HostCliEvent::Trace {
            ts_us: 5_150_880,
            track,
            category,
            value: Some(v),
            unit: Some(u),
            ..
        } if track == "MCU / Board Thermal" && category == "sensor" && (*v - 25_062.0).abs() < 1e-6 && u == "mC"
    )));

    // Verify NDJSON serialization, CSV export, and Perfetto JSON export
    let ndjson: serde_json::Value = serde_json::from_str(&emitted[0].to_json_line()).unwrap();
    assert_eq!(ndjson["event"], "trace_event");
    assert_eq!(ndjson["track"], "CPU Core 1");

    let csv_out = export_trace_events_to_csv(&emitted);
    assert!(csv_out.starts_with(
        "timestamp_us,track,phase,category,name,value,unit,record_type,val1,val2,val3,val4\n"
    ));
    assert!(csv_out.contains(
        "5204110,Proximity (North),C,sensor,Proximity (North),118,mm,Proximity,Valid,118,North,"
    ));
    assert!(csv_out.contains(
        "5150880,MCU / Board Thermal,C,sensor,MCU Temperature (mC),25062,mC,Thermal,25062,false,,"
    ));

    let perfetto_str = export_trace_events_to_perfetto_json(&emitted, "rp2040");
    let perfetto_doc: serde_json::Value = serde_json::from_str(&perfetto_str).unwrap();
    let trace_events = perfetto_doc["traceEvents"].as_array().unwrap();
    assert!(trace_events
        .iter()
        .any(|e| e["name"] == "process_name" && e["args"]["name"] == "RP2040 MCU"));
    assert!(trace_events
        .iter()
        .any(|e| e["name"] == "motor_controller::update" && e["ph"] == "B" && e["tid"] == 2));
    assert!(trace_events.iter().any(|e| e["name"] == "Proximity (North)"
        && e["ph"] == "C"
        && e["args"]["value"] == 118.0));
}
