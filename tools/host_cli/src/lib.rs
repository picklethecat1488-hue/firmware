//! and RTT source abstractions for host_cli.

pub mod dump;
pub mod stream;

pub use dump::dump_logs;
pub use stream::stream_logs;
pub use tool_common::autodetect_project_info;

/// Trait abstracting a defmt log source.
pub trait DefmtLogSource {
    /// Reads bytes from the log source into the provided buffer.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String>;
}
