use std::fs;
use tracing_subscriber::prelude::*;

/// Initializes the Chrome tracing subscriber if a trace file is provided.
///
/// Configures a `ChromeLayer` to output traces to the designated filepath. Target logging
/// filters are set up to silence heavy or redundant USB and target debugger logs (e.g., `nusb` and `probe-rs`).
///
/// Returns the flush guard to keep the trace active until the program exits.
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

/// Helper to check if a span ID represents a root or invalid context.
fn is_root_or_empty_id(id: &str) -> bool {
    id.is_empty() || id == "0000000000000000" || id == "0"
}

/// Helper to determine if a module segment represents a compiler/executor target wrapper.
fn is_target_segment(s: &str) -> bool {
    s == "task"
        || s == "run"
        || s.contains("{impl#")
        || s.contains("__task")
        || s.contains("async_fn")
}

/// Helper to determine if a module segment represents a valid user-defined task/run root name.
/// This is the logical inverse of `is_target_segment`.
fn is_root_segment(s: &str) -> bool {
    !is_target_segment(s)
}

/// Helper to determine if an active span name (which might have namespace prefixes and parameters)
/// matches the exiting target name.
fn is_span_name_match(active_name: &str, exit_target_name: &str) -> bool {
    let base_active = active_name.split('(').next().unwrap_or(active_name).trim();
    base_active == exit_target_name || base_active.ends_with(&format!("::{}", exit_target_name))
}

/// Helper to strip leading and trailing double quotes from a string.
fn strip_quotes(s: &str) -> String {
    let s = s.trim();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Post-processes the generated trace JSON file to group microcontroller events
/// by virtual thread timelines (tasks).
///
/// Since async task executors cooperative multitasking interleaves events on a single hardware thread,
/// this post-processor acts as a reconstruction layer:
/// 1. Chronologically sorts incoming logs.
/// 2. Groups active spans into isolated per-controller stacks (`active_spans_map`) to prevent task interleaving bugs.
/// 3. Resolves span exit events back-to-front on their respective stack, correcting target context-reversion parent IDs.
/// 4. Generates implicit exit events when new root spans are entered, recovering gracefully from RTT log packet drops.
/// 5. Injects metadata events to name each Chrome Trace virtual thread dynamically based on its ELF controller module path.
pub fn post_process_trace(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(path).exists() {
        return Ok(());
    }
    let content = fs::read_to_string(path)?;
    if content.trim().is_empty() {
        return Ok(());
    }

    let mut events: Vec<serde_json::Value> = serde_json::from_str(&content)?;

    // Stage 1: Sort events chronologically
    sort_events_chronologically(&mut events);

    // Stage 2: Filter and process events
    let processed_events = process_trace_events(events);

    // Stage 3: Write back serialized trace output
    save_processed_trace(path, processed_events)?;

    Ok(())
}

/// Stage 1: Sorts events chronologically by timestamp (if present), keeping metadata events at the front.
fn sort_events_chronologically(events: &mut [serde_json::Value]) {
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
}

/// Stage 2: Filters and processes trace events chronologically on their native PID/TID.
fn process_trace_events(events: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
    let mut name_map = std::collections::HashMap::new();
    let mut parent_map = std::collections::HashMap::new();
    let mut module_map = std::collections::HashMap::new();
    let mut start_time_map = std::collections::HashMap::new();
    let mut pid_map = std::collections::HashMap::new();
    let mut tid_map = std::collections::HashMap::new();
    let mut global_active_spans: Vec<String> = Vec::new();
    let mut generic_task_spans = std::collections::HashSet::new();

    let mut processed_events = Vec::new();

    for event in events {
        let mut keep_event = true;
        let mut final_event = event.clone();

        if let Some(obj) = event.as_object() {
            let target = obj.get("cat").and_then(|c| c.as_str()).unwrap_or("");
            match target {
                "device_span_enter" => {
                    let span_id = strip_quotes(
                        obj.get("args")
                            .and_then(|a| a.get("span_id"))
                            .and_then(|s| s.as_str())
                            .unwrap_or(""),
                    );
                    let mut parent_id = strip_quotes(
                        obj.get("args")
                            .and_then(|a| a.get("parent_id"))
                            .and_then(|s| s.as_str())
                            .unwrap_or(""),
                    );

                    let module = strip_quotes(
                        obj.get("args")
                            .and_then(|a| a.get("module"))
                            .and_then(|s| s.as_str())
                            .unwrap_or(""),
                    );

                    if parent_id.is_empty()
                        && obj.get("args").and_then(|a| a.get("parent_id")).is_none()
                    {
                        if let Some(active_id) = global_active_spans.last() {
                            parent_id = active_id.clone();
                        }
                    }

                    let mut span_name = strip_quotes(
                        obj.get("args")
                            .and_then(|a| a.get("span_name"))
                            .and_then(|s| s.as_str())
                            .unwrap_or(""),
                    );

                    if (span_name == "run" || span_name == "task") && !module.is_empty() {
                        let segments: Vec<&str> = module.split("::").collect();
                        if let Some(target_segment) =
                            segments.iter().rev().find(|&&s| is_root_segment(s))
                        {
                            span_name = target_segment.to_string();
                        }
                    }

                    let event_pid = obj.get("pid").and_then(|p| p.as_i64()).unwrap_or(1);
                    let event_tid = obj.get("tid").and_then(|t| t.as_i64()).unwrap_or(1);

                    name_map.insert(span_id.clone(), span_name.clone());
                    parent_map.insert(span_id.clone(), parent_id.clone());
                    if !module.is_empty() {
                        module_map.insert(span_id.clone(), module.clone());
                    }
                    start_time_map.insert(
                        span_id.clone(),
                        obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0)),
                    );
                    pid_map.insert(span_id.clone(), event_pid);
                    tid_map.insert(span_id.clone(), event_tid);

                    let is_root = is_root_or_empty_id(&parent_id);
                    if is_root {
                        while let Some(exited_id) = global_active_spans.pop() {
                            if generic_task_spans.contains(&exited_id) {
                                continue;
                            }

                            let name = name_map
                                .get(&exited_id)
                                .cloned()
                                .unwrap_or_else(|| "unknown".to_string());
                            let start_ts =
                                start_time_map.get(&exited_id).cloned().unwrap_or_else(|| {
                                    obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0))
                                });
                            let exit_pid = pid_map.get(&exited_id).cloned().unwrap_or(event_pid);
                            let exit_tid = tid_map.get(&exited_id).cloned().unwrap_or(event_tid);
                            let implicit_exit = serde_json::json!({
                                "cat": "device",
                                "ph": "E",
                                "name": name,
                                "ts": start_ts,
                                "pid": exit_pid,
                                "tid": exit_tid,
                                "args": {
                                    "implicit": true
                                }
                            });
                            processed_events.push(implicit_exit);
                        }
                    }

                    global_active_spans.push(span_id.clone());

                    if span_name == "run" || span_name == "task" {
                        generic_task_spans.insert(span_id.clone());
                        keep_event = false;
                    }

                    final_event["cat"] = serde_json::Value::from("device");
                    final_event["ph"] = serde_json::Value::from("B");
                    final_event["name"] = serde_json::Value::from(span_name);
                }
                "device_span_exit" => {
                    let span_id = strip_quotes(
                        obj.get("args")
                            .and_then(|a| a.get("span_id"))
                            .and_then(|s| s.as_str())
                            .unwrap_or(""),
                    );
                    let target_name = strip_quotes(
                        obj.get("args")
                            .and_then(|a| a.get("span_name"))
                            .and_then(|s| s.as_str())
                            .unwrap_or(""),
                    );

                    let mut resolved_span_id = span_id.clone();

                    if !target_name.is_empty() {
                        if let Some(pos) = global_active_spans.iter().rposition(|id| {
                            if let Some(n) = name_map.get(id) {
                                is_span_name_match(n, &target_name)
                            } else {
                                false
                            }
                        }) {
                            resolved_span_id = global_active_spans[pos].clone();
                        }
                    }

                    let span_name = name_map.get(&resolved_span_id).cloned().unwrap_or_else(|| {
                        if !target_name.is_empty() {
                            target_name.clone()
                        } else {
                            "unknown".to_string()
                        }
                    });

                    let is_generic = generic_task_spans.remove(&resolved_span_id);
                    if is_generic {
                        keep_event = false;
                    }

                    let ts = obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0));
                    final_event["cat"] = serde_json::Value::from("device");
                    final_event["ph"] = serde_json::Value::from("E");
                    final_event["name"] = serde_json::Value::from(span_name);

                    if let Some(pos) = global_active_spans
                        .iter()
                        .position(|x| x == &resolved_span_id)
                    {
                        while global_active_spans.len() > pos + 1 {
                            if let Some(exited_id) = global_active_spans.pop() {
                                if generic_task_spans.contains(&exited_id) {
                                    continue;
                                }

                                let name = name_map
                                    .get(&exited_id)
                                    .cloned()
                                    .unwrap_or_else(|| "unknown".to_string());
                                let start_ts = start_time_map
                                    .get(&exited_id)
                                    .cloned()
                                    .unwrap_or_else(|| ts.clone());
                                let exit_pid = pid_map.get(&exited_id).cloned().unwrap_or(1);
                                let exit_tid = tid_map.get(&exited_id).cloned().unwrap_or(1);
                                let implicit_exit = serde_json::json!({
                                    "cat": "device",
                                    "ph": "E",
                                    "name": name,
                                    "ts": start_ts,
                                    "pid": exit_pid,
                                    "tid": exit_tid,
                                    "args": {
                                        "implicit": true
                                    }
                                });
                                processed_events.push(implicit_exit);
                            }
                        }
                        global_active_spans.pop();
                    }
                }
                "device_log" => {
                    let msg = strip_quotes(
                        obj.get("args")
                            .and_then(|args| args.get("message").or_else(|| args.get("val")))
                            .and_then(|v| v.as_str())
                            .unwrap_or(""),
                    );

                    final_event["name"] =
                        serde_json::Value::from(if msg.is_empty() { "log" } else { &msg });
                }
                _ => {}
            }
        }

        if keep_event {
            processed_events.push(final_event);
        }
    }

    processed_events
}

/// Stage 3: Serializes processed events back to trace file.
fn save_processed_trace(
    path: &str,
    processed_events: Vec<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize back to file
    let serialized = serde_json::to_string_pretty(&processed_events)?;
    fs::write(path, serialized)?;
    Ok(())
}

/// A structured representation of a parsed target defmt tracing line.
pub struct ParsedTracingLine {
    /// True if this represents a span entry, false for a span exit.
    pub is_enter: bool,
    /// The unique identifier of the active span.
    pub span_id: String,
    /// The parent span identifier (only populated for enters, empty otherwise).
    pub parent_id: String,
    /// The cleaned name of the span (with quotes stripped).
    pub span_name: String,
}

impl ParsedTracingLine {
    /// Parses a raw defmt log line into a structured tracing line if it represents
    /// a span transition (enter or exit).
    pub fn parse(line: &str) -> Option<Self> {
        let is_enter = line.contains(" span_enter: ");
        let is_exit = line.contains(" span_exit: ");
        if !is_enter && !is_exit {
            return None;
        }

        let keyword = if is_enter {
            " span_enter: "
        } else {
            " span_exit: "
        };
        let keyword_pos = line.find(keyword)?;

        // Extract and clean the span name
        let raw_name = &line[keyword_pos + keyword.len()..];
        let mut span_name = raw_name.trim();
        if span_name.starts_with('"') && span_name.ends_with('"') && span_name.len() >= 2 {
            span_name = &span_name[1..span_name.len() - 1];
        }
        let span_name = span_name.to_string();

        // Extract the prefix part before the keyword
        let prefix = &line[..keyword_pos];

        // Isolate the span description payload (skip metadata prefix like timestamps and [module])
        let payload = if let Some(last_bracket) = prefix.rfind(']') {
            prefix[last_bracket + 1..].trim()
        } else {
            prefix.trim()
        };

        // Extract raw IDs (strip ctx= prefix if present)
        let raw_ids = if payload.starts_with("ctx=") {
            &payload[4..]
        } else {
            payload
        };

        // Get the first part of the raw_ids (separated by spaces, e.g. parent= is separated by space)
        let id_part = raw_ids.split(' ').next().unwrap_or(raw_ids);

        // Get the last ID in the colon-separated list of active contexts
        let ids: Vec<&str> = id_part.split(':').collect();
        let span_id = ids.last().unwrap_or(&id_part).to_string();

        if span_id.is_empty() {
            return None;
        }

        // Parent ID is the second-to-last ID in the list, fallback to parent= if list is 1 item
        let parent_id = if ids.len() >= 2 {
            ids[ids.len() - 2].to_string()
        } else if let Some(parent_pos) = payload.find("parent=") {
            let p_part = &payload[parent_pos + 7..];
            p_part.split(' ').next().unwrap_or(p_part).to_string()
        } else {
            String::new()
        };

        Some(Self {
            is_enter,
            span_id,
            parent_id,
            span_name,
        })
    }
}

/// Parses target defmt log prefix frames to intercept span enters and exits.
///
/// Log lines formatted as:
/// - `{time} TRACE [module] ctx={ids} parent={ids} span_enter: {name}`
/// - `{time} TRACE [module] {ids} parent={ids} span_exit: {name}`
///
/// Are parsed, converted to standard host `device_span_enter` and `device_span_exit`
/// telemetry events, and forwarded to the host tracing pipeline.
pub fn handle_tracing_line(line: &str, module: Option<&str>) -> Result<bool, &'static str> {
    if let Some(parsed) = ParsedTracingLine::parse(line) {
        if parsed.is_enter {
            tracing::info!(
                target: "device_span_enter",
                span_name = parsed.span_name,
                span_id = parsed.span_id,
                parent_id = parsed.parent_id,
                module = module.unwrap_or("")
            );
        } else {
            tracing::info!(
                target: "device_span_exit",
                span_id = parsed.span_id,
                span_name = parsed.span_name
            );
        }
        Ok(true)
    } else {
        Ok(false)
    }
}
