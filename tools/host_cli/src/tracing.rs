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

/// Serializes processed events back to the trace file path.
fn save_processed_trace(
    path: &str,
    processed_events: Vec<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let serialized = serde_json::to_string_pretty(&processed_events)?;
    fs::write(path, serialized)?;
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

/// Pipeline stage that converts CPU Idle span events into a continuous
/// CPU Usage (%) counter track, removing the idle spans from the thread slice timeline.
pub struct CpuUsageGenerator;

impl TraceStage for CpuUsageGenerator {
    fn run(
        &mut self,
        events: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let mut final_events = Vec::new();
        let mut cores_seen = std::collections::HashSet::new();

        // 1. Find min_ts
        let mut min_ts = f64::MAX;
        for event in &events {
            if let Some(ts) = event.get("ts").and_then(|t| t.as_f64()) {
                if ts < min_ts {
                    min_ts = ts;
                }
            }
        }
        if min_ts == f64::MAX {
            min_ts = 0.0;
        }

        // 2. Process events
        for event in events {
            if let Some(obj) = event.as_object() {
                let name = obj.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let ph = obj.get("ph").and_then(|p| p.as_str()).unwrap_or("");
                let ts = obj.get("ts").and_then(|t| t.as_f64()).unwrap_or(0.0);

                if name == "CPU Idle Core 0" || name == "CPU Idle Core 1" {
                    let core = if name == "CPU Idle Core 0" { 0 } else { 1 };

                    // On the first idle event for a core, emit an initial 100% usage event at min_ts
                    if cores_seen.insert(core) && min_ts < ts {
                        let initial_event = serde_json::json!({
                            "cat": "cpu_usage",
                            "name": format!("Core {} CPU Usage (%)", core),
                            "ph": "C",
                            "pid": 1,
                            "ts": min_ts,
                            "args": {
                                "value": 100
                            }
                        });
                        final_events.push(initial_event);
                    }

                    let val = if ph == "B" { 0 } else { 100 };
                    let counter_event = serde_json::json!({
                        "cat": "cpu_usage",
                        "name": format!("Core {} CPU Usage (%)", core),
                        "ph": "C",
                        "pid": 1,
                        "ts": ts,
                        "args": {
                            "value": val
                        }
                    });
                    final_events.push(counter_event);

                    // Filter out the CPU Idle event itself
                    continue;
                }
            }
            final_events.push(event);
        }

        // Resort events by ts to ensure counter events are correctly ordered
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

        // 1. Check for timestamp at the very beginning
        if idx < words.len() {
            if let Ok(ts_sec) = words[idx].parse::<f64>() {
                device_ts = Some(ts_sec * 1_000_000.0);
                idx += 1;
            }
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
