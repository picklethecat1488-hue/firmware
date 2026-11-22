use std::fs;
use tracing_subscriber::prelude::*;

pub struct TraceEventRef<'a> {
    obj: &'a serde_json::Map<String, serde_json::Value>,
}

impl<'a> TraceEventRef<'a> {
    pub fn new(event: &'a serde_json::Value) -> Option<Self> {
        event.as_object().map(|obj| Self { obj })
    }

    pub fn from_map(obj: &'a serde_json::Map<String, serde_json::Value>) -> Self {
        Self { obj }
    }

    pub fn category(&self) -> TraceCategory {
        let cat_str = self.obj.get("cat").and_then(|c| c.as_str()).unwrap_or("");
        TraceCategory::parse(cat_str)
    }

    pub fn span_id(&self) -> String {
        self.obj
            .get("span_id")
            .or_else(|| self.obj.get("args").and_then(|a| a.get("span_id")))
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string()
    }

    pub fn ts(&self) -> f64 {
        self.obj.get("ts").and_then(|t| t.as_f64()).unwrap_or(0.0)
    }

    pub fn pid(&self) -> i64 {
        self.obj.get("pid").and_then(|p| p.as_i64()).unwrap_or(1)
    }

    pub fn tid(&self) -> i64 {
        self.obj.get("tid").and_then(|t| t.as_i64()).unwrap_or(0)
    }

    pub fn name(&self) -> String {
        self.obj
            .get("args")
            .and_then(|a| a.get("span_name"))
            .or_else(|| self.obj.get("span_name"))
            .or_else(|| self.obj.get("name"))
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string()
    }

    pub fn target_name(&self) -> String {
        self.obj
            .get("target_name")
            .or_else(|| self.obj.get("args").and_then(|a| a.get("span_name")))
            .or_else(|| self.obj.get("span_name"))
            .or_else(|| self.obj.get("name"))
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string()
    }

    pub fn msg(&self) -> String {
        self.obj
            .get("msg")
            .or_else(|| {
                self.obj.get("args").and_then(|a| {
                    a.get("message")
                        .or_else(|| a.get("msg"))
                        .or_else(|| a.get("val"))
                })
            })
            .or_else(|| self.obj.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string()
    }

    pub fn parent_id(&self) -> String {
        self.obj
            .get("parent_id")
            .or_else(|| self.obj.get("args").and_then(|a| a.get("parent_id")))
            .and_then(|p| p.as_str())
            .unwrap_or("")
            .to_string()
    }

    pub fn module(&self) -> String {
        self.obj
            .get("module")
            .or_else(|| self.obj.get("args").and_then(|a| a.get("module")))
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string()
    }

    pub fn has_arg(&self, key: &str) -> bool {
        self.obj.get("args").and_then(|a| a.get(key)).is_some()
    }

    pub fn file(&self) -> Option<&str> {
        self.obj.get(".file").and_then(|f| f.as_str())
    }

    pub fn line(&self) -> Option<i64> {
        self.obj.get(".line").and_then(|l| l.as_i64())
    }

    pub fn device_ts(&self) -> Option<f64> {
        self.obj
            .get("args")
            .and_then(|a| a.get("device_ts"))
            .and_then(|t| {
                if let Some(f) = t.as_f64() {
                    Some(f)
                } else if let Some(s) = t.as_str() {
                    s.parse::<f64>().ok()
                } else {
                    None
                }
            })
    }
}

pub struct TraceEventMut<'a> {
    obj: &'a mut serde_json::Map<String, serde_json::Value>,
}

impl<'a> TraceEventMut<'a> {
    pub fn new(event: &'a mut serde_json::Value) -> Option<Self> {
        event.as_object_mut().map(|obj| Self { obj })
    }

    pub fn from_map(obj: &'a mut serde_json::Map<String, serde_json::Value>) -> Self {
        Self { obj }
    }

    pub fn as_ref(&self) -> TraceEventRef<'_> {
        TraceEventRef { obj: self.obj }
    }

    pub fn category(&self) -> TraceCategory {
        self.as_ref().category()
    }

    pub fn span_id(&self) -> String {
        self.as_ref().span_id()
    }

    pub fn msg(&self) -> String {
        self.as_ref().msg()
    }

    pub fn parent_id(&self) -> String {
        self.as_ref().parent_id()
    }

    pub fn module(&self) -> String {
        self.as_ref().module()
    }

    pub fn has_arg(&self, key: &str) -> bool {
        self.as_ref().has_arg(key)
    }

    pub fn file(&self) -> Option<&str> {
        self.obj.get(".file").and_then(|f| f.as_str())
    }

    pub fn line(&self) -> Option<i64> {
        self.obj.get(".line").and_then(|l| l.as_i64())
    }

    pub fn name(&self) -> String {
        self.as_ref().name()
    }

    pub fn target_name(&self) -> String {
        self.as_ref().target_name()
    }

    pub fn device_ts(&self) -> Option<f64> {
        self.as_ref().device_ts()
    }

    pub fn ts(&self) -> f64 {
        self.as_ref().ts()
    }

    pub fn pid(&self) -> i64 {
        self.as_ref().pid()
    }

    pub fn tid(&self) -> i64 {
        self.as_ref().tid()
    }

    pub fn set_name(&mut self, name: &str) {
        self.obj
            .insert("name".to_string(), serde_json::Value::from(name));
        self.obj
            .insert("span_name".to_string(), serde_json::Value::from(name));
        if let Some(args) = self.obj.get_mut("args").and_then(|a| a.as_object_mut()) {
            args.insert("span_name".to_string(), serde_json::Value::from(name));
        }
    }

    pub fn set_target_name(&mut self, name: &str) {
        self.obj
            .insert("target_name".to_string(), serde_json::Value::from(name));
        if let Some(args) = self.obj.get_mut("args").and_then(|a| a.as_object_mut()) {
            args.insert("span_name".to_string(), serde_json::Value::from(name));
        }
    }

    pub fn set_span_id(&mut self, id: &str) {
        self.obj
            .insert("span_id".to_string(), serde_json::Value::from(id));
    }

    pub fn set_parent_id(&mut self, id: &str) {
        self.obj
            .insert("parent_id".to_string(), serde_json::Value::from(id));
    }

    pub fn set_module(&mut self, module: &str) {
        self.obj
            .insert("module".to_string(), serde_json::Value::from(module));
    }

    pub fn set_msg(&mut self, msg: &str) {
        self.obj
            .insert("name".to_string(), serde_json::Value::from(msg));
        self.obj
            .insert("msg".to_string(), serde_json::Value::from(msg));
        if let Some(args) = self.obj.get_mut("args").and_then(|a| a.as_object_mut()) {
            if args.contains_key("message") {
                args.insert("message".to_string(), serde_json::Value::from(msg));
            }
            if args.contains_key("msg") {
                args.insert("msg".to_string(), serde_json::Value::from(msg));
            }
        }
    }

    pub fn set_pid(&mut self, pid: i64) {
        self.obj.insert("pid".to_string(), serde_json::json!(pid));
    }

    pub fn set_tid(&mut self, tid: i64) {
        self.obj.insert("tid".to_string(), serde_json::json!(tid));
    }

    pub fn set_ts(&mut self, ts: f64) {
        self.obj.insert("ts".to_string(), serde_json::json!(ts));
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

/// Strongly-typed thread IDs representing execution tracks in Perfetto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum ThreadId {
    /// CPU Core 0 timeline track
    Core0 = 1,
    /// CPU Core 1 timeline track
    Core1 = 2,
    /// Unified Device Logs & Telemetry track
    DeviceLogsAndTelemetry = 3,
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

    pub fn keyword(&self) -> &'static str {
        match self {
            Self::SpanEnter => " span_enter: ",
            Self::SpanExit => " span_exit: ",
            _ => "",
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
    pub fn decode(&self, ev: &mut TraceEventMut<'_>) {
        let span_id = ev.span_id();
        let module = ev.module();
        let span_name = ev.name();

        ev.set_span_id(&span_id);
        ev.set_module(&module);
        ev.set_name(&span_name);

        if ev.has_arg("parent_id") {
            let parent_id = ev.parent_id();
            ev.set_parent_id(&parent_id);
        }
    }
}

/// Decoder strategy for span exit events.
pub struct SpanExitDecoder;

impl SpanExitDecoder {
    /// Decodes exit-specific trace telemetry arguments into the mutable JSON map.
    pub fn decode(&self, ev: &mut TraceEventMut<'_>) {
        let span_id = ev.span_id();
        let target_name = ev.target_name();

        ev.set_span_id(&span_id);
        ev.set_target_name(&target_name);
    }
}

/// Decoder strategy for device logs.
pub struct DeviceLogDecoder;

impl DeviceLogDecoder {
    /// Decodes log-specific trace telemetry arguments into the mutable JSON map.
    pub fn decode(&self, ev: &mut TraceEventMut<'_>) {
        let msg = ev.msg();
        ev.set_msg(&msg);
    }
}

fn is_trace_metadata(msg: &str, ev: TraceEventRef<'_>) -> bool {
    if msg.is_empty() || msg.contains("span_enter:") || msg.contains("span_exit:") {
        return true;
    }
    if let Some(file) = ev.file() {
        if file.contains("tracing-defmt-decoder") {
            if let Some(line) = ev.line() {
                if line != 229 {
                    return true;
                }
            }
        }
    }
    false
}

/// Pipeline stage that decodes raw JSON arguments into clean, top-level values on each event.
pub struct TelemetryDecoder {
    enter_decoder: SpanEnterDecoder,
    exit_decoder: SpanExitDecoder,
    log_decoder: DeviceLogDecoder,
    telemetry_parser: tool_common::TraceTelemetryParser,
}

impl TelemetryDecoder {
    /// Creates a new TelemetryDecoder with its inner strategies initialized.
    pub fn new() -> Self {
        Self {
            enter_decoder: SpanEnterDecoder,
            exit_decoder: SpanExitDecoder,
            log_decoder: DeviceLogDecoder,
            telemetry_parser: tool_common::TraceTelemetryParser::new(
                ThreadId::DeviceLogsAndTelemetry as u64,
            ),
        }
    }
}

impl TraceStage for TelemetryDecoder {
    fn run(
        &mut self,
        events: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let mut processed_events = Vec::new();
        for event in events {
            let mut event = event;
            if let Some(obj) = event.as_object_mut() {
                let mut ev = TraceEventMut::from_map(obj);
                let category = ev.category();
                match category {
                    TraceCategory::SpanEnter => {
                        self.enter_decoder.decode(&mut ev);
                        processed_events.push(event);
                    }
                    TraceCategory::SpanExit => {
                        self.exit_decoder.decode(&mut ev);
                        processed_events.push(event);
                    }
                    TraceCategory::Log => {
                        self.log_decoder.decode(&mut ev);

                        let msg = ev.msg();

                        if is_trace_metadata(&msg, ev.as_ref()) {
                            continue;
                        }

                        let ts = ev.ts();

                        // 2. Identify structured telemetry log lines and parse/route them
                        if let Some(parsed_events) = self.telemetry_parser.parse_log(&msg, ts) {
                            processed_events.extend(parsed_events);
                        } else {
                            // Place normal logs onto the "Device Logs & Telemetry" track (tid: 3)
                            ev.set_tid(ThreadId::DeviceLogsAndTelemetry as i64);
                            ev.set_name(&msg);
                            ev.set_msg(&msg);
                            processed_events.push(event);
                        }
                    }
                    TraceCategory::Other => {
                        processed_events.push(event);
                    }
                }
            } else {
                processed_events.push(event);
            }
        }
        Ok(processed_events)
    }
}

pub struct CoreMetadata {
    pub name: String,
    pub core: Option<i64>,
    pub is_idle: bool,
}

impl CoreMetadata {
    pub fn new(mut name: String) -> Self {
        let mut core = None;
        let is_idle = name.contains("cpu_idle_c") || name.contains("CPU Idle Core");

        if name.contains("Core 1:")
            || name.contains("cpu_idle_c1")
            || name.contains("CPU Idle Core 1")
        {
            core = Some(1);
        } else if name.contains("Core 0:")
            || name.contains("cpu_idle_c0")
            || name.contains("CPU Idle Core 0")
        {
            core = Some(0);
        }

        if name.starts_with("Core 1: ") {
            name = name["Core 1: ".len()..].to_string();
        } else if name.starts_with("Core 0: ") {
            name = name["Core 0: ".len()..].to_string();
        }

        Self {
            name,
            core,
            is_idle,
        }
    }
}

/// Pipeline stage that assigns each trace event to CPU Core 0 or Core 1.
pub struct CoreAssigner {
    pub elf_path: Option<String>,
}

impl CoreAssigner {
    pub fn new(elf_path: Option<String>) -> Self {
        Self { elf_path }
    }
}

impl TraceStage for CoreAssigner {
    fn run(
        &mut self,
        events: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let mut span_cores = std::collections::HashMap::new();
        let mut span_pids = std::collections::HashMap::new();
        let mut last_core = 0;
        let mut last_pid = 1;
        let mut seen_pids = std::collections::HashSet::new();

        let mut mcu_events = Vec::new();
        for event in events {
            if let Some(obj) = event.as_object() {
                let ph = obj.get("ph").and_then(|p| p.as_str()).unwrap_or("");
                if ph == "M" {
                    continue;
                }
            }
            mcu_events.push(event);
        }

        for event in &mut mcu_events {
            if let Some(obj) = event.as_object_mut() {
                let mut ev = TraceEventMut::from_map(obj);
                let tid = ev.tid();
                if tid == ThreadId::DeviceLogsAndTelemetry as i64 {
                    let resolved_pid = ev.pid().max(last_pid);
                    seen_pids.insert(resolved_pid);
                    ev.set_pid(resolved_pid);
                    continue;
                }

                let category = ev.category();

                match category {
                    TraceCategory::SpanEnter => {
                        let span_name = ev.name();
                        let parent_id = ev.parent_id();
                        let span_id = ev.span_id();

                        let metadata = CoreMetadata::new(span_name);
                        let cleaned_name = metadata.name;
                        let core = metadata.core;

                        let resolved_core = if let Some(c) = core {
                            c
                        } else if !span_id.is_empty() && span_cores.contains_key(&span_id) {
                            *span_cores.get(&span_id).unwrap_or(&0)
                        } else if !parent_id.is_empty() {
                            *span_cores.get(&parent_id).unwrap_or(&0)
                        } else {
                            0
                        };

                        let event_pid = ev.pid();
                        let resolved_pid = if !parent_id.is_empty() {
                            *span_pids.get(&parent_id).unwrap_or(&event_pid)
                        } else {
                            event_pid
                        };

                        if !span_id.is_empty() {
                            span_cores.insert(span_id.clone(), resolved_core);
                            span_pids.insert(span_id, resolved_pid);
                        }

                        last_core = resolved_core;
                        last_pid = resolved_pid;
                        seen_pids.insert(resolved_pid);

                        ev.set_name(&cleaned_name);
                        ev.set_pid(resolved_pid);
                        ev.set_tid(resolved_core + 1);
                    }
                    TraceCategory::SpanExit => {
                        let span_name = ev.target_name();
                        let span_id = ev.span_id();

                        let metadata = CoreMetadata::new(span_name);
                        let cleaned_name = metadata.name;
                        let core = metadata.core;

                        let resolved_core =
                            if !span_id.is_empty() && span_cores.contains_key(&span_id) {
                                *span_cores.get(&span_id).unwrap_or(&0)
                            } else {
                                core.unwrap_or(0)
                            };

                        let resolved_pid = if !span_id.is_empty() {
                            *span_pids.get(&span_id).unwrap_or(&1)
                        } else {
                            1
                        };

                        last_core = resolved_core;
                        last_pid = resolved_pid;
                        seen_pids.insert(resolved_pid);

                        ev.set_name(&cleaned_name);
                        ev.set_target_name(&cleaned_name);
                        ev.set_pid(resolved_pid);
                        let core_tid = if resolved_core == 1 {
                            ThreadId::Core1 as i64
                        } else {
                            ThreadId::Core0 as i64
                        };
                        ev.set_tid(core_tid);
                    }
                    TraceCategory::Log => {
                        let msg = ev.msg();

                        let metadata = CoreMetadata::new(msg);
                        let cleaned_name = metadata.name;
                        let core = metadata.core;

                        let resolved_core = core.unwrap_or(last_core);
                        let resolved_pid = ev.pid().max(last_pid);

                        last_core = resolved_core;
                        last_pid = resolved_pid;
                        seen_pids.insert(resolved_pid);

                        ev.set_msg(&cleaned_name);
                        ev.set_pid(resolved_pid);
                        let core_tid = if resolved_core == 1 {
                            ThreadId::Core1 as i64
                        } else {
                            ThreadId::Core0 as i64
                        };
                        ev.set_tid(core_tid);
                    }
                    TraceCategory::Other => {
                        let resolved_pid = ev.pid();
                        seen_pids.insert(resolved_pid);
                        ev.set_pid(resolved_pid);
                        ev.set_tid(ThreadId::Core0 as i64);
                    }
                }
            }
        }

        let mut target_name = "MCU".to_string();
        if let Some(ref ep) = self.elf_path {
            if let Ok(info) = tool_common::autodetect_project_info(std::path::Path::new(ep)) {
                target_name = format!("{} MCU", info.chip.to_uppercase());
            }
        }

        let mut final_events = Vec::new();
        // Add metadata for each seen process ID
        for pid in seen_pids {
            final_events.push(serde_json::json!({
                "name": "process_name",
                "ph": "M",
                "pid": pid,
                "args": {
                    "name": target_name
                }
            }));
            final_events.push(serde_json::json!({
                "name": "thread_name",
                "ph": "M",
                "pid": pid,
                "tid": ThreadId::Core0 as i64,
                "args": {
                    "name": "CPU Core 0"
                }
            }));
            final_events.push(serde_json::json!({
                "name": "thread_name",
                "ph": "M",
                "pid": pid,
                "tid": ThreadId::Core1 as i64,
                "args": {
                    "name": "CPU Core 1"
                }
            }));
            final_events.push(serde_json::json!({
                "name": "thread_name",
                "ph": "M",
                "pid": pid,
                "tid": ThreadId::DeviceLogsAndTelemetry as i64,
                "args": {
                    "name": "Device Logs & Telemetry"
                }
            }));
        }
        final_events.extend(mcu_events);

        Ok(final_events)
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

/// Context data containing all tracked spans state.
pub struct SpanContext {
    /// Database of all tracked spans, keyed by span ID.
    pub spans: std::collections::HashMap<String, SpanInfo>,
    /// Maps task context name to its stack of active span IDs.
    pub active_spans_map: std::collections::HashMap<String, Vec<String>>,
    /// Tracks the globally most recently active span ID for parent fallback.
    pub global_last_active_span: Option<String>,
    /// Tracks the currently active task context.
    pub active_task: Option<String>,
    /// Thread ID associated with this core's context.
    pub tid: i64,
}

impl SpanContext {
    /// Creates a new empty context for the given thread ID.
    pub fn new(tid: i64) -> Self {
        Self {
            spans: std::collections::HashMap::new(),
            active_spans_map: std::collections::HashMap::new(),
            global_last_active_span: None,
            active_task: None,
            tid,
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
        self.active_task.as_deref()
    }

    fn set_current_task(&mut self, task: Option<String>) {
        self.active_task = task;
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
                let implicit_exit = serde_json::json!({
                    "cat": "device",
                    "ph": "E",
                    "name": name,
                    "ts": ts.clone(),
                    "pid": exit_pid,
                    "tid": self.tid,
                    "span_id": exited_id,
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
                let implicit_enter = serde_json::json!({
                    "cat": "device",
                    "ph": "B",
                    "name": name,
                    "ts": ts.clone(),
                    "pid": resume_pid,
                    "tid": self.tid,
                    "span_id": resumed_id,
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
        let ev = TraceEventRef::from_map(obj);
        let span_id = ev.span_id();
        let mut parent_id = ev.parent_id();
        let module = obj.get("module").and_then(|s| s.as_str()).unwrap_or("");

        if parent_id.is_empty() && obj.get("parent_id").is_none() {
            if let Some(active_id) = &context.global_last_active_span {
                parent_id = active_id.clone();
            }
        }

        if !is_root_or_empty_id(&parent_id) {
            if !context.spans.contains_key(&parent_id) {
                parent_id = String::new();
            }
        }

        let mut span_name = ev.name();

        if (span_name == "run" || span_name == "task") && !module.is_empty() {
            let segments: Vec<&str> = module.split("::").collect();
            if let Some(target_segment) = segments.iter().rev().find(|&&s| is_root_segment(s)) {
                span_name = target_segment.to_string();
            }
        }

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

        let event_pid = obj.get("pid").and_then(|p| p.as_i64()).unwrap_or(1);
        let event_tid = context.tid;

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
                    "span_id": exited_id,
                    "args": {
                        "implicit": true
                    }
                });
                processed_events.push(implicit_exit);
            }
        } else {
            // Check for recursive span enter or sibling enter beneath an already-open parent due to lost exit events
            let unwind_from = active
                .iter()
                .position(|id| {
                    if let Some(n) = context.get_name(id) {
                        n == span_name || is_span_name_match(&n, &span_name)
                    } else {
                        false
                    }
                })
                .or_else(|| {
                    active
                        .iter()
                        .position(|id| id == &parent_id)
                        .map(|parent_pos| parent_pos + 1)
                        .filter(|&child_pos| child_pos < active.len())
                });
            if let Some(pos) = unwind_from {
                while active.len() > pos {
                    if let Some(exited_id) = active.pop() {
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
                            "span_id": exited_id,
                            "args": {
                                "implicit": true
                            }
                        });
                        processed_events.push(implicit_exit);
                    }
                }
            }
        }

        active.push(span_id.to_string());
        context.active_spans_map.insert(task_context, active);
        context.global_last_active_span = Some(span_id.to_string());

        final_event["cat"] = serde_json::Value::from("device");
        final_event["ph"] = serde_json::Value::from("B");
        final_event["name"] = serde_json::Value::from(span_name);
        final_event["pid"] = serde_json::Value::from(event_pid);
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
        let ev = TraceEventRef::from_map(obj);
        let span_id = ev.span_id();
        let target_name = ev.target_name();

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
        final_event["span_id"] = serde_json::Value::from(resolved_span_id.clone());
        if let Some(t_pid) = context.get_pid(&resolved_span_id) {
            final_event["pid"] = serde_json::Value::from(t_pid);
        }
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
                            let exit_tid = context.get_tid(&exited_id).unwrap_or(0);
                            let implicit_exit = serde_json::json!({
                                "cat": "device",
                                "ph": "E",
                                "name": name,
                                "ts": ts.clone(),
                                "pid": exit_pid,
                                "tid": exit_tid,
                                "span_id": exited_id,
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
    contexts: std::collections::HashMap<i64, SpanContext>,
    enter_processor: SpanEnterProcessor,
    exit_processor: SpanExitProcessor,
}

impl SpanProcessor {
    /// Creates a new span processor with empty tracing state.
    pub fn new() -> Self {
        Self {
            contexts: std::collections::HashMap::new(),
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

            if let Some(ev) = TraceEventRef::new(&event) {
                let category = ev.category();
                let event_tid = ev.tid().max(1);
                let context = self
                    .contexts
                    .entry(event_tid)
                    .or_insert_with(|| SpanContext::new(event_tid));

                match category {
                    TraceCategory::SpanEnter => {
                        if let Some(obj) = event.as_object() {
                            self.enter_processor.process(
                                context,
                                obj,
                                &mut final_event,
                                &mut processed_events,
                            );
                        }
                    }
                    TraceCategory::SpanExit => {
                        if let Some(obj) = event.as_object() {
                            let keep = self.exit_processor.process(
                                context,
                                obj,
                                &mut final_event,
                                &mut processed_events,
                            );
                            if !keep {
                                continue;
                            }
                        }
                    }
                    TraceCategory::Log => {
                        let msg = ev.msg();
                        if msg.is_empty()
                            || msg.contains("span_enter:")
                            || msg.contains("span_exit:")
                        {
                            continue;
                        }

                        final_event["name"] = serde_json::Value::from(msg);
                    }
                    TraceCategory::Other => {}
                }
            }
            processed_events.push(final_event);
        }

        Ok(processed_events)
    }
}

/// Serializes processed events back to the trace file path (Perfetto JSON or `.csv`).
fn save_processed_trace(
    path: &str,
    processed_events: Vec<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    if path.ends_with(".csv") {
        let csv_text = format_perfetto_events_as_csv(&processed_events);
        fs::write(path, csv_text)?;
    } else {
        let serialized = serde_json::to_string_pretty(&processed_events)?;
        fs::write(path, serialized)?;
    }
    Ok(())
}

/// Pipeline stage that aligns all target events to the host time domain
/// using the target microcontroller timestamp to eliminate transport and decoding jitter.
pub struct DeviceTimeAligner;

impl TraceStage for DeviceTimeAligner {
    fn run(
        &mut self,
        mut events: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        // Find the first event with device_ts to compute the synchronization offset
        let mut sync_offset = None;
        for event in &events {
            if let Some(ev) = TraceEventRef::new(event) {
                if let Some(device_ts) = ev.device_ts() {
                    let host_ts = ev.ts();
                    sync_offset = Some(host_ts - device_ts);
                    break;
                }
            }
        }

        // Apply the synchronization offset to all events containing device_ts
        if let Some(offset) = sync_offset {
            for event in &mut events {
                let mut maybe_device_ts = None;
                if let Some(ev) = TraceEventRef::new(event) {
                    maybe_device_ts = ev.device_ts();
                }
                if let Some(device_ts) = maybe_device_ts {
                    if let Some(obj) = event.as_object_mut() {
                        obj.insert("ts".to_string(), serde_json::json!(device_ts + offset));
                    }
                }
            }
        }

        Ok(events)
    }
}

fn normalize_value_quotes(val: &mut serde_json::Value) {
    match val {
        serde_json::Value::String(s) => {
            let trimmed = s.trim();
            if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
                *s = trimmed[1..trimmed.len() - 1].to_string();
            } else if trimmed != s {
                *s = trimmed.to_string();
            }
        }
        serde_json::Value::Object(map) => {
            for v in map.values_mut() {
                normalize_value_quotes(v);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                normalize_value_quotes(v);
            }
        }
        _ => {}
    }
}

/// Sliding window width in microseconds (`50,000 µs` = 50 ms) for integrating `CPU Idle` duty cycle.
const CPU_USAGE_WINDOW_US: u64 = 50_000;

/// Step spacing in microseconds (`25,000 µs` = 25 ms = 40 Hz) for continuous CPU Usage (%) curve emission.
const CPU_USAGE_SAMPLE_STEP_US: u64 = 25_000;

/// Causal exponential smoothing factor (`0.45`) applied to windowed CPU utilization samples.
const CPU_USAGE_EMA_ALPHA: f64 = 0.45;

/// Minimum spacing (`2,000 µs`) between forced transition samples on a core's CPU Usage (%) curve.
const CPU_USAGE_MIN_FORCE_GAP_US: u64 = 2_000;

/// Per-core sliding-window integrator converting discrete `(idle_enter_us, idle_exit_us)` spans
/// into a smooth, continuous `0.0%..100.0%` CPU Usage (%) curve.
#[derive(Default, Clone)]
pub struct CoreCpuUsageIntegrator {
    first_ts_us: Option<u64>,
    last_sample_ts_us: Option<u64>,
    smoothed_pct: Option<f64>,
    open_idle_start_us: Option<u64>,
    has_seen_idle: bool,
    idle_intervals: std::collections::VecDeque<(u64, u64)>,
}

impl CoreCpuUsageIntegrator {
    fn touch_first_ts(&mut self, ts_us: u64) {
        match self.first_ts_us {
            None => self.first_ts_us = Some(ts_us),
            Some(cur) if ts_us < cur => self.first_ts_us = Some(ts_us),
            _ => {}
        }
    }

    fn compute_raw_window_pct(&self, sample_ts_us: u64) -> f64 {
        let first = self.first_ts_us.unwrap_or(sample_ts_us);
        let w_start = sample_ts_us.saturating_sub(CPU_USAGE_WINDOW_US).max(first);
        if sample_ts_us <= w_start {
            let in_idle = self
                .open_idle_start_us
                .map(|s| s <= sample_ts_us)
                .unwrap_or(false)
                || self
                    .idle_intervals
                    .iter()
                    .any(|&(s, e)| s <= sample_ts_us && sample_ts_us <= e);
            return if in_idle { 0.0 } else { 100.0 };
        }

        let window_len = sample_ts_us - w_start;
        let mut idle_overlap = 0u64;
        for &(s, e) in &self.idle_intervals {
            let ov_start = s.max(w_start);
            let ov_end = e.min(sample_ts_us);
            if ov_end > ov_start {
                idle_overlap = idle_overlap.saturating_add(ov_end - ov_start);
            }
        }
        if let Some(open_s) = self.open_idle_start_us {
            let ov_start = open_s.max(w_start);
            if sample_ts_us > ov_start {
                idle_overlap = idle_overlap.saturating_add(sample_ts_us - ov_start);
            }
        }

        let clamped_idle = idle_overlap.min(window_len);
        (((window_len - clamped_idle) as f64 / window_len as f64) * 100.0).clamp(0.0, 100.0)
    }

    fn emit_sample_at(&mut self, ts_us: u64) -> (u64, f64) {
        let raw_pct = self.compute_raw_window_pct(ts_us);
        let smoothed = match self.smoothed_pct {
            None => raw_pct,
            Some(prev) => prev * (1.0 - CPU_USAGE_EMA_ALPHA) + raw_pct * CPU_USAGE_EMA_ALPHA,
        };
        let rounded = ((smoothed * 10.0).round() / 10.0).clamp(0.0, 100.0);
        self.smoothed_pct = Some(rounded);
        self.last_sample_ts_us = Some(ts_us);
        (ts_us, rounded)
    }

    fn prune_old_intervals(&mut self, horizon_us: u64) {
        let keep_after = horizon_us.saturating_sub(CPU_USAGE_WINDOW_US * 2);
        while let Some(&(_, end_us)) = self.idle_intervals.front() {
            if end_us < keep_after {
                self.idle_intervals.pop_front();
            } else {
                break;
            }
        }
    }

    fn advance_samples_to(&mut self, target_ts_us: u64, force_at_target: bool) -> Vec<(u64, f64)> {
        let mut samples = Vec::new();
        let start_cursor = match self.last_sample_ts_us {
            None => {
                let anchor = self.first_ts_us.unwrap_or(target_ts_us);
                samples.push(self.emit_sample_at(anchor));
                anchor
            }
            Some(last) => last,
        };

        if target_ts_us > start_cursor {
            let mut next_step = start_cursor.saturating_add(CPU_USAGE_SAMPLE_STEP_US);
            while next_step <= target_ts_us {
                samples.push(self.emit_sample_at(next_step));
                next_step = next_step.saturating_add(CPU_USAGE_SAMPLE_STEP_US);
            }
            let last_emitted = self.last_sample_ts_us.unwrap_or(start_cursor);
            if force_at_target
                && target_ts_us > last_emitted
                && (target_ts_us - last_emitted >= CPU_USAGE_MIN_FORCE_GAP_US || samples.is_empty())
            {
                samples.push(self.emit_sample_at(target_ts_us));
            }
        }

        self.prune_old_intervals(target_ts_us);
        samples
    }

    /// Record a non-idle activity timestamp on this core, advancing integrated curve samples if active.
    pub fn record_busy_activity(&mut self, ts_us: u64) -> Vec<(u64, f64)> {
        self.touch_first_ts(ts_us);
        if !self.has_seen_idle {
            return Vec::new();
        }
        self.advance_samples_to(ts_us, false)
    }

    /// Record a `CPU Idle` `span_enter` event at `enter_us` and emit any due integrated curve samples.
    pub fn record_idle_enter(&mut self, enter_us: u64) -> Vec<(u64, f64)> {
        self.touch_first_ts(enter_us);
        self.has_seen_idle = true;
        // Integrate any preceding busy interval up to `enter_us` BEFORE marking the core idle
        let mut samples = if self.first_ts_us.unwrap_or(enter_us) < enter_us {
            self.advance_samples_to(enter_us, true)
        } else {
            Vec::new()
        };
        self.open_idle_start_us = Some(enter_us);
        if self.last_sample_ts_us.is_none() {
            samples.push(self.emit_sample_at(enter_us));
        }
        samples
    }

    /// Record a `CPU Idle` `span_exit` event at `exit_us`, closing the idle interval and integrating usage.
    pub fn record_idle_exit(&mut self, exit_us: u64) -> Vec<(u64, f64)> {
        self.touch_first_ts(exit_us);
        self.has_seen_idle = true;
        if let Some(enter_us) = self.open_idle_start_us.take() {
            if exit_us > enter_us {
                self.idle_intervals.push_back((enter_us, exit_us));
            }
        }
        self.advance_samples_to(exit_us, true)
    }
}

/// Pipeline stage that integrates `CPU Idle` span events into a continuous
/// `Core N CPU Usage (%)` curve (`0.0%..100.0%`), removing raw idle spans from thread slices.
pub struct CpuUsageGenerator;

impl TraceStage for CpuUsageGenerator {
    fn run(
        &mut self,
        events: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let mut final_events = Vec::new();
        let mut integrators: std::collections::BTreeMap<i64, CoreCpuUsageIntegrator> =
            std::collections::BTreeMap::new();

        // Seed earliest observed timestamp across trace so busy pre-idle intervals integrate accurately
        let mut global_min_ts: Option<u64> = None;
        for event in &events {
            if let Some(ts) = event.get("ts").and_then(|t| t.as_f64()) {
                let ts_u64 = ts.max(0.0).round() as u64;
                global_min_ts = Some(match global_min_ts {
                    None => ts_u64,
                    Some(cur) => cur.min(ts_u64),
                });
            }
        }

        for event in events {
            if let Some(obj) = event.as_object() {
                let name = obj.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let ph = obj.get("ph").and_then(|p| p.as_str()).unwrap_or("");
                let ts_us = obj
                    .get("ts")
                    .and_then(|t| t.as_f64())
                    .unwrap_or(0.0)
                    .max(0.0)
                    .round() as u64;

                if name == "CPU Idle Core 0" || name == "CPU Idle Core 1" {
                    let core = if name == "CPU Idle Core 0" { 0 } else { 1 };
                    let integ = integrators.entry(core).or_default();
                    if let Some(min_ts) = global_min_ts {
                        if min_ts < ts_us {
                            integ.touch_first_ts(min_ts);
                        }
                    }
                    let samples = if ph == "B" {
                        integ.record_idle_enter(ts_us)
                    } else {
                        integ.record_idle_exit(ts_us)
                    };
                    for (s_ts, pct) in samples {
                        final_events.push(serde_json::json!({
                            "cat": "cpu_usage",
                            "name": format!("Core {} CPU Usage (%)", core),
                            "ph": "C",
                            "pid": 1,
                            "ts": s_ts as f64,
                            "args": {
                                "value": pct
                            }
                        }));
                    }
                    continue;
                }
            }
            final_events.push(event);
        }

        final_events.sort_by(|a, b| {
            let ts_a = a.get("ts").and_then(|t| t.as_f64()).unwrap_or(0.0);
            let ts_b = b.get("ts").and_then(|t| t.as_f64()).unwrap_or(0.0);
            ts_a.partial_cmp(&ts_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(final_events)
    }
}

/// Post-processes the generated trace JSON file using a multi-stage transformation pipeline.
pub fn post_process_trace(
    path: &str,
    elf_path: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(path).exists() {
        return Ok(());
    }
    let content = fs::read_to_string(path)?;
    if content.trim().is_empty() {
        return Ok(());
    }

    let mut events: Vec<serde_json::Value> = serde_json::from_str(&content)?;
    for ev in &mut events {
        normalize_value_quotes(ev);
    }

    let mut pipeline = TracePipeline::new()
        .add_stage(DeviceTimeAligner)
        .add_stage(ChronologicalSorter)
        .add_stage(TelemetryDecoder::new())
        .add_stage(CoreAssigner::new(elf_path.map(|s| s.to_string())))
        .add_stage(SpanProcessor::new())
        .add_stage(CpuUsageGenerator);

    let processed_events = pipeline.execute(events)?;

    save_processed_trace(path, processed_events)?;

    Ok(())
}

fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut in_escape = false;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                in_escape = true;
                chars.next(); // consume '['
                continue;
            }
        }
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
            continue;
        }
        result.push(c);
    }
    result
}

/// A structured representation of a parsed target defmt tracing line.
pub struct ParsedTracingLine<'a> {
    /// The original raw tracing line reference.
    line: &'a str,
}

impl<'a> ParsedTracingLine<'a> {
    pub fn line(&self) -> &'a str {
        self.line
    }

    pub fn category(&self) -> TraceCategory {
        if self.line.contains(" span_enter: ") {
            TraceCategory::SpanEnter
        } else if self.line.contains(" span_exit: ") {
            TraceCategory::SpanExit
        } else {
            TraceCategory::Other
        }
    }

    pub fn is_enter(&self) -> bool {
        self.category() == TraceCategory::SpanEnter
    }

    pub fn span_name(&self) -> String {
        let clean_line = strip_ansi_codes(self.line);
        let cat = self.category();
        let keyword = cat.keyword();
        if let Some(keyword_pos) = clean_line.find(keyword) {
            let raw_name = &clean_line[keyword_pos + keyword.len()..];
            let mut span_name = raw_name.trim();
            if let Some(ts_pos) = span_name.find(" ts=") {
                span_name = span_name[..ts_pos].trim();
            }
            if span_name.starts_with('"') && span_name.ends_with('"') && span_name.len() >= 2 {
                span_name = &span_name[1..span_name.len() - 1];
            }
            span_name.to_string()
        } else {
            String::new()
        }
    }

    pub fn device_ts(&self) -> Option<f64> {
        let clean_line = strip_ansi_codes(self.line);
        if let Some(pos) = clean_line.find(" ts=") {
            let after_ts = &clean_line[pos + 4..];
            let ts_str = after_ts
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_matches('"');
            if let Ok(ts_val) = ts_str.parse::<f64>() {
                return Some(ts_val);
            }
        }
        let cat = self.category();
        let keyword = cat.keyword();
        let keyword_pos = clean_line.find(keyword)?;
        let prefix = &clean_line[..keyword_pos];
        let words: Vec<&str> = prefix.split_whitespace().collect();
        if !words.is_empty() {
            if let Ok(ts_sec) = words[0].parse::<f64>() {
                return Some(ts_sec * 1_000_000.0);
            }
        }
        None
    }

    pub fn parse_ids(&self) -> (String, String) {
        let clean_line = strip_ansi_codes(self.line);
        let cat = self.category();
        let keyword = cat.keyword();
        let keyword_pos = match clean_line.find(keyword) {
            Some(pos) => pos,
            None => return (String::new(), String::new()),
        };

        // Extract the prefix part before the keyword
        let prefix = &clean_line[..keyword_pos];

        // Split prefix into words to handle variable formatting with/without timestamps or module names
        let words: Vec<&str> = prefix.split_whitespace().collect();
        let mut idx = 0;
        let mut device_ts = None;

        if let Some(pos) = clean_line.find(" ts=") {
            let after_ts = &clean_line[pos + 4..];
            let ts_str = after_ts
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_matches('"');
            if let Ok(ts_val) = ts_str.parse::<f64>() {
                device_ts = Some(ts_val);
            }
        }

        // 1. Check for timestamp at the very beginning
        if device_ts.is_none() && idx < words.len() {
            if let Ok(ts_sec) = words[idx].parse::<f64>() {
                device_ts = Some(ts_sec * 1_000_000.0);
            }
        }
        if idx < words.len() && words[idx].parse::<f64>().is_ok() {
            idx += 1;
        }

        // 2. Check for optional Log Level
        if idx < words.len() {
            let w = words[idx];
            if w == "TRACE" || w == "DEBUG" || w == "INFO" || w == "WARN" || w == "ERROR" {
                idx += 1;
            }
        }

        // 3. Skip module context brackets or paths
        while idx < words.len()
            && (words[idx].starts_with('[')
                || words[idx].contains("::")
                || words[idx].ends_with(':'))
        {
            idx += 1;
        }

        // 4. Extract span_id and parent_id
        let mut span_id = String::new();
        let mut ids = Vec::new();

        if idx < words.len() {
            let w = words[idx];
            if !w.starts_with("parent=") {
                let raw_ids = if w.starts_with("ctx=") { &w[4..] } else { w };
                ids = raw_ids.split(':').collect();
                span_id = ids.last().unwrap_or(&raw_ids).to_string();
            }
        }

        // 5. Look for parent ID in all words
        let mut parent_id = String::new();
        for &w in &words {
            if w.starts_with("parent=") {
                parent_id = w[7..].to_string();
                break;
            }
        }

        // Fallback to hierarchy for parent_id if not found via parent=
        if parent_id.is_empty() && ids.len() >= 2 {
            parent_id = ids[ids.len() - 2].to_string();
        }

        // If span_id is still empty, auto-generate a unique one from device_ts
        if span_id.is_empty() {
            if let Some(ts) = device_ts {
                span_id = format!("auto_{}", ts as u64);
            }
        }

        (span_id, parent_id)
    }

    /// Parses a raw defmt log line into a structured tracing line if it represents
    /// a span transition (enter or exit).
    pub fn parse(line: &'a str) -> Option<Self> {
        let temp = Self { line };
        let cat = temp.category();
        if cat == TraceCategory::SpanEnter || cat == TraceCategory::SpanExit {
            Some(temp)
        } else {
            None
        }
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
pub fn handle_tracing_line(
    line: &str,
    module: Option<&str>,
    external_ts: Option<f64>,
) -> Result<bool, &'static str> {
    if let Some(parsed) = ParsedTracingLine::parse(line) {
        let opt_ts = parsed.device_ts().or(external_ts);
        let (span_id, parent_id) = parsed.parse_ids();
        if let Some(ts) = opt_ts {
            if parsed.is_enter() {
                tracing::info!(
                    target: "device_span_enter",
                    span_name = parsed.span_name(),
                    span_id = span_id,
                    parent_id = parent_id,
                    module = module.unwrap_or(""),
                    device_ts = ts
                );
            } else {
                tracing::info!(
                    target: "device_span_exit",
                    span_id = span_id,
                    span_name = parsed.span_name(),
                    device_ts = ts
                );
            }
        } else {
            if parsed.is_enter() {
                tracing::info!(
                    target: "device_span_enter",
                    span_name = parsed.span_name(),
                    span_id = span_id,
                    parent_id = parent_id,
                    module = module.unwrap_or("")
                );
            } else {
                tracing::info!(
                    target: "device_span_exit",
                    span_id = span_id,
                    span_name = parsed.span_name()
                );
            }
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Intelligently formats a raw target function or location namespace into crate and module name only.
///
/// Strips compiler-generated implementation blocks (`{impl#...}`), closures (`{closure#...}`),
/// async function artifacts (`{async_fn#...}`), and trailing function/method names to isolate
/// the canonical crate and module path (e.g. `controller::motor_controller`, `peripheral::tmc6300`).
pub fn format_target_namespace(raw: &str) -> String {
    let trimmed = raw.trim().trim_matches(|c| c == '[' || c == ']');
    if trimmed.is_empty() {
        return String::new();
    }

    let segments: Vec<&str> = trimmed.split("::").collect();
    if segments.is_empty() {
        return String::new();
    }

    // 1. If an impl block ({impl#...}) is encountered, all preceding segments define the module.
    if let Some(impl_idx) = segments.iter().position(|s| s.starts_with("{impl#")) {
        let valid_prefix = &segments[..impl_idx];
        if !valid_prefix.is_empty() {
            return valid_prefix.join("::");
        }
    }

    // 2. Otherwise, strip any compiler-generated segments ({closure#...}, {async_fn#...}, <...>)
    let non_compiler: Vec<&str> = segments
        .into_iter()
        .take_while(|s| !s.starts_with('{') && !s.starts_with('<'))
        .collect();

    if non_compiler.is_empty() {
        return String::new();
    }

    // 3. For 3+ segments (crate::module::func or crate::sub1::sub2::func), drop the function name.
    if non_compiler.len() >= 3 {
        return non_compiler[..non_compiler.len() - 1].join("::");
    }

    // 4. For 2 segments (crate::main or crate::module), drop "main" if present.
    if non_compiler.len() == 2 {
        if non_compiler[1] == "main" {
            return non_compiler[0].to_string();
        }
        return non_compiler.join("::");
    }

    // 5. Single segment
    non_compiler[0].to_string()
}

/// Metadata for a single live execution span tracked by [`LiveCoreSpanState`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveSpanEntry {
    span_id: String,
    parent_id: Option<String>,
    name: String,
    module: Option<String>,
    task_key: String,
}

impl LiveSpanEntry {
    /// Construct a new [`LiveSpanEntry`].
    pub fn new(
        span_id: String,
        parent_id: Option<String>,
        name: String,
        module: Option<String>,
        task_key: String,
    ) -> Self {
        Self {
            span_id,
            parent_id,
            name,
            module,
            task_key,
        }
    }

    /// Compute the effective lookup identifier (`span_id` if non-empty, otherwise `cleaned_name`).
    pub fn effective_id(span_id: &str, cleaned_name: &str) -> String {
        if span_id.is_empty() {
            cleaned_name.to_string()
        } else {
            span_id.to_string()
        }
    }

    /// Raw span identifier emitted by firmware (empty when anonymous).
    pub fn span_id(&self) -> &str {
        &self.span_id
    }

    /// Optional parent span identifier.
    pub fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }

    /// Normalized display name of this span.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Optional Rust module path associated with this span.
    pub fn module(&self) -> Option<&str> {
        self.module.as_deref()
    }

    /// Owning cooperative async task key on this core.
    pub fn task_key(&self) -> &str {
        &self.task_key
    }
}

/// Per-task call stack of active [`LiveSpanEntry`] frames supporting duplicate unwinding and child-pop queries.
#[derive(Clone, Debug, Default)]
pub struct LiveTaskSpanStack {
    frames: Vec<(String, LiveSpanEntry)>,
}

impl LiveTaskSpanStack {
    /// Return `true` when this task currently has zero open spans on its call stack.
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Return the current nesting depth of open spans on this task stack.
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// Return `true` when `target_id` is currently open on this task stack.
    pub fn contains_id(&self, target_id: &str) -> bool {
        self.frames.iter().any(|(id, _)| id == target_id)
    }

    /// Iterate over active [`LiveSpanEntry`] frames from outermost root to innermost leaf.
    pub fn entries(&self) -> impl DoubleEndedIterator<Item = &LiveSpanEntry> {
        self.frames.iter().map(|(_, entry)| entry)
    }

    /// Search from innermost leaf to outermost root for an open frame matching `span_name`.
    pub fn find_matching_id_rev(&self, span_name: &str) -> Option<&str> {
        self.frames
            .iter()
            .rev()
            .find(|(_, entry)| is_span_name_match(&entry.name, span_name))
            .map(|(id, _)| id.as_str())
    }

    /// Push `(eff_id, entry)` onto the stack, unwinding and returning (in LIFO order) any stale duplicate or sibling frame and its descendants.
    pub fn push_dedup(&mut self, eff_id: String, entry: LiveSpanEntry) -> Vec<LiveSpanEntry> {
        let mut implicit_closed = Vec::new();
        let unwind_from = self
            .frames
            .iter()
            .position(|(id, existing)| {
                id == &eff_id
                    || existing.name == entry.name
                    || is_span_name_match(&existing.name, &entry.name)
            })
            .or_else(|| {
                entry.parent_id().and_then(|parent_id| {
                    self.frames
                        .iter()
                        .position(|(id, _)| id == parent_id)
                        .map(|parent_pos| parent_pos + 1)
                        .filter(|&child_pos| child_pos < self.frames.len())
                })
            });
        if let Some(pos) = unwind_from {
            while self.frames.len() > pos {
                if let Some((_, popped_entry)) = self.frames.pop() {
                    implicit_closed.push(popped_entry);
                }
            }
        }
        self.frames.push((eff_id, entry));
        implicit_closed
    }

    /// Pop `target_id` from the stack, first unwinding and returning (in LIFO order) any unclosed descendant frames above it.
    pub fn pop_through(&mut self, target_id: &str) -> Option<(Vec<LiveSpanEntry>, LiveSpanEntry)> {
        let pos = self.frames.iter().position(|(id, _)| id == target_id)?;
        let mut implicit_children = Vec::new();
        while self.frames.len() > pos + 1 {
            if let Some((_, child_entry)) = self.frames.pop() {
                implicit_children.push(child_entry);
            }
        }
        let (_, target_entry) = self.frames.pop()?;
        Some((implicit_children, target_entry))
    }
}

/// Per-core cooperative async task and span state tracker used by [`LiveTraceStreamProcessor`].
#[derive(Default)]
pub struct LiveCoreSpanState {
    spans: std::collections::HashMap<String, LiveSpanEntry>,
    task_stacks: std::collections::HashMap<String, LiveTaskSpanStack>,
    running_task: Option<String>,
}

impl LiveCoreSpanState {
    /// Currently running cooperative task key on this core, if any.
    pub fn running_task(&self) -> Option<&str> {
        self.running_task.as_deref()
    }

    /// Look up a known [`LiveSpanEntry`] by its effective span identifier.
    pub fn span_entry(&self, id: &str) -> Option<&LiveSpanEntry> {
        self.spans.get(id)
    }

    /// Look up the owning cooperative `task_key` for `span_id`, if recorded.
    pub fn task_key_for_span(&self, span_id: &str) -> Option<&str> {
        self.spans.get(span_id).map(LiveSpanEntry::task_key)
    }

    /// Borrow the [`LiveTaskSpanStack`] for `task_key`, if currently open.
    pub fn task_stack(&self, task_key: &str) -> Option<&LiveTaskSpanStack> {
        self.task_stacks.get(task_key)
    }

    /// Return `true` when `target_id` is currently open on `task_key`'s stack.
    pub fn is_span_on_task_stack(&self, task_key: &str, target_id: &str) -> bool {
        self.task_stacks
            .get(task_key)
            .map(|stack| stack.contains_id(target_id))
            .unwrap_or(false)
    }

    /// Resolve the cooperative `task_key` for a newly entered span from its root status, `span_id`, `parent_id`, and `cleaned_name`.
    pub fn resolve_task_key(
        &self,
        is_root: bool,
        span_id: &str,
        parent_id: &str,
        cleaned_name: &str,
    ) -> String {
        if !is_root {
            self.task_key_for_span(parent_id)
                .map(str::to_string)
                .unwrap_or_else(|| parent_id.to_string())
        } else if !span_id.is_empty() {
            span_id.to_string()
        } else {
            cleaned_name.to_string()
        }
    }

    /// Resolve an exiting span's effective ID either by exact active `span_id` lookup or reverse name match across active task stacks.
    pub fn find_open_span_id(&self, span_id: &str, cleaned_name: &str) -> Option<String> {
        if !span_id.is_empty() {
            if let Some(entry) = self.spans.get(span_id) {
                if self.is_span_on_task_stack(entry.task_key(), span_id)
                    && (cleaned_name.is_empty() || is_span_name_match(&entry.name, cleaned_name))
                {
                    return Some(span_id.to_string());
                }
            }
        }
        for stack in self.task_stacks.values() {
            if let Some(found) = stack.find_matching_id_rev(cleaned_name) {
                return Some(found.to_string());
            }
        }
        None
    }

    fn emit_span_event(
        core: i64,
        ts_us: u64,
        ph: &str,
        entry: &LiveSpanEntry,
        extra_flag: Option<&str>,
    ) -> crate::HostCliEvent {
        let args = match extra_flag {
            Some("suspended") => serde_json::json!({ "core": core, "suspended": true }),
            Some("resumed") => serde_json::json!({ "core": core, "resumed": true }),
            Some("implicit") => serde_json::json!({ "core": core, "implicit": true }),
            _ => serde_json::json!({ "core": core }),
        };
        crate::HostCliEvent::Trace {
            ts_us,
            track: format!("CPU Core {}", core),
            ph: ph.to_string(),
            category: "span".to_string(),
            name: entry.name.clone(),
            span_id: if entry.span_id.is_empty() {
                None
            } else {
                Some(entry.span_id.clone())
            },
            parent_id: entry.parent_id.clone(),
            module: entry.module.clone(),
            value: None,
            unit: None,
            args,
        }
    }

    /// Suspend the currently running task on `core` (emitting `"E"` slices marked `"suspended"`).
    pub fn suspend_running(&mut self, core: i64, ts_us: u64, out: &mut Vec<crate::HostCliEvent>) {
        let Some(cur_task) = self.running_task.take() else {
            return;
        };
        if let Some(stack) = self.task_stacks.get(&cur_task) {
            for entry in stack.entries().rev() {
                out.push(Self::emit_span_event(
                    core,
                    ts_us,
                    "E",
                    entry,
                    Some("suspended"),
                ));
            }
        }
    }

    fn switch_to_task(
        &mut self,
        core: i64,
        task_key: &str,
        ts_us: u64,
        resume_existing: bool,
        out: &mut Vec<crate::HostCliEvent>,
    ) {
        if self.running_task() == Some(task_key) {
            return;
        }
        self.suspend_running(core, ts_us, out);
        if resume_existing {
            if let Some(stack) = self.task_stacks.get(task_key) {
                for entry in stack.entries() {
                    out.push(Self::emit_span_event(
                        core,
                        ts_us,
                        "B",
                        entry,
                        Some("resumed"),
                    ));
                }
            }
        }
        self.running_task = Some(task_key.to_string());
    }

    /// Record a span entry on `core`, performing task switching and duplicate-stack unwinding.
    #[allow(clippy::too_many_arguments)]
    pub fn enter_span(
        &mut self,
        core: i64,
        ts_us: u64,
        is_root: bool,
        span_id: String,
        parent_id: String,
        cleaned_name: String,
        module: Option<&str>,
        out_events: &mut Vec<crate::HostCliEvent>,
    ) {
        let task_key = self.resolve_task_key(is_root, &span_id, &parent_id, &cleaned_name);
        if is_root {
            if self.running_task() == Some(task_key.as_str()) {
                self.suspend_running(core, ts_us, out_events);
            }
            self.task_stacks.remove(&task_key);
            self.switch_to_task(core, &task_key, ts_us, false, out_events);
        } else {
            self.switch_to_task(core, &task_key, ts_us, true, out_events);
        }

        let eff_id = LiveSpanEntry::effective_id(&span_id, &cleaned_name);
        let entry = LiveSpanEntry::new(
            span_id,
            if parent_id.is_empty() {
                None
            } else {
                Some(parent_id)
            },
            cleaned_name,
            module.filter(|m| !m.is_empty()).map(str::to_string),
            task_key.clone(),
        );

        let implicit_closed = self
            .task_stacks
            .entry(task_key)
            .or_default()
            .push_dedup(eff_id.clone(), entry.clone());
        for popped_entry in &implicit_closed {
            let popped_eff_id =
                LiveSpanEntry::effective_id(&popped_entry.span_id, &popped_entry.name);
            if popped_eff_id != eff_id {
                self.spans.remove(&popped_eff_id);
            }
            out_events.push(Self::emit_span_event(
                core,
                ts_us,
                "E",
                popped_entry,
                Some("implicit"),
            ));
        }
        out_events.push(Self::emit_span_event(core, ts_us, "B", &entry, None));
        self.spans.insert(eff_id, entry);
    }

    /// Record a span exit on `core`, unwinding any unclosed child spans on the task stack.
    pub fn exit_span(
        &mut self,
        core: i64,
        ts_us: u64,
        span_id: &str,
        parent_id: &str,
        cleaned_name: String,
        module: Option<&str>,
        out_events: &mut Vec<crate::HostCliEvent>,
    ) {
        let Some(target_id) = self.find_open_span_id(span_id, &cleaned_name) else {
            out_events.push(crate::HostCliEvent::Trace {
                ts_us,
                track: format!("CPU Core {}", core),
                ph: "E".to_string(),
                category: "span".to_string(),
                name: cleaned_name,
                span_id: if span_id.is_empty() {
                    None
                } else {
                    Some(span_id.to_string())
                },
                parent_id: if parent_id.is_empty() {
                    None
                } else {
                    Some(parent_id.to_string())
                },
                module: module.filter(|m| !m.is_empty()).map(str::to_string),
                value: None,
                unit: None,
                args: serde_json::json!({ "core": core }),
            });
            return;
        };

        let task_key = self
            .task_key_for_span(&target_id)
            .unwrap_or_default()
            .to_string();
        if !self.is_span_on_task_stack(&task_key, &target_id) {
            return;
        }

        let was_running = self.running_task() == Some(task_key.as_str());
        if let Some(mut stack) = self.task_stacks.remove(&task_key) {
            if let Some((implicit_children, target_entry)) = stack.pop_through(&target_id) {
                for child_entry in &implicit_children {
                    let child_eff_id =
                        LiveSpanEntry::effective_id(&child_entry.span_id, &child_entry.name);
                    self.spans.remove(&child_eff_id);
                    if was_running {
                        out_events.push(Self::emit_span_event(
                            core,
                            ts_us,
                            "E",
                            child_entry,
                            Some("implicit"),
                        ));
                    }
                }
                let target_eff_id =
                    LiveSpanEntry::effective_id(&target_entry.span_id, &target_entry.name);
                self.spans.remove(&target_eff_id);
                if was_running {
                    out_events.push(Self::emit_span_event(core, ts_us, "E", &target_entry, None));
                }
            }
            if stack.is_empty() {
                if was_running {
                    self.running_task = None;
                }
            } else {
                self.task_stacks.insert(task_key.clone(), stack);
                if !was_running {
                    self.switch_to_task(core, &task_key, ts_us, true, out_events);
                }
            }
        }
    }
}

/// Stateful real-time RTT trace decoder converting `defmt` span and telemetry frames into [`crate::HostCliEvent::Trace`] events.
#[derive(Default)]
pub struct LiveTraceStreamProcessor {
    span_cores: std::collections::HashMap<String, i64>,
    core_spans: std::collections::HashMap<i64, LiveCoreSpanState>,
    cpu_integrators: std::collections::HashMap<i64, CoreCpuUsageIntegrator>,
}

impl LiveTraceStreamProcessor {
    /// Construct a new empty [`LiveTraceStreamProcessor`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Decode a single `defmt` frame (`plain_line` + `raw_message`) into zero or more [`crate::HostCliEvent::Trace`] events.
    pub fn process_defmt_frame(
        &mut self,
        plain_line: &str,
        raw_message: &str,
        module: Option<&str>,
        device_ts_us: Option<f64>,
    ) -> Vec<crate::HostCliEvent> {
        if let Some(parsed) = ParsedTracingLine::parse(plain_line) {
            return self.process_span_transition(&parsed, module, device_ts_us);
        }

        let telem_str = if raw_message.starts_with("Device Telemetry: ") {
            Some(raw_message)
        } else {
            plain_line
                .find("Device Telemetry: ")
                .map(|idx| &plain_line[idx..])
        };

        if let Some(msg) = telem_str {
            return self.process_device_telemetry(msg, device_ts_us);
        }

        Vec::new()
    }

    fn process_span_transition(
        &mut self,
        parsed: &ParsedTracingLine<'_>,
        module: Option<&str>,
        device_ts_us: Option<f64>,
    ) -> Vec<crate::HostCliEvent> {
        let ts_us = parsed
            .device_ts()
            .or(device_ts_us)
            .unwrap_or(0.0)
            .max(0.0)
            .round() as u64;
        let (span_id, parent_id) = parsed.parse_ids();
        let meta = CoreMetadata::new(parsed.span_name());

        let resolved_core = if let Some(c) = meta.core {
            c
        } else if !span_id.is_empty() {
            *self.span_cores.get(&span_id).unwrap_or(&0)
        } else if !parent_id.is_empty() {
            *self.span_cores.get(&parent_id).unwrap_or(&0)
        } else {
            0
        };

        if parsed.is_enter() && !span_id.is_empty() {
            self.span_cores.insert(span_id.clone(), resolved_core);
        }

        if meta.is_idle {
            let mut out_events = Vec::new();
            if parsed.is_enter() {
                self.core_spans
                    .entry(resolved_core)
                    .or_default()
                    .suspend_running(resolved_core, ts_us, &mut out_events);
            }
            let track = format!("Core {} CPU Usage (%)", resolved_core);
            let integ = self.cpu_integrators.entry(resolved_core).or_default();
            let samples = if parsed.is_enter() {
                integ.record_idle_enter(ts_us)
            } else {
                integ.record_idle_exit(ts_us)
            };
            out_events.extend(samples.into_iter().map(|(sample_ts, usage_pct)| {
                crate::HostCliEvent::Trace {
                    ts_us: sample_ts,
                    track: track.clone(),
                    ph: "C".to_string(),
                    category: "cpu_usage".to_string(),
                    name: track.clone(),
                    span_id: None,
                    parent_id: None,
                    module: None,
                    value: Some(usage_pct),
                    unit: Some("%".to_string()),
                    args: serde_json::json!({ "core": resolved_core, "value": usage_pct }),
                }
            }));
            return out_events;
        }

        let mut out_events = Vec::new();
        let usage_track = format!("Core {} CPU Usage (%)", resolved_core);
        let busy_samples = self
            .cpu_integrators
            .entry(resolved_core)
            .or_default()
            .record_busy_activity(ts_us);
        for (sample_ts, usage_pct) in busy_samples {
            out_events.push(crate::HostCliEvent::Trace {
                ts_us: sample_ts,
                track: usage_track.clone(),
                ph: "C".to_string(),
                category: "cpu_usage".to_string(),
                name: usage_track.clone(),
                span_id: None,
                parent_id: None,
                module: None,
                value: Some(usage_pct),
                unit: Some("%".to_string()),
                args: serde_json::json!({ "core": resolved_core, "value": usage_pct }),
            });
        }

        let mut cleaned_name = meta.name;
        if let Some(mod_str) = module {
            if (cleaned_name == "run" || cleaned_name == "task") && !mod_str.is_empty() {
                if let Some(seg) = mod_str.rsplit("::").find(|&s| is_root_segment(s)) {
                    cleaned_name = seg.to_string();
                }
            }
        }

        let core_state = self.core_spans.entry(resolved_core).or_default();
        if parsed.is_enter() {
            let is_root = is_root_or_empty_id(&parent_id);
            core_state.enter_span(
                resolved_core,
                ts_us,
                is_root,
                span_id,
                parent_id,
                cleaned_name,
                module,
                &mut out_events,
            );
        } else {
            core_state.exit_span(
                resolved_core,
                ts_us,
                &span_id,
                &parent_id,
                cleaned_name,
                module,
                &mut out_events,
            );
        }
        out_events
    }

    fn process_device_telemetry(
        &self,
        msg: &str,
        fallback_ts_us: Option<f64>,
    ) -> Vec<crate::HostCliEvent> {
        use tool_common::TelemetryParser as _;
        let Some(array_str) = msg.strip_prefix("Device Telemetry: ").map(str::trim) else {
            return Vec::new();
        };
        if !array_str.starts_with('[') || !array_str.ends_with(']') {
            return Vec::new();
        }
        let content = &array_str[1..array_str.len() - 1];
        let mut bytes = Vec::new();
        for part in content.split(',') {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            let Ok(byte) = trimmed.parse::<u8>() else {
                return Vec::new();
            };
            bytes.push(byte);
        }

        let parser =
            tool_common::TraceTelemetryParser::new(ThreadId::DeviceLogsAndTelemetry as u64);
        let Some((cbor_ts_us, record)) = parser.deserialize_raw_cbor(&bytes) else {
            return Vec::new();
        };
        let ts_us = if cbor_ts_us > 0 {
            cbor_ts_us
        } else {
            fallback_ts_us.unwrap_or(0.0).max(0.0).round() as u64
        };
        telemetry_record_to_trace_events(ts_us, &record)
    }
}

/// Convert a decoded [`model::telemetry::TelemetryRecord`] into structured [`crate::HostCliEvent::Trace`] events.
pub fn telemetry_record_to_trace_events(
    ts_us: u64,
    rec: &model::telemetry::TelemetryRecord,
) -> Vec<crate::HostCliEvent> {
    use model::telemetry::TelemetryRecord;
    match rec {
        TelemetryRecord::Proximity(model::types::SensorTelemetry::Status(dir, reading)) => {
            let dir_str = match dir {
                model::types::Direction::North => "North",
                model::types::Direction::East => "East",
                model::types::Direction::West => "West",
            };
            let track = format!("Proximity ({})", dir_str);
            match reading {
                model::types::SensorReading::Proximity(distance_mm) => {
                    vec![crate::HostCliEvent::Trace {
                        ts_us,
                        track: track.clone(),
                        ph: "C".to_string(),
                        category: "sensor".to_string(),
                        name: track,
                        span_id: None,
                        parent_id: None,
                        module: Some("controller::sensor_controller".to_string()),
                        value: Some(*distance_mm as f64),
                        unit: Some("mm".to_string()),
                        args: serde_json::json!({
                            "record_type": "Proximity",
                            "direction": dir_str,
                            "valid": true,
                            "value": distance_mm,
                            "csv": ["Valid", distance_mm.to_string(), dir_str, ""],
                        }),
                    }]
                }
                model::types::SensorReading::Invalid => {
                    vec![crate::HostCliEvent::Trace {
                        ts_us,
                        track: track.clone(),
                        ph: "i".to_string(),
                        category: "sensor".to_string(),
                        name: track,
                        span_id: None,
                        parent_id: None,
                        module: Some("controller::sensor_controller".to_string()),
                        value: None,
                        unit: Some("mm".to_string()),
                        args: serde_json::json!({
                            "record_type": "Proximity",
                            "direction": dir_str,
                            "valid": false,
                            "value": null,
                            "csv": ["Invalid", "0", dir_str, ""],
                        }),
                    }]
                }
            }
        }
        TelemetryRecord::Thermal(model::types::ThermalStatus::TempOverheating(
            temp_mc,
            overheating,
        )) => {
            vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "MCU / Board Thermal".to_string(),
                ph: "C".to_string(),
                category: "sensor".to_string(),
                name: "MCU Temperature (mC)".to_string(),
                span_id: None,
                parent_id: None,
                module: Some("controller::thermal_controller".to_string()),
                value: Some(*temp_mc as f64),
                unit: Some("mC".to_string()),
                args: serde_json::json!({
                    "record_type": "Thermal",
                    "value": temp_mc,
                    "temp_c": (*temp_mc as f64) / 1000.0,
                    "overheating": overheating,
                    "csv": [temp_mc.to_string(), overheating.to_string(), "", ""],
                }),
            }]
        }
        TelemetryRecord::Gesture(g) => {
            let g_str = match g {
                model::types::Gesture::DualLongPress(_) => "DualLongPress",
                model::types::Gesture::SingleTap(_) => "SingleTap",
                model::types::Gesture::DoubleTap(_) => "DoubleTap",
                model::types::Gesture::SwipeForward(_) => "SwipeForward",
                model::types::Gesture::SwipeBack(_) => "SwipeBack",
                model::types::Gesture::LongPress(_) => "LongPress",
                model::types::Gesture::ExtraLongPress(_) => "ExtraLongPress",
            };
            vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "Gesture & Touch".to_string(),
                ph: "i".to_string(),
                category: "sensor".to_string(),
                name: format!("Gesture::{}", g_str),
                span_id: None,
                parent_id: None,
                module: Some("controller::sensor_controller".to_string()),
                value: None,
                unit: None,
                args: serde_json::json!({
                    "record_type": "Gesture",
                    "gesture": g_str,
                    "csv": [g_str, "", "", ""],
                }),
            }]
        }
        TelemetryRecord::Battery(model::types::BatteryStatus::VolTempState(
            vol,
            temp,
            state,
            locks,
        )) => {
            let state_str = format!("{:?}", state);
            vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "System & Power Events".to_string(),
                ph: "i".to_string(),
                category: "telemetry".to_string(),
                name: "Battery Status".to_string(),
                span_id: None,
                parent_id: None,
                module: Some("controller::battery_controller".to_string()),
                value: Some(*vol as f64),
                unit: Some("mV".to_string()),
                args: serde_json::json!({
                    "record_type": "Battery",
                    "voltage_mv": vol,
                    "temp_mc": temp,
                    "state": state_str,
                    "active_locks": locks,
                    "csv": [vol.to_string(), temp.to_string(), state_str, locks.to_string()],
                }),
            }]
        }
        TelemetryRecord::FuelGauge(model::types::FuelGaugeTelemetry::VolSoc(vol, soc)) => {
            vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "System & Power Events".to_string(),
                ph: "i".to_string(),
                category: "telemetry".to_string(),
                name: "FuelGauge Voltage / SoC".to_string(),
                span_id: None,
                parent_id: None,
                module: Some("controller::battery_controller".to_string()),
                value: Some(*vol as f64),
                unit: Some("mV".to_string()),
                args: serde_json::json!({
                    "record_type": "FuelGauge",
                    "voltage_mv": vol,
                    "soc_pct": soc,
                    "csv": [vol.to_string(), soc.to_string(), "", ""],
                }),
            }]
        }
        TelemetryRecord::Motor(motor) => match motor {
            model::types::MotorStatus::Brake => vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "System & Power Events".to_string(),
                ph: "i".to_string(),
                category: "telemetry".to_string(),
                name: "Motor Brake".to_string(),
                span_id: None,
                parent_id: None,
                module: Some("controller::motor_controller".to_string()),
                value: Some(0.0),
                unit: Some("%".to_string()),
                args: serde_json::json!({
                    "record_type": "Motor",
                    "speed": 0,
                    "running": false,
                    "csv": ["0", "false", "25000", ""],
                }),
            }],
            model::types::MotorStatus::Running(telem) => vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "System & Power Events".to_string(),
                ph: "i".to_string(),
                category: "telemetry".to_string(),
                name: "Motor Telemetry".to_string(),
                span_id: None,
                parent_id: None,
                module: Some("controller::motor_controller".to_string()),
                value: Some(telem.measured_rpm as f64),
                unit: Some("RPM".to_string()),
                args: serde_json::json!({
                    "record_type": "Motor",
                    "speed": telem.speed.get(),
                    "running": true,
                    "current_ma": telem.current_ma,
                    "measured_rpm": telem.measured_rpm,
                    "estimated_rpm": telem.estimated_rpm,
                    "csv": [telem.speed.get().to_string(), "true", telem.current_ma.to_string(), telem.measured_rpm.to_string()],
                }),
            }],
        },
        TelemetryRecord::System(sys) => {
            let s_str = format!("{:?}", sys);
            vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "System & Power Events".to_string(),
                ph: "i".to_string(),
                category: "telemetry".to_string(),
                name: format!("System::{}", s_str),
                span_id: None,
                parent_id: None,
                module: Some("controller::system_controller".to_string()),
                value: None,
                unit: None,
                args: serde_json::json!({
                    "record_type": "System",
                    "status": s_str,
                    "csv": [s_str, "", "", ""],
                }),
            }]
        }
        TelemetryRecord::Led(led) => {
            let l_str = format!("{:?}", led);
            vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "System & Power Events".to_string(),
                ph: "i".to_string(),
                category: "telemetry".to_string(),
                name: format!("LED::{}", l_str),
                span_id: None,
                parent_id: None,
                module: Some("controller::led_controller".to_string()),
                value: None,
                unit: None,
                args: serde_json::json!({
                    "record_type": "Led",
                    "led": l_str,
                    "csv": [l_str, "", "", ""],
                }),
            }]
        }
        TelemetryRecord::FlashTelemetry(ft) => {
            vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "System & Power Events".to_string(),
                ph: "i".to_string(),
                category: "telemetry".to_string(),
                name: format!("Flash Erase s{}", ft.sector),
                span_id: None,
                parent_id: None,
                module: Some("controller::filesystem_controller".to_string()),
                value: Some(ft.duration_ms as f64),
                unit: Some("ms".to_string()),
                args: serde_json::json!({
                    "record_type": "FlashTelemetry",
                    "sector": ft.sector,
                    "duration_ms": ft.duration_ms,
                    "erase_count": ft.erase_count,
                    "csv": [ft.sector.to_string(), ft.duration_ms.to_string(), ft.erase_count.to_string(), ""],
                }),
            }]
        }
        TelemetryRecord::ChargerState(ch) => {
            let c_str = format!("{:?}", ch);
            vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "System & Power Events".to_string(),
                ph: "i".to_string(),
                category: "telemetry".to_string(),
                name: format!("Charger::{}", c_str),
                span_id: None,
                parent_id: None,
                module: Some("controller::battery_controller".to_string()),
                value: None,
                unit: None,
                args: serde_json::json!({
                    "record_type": "ChargerState",
                    "state": c_str,
                    "csv": [c_str, "", "", ""],
                }),
            }]
        }
        TelemetryRecord::PeripheralError(err) => {
            let err_str = format!("{:?}", err);
            vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "System & Power Events".to_string(),
                ph: "i".to_string(),
                category: "telemetry".to_string(),
                name: format!("PeripheralError::{}", err_str),
                span_id: None,
                parent_id: None,
                module: None,
                value: None,
                unit: None,
                args: serde_json::json!({
                    "record_type": "PeripheralError",
                    "error": err_str,
                    "csv": [err_str, "", "", ""],
                }),
            }]
        }
        TelemetryRecord::Boot(reason) => {
            let r_str = format!("{:?}", reason);
            vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "System & Power Events".to_string(),
                ph: "i".to_string(),
                category: "telemetry".to_string(),
                name: format!("Boot::{}", r_str),
                span_id: None,
                parent_id: None,
                module: None,
                value: None,
                unit: None,
                args: serde_json::json!({
                    "record_type": "Boot",
                    "reason": r_str,
                    "csv": [r_str, "", "", ""],
                }),
            }]
        }
        TelemetryRecord::PeriodicInterval(device, interval) => {
            let d_str = format!("{:?}", device);
            let i_str = match interval {
                model::types::PeriodicInterval::None => "None".to_string(),
                model::types::PeriodicInterval::UpdateMs(ms) => ms.to_string(),
            };
            vec![crate::HostCliEvent::Trace {
                ts_us,
                track: "System & Power Events".to_string(),
                ph: "i".to_string(),
                category: "telemetry".to_string(),
                name: format!("PeriodicInterval::{}", d_str),
                span_id: None,
                parent_id: None,
                module: None,
                value: None,
                unit: Some("ms".to_string()),
                args: serde_json::json!({
                    "record_type": "PeriodicInterval",
                    "device": d_str,
                    "interval": i_str,
                    "csv": [d_str, i_str, "", ""],
                }),
            }]
        }
    }
}

fn csv_escape_field(val: &str) -> String {
    if val.contains(',') || val.contains('"') || val.contains('\n') {
        format!("\"{}\"", val.replace('"', "\"\""))
    } else {
        val.to_string()
    }
}

fn format_perfetto_events_as_csv(events: &[serde_json::Value]) -> String {
    let mut out = String::from(
        "timestamp_us,track,phase,category,name,value,unit,record_type,val1,val2,val3,val4\n",
    );
    for ev in events {
        let Some(obj) = ev.as_object() else {
            continue;
        };
        let ph = obj.get("ph").and_then(|v| v.as_str()).unwrap_or("");
        if ph == "M" {
            continue;
        }
        let ts_us = obj
            .get("ts")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0)
            .round() as i64;
        let tid = obj.get("tid").and_then(|v| v.as_i64()).unwrap_or(1);
        let track = match tid {
            1 => "CPU Core 0",
            2 => "CPU Core 1",
            _ => "Device Logs & Telemetry",
        };
        let cat = obj.get("cat").and_then(|v| v.as_str()).unwrap_or("");
        let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let val = obj
            .get("args")
            .and_then(|a| a.get("value"))
            .map(|v| v.to_string())
            .unwrap_or_default();
        out.push_str(&format!(
            "{},{},{},{},{},{},,,,,,\n",
            ts_us,
            csv_escape_field(track),
            csv_escape_field(ph),
            csv_escape_field(cat),
            csv_escape_field(name),
            csv_escape_field(&val)
        ));
    }
    out
}

/// Export a slice of [`crate::HostCliEvent::Trace`] events into canonical CSV text.
pub fn export_trace_events_to_csv(events: &[crate::HostCliEvent]) -> String {
    let mut out = String::from(
        "timestamp_us,track,phase,category,name,value,unit,record_type,val1,val2,val3,val4\n",
    );
    for ev in events {
        if let crate::HostCliEvent::Trace {
            ts_us,
            track,
            ph,
            category,
            name,
            value,
            unit,
            args,
            ..
        } = ev
        {
            let val_str = value.map(|v| format!("{}", v)).unwrap_or_default();
            let unit_str = unit.as_deref().unwrap_or("");
            let rec_type = args
                .get("record_type")
                .and_then(|r| r.as_str())
                .unwrap_or(category.as_str());
            let mut csv_cols = [String::new(), String::new(), String::new(), String::new()];
            if let Some(arr) = args.get("csv").and_then(|c| c.as_array()) {
                for (idx, item) in arr.iter().take(4).enumerate() {
                    if let Some(s) = item.as_str() {
                        csv_cols[idx] = s.to_string();
                    } else if !item.is_null() {
                        csv_cols[idx] = item.to_string();
                    }
                }
            }
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                ts_us,
                csv_escape_field(track),
                csv_escape_field(ph),
                csv_escape_field(category),
                csv_escape_field(name),
                csv_escape_field(&val_str),
                csv_escape_field(unit_str),
                csv_escape_field(rec_type),
                csv_escape_field(&csv_cols[0]),
                csv_escape_field(&csv_cols[1]),
                csv_escape_field(&csv_cols[2]),
                csv_escape_field(&csv_cols[3]),
            ));
        }
    }
    out
}

/// Export a slice of [`crate::HostCliEvent::Trace`] events into Chrome / Perfetto JSON Trace Event Format.
pub fn export_trace_events_to_perfetto_json(
    events: &[crate::HostCliEvent],
    chip_name: &str,
) -> String {
    let mut trace_events = Vec::new();
    let proc_label = format!("{} MCU", chip_name.to_uppercase());
    trace_events.push(serde_json::json!({
        "name": "process_name",
        "ph": "M",
        "pid": 1,
        "args": { "name": proc_label }
    }));

    let mut track_tids: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
    track_tids.insert("CPU Core 0".to_string(), ThreadId::Core0 as i64);
    track_tids.insert("CPU Core 1".to_string(), ThreadId::Core1 as i64);
    track_tids.insert(
        "System & Power Events".to_string(),
        ThreadId::DeviceLogsAndTelemetry as i64,
    );
    let mut next_tid: i64 = 10;

    for (track_name, tid) in [
        ("CPU Core 0", ThreadId::Core0 as i64),
        ("CPU Core 1", ThreadId::Core1 as i64),
        (
            "System & Power Events",
            ThreadId::DeviceLogsAndTelemetry as i64,
        ),
    ] {
        trace_events.push(serde_json::json!({
            "name": "thread_name",
            "ph": "M",
            "pid": 1,
            "tid": tid,
            "args": { "name": track_name }
        }));
    }

    for ev in events {
        if let crate::HostCliEvent::Trace {
            ts_us,
            track,
            ph,
            category,
            name,
            span_id,
            parent_id,
            module,
            value,
            unit,
            args,
        } = ev
        {
            if ph == "C" {
                trace_events.push(serde_json::json!({
                    "cat": category,
                    "name": track,
                    "ph": "C",
                    "pid": 1,
                    "ts": *ts_us as f64,
                    "args": {
                        "value": value.unwrap_or(0.0),
                        "unit": unit,
                    }
                }));
                continue;
            }

            let tid = if let Some(&existing) = track_tids.get(track) {
                existing
            } else {
                let assigned = next_tid;
                next_tid += 1;
                track_tids.insert(track.clone(), assigned);
                trace_events.push(serde_json::json!({
                    "name": "thread_name",
                    "ph": "M",
                    "pid": 1,
                    "tid": assigned,
                    "args": { "name": track }
                }));
                assigned
            };

            let mut ev_args = args.clone();
            if let Some(obj) = ev_args.as_object_mut() {
                if let Some(m) = module {
                    obj.insert("module".to_string(), serde_json::json!(m));
                }
                if let Some(sid) = span_id {
                    obj.insert("span_id".to_string(), serde_json::json!(sid));
                }
                if let Some(pid_str) = parent_id {
                    obj.insert("parent_id".to_string(), serde_json::json!(pid_str));
                }
                if let Some(u) = unit {
                    obj.insert("unit".to_string(), serde_json::json!(u));
                }
            }

            trace_events.push(serde_json::json!({
                "cat": category,
                "name": name,
                "ph": ph,
                "pid": 1,
                "tid": tid,
                "ts": *ts_us as f64,
                "s": if ph == "i" { Some("t") } else { None },
                "args": ev_args,
            }));
        }
    }

    serde_json::to_string_pretty(&serde_json::json!({
        "traceEvents": trace_events,
        "displayTimeUnit": "ms"
    }))
    .unwrap_or_else(|_| "{\"traceEvents\":[]}".to_string())
}
