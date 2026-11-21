//! and RTT source abstractions for host_cli.

pub mod dump;
pub mod events;
pub mod stream;
pub mod tracing;

pub use dump::dump_logs;
pub use events::{
    detect_shell_completion_event, extract_defmt_level, BootStatusPoller, CliChunkParser,
    HostCliEvent, ShellIdlePoller, TargetBootStatusReport,
};
pub use stream::{cancel_echo, stream_logs};
pub use tool_common::autodetect_project_info;
pub use tracing::{
    export_trace_events_to_csv, export_trace_events_to_perfetto_json, format_target_namespace,
    LiveCoreSpanState, LiveSpanEntry, LiveTaskSpanStack, LiveTraceStreamProcessor,
};

/// Trait abstracting a defmt log source.
pub trait DefmtLogSource {
    /// Reads bytes from the log source into the provided buffer.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String>;
}
