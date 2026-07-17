use std::fs;
use tracing_subscriber::prelude::*;

/// Initializes the Chrome tracing subscriber if a trace file is provided.
/// Returns the flush guard to keep the trace active.
pub fn init_tracing(trace_file: Option<&str>) -> Option<tracing_chrome::FlushGuard> {
    if let Some(path) = trace_file {
        let (chrome_layer, guard) = tracing_chrome::ChromeLayerBuilder::new()
            .file(path)
            .include_args(true)
            .build();
        let filter = tracing_subscriber::filter::Targets::new()
            .with_target("nusb", tracing_subscriber::filter::LevelFilter::OFF)
            .with_target("probe_rs", tracing_subscriber::filter::LevelFilter::OFF)
            .with_target("probe-rs", tracing_subscriber::filter::LevelFilter::OFF)
            .with_default(tracing_subscriber::filter::LevelFilter::TRACE);
        tracing_subscriber::registry()
            .with(chrome_layer.with_filter(filter))
            .init();
        Some(guard)
    } else {
        None
    }
}

/// Post-processes the generated trace JSON file to group microcontroller events
/// by virtual thread timelines (tasks).
pub fn post_process_trace(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(path).exists() {
        return Ok(());
    }
    let content = fs::read_to_string(path)?;
    if content.trim().is_empty() {
        return Ok(());
    }

    let mut events: Vec<serde_json::Value> = serde_json::from_str(&content)?;

    // Find the pid used in the trace
    let pid = events
        .iter()
        .find_map(|val| val.get("pid").and_then(|p| p.as_i64()))
        .unwrap_or(1);

    // Sort events by timestamp (if present), keeping metadata events (without ts) at the front
    events.sort_by(|a, b| {
        let ts_a = a.get("ts").and_then(|t| t.as_f64());
        let ts_b = b.get("ts").and_then(|t| t.as_f64());
        match (ts_a, ts_b) {
            (Some(ta), Some(tb)) => ta.partial_cmp(&tb).unwrap_or(std::cmp::Ordering::Equal),
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });

    let mut thread_map = std::collections::HashMap::new();
    let mut next_tid = 10;

    let mut name_map = std::collections::HashMap::new();
    let mut active_spans = Vec::new();

    // Reconstruct spans and group logs into timelines
    let mut processed_events = Vec::new();

    for mut event in events {
        let mut keep_event = true;

        if let Some(obj) = event.as_object_mut() {
            let target = obj.get("cat").and_then(|c| c.as_str()).unwrap_or("");
            match target {
                "device_span_enter" => {
                    let span_id = obj
                        .get("args")
                        .and_then(|a| a.get("span_id"))
                        .and_then(|s| s.as_str())
                        .unwrap_or("")
                        .to_string();
                    let span_name = obj
                        .get("args")
                        .and_then(|a| a.get("span_name"))
                        .and_then(|s| s.as_str())
                        .unwrap_or("")
                        .to_string();

                    name_map.insert(span_id.clone(), span_name.clone());
                    active_spans.push(span_id.clone());

                    let root_span_id = active_spans.first().cloned().unwrap_or(span_id);
                    let root_span_name = name_map
                        .get(&root_span_id)
                        .cloned()
                        .unwrap_or_else(|| "unknown".to_string());

                    let vt_id = *thread_map.entry(root_span_name).or_insert_with(|| {
                        let id = next_tid;
                        next_tid += 1;
                        id
                    });

                    // Convert this log event into a standard Chrome Trace "B" (Begin) event
                    obj.insert("ph".to_string(), serde_json::Value::from("B"));
                    obj.insert("name".to_string(), serde_json::Value::from(span_name));
                    obj.insert("cat".to_string(), serde_json::Value::from("device"));
                    obj.insert("tid".to_string(), serde_json::Value::from(vt_id));
                    obj.remove("args");
                }
                "device_span_exit" => {
                    let span_id = obj
                        .get("args")
                        .and_then(|a| a.get("span_id"))
                        .and_then(|s| s.as_str())
                        .unwrap_or("")
                        .to_string();

                    let span_name = name_map
                        .get(&span_id)
                        .cloned()
                        .unwrap_or_else(|| "unknown".to_string());
                    let root_span_id = active_spans.first().cloned().unwrap_or(span_id.clone());
                    let root_span_name = name_map
                        .get(&root_span_id)
                        .cloned()
                        .unwrap_or_else(|| "unknown".to_string());

                    let vt_id = *thread_map.entry(root_span_name).or_insert_with(|| {
                        let id = next_tid;
                        next_tid += 1;
                        id
                    });

                    // Convert this log event into a standard Chrome Trace "E" (End) event
                    obj.insert("ph".to_string(), serde_json::Value::from("E"));
                    obj.insert("name".to_string(), serde_json::Value::from(span_name));
                    obj.insert("cat".to_string(), serde_json::Value::from("device"));
                    obj.insert("tid".to_string(), serde_json::Value::from(vt_id));
                    obj.remove("args");

                    // Remove from active stack (and any un-exited children nested inside it)
                    if let Some(pos) = active_spans.iter().position(|x| x == &span_id) {
                        active_spans.truncate(pos);
                    }
                }
                "device_log" => {
                    let msg = obj
                        .get("args")
                        .and_then(|args| args.get("message").or_else(|| args.get("val")))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    if !msg.is_empty() {
                        obj.insert("name".to_string(), serde_json::Value::from(msg));
                    }

                    if let Some(root_span_id) = active_spans.first() {
                        let root_span_name = name_map
                            .get(root_span_id)
                            .cloned()
                            .unwrap_or_else(|| "unknown".to_string());
                        let vt_id = *thread_map.entry(root_span_name).or_insert_with(|| {
                            let id = next_tid;
                            next_tid += 1;
                            id
                        });
                        obj.insert("tid".to_string(), serde_json::Value::from(vt_id));
                    }
                }
                _ => {
                    let ph = obj.get("ph").and_then(|p| p.as_str()).unwrap_or("");
                    if ph == "M" {
                        keep_event = false;
                    }
                }
            }
        }

        if keep_event {
            processed_events.push(event);
        }
    }

    // Add metadata events to define thread names dynamically from the root span names
    for (name, tid) in thread_map {
        let mut meta = serde_json::Map::new();
        meta.insert("name".to_string(), serde_json::Value::from("thread_name"));
        meta.insert("ph".to_string(), serde_json::Value::from("M"));
        meta.insert("pid".to_string(), serde_json::Value::from(pid));
        meta.insert("tid".to_string(), serde_json::Value::from(tid));

        let mut args = serde_json::Map::new();
        args.insert("name".to_string(), serde_json::Value::from(name));
        meta.insert(
            "args".to_string(),
            serde_json::Value::from(serde_json::Value::Object(args)),
        );

        processed_events.insert(0, serde_json::Value::Object(meta));
    }

    // Serialize back to file
    let serialized = serde_json::to_string_pretty(&processed_events)?;
    fs::write(path, serialized)?;
    Ok(())
}

pub fn handle_tracing_line(line: &str) -> Result<bool, &'static str> {
    if let Some(pos) = line.find(" span_enter: ") {
        let name = &line[pos + 13..];
        let prefix = &line[..pos];
        if let Some(colon_pos) = prefix.find(':') {
            let space_pos = prefix[colon_pos..]
                .find(' ')
                .map(|p| colon_pos + p)
                .unwrap_or(prefix.len());
            let span_id = &prefix[colon_pos + 1..space_pos];
            let parent_id = if let Some(parent_pos) = prefix.find("parent=") {
                let p_part = &prefix[parent_pos + 7..];
                p_part.split(' ').next().unwrap_or(p_part)
            } else {
                ""
            };
            tracing::info!(
                target: "device_span_enter",
                span_name = name,
                span_id = span_id,
                parent_id = parent_id
            );
            Ok(true)
        } else {
            Err("Malformed span_enter: missing colon in prefix")
        }
    } else if let Some(pos) = line.find(" span_exit: ") {
        let prefix = &line[..pos];
        if let Some(colon_pos) = prefix.find(':') {
            let space_pos = prefix[colon_pos..]
                .find(' ')
                .map(|p| colon_pos + p)
                .unwrap_or(prefix.len());
            let span_id = &prefix[colon_pos + 1..space_pos];
            tracing::info!(
                target: "device_span_exit",
                span_id = span_id
            );
            Ok(true)
        } else {
            Err("Malformed span_exit: missing colon in prefix")
        }
    } else {
        Ok(false)
    }
}
