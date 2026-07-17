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

    // Run host_cli with --trace option on non-existent ELF
    let output = Command::new(bin_path)
        .arg("--elf")
        .arg("non_existent_file.elf")
        .arg("--trace")
        .arg("my_test_trace.json")
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Failed to read ELF file") || stderr.contains("error"));
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
    host_cli::tracing::post_process_trace(path).unwrap();

    // Read and parse output
    let content = std::fs::read_to_string(path).unwrap();
    let events: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap();

    // Clean up temp file
    let _ = std::fs::remove_file(path);

    // We expect:
    // - 2 thread metadata events at the beginning: one for "sensor_task" and one for "motor_task"
    // - 9 original events modified with virtual TIDs
    assert!(events.len() >= 11);

    // Check metadata events
    let mut sensor_tid = None;
    let mut motor_tid = None;

    for event in &events {
        let name = event.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let ph = event.get("ph").and_then(|p| p.as_str()).unwrap_or("");
        if ph == "M" && name == "thread_name" {
            let thread_name = event
                .get("args")
                .and_then(|a| a.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("");
            let tid = event.get("tid").and_then(|t| t.as_i64()).unwrap();
            if thread_name == "sensor_task" {
                sensor_tid = Some(tid);
            } else if thread_name == "motor_task" {
                motor_tid = Some(tid);
            }
        }
    }

    assert!(sensor_tid.is_some());
    assert!(motor_tid.is_some());
    assert_ne!(sensor_tid, motor_tid);

    // Verify that events have been mapped to correct virtual TIDs
    let orig_events: Vec<&serde_json::Value> = events
        .iter()
        .filter(|e| e.get("ph").and_then(|p| p.as_str()).unwrap_or("") != "M")
        .collect();

    assert_eq!(orig_events.len(), 9);

    // sensor_task events should have sensor_tid
    assert_eq!(
        orig_events[0].get("tid").and_then(|t| t.as_i64()),
        sensor_tid
    ); // sensor_task enter
    assert_eq!(
        orig_events[1].get("tid").and_then(|t| t.as_i64()),
        sensor_tid
    ); // read_distance enter
    assert_eq!(
        orig_events[2].get("tid").and_then(|t| t.as_i64()),
        sensor_tid
    ); // device_log
    assert_eq!(
        orig_events[2].get("name").and_then(|n| n.as_str()),
        Some("read 120mm")
    ); // device_log renamed to message
    assert_eq!(
        orig_events[3].get("tid").and_then(|t| t.as_i64()),
        sensor_tid
    ); // read_distance exit
    assert_eq!(
        orig_events[4].get("tid").and_then(|t| t.as_i64()),
        sensor_tid
    ); // sensor_task exit

    // motor_task events should have motor_tid
    assert_eq!(
        orig_events[5].get("tid").and_then(|t| t.as_i64()),
        motor_tid
    );
    assert_eq!(
        orig_events[6].get("tid").and_then(|t| t.as_i64()),
        motor_tid
    );
    assert_eq!(
        orig_events[7].get("tid").and_then(|t| t.as_i64()),
        motor_tid
    );
    assert_eq!(
        orig_events[8].get("tid").and_then(|t| t.as_i64()),
        motor_tid
    );
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

    // Test span_enter with ctx prefix
    assert_eq!(
        host_cli::tracing::handle_tracing_line("ctx=000102030405060708090a0b0c0d0e0f:0001020304050607 parent=0000000000000000 span_enter: sensor_task"),
        Ok(true)
    );

    // Test span_exit without ctx prefix (raw hex id)
    assert_eq!(
        host_cli::tracing::handle_tracing_line("000102030405060708090a0b0c0d0e0f:0001020304050607 parent=0000000000000000 span_exit: sensor_task"),
        Ok(true)
    );

    // Test normal non-tracing line
    assert_eq!(
        host_cli::tracing::handle_tracing_line("some normal log message"),
        Ok(false)
    );

    let recorded = events.lock().unwrap();
    assert_eq!(recorded.len(), 2);

    assert_eq!(recorded[0].0, "device_span_enter");
    assert_eq!(recorded[0].1, "sensor_task");
    assert_eq!(recorded[0].2, "0001020304050607");

    assert_eq!(recorded[1].0, "device_span_exit");
    assert_eq!(recorded[1].2, "0001020304050607");
}
