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
    s == "run"
        || s == "task"
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

/// Pipeline stage that classifies events into CPU cores based on prefixes and metadata.
pub struct CoreClassifier;

impl TraceStage for CoreClassifier {
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
                        let mut span_name = obj
                            .get("span_name")
                            .and_then(|s| s.as_str())
                            .unwrap_or("")
                            .to_string();

                        let mut core = None;
                        if span_name.starts_with("Core 1: ") {
                            core = Some(1);
                            span_name = span_name["Core 1: ".len()..].to_string();
                        } else if span_name.starts_with("Core 0: ") {
                            core = Some(0);
                            span_name = span_name["Core 0: ".len()..].to_string();
                        }

                        if let Some(c) = core {
                            obj.insert("core".to_string(), serde_json::json!(c));
                        }
                        obj.insert("span_name".to_string(), serde_json::Value::from(span_name));
                    }
                    TraceCategory::Log => {
                        let mut msg = obj
                            .get("msg")
                            .and_then(|s| s.as_str())
                            .unwrap_or("")
                            .to_string();

                        let mut core = None;
                        if msg.starts_with("Core 1: ") {
                            core = Some(1);
                            msg = msg["Core 1: ".len()..].to_string();
                        } else if msg.starts_with("Core 0: ") {
                            core = Some(0);
                            msg = msg["Core 0: ".len()..].to_string();
                        }

                        if let Some(c) = core {
                            obj.insert("core".to_string(), serde_json::json!(c));
                        }
                        obj.insert("msg".to_string(), serde_json::Value::from(msg));
                    }
                    _ => {}
                }
            }
        }
        Ok(events)
    }
}

/// Detailed attributes of a tracked span.
#[derive(Clone, Debug)]
pub struct SpanInfo {
    /// Cleaned name of the span.
    pub name: String,
    /// Parent span ID.
    pub parent_id: String,
    /// Rust module namespace path where it was defined.
    pub module: String,
    /// Start timestamp value.
    pub start_time: serde_json::Value,
    /// Telemetry process identifier.
    pub pid: i64,
    /// Telemetry thread identifier.
    pub tid: i64,
    /// Parent task context name.
    pub task: String,
}

/// Trait for looking up attributes of a span.
pub trait SpanLookup {
    /// Returns the cleaned name of the span.
    fn get_name(&self, span_id: &str) -> Option<String>;
    /// Returns the parent span ID.
    fn get_parent_id(&self, span_id: &str) -> Option<String>;
    /// Returns the module namespace.
    fn get_module(&self, span_id: &str) -> Option<String>;
    /// Returns the start time.
    fn get_start_time(&self, span_id: &str) -> Option<serde_json::Value>;
    /// Returns the PID.
    fn get_pid(&self, span_id: &str) -> Option<i64>;
    /// Returns the TID.
    fn get_tid(&self, span_id: &str) -> Option<i64>;
    /// Returns the task context name.
    fn get_task(&self, span_id: &str) -> Option<String>;
}

/// State container tracking the currently active task context.
#[derive(Clone, Debug, Default)]
pub struct ActiveTaskState {
    /// Name of the currently active task context.
    pub current: Option<String>,
}

/// Context data containing all tracked spans state.
pub struct SpanContext {
    /// Database of all tracked spans, keyed by span ID.
    pub spans: std::collections::HashMap<String, SpanInfo>,
    /// Maps task context name to its stack of active span IDs.
    pub active_spans_map: std::collections::HashMap<String, Vec<String>>,
    /// Tracks the globally most recently active span ID for parent fallback.
    pub global_last_active_span: Option<String>,
    /// Tracks the currently active task context for cooperative context switching.
    pub active_task: ActiveTaskState,
}

impl SpanContext {
    /// Creates a new empty context.
    pub fn new() -> Self {
        Self {
            spans: std::collections::HashMap::new(),
            active_spans_map: std::collections::HashMap::new(),
            global_last_active_span: None,
            active_task: ActiveTaskState::default(),
        }
    }
}

impl SpanLookup for SpanContext {
    fn get_name(&self, span_id: &str) -> Option<String> {
        self.spans.get(span_id).map(|s| s.name.clone())
    }
    fn get_parent_id(&self, span_id: &str) -> Option<String> {
        self.spans.get(span_id).map(|s| s.parent_id.clone())
    }
    fn get_module(&self, span_id: &str) -> Option<String> {
        self.spans.get(span_id).map(|s| s.module.clone())
    }
    fn get_start_time(&self, span_id: &str) -> Option<serde_json::Value> {
        self.spans.get(span_id).map(|s| s.start_time.clone())
    }
    fn get_pid(&self, span_id: &str) -> Option<i64> {
        self.spans.get(span_id).map(|s| s.pid)
    }
    fn get_tid(&self, span_id: &str) -> Option<i64> {
        self.spans.get(span_id).map(|s| s.tid)
    }
    fn get_task(&self, span_id: &str) -> Option<String> {
        self.spans.get(span_id).map(|s| s.task.clone())
    }
}

/// Trait for managing cooperative task context suspension and resumption.
pub trait TaskContext {
    /// Returns the currently active task context name.
    fn current_task(&self) -> Option<&str>;
    /// Sets the currently active task context name.
    fn set_current_task(&mut self, task: Option<String>);
    /// Suspends active spans for the given task context.
    fn suspend(
        &self,
        task_name: &str,
        ts: &serde_json::Value,
        processed_events: &mut Vec<serde_json::Value>,
    );
    /// Resumes active spans for the given task context.
    fn resume(
        &self,
        task_name: &str,
        ts: &serde_json::Value,
        processed_events: &mut Vec<serde_json::Value>,
    );

    /// Performs cooperative context switching to a new target task.
    fn switch_context(
        &mut self,
        target_task: &str,
        ts: &serde_json::Value,
        processed_events: &mut Vec<serde_json::Value>,
    ) {
        let mut switched = false;
        if let Some(prev_task) = self.current_task() {
            if prev_task != target_task {
                self.suspend(prev_task, ts, processed_events);
                switched = true;
            }
        }
        if switched || self.current_task().is_none() {
            self.resume(target_task, ts, processed_events);
        }
        self.set_current_task(Some(target_task.to_string()));
    }
}

impl TaskContext for SpanContext {
    fn current_task(&self) -> Option<&str> {
        self.active_task.current.as_deref()
    }

    fn set_current_task(&mut self, task: Option<String>) {
        self.active_task.current = task;
    }

    fn suspend(
        &self,
        task_name: &str,
        ts: &serde_json::Value,
        processed_events: &mut Vec<serde_json::Value>,
    ) {
        if let Some(active) = self.active_spans_map.get(task_name) {
            for exited_id in active.iter().rev() {
                let name = self
                    .get_name(exited_id)
                    .unwrap_or_else(|| "unknown".to_string());
                let exit_pid = self.get_pid(exited_id).unwrap_or(1);
                let exit_tid = self.get_tid(exited_id).unwrap_or(0);
                let implicit_exit = serde_json::json!({
                    "cat": "device",
                    "ph": "E",
                    "name": name,
                    "ts": ts.clone(),
                    "pid": exit_pid,
                    "tid": exit_tid,
                    "args": {
                        "suspended": true
                    }
                });
                processed_events.push(implicit_exit);
            }
        }
    }

    fn resume(
        &self,
        task_name: &str,
        ts: &serde_json::Value,
        processed_events: &mut Vec<serde_json::Value>,
    ) {
        if let Some(active) = self.active_spans_map.get(task_name) {
            for resumed_id in active.iter() {
                let name = self
                    .get_name(resumed_id)
                    .unwrap_or_else(|| "unknown".to_string());
                let resume_pid = self.get_pid(resumed_id).unwrap_or(1);
                let resume_tid = self.get_tid(resumed_id).unwrap_or(0);
                let implicit_enter = serde_json::json!({
                    "cat": "device",
                    "ph": "B",
                    "name": name,
                    "ts": ts.clone(),
                    "pid": resume_pid,
                    "tid": resume_tid,
                    "args": {
                        "resumed": true
                    }
                });
                processed_events.push(implicit_enter);
            }
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

        let mut span_name = obj
            .get("span_name")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();

        if (span_name == "run" || span_name == "task") && !module.is_empty() {
            let segments: Vec<&str> = module.split("::").collect();
            if let Some(target_segment) = segments.iter().rev().find(|&&s| is_root_segment(s)) {
                span_name = target_segment.to_string();
            }
        }

        let event_pid = obj.get("pid").and_then(|p| p.as_i64()).unwrap_or(1);

        // Determine the task context for this new span, using module path for uniqueness if available
        let task_context = if !is_root_or_empty_id(&parent_id) {
            context.get_task(&parent_id).unwrap_or_else(|| {
                if !module.is_empty() {
                    module.to_string()
                } else {
                    span_name.clone()
                }
            })
        } else {
            if !module.is_empty() {
                module.to_string()
            } else {
                span_name.clone()
            }
        };

        let core = obj.get("core").and_then(|c| c.as_i64());
        let event_tid = if let Some(c) = core {
            c
        } else if !is_root_or_empty_id(&parent_id) {
            context.get_tid(&parent_id).unwrap_or(0)
        } else {
            0
        };

        // Populate and insert the unified SpanInfo database entry
        context.spans.insert(
            span_id.to_string(),
            SpanInfo {
                name: span_name.clone(),
                parent_id: parent_id.clone(),
                module: module.to_string(),
                start_time: obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0)),
                pid: event_pid,
                tid: event_tid,
                task: task_context.clone(),
            },
        );

        let ts = obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0));

        // Cooperative Context Switch
        context.switch_context(&task_context, &ts, processed_events);

        // Remove active stack temporarily to resolve borrow checker conflict when calling context.get_xxx
        let mut active = context
            .active_spans_map
            .remove(&task_context)
            .unwrap_or_default();

        let is_root = is_root_or_empty_id(&parent_id);
        if is_root {
            while let Some(exited_id) = active.pop() {
                let name = context
                    .get_name(&exited_id)
                    .unwrap_or_else(|| "unknown".to_string());
                let exit_pid = context.get_pid(&exited_id).unwrap_or(event_pid);
                let exit_tid = context.get_tid(&exited_id).unwrap_or(event_tid);
                let implicit_exit = serde_json::json!({
                    "cat": "device",
                    "ph": "E",
                    "name": name,
                    "ts": ts.clone(),
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
        context.active_spans_map.insert(task_context, active);
        context.global_last_active_span = Some(span_id.to_string());

        final_event["cat"] = serde_json::Value::from("device");
        final_event["ph"] = serde_json::Value::from("B");
        final_event["name"] = serde_json::Value::from(span_name);
        final_event["tid"] = serde_json::Value::from(event_tid);
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
    ) -> bool {
        let span_id = obj.get("span_id").and_then(|s| s.as_str()).unwrap_or("");
        let target_name = obj
            .get("target_name")
            .and_then(|s| s.as_str())
            .unwrap_or("");

        let mut resolved_span_id = span_id.to_string();
        let mut task_context = context.get_task(&resolved_span_id).unwrap_or_default();

        if task_context.is_empty() && !target_name.is_empty() {
            for (t_name, active) in &context.active_spans_map {
                if let Some(pos) = active.iter().rposition(|id| {
                    if let Some(n) = context.get_name(id) {
                        is_span_name_match(&n, &target_name)
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

        let mut keep = true;
        if !task_context.is_empty() {
            if let Some(active) = context.active_spans_map.get(&task_context) {
                if !active.iter().any(|x| x == &resolved_span_id) {
                    keep = false;
                }
            } else {
                keep = false;
            }
        }

        if !keep {
            return false;
        }

        let span_name = context.get_name(&resolved_span_id).unwrap_or_else(|| {
            if !target_name.is_empty() {
                target_name.to_string()
            } else {
                "unknown".to_string()
            }
        });

        let ts = obj.get("ts").cloned().unwrap_or(serde_json::Value::from(0));

        // Cooperative Context Switch
        context.switch_context(&task_context, &ts, processed_events);

        final_event["cat"] = serde_json::Value::from("device");
        final_event["ph"] = serde_json::Value::from("E");
        final_event["name"] = serde_json::Value::from(span_name);
        if let Some(t_tid) = context.get_tid(&resolved_span_id) {
            final_event["tid"] = serde_json::Value::from(t_tid);
        }

        // Pop the span and any children that exited implicitly from active stack
        if !task_context.is_empty() {
            if let Some(mut active) = context.active_spans_map.remove(&task_context) {
                if let Some(pos) = active.iter().position(|x| x == &resolved_span_id) {
                    while active.len() > pos + 1 {
                        if let Some(exited_id) = active.pop() {
                            let name = context
                                .get_name(&exited_id)
                                .unwrap_or_else(|| "unknown".to_string());
                            let exit_pid = context.get_pid(&exited_id).unwrap_or(1);
                            let exit_tid = context.get_tid(&exited_id).unwrap_or(1);
                            let implicit_exit = serde_json::json!({
                                "cat": "device",
                                "ph": "E",
                                "name": name,
                                "ts": ts.clone(),
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
                context
                    .active_spans_map
                    .insert(task_context.clone(), active);
            }
        }

        context.global_last_active_span = context.get_parent_id(&resolved_span_id);
        true
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
                        let keep = self.exit_processor.process(
                            &mut self.context,
                            obj,
                            &mut final_event,
                            &mut processed_events,
                        );
                        if !keep {
                            continue;
                        }
                    }
                    TraceCategory::Log => {
                        let msg = obj.get("msg").and_then(|m| m.as_str()).unwrap_or("");
                        if msg.is_empty() {
                            continue;
                        }

                        final_event["name"] = serde_json::Value::from(msg);
                        let core = obj.get("core").and_then(|c| c.as_i64());
                        let log_tid = if let Some(c) = core {
                            c
                        } else if let Some(ref last_id) = self.context.global_last_active_span {
                            self.context.get_tid(last_id).unwrap_or(0)
                        } else {
                            0
                        };
                        final_event["tid"] = serde_json::Value::from(log_tid);
                    }
                    TraceCategory::Other => {}
                }

                let is_mapped = match category {
                    TraceCategory::SpanEnter | TraceCategory::SpanExit | TraceCategory::Log => true,
                    TraceCategory::Other => false,
                };
                if !is_mapped
                    && final_event.get("tid").is_some()
                    && final_event["tid"] == serde_json::json!(1)
                {
                    final_event["tid"] = serde_json::json!(0);
                }
            }
            processed_events.push(final_event);
        }

        let mut final_events = Vec::new();
        final_events.push(serde_json::json!({
            "name": "thread_name",
            "ph": "M",
            "pid": 1,
            "tid": 0,
            "args": {
                "name": "CPU Core 0"
            }
        }));
        final_events.push(serde_json::json!({
            "name": "thread_name",
            "ph": "M",
            "pid": 1,
            "tid": 1,
            "args": {
                "name": "CPU Core 1"
            }
        }));
        final_events.extend(processed_events);

        Ok(final_events)
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
        .add_stage(CoreClassifier)
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

        // Split prefix into words to handle variable formatting with/without timestamps or module names
        let words: Vec<&str> = prefix.split_whitespace().collect();
        let mut start_idx = 0;
        if words.len() >= 2 {
            let second_word = words[1];
            if second_word == "TRACE"
                || second_word == "DEBUG"
                || second_word == "INFO"
                || second_word == "WARN"
                || second_word == "ERROR"
            {
                start_idx = 2;
            }
        }

        // Skip module bracket context if present (e.g. [module])
        if start_idx < words.len()
            && words[start_idx].starts_with('[')
            && words[start_idx].ends_with(']')
        {
            start_idx += 1;
        }

        if start_idx >= words.len() {
            return None;
        }

        let ctx_word = words[start_idx];
        let raw_ids = if ctx_word.starts_with("ctx=") {
            &ctx_word[4..]
        } else {
            ctx_word
        };

        let ids: Vec<&str> = raw_ids.split(':').collect();
        let span_id = ids.last().unwrap_or(&raw_ids).to_string();

        if span_id.is_empty() {
            return None;
        }

        let mut parent_id = String::new();
        for &w in &words[start_idx..] {
            if w.starts_with("parent=") {
                parent_id = w[7..].to_string();
                break;
            }
        }

        if parent_id.is_empty() && ids.len() >= 2 {
            parent_id = ids[ids.len() - 2].to_string();
        }

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
