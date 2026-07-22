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
        host_cli::tracing::handle_tracing_line("33.986126 TRACE [peripherals::l9110s::{impl#2}::tick] ctx=000102030405060708090a0b0c0d0e0f:0001020304050607 parent=0000000000000000 span_enter: sensor_task", None, None),
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
        host_cli::tracing::handle_tracing_line("33.986126 TRACE [peripherals::l9110s::{impl#2}::tick] 000102030405060708090a0b0c0d0e0f:0001020304050607 parent=0000000000000000 span_exit: sensor_task", None, None),
        Ok(true)
    );

    // Test with ANSI color escape codes to ensure they are stripped correctly
    assert_eq!(
        host_cli::tracing::handle_tracing_line("\u{1b}[2m33.986126\u{1b}[0m \u{1b}[2mTRACE\u{1b}[0m \u{1b}[2m[peripherals::l9110s::{impl#2}::tick]\u{1b}[0m ctx=000102030405060708090a0b0c0d0e0f:0001020304050607 parent=0000000000000000 span_enter: sensor_task", None, None),
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
    use model::types::{MotorSpeed, MotorStatus};
    use std::io::Write;

    let rec = TelemetryRecord::Motor(MotorStatus::Running(MotorSpeed::new(75).unwrap()));
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

    // We expect 5 events (4 metadata + 1 parsed event)
    assert_eq!(events.len(), 5);
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
