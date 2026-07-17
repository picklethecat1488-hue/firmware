use std::fs;
use tracing_subscriber::prelude::*;

/// A builder for constructing Chrome Trace Event JSON objects.
///
/// Follows the standard Chrome Trace Event Format specification, enabling fluent construction
/// of events with custom categories, phases, timestamps, and process/thread identifiers.
pub struct ChromeTraceEventBuilder {
    event: serde_json::Map<String, serde_json::Value>,
}

impl ChromeTraceEventBuilder {
    /// Creates a new, empty trace event builder.
    pub fn new() -> Self {
        Self {
            event: serde_json::Map::new(),
        }
    }

    /// Sets the event category (e.g., "device" or "device_log").
    pub fn category(mut self, cat: &str) -> Self {
        self.event
            .insert("cat".to_string(), serde_json::Value::from(cat));
        self
    }

    /// Sets the event phase (e.g., "B" for Begin, "E" for End, "M" for Metadata).
    pub fn phase(mut self, ph: &str) -> Self {
        self.event
            .insert("ph".to_string(), serde_json::Value::from(ph));
        self
    }

    /// Sets the event name (e.g., the span name or log message).
    pub fn name(mut self, name: &str) -> Self {
        self.event
            .insert("name".to_string(), serde_json::Value::from(name));
        self
    }

    /// Sets the host-side timestamp value in microseconds.
    pub fn timestamp(mut self, ts: serde_json::Value) -> Self {
        self.event.insert("ts".to_string(), ts);
        self
    }

    /// Sets the Process Identifier (PID).
    pub fn pid(mut self, pid: i64) -> Self {
        self.event
            .insert("pid".to_string(), serde_json::Value::from(pid));
        self
    }

    /// Sets the Thread Identifier (TID). In our post-processing, this maps to a virtual thread.
    pub fn tid(mut self, tid: i64) -> Self {
        self.event
            .insert("tid".to_string(), serde_json::Value::from(tid));
        self
    }

    /// Inserts a key-value argument into the event's "args" dictionary.
    pub fn arg(mut self, key: &str, value: serde_json::Value) -> Self {
        let args = self
            .event
            .entry("args".to_string())
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
        if let Some(args_obj) = args.as_object_mut() {
            args_obj.insert(key.to_string(), value);
        }
        self
    }

    /// Consumes the builder and returns the constructed `serde_json::Value`.
    pub fn build(self) -> serde_json::Value {
        serde_json::Value::Object(self.event)
    }
}

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

/// Extracts a dynamic, human-readable grouping/thread name from a nested Rust module path.
///
/// It splits the module path by colons (`::`) and retrieves the second segment
/// (e.g., `"battery_controller"` from `"controller::battery_controller"` or `"max17048"` from `"peripherals::max17048"`).
/// This allows the host CLI to remain project-agnostic while still grouping traces logically.
fn get_group_name_from_module(module_path: &str) -> String {
    module_path
        .split("::")
        .nth(1)
        .or_else(|| module_path.split("::").next())
        .filter(|s| !s.is_empty())
        .unwrap_or("unknown")
        .to_string()
}

/// Helper to check if a span ID represents a root or invalid context.
fn is_root_or_empty_id(id: &str) -> bool {
    id.is_empty() || id == "0000000000000000" || id == "0"
}

/// Traces a span ID up the `parent_map` hierarchy to locate the root span.
///
/// Returns a tuple containing the root span's compile-time module path (extracted from ELF)
/// and the root span's string name. Includes safety guards to prevent infinite loops if
/// cyclical dependencies are somehow logged.
fn find_root_module_and_name(
    span_id: &str,
    parent_map: &std::collections::HashMap<String, String>,
    module_map: &std::collections::HashMap<String, String>,
    name_map: &std::collections::HashMap<String, String>,
) -> (String, String) {
    let mut current_id = span_id.to_string();

    for _ in 0..1000 {
        if let Some(parent) = parent_map.get(&current_id) {
            let is_invalid = is_root_or_empty_id(parent)
                || parent == &current_id
                || !name_map.contains_key(parent);
            if is_invalid {
                break;
            }
            current_id = parent.clone();
        } else {
            break;
        }
    }

    let root_span_name = name_map
        .get(&current_id)
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());
    let root_module = module_map.get(&current_id).cloned().unwrap_or_default();
    (root_module, root_span_name)
}

/// Helper to determine if an active span name (which might have namespace prefixes and parameters)
/// matches the exiting target name.
fn is_span_name_match(active_name: &str, exit_target_name: &str) -> bool {
    let base_active = active_name.split('(').next().unwrap_or(active_name).trim();
    base_active == exit_target_name || base_active.ends_with(&format!("::{}", exit_target_name))
}

/// Traces a span ID to its root and returns the logical controller/thread name for it.
fn get_controller_name(
    span_id: &str,
    parent_map: &std::collections::HashMap<String, String>,
    module_map: &std::collections::HashMap<String, String>,
    name_map: &std::collections::HashMap<String, String>,
) -> String {
    let (root_module, root_span_name) =
        find_root_module_and_name(span_id, parent_map, module_map, name_map);

    if root_span_name.contains("::") {
        root_span_name
            .split("::")
            .next()
            .unwrap_or(&root_span_name)
            .to_string()
    } else if !root_module.is_empty() {
        get_group_name_from_module(&root_module)
    } else {
        root_span_name
    }
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
    let mut parent_map = std::collections::HashMap::new();
    let mut module_map = std::collections::HashMap::new();
    let mut start_time_map = std::collections::HashMap::new();
    let mut active_spans_map: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let mut global_active_spans: Vec<String> = Vec::new();

    // Reconstruct spans and group logs into timelines
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
                    let module = strip_quotes(
                        obj.get("args")
                            .and_then(|a| a.get("module"))
                            .and_then(|s| s.as_str())
                            .unwrap_or(""),
                    );

                    if span_name == "run" && !module.is_empty() {
                        if let Some(last_segment) = module.split("::").last() {
                            span_name = last_segment.to_string();
                        }
                    }

                    name_map.insert(span_id.clone(), span_name.clone());
                    parent_map.insert(span_id.clone(), parent_id.clone());
                    if !module.is_empty() {
                        module_map.insert(span_id.clone(), module.clone());
                    }
                    start_time_map.insert(
                        span_id.clone(),
                        obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0)),
                    );

                    let controller_name =
                        get_controller_name(&span_id, &parent_map, &module_map, &name_map);

                    let vt_id = *thread_map
                        .entry(controller_name.clone())
                        .or_insert_with(|| {
                            let id = next_tid;
                            next_tid += 1;
                            id
                        });

                    // If it is a root span (no parent), auto-close any remaining active spans on this controller's thread
                    // to prevent RTT frame drops from stretching their duration. We close them using their original
                    // start timestamps, collapsing them to 0-duration events on the timeline.
                    let is_root = is_root_or_empty_id(&parent_id);
                    if is_root {
                        let active = active_spans_map.entry(controller_name.clone()).or_default();
                        while let Some(exited_id) = active.pop() {
                            let name = name_map
                                .get(&exited_id)
                                .cloned()
                                .unwrap_or_else(|| "unknown".to_string());
                            let start_ts =
                                start_time_map.get(&exited_id).cloned().unwrap_or_else(|| {
                                    obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0))
                                });
                            let implicit_exit = ChromeTraceEventBuilder::new()
                                .category("device")
                                .phase("E")
                                .name(&name)
                                .timestamp(start_ts)
                                .pid(pid)
                                .tid(vt_id)
                                .arg("implicit", serde_json::Value::from(true))
                                .build();
                            processed_events.push(implicit_exit);

                            if let Some(pos) =
                                global_active_spans.iter().position(|x| x == &exited_id)
                            {
                                global_active_spans.remove(pos);
                            }
                        }
                    }

                    active_spans_map
                        .entry(controller_name)
                        .or_default()
                        .push(span_id.clone());
                    global_active_spans.push(span_id.clone());

                    // Convert this log event into a standard Chrome Trace "B" (Begin) event
                    let ts = obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0));
                    final_event = ChromeTraceEventBuilder::new()
                        .category("device")
                        .phase("B")
                        .name(&span_name)
                        .timestamp(ts)
                        .pid(pid)
                        .tid(vt_id)
                        .build();
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

                    let controller_name =
                        get_controller_name(&span_id, &parent_map, &module_map, &name_map);

                    let mut resolved_span_id = span_id.clone();
                    let mut resolved_controller = controller_name.clone();

                    if !target_name.is_empty() {
                        // First, search the local controller's active stack
                        let active = active_spans_map.entry(controller_name.clone()).or_default();
                        if let Some(pos) = active.iter().rposition(|id| {
                            if let Some(n) = name_map.get(id) {
                                is_span_name_match(n, &target_name)
                            } else {
                                false
                            }
                        }) {
                            resolved_span_id = active[pos].clone();
                        } else if controller_name == "unknown" || is_root_or_empty_id(&span_id) {
                            // If not found (often because the context reverted to 0/unknown on root span exit),
                            // scan the active stacks of other controllers to route this exit event correctly.
                            for (c_name, active_list) in active_spans_map.iter() {
                                if let Some(pos) = active_list.iter().rposition(|id| {
                                    if let Some(n) = name_map.get(id) {
                                        is_span_name_match(n, &target_name)
                                    } else {
                                        false
                                    }
                                }) {
                                    resolved_span_id = active_list[pos].clone();
                                    resolved_controller = c_name.clone();
                                    break;
                                }
                            }
                        }
                    }

                    let active = active_spans_map
                        .entry(resolved_controller.clone())
                        .or_default();

                    let span_name = name_map.get(&resolved_span_id).cloned().unwrap_or_else(|| {
                        if !target_name.is_empty() {
                            target_name.clone()
                        } else {
                            "unknown".to_string()
                        }
                    });

                    let vt_id = *thread_map
                        .entry(resolved_controller.clone())
                        .or_insert_with(|| {
                            let id = next_tid;
                            next_tid += 1;
                            id
                        });

                    // Convert this log event into a standard Chrome Trace "E" (End) event
                    let ts = obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0));
                    final_event = ChromeTraceEventBuilder::new()
                        .category("device")
                        .phase("E")
                        .name(&span_name)
                        .timestamp(ts.clone())
                        .pid(pid)
                        .tid(vt_id)
                        .build();

                    if let Some(pos) = active.iter().position(|x| x == &resolved_span_id) {
                        while active.len() > pos + 1 {
                            if let Some(exited_id) = active.pop() {
                                let name = name_map
                                    .get(&exited_id)
                                    .cloned()
                                    .unwrap_or_else(|| "unknown".to_string());
                                let implicit_exit = ChromeTraceEventBuilder::new()
                                    .category("device")
                                    .phase("E")
                                    .name(&name)
                                    .timestamp(ts.clone())
                                    .pid(pid)
                                    .tid(vt_id)
                                    .arg("implicit", serde_json::Value::from(true))
                                    .build();
                                processed_events.push(implicit_exit);

                                if let Some(g_pos) =
                                    global_active_spans.iter().position(|x| x == &exited_id)
                                {
                                    global_active_spans.remove(g_pos);
                                }
                            }
                        }
                        active.pop();
                    }
                    if let Some(pos) = global_active_spans
                        .iter()
                        .position(|x| x == &resolved_span_id)
                    {
                        global_active_spans.remove(pos);
                    }
                }
                "device_log" => {
                    let msg = strip_quotes(
                        obj.get("args")
                            .and_then(|args| args.get("message").or_else(|| args.get("val")))
                            .and_then(|v| v.as_str())
                            .unwrap_or(""),
                    );

                    let mut builder = ChromeTraceEventBuilder::new()
                        .category("device_log")
                        .phase(obj.get("ph").and_then(|p| p.as_str()).unwrap_or("i"))
                        .name(if msg.is_empty() { "log" } else { &msg })
                        .timestamp(obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0)))
                        .pid(pid);

                    if let Some(active_id) = global_active_spans.last() {
                        let controller_name =
                            get_controller_name(active_id, &parent_map, &module_map, &name_map);
                        let vt_id = *thread_map.entry(controller_name).or_insert_with(|| {
                            let id = next_tid;
                            next_tid += 1;
                            id
                        });
                        builder = builder.tid(vt_id);
                    } else {
                        builder = builder.tid(obj.get("tid").and_then(|t| t.as_i64()).unwrap_or(1));
                    }
                    final_event = builder.build();
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
            processed_events.push(final_event);
        }
    }

    // Add metadata events to define thread names dynamically from the root span names
    for (name, tid) in thread_map {
        let meta = ChromeTraceEventBuilder::new()
            .name("thread_name")
            .phase("M")
            .pid(pid)
            .tid(tid)
            .arg("name", serde_json::Value::from(name))
            .build();

        processed_events.insert(0, meta);
    }

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
