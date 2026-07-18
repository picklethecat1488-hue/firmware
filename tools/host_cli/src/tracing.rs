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

/// Strongly-typed categories representing trace events generated on the target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceCategory {
    /// Telemetry marking a span enter transition.
    SpanEnter,
    /// Telemetry marking a span exit transition.
    SpanExit,
    /// Telemetry representing target logs.
    Log,
    /// Unknown or unhandled telemetry category.
    Other,
}

impl TraceCategory {
    /// Parsed string representation of target trace category into strongly-typed enum.
    pub fn parse(s: &str) -> Self {
        match s {
            "device_span_enter" => Self::SpanEnter,
            "device_span_exit" => Self::SpanExit,
            "device_log" => Self::Log,
            _ => Self::Other,
        }
    }
}

/// A processing stage within the tracing telemetry transformation pipeline.
pub trait TraceStage {
    /// Executes the transformation stage on the list of events.
    fn run(
        &mut self,
        events: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>>;
}

/// A pipeline of trace processing stages executed sequentially.
pub struct TracePipeline {
    stages: Vec<Box<dyn TraceStage>>,
}

impl TracePipeline {
    /// Creates a new empty trace processing pipeline.
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    /// Appends a processing stage to the pipeline.
    pub fn add_stage<S: TraceStage + 'static>(mut self, stage: S) -> Self {
        self.stages.push(Box::new(stage));
        self
    }

    /// Executes all registered pipeline stages on the trace events.
    pub fn execute(
        &mut self,
        mut events: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        for stage in &mut self.stages {
            events = stage.run(events)?;
        }
        Ok(events)
    }
}

/// Pipeline stage that sorts trace events chronologically by their microcontroller timestamps.
pub struct ChronologicalSorter;

impl TraceStage for ChronologicalSorter {
    fn run(
        &mut self,
        mut events: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
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
        Ok(events)
    }
}

/// Decoder strategy for span enter events.
pub struct SpanEnterDecoder;

impl SpanEnterDecoder {
    /// Decodes entry-specific trace telemetry arguments into the mutable JSON map.
    pub fn decode(&self, obj: &mut serde_json::Map<String, serde_json::Value>) {
        let span_id = strip_quotes(
            obj.get("args")
                .and_then(|a| a.get("span_id"))
                .and_then(|s| s.as_str())
                .unwrap_or(""),
        );
        let module = strip_quotes(
            obj.get("args")
                .and_then(|a| a.get("module"))
                .and_then(|s| s.as_str())
                .unwrap_or(""),
        );
        let span_name = strip_quotes(
            obj.get("args")
                .and_then(|a| a.get("span_name"))
                .and_then(|s| s.as_str())
                .unwrap_or(""),
        );

        obj.insert("span_id".to_string(), serde_json::Value::from(span_id));
        obj.insert("module".to_string(), serde_json::Value::from(module));
        obj.insert("span_name".to_string(), serde_json::Value::from(span_name));

        if let Some(p_val) = obj.get("args").and_then(|a| a.get("parent_id")) {
            let parent_id = strip_quotes(p_val.as_str().unwrap_or(""));
            obj.insert("parent_id".to_string(), serde_json::Value::from(parent_id));
        }
    }
}

/// Decoder strategy for span exit events.
pub struct SpanExitDecoder;

impl SpanExitDecoder {
    /// Decodes exit-specific trace telemetry arguments into the mutable JSON map.
    pub fn decode(&self, obj: &mut serde_json::Map<String, serde_json::Value>) {
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

        obj.insert("span_id".to_string(), serde_json::Value::from(span_id));
        obj.insert(
            "target_name".to_string(),
            serde_json::Value::from(target_name),
        );
    }
}

/// Decoder strategy for device logs.
pub struct DeviceLogDecoder;

impl DeviceLogDecoder {
    /// Decodes log-specific trace telemetry arguments into the mutable JSON map.
    pub fn decode(&self, obj: &mut serde_json::Map<String, serde_json::Value>) {
        let msg = strip_quotes(
            obj.get("args")
                .and_then(|args| args.get("message").or_else(|| args.get("val")))
                .and_then(|v| v.as_str())
                .unwrap_or(""),
        );
        obj.insert("msg".to_string(), serde_json::Value::from(msg));
    }
}

/// Pipeline stage that decodes raw JSON arguments into clean, top-level values on each event.
pub struct TelemetryDecoder {
    enter_decoder: SpanEnterDecoder,
    exit_decoder: SpanExitDecoder,
    log_decoder: DeviceLogDecoder,
}

impl TelemetryDecoder {
    /// Creates a new TelemetryDecoder with its inner strategies initialized.
    pub fn new() -> Self {
        Self {
            enter_decoder: SpanEnterDecoder,
            exit_decoder: SpanExitDecoder,
            log_decoder: DeviceLogDecoder,
        }
    }
}

impl TraceStage for TelemetryDecoder {
    fn run(
        &mut self,
        mut events: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        for event in &mut events {
            if let Some(obj) = event.as_object_mut() {
                let target = obj.get("cat").and_then(|c| c.as_str()).unwrap_or("");
                let category = TraceCategory::parse(target);
                match category {
                    TraceCategory::SpanEnter => {
                        self.enter_decoder.decode(obj);
                    }
                    TraceCategory::SpanExit => {
                        self.exit_decoder.decode(obj);
                    }
                    TraceCategory::Log => {
                        self.log_decoder.decode(obj);
                    }
                    TraceCategory::Other => {}
                }
            }
        }
        Ok(events)
    }
}

/// Context data containing all tracked spans state.
pub struct SpanContext {
    /// Maps span ID to the cleaned name of the span.
    pub name_map: std::collections::HashMap<String, String>,
    /// Maps span ID to its parent span ID.
    pub parent_map: std::collections::HashMap<String, String>,
    /// Maps span ID to the Rust module namespace path where it was defined.
    pub module_map: std::collections::HashMap<String, String>,
    /// Maps span ID to its start timestamp value.
    pub start_time_map: std::collections::HashMap<String, serde_json::Value>,
    /// Maps span ID to its telemetry process identifier.
    pub pid_map: std::collections::HashMap<String, i64>,
    /// Maps span ID to its telemetry thread identifier.
    pub tid_map: std::collections::HashMap<String, i64>,
    /// Maps task context name to its stack of active span IDs.
    pub active_spans_map: std::collections::HashMap<String, Vec<String>>,
    /// Maps span ID to its parent task context name.
    pub span_to_task: std::collections::HashMap<String, String>,
    /// Tracks the globally most recently active span ID for parent fallback.
    pub global_last_active_span: Option<String>,
}

impl SpanContext {
    /// Creates a new empty context.
    pub fn new() -> Self {
        Self {
            name_map: std::collections::HashMap::new(),
            parent_map: std::collections::HashMap::new(),
            module_map: std::collections::HashMap::new(),
            start_time_map: std::collections::HashMap::new(),
            pid_map: std::collections::HashMap::new(),
            tid_map: std::collections::HashMap::new(),
            active_spans_map: std::collections::HashMap::new(),
            span_to_task: std::collections::HashMap::new(),
            global_last_active_span: None,
        }
    }
}

/// Transition handler for span enter events.
pub struct SpanEnterProcessor;

impl SpanEnterProcessor {
    /// Processes a span enter event, mutating the SpanContext and returning whether to keep the event.
    pub fn process(
        &self,
        context: &mut SpanContext,
        obj: &serde_json::Map<String, serde_json::Value>,
        final_event: &mut serde_json::Value,
        processed_events: &mut Vec<serde_json::Value>,
    ) {
        let span_id = obj.get("span_id").and_then(|s| s.as_str()).unwrap_or("");
        let mut parent_id = obj
            .get("parent_id")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();
        let module = obj.get("module").and_then(|s| s.as_str()).unwrap_or("");

        if parent_id.is_empty() && obj.get("parent_id").is_none() {
            if let Some(active_id) = &context.global_last_active_span {
                parent_id = active_id.clone();
            }
        }

        let span_name = obj
            .get("span_name")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();

        let event_pid = obj.get("pid").and_then(|p| p.as_i64()).unwrap_or(1);
        let event_tid = obj.get("tid").and_then(|t| t.as_i64()).unwrap_or(1);

        context
            .name_map
            .insert(span_id.to_string(), span_name.clone());
        context
            .parent_map
            .insert(span_id.to_string(), parent_id.clone());
        if !module.is_empty() {
            context
                .module_map
                .insert(span_id.to_string(), module.to_string());
        }
        context.start_time_map.insert(
            span_id.to_string(),
            obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0)),
        );
        context.pid_map.insert(span_id.to_string(), event_pid);
        context.tid_map.insert(span_id.to_string(), event_tid);

        // Determine the task context for this new span
        let task_context = if !is_root_or_empty_id(&parent_id) {
            context
                .span_to_task
                .get(&parent_id)
                .cloned()
                .unwrap_or_else(|| span_name.clone())
        } else {
            span_name.clone()
        };
        context
            .span_to_task
            .insert(span_id.to_string(), task_context.clone());

        // Get the active spans stack for this task context
        let active = context.active_spans_map.entry(task_context).or_default();

        let is_root = is_root_or_empty_id(&parent_id);
        if is_root {
            while let Some(exited_id) = active.pop() {
                let name = context
                    .name_map
                    .get(&exited_id)
                    .cloned()
                    .unwrap_or_else(|| "unknown".to_string());
                let start_ts = context
                    .start_time_map
                    .get(&exited_id)
                    .cloned()
                    .unwrap_or_else(|| {
                        obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0))
                    });
                let exit_pid = context
                    .pid_map
                    .get(&exited_id)
                    .cloned()
                    .unwrap_or(event_pid);
                let exit_tid = context
                    .tid_map
                    .get(&exited_id)
                    .cloned()
                    .unwrap_or(event_tid);
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

        active.push(span_id.to_string());
        context.global_last_active_span = Some(span_id.to_string());

        final_event["cat"] = serde_json::Value::from("device");
        final_event["ph"] = serde_json::Value::from("B");
        final_event["name"] = serde_json::Value::from(span_name);
    }
}

/// Transition handler for span exit events.
pub struct SpanExitProcessor;

impl SpanExitProcessor {
    /// Processes a span exit event, mutating the SpanContext and returning whether to keep the event.
    pub fn process(
        &self,
        context: &mut SpanContext,
        obj: &serde_json::Map<String, serde_json::Value>,
        final_event: &mut serde_json::Value,
        processed_events: &mut Vec<serde_json::Value>,
    ) {
        let span_id = obj.get("span_id").and_then(|s| s.as_str()).unwrap_or("");
        let target_name = obj
            .get("target_name")
            .and_then(|s| s.as_str())
            .unwrap_or("");

        let mut resolved_span_id = span_id.to_string();
        let mut task_context = context
            .span_to_task
            .get(&resolved_span_id)
            .cloned()
            .unwrap_or_default();

        if task_context.is_empty() && !target_name.is_empty() {
            for (t_name, active) in &context.active_spans_map {
                if let Some(pos) = active.iter().rposition(|id| {
                    if let Some(n) = context.name_map.get(id) {
                        is_span_name_match(n, &target_name)
                    } else {
                        false
                    }
                }) {
                    resolved_span_id = active[pos].clone();
                    task_context = t_name.clone();
                    break;
                }
            }
        }

        let span_name = context
            .name_map
            .get(&resolved_span_id)
            .cloned()
            .unwrap_or_else(|| {
                if !target_name.is_empty() {
                    target_name.to_string()
                } else {
                    "unknown".to_string()
                }
            });

        let ts = obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0));
        final_event["cat"] = serde_json::Value::from("device");
        final_event["ph"] = serde_json::Value::from("E");
        final_event["name"] = serde_json::Value::from(span_name);

        if !task_context.is_empty() {
            if let Some(active) = context.active_spans_map.get_mut(&task_context) {
                if let Some(pos) = active.iter().position(|x| x == &resolved_span_id) {
                    while active.len() > pos + 1 {
                        if let Some(exited_id) = active.pop() {
                            let name = context
                                .name_map
                                .get(&exited_id)
                                .cloned()
                                .unwrap_or_else(|| "unknown".to_string());
                            let start_ts = context
                                .start_time_map
                                .get(&exited_id)
                                .cloned()
                                .unwrap_or_else(|| ts.clone());
                            let exit_pid = context.pid_map.get(&exited_id).cloned().unwrap_or(1);
                            let exit_tid = context.tid_map.get(&exited_id).cloned().unwrap_or(1);
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
                    active.pop();
                }
            }
        }

        context.global_last_active_span = context.parent_map.get(&resolved_span_id).cloned();
    }
}

/// Pipeline stage that processes spans (enter/exit transitions, implicit closures, and name mapping)
/// while maintaining tracing context state.
pub struct SpanProcessor {
    context: SpanContext,
    enter_processor: SpanEnterProcessor,
    exit_processor: SpanExitProcessor,
}

impl SpanProcessor {
    /// Creates a new span processor with empty tracing state.
    pub fn new() -> Self {
        Self {
            context: SpanContext::new(),
            enter_processor: SpanEnterProcessor,
            exit_processor: SpanExitProcessor,
        }
    }
}

impl TraceStage for SpanProcessor {
    fn run(
        &mut self,
        events: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let mut processed_events = Vec::new();

        for event in events {
            let mut final_event = event.clone();

            if let Some(obj) = event.as_object() {
                let target = obj.get("cat").and_then(|c| c.as_str()).unwrap_or("");
                let category = TraceCategory::parse(target);
                match category {
                    TraceCategory::SpanEnter => {
                        self.enter_processor.process(
                            &mut self.context,
                            obj,
                            &mut final_event,
                            &mut processed_events,
                        );
                    }
                    TraceCategory::SpanExit => {
                        self.exit_processor.process(
                            &mut self.context,
                            obj,
                            &mut final_event,
                            &mut processed_events,
                        );
                    }
                    TraceCategory::Log => {
                        let msg = obj.get("msg").and_then(|m| m.as_str()).unwrap_or("");
                        final_event["name"] =
                            serde_json::Value::from(if msg.is_empty() { "log" } else { msg });
                    }
                    TraceCategory::Other => {}
                }
            }
            processed_events.push(final_event);
        }

        Ok(processed_events)
    }
}

/// Serializes processed events back to the trace file path.
fn save_processed_trace(
    path: &str,
    processed_events: Vec<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let serialized = serde_json::to_string_pretty(&processed_events)?;
    fs::write(path, serialized)?;
    Ok(())
}

/// Post-processes the generated trace JSON file using a multi-stage transformation pipeline.
pub fn post_process_trace(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(path).exists() {
        return Ok(());
    }
    let content = fs::read_to_string(path)?;
    if content.trim().is_empty() {
        return Ok(());
    }

    let events: Vec<serde_json::Value> = serde_json::from_str(&content)?;

    let mut pipeline = TracePipeline::new()
        .add_stage(ChronologicalSorter)
        .add_stage(TelemetryDecoder::new())
        .add_stage(SpanProcessor::new());

    let processed_events = pipeline.execute(events)?;

    save_processed_trace(path, processed_events)?;

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
