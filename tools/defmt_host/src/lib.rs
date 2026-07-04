//! Library containing the core log streaming engine, project settings parsing,
//! and RTT source abstractions for defmt_host.

pub mod dump;
pub mod stream;

pub use dump::dump_logs;
pub use stream::stream_logs;
pub use tool_common::decode_project_info;

/// Trait abstracting a defmt log source.
pub trait DefmtLogSource {
    /// Reads bytes from the log source into the provided buffer.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String>;
}
