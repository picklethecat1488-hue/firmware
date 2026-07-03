//! Library containing the core log streaming engine, project settings parsing,
//! and RTT source abstractions for log_tool.

use defmt_decoder::Table;
use std::io::Write;
use std::time::Duration;
pub use tool_common::decode_project_info;

/// Trait abstracting an RTT channel log source.
pub trait RttLogSource {
    /// Reads bytes from the RTT channel into the provided buffer.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String>;
}

/// Generic log streaming engine that reads raw defmt frames from an RttLogSource,
/// decodes them using the Table, and writes the plaintext logs to the writer.
pub fn stream_logs<S: RttLogSource, W: Write>(
    mut source: S,
    table: &Table,
    mut writer: W,
    poll_interval: Duration,
    should_exit: impl Fn() -> bool,
) -> Result<(), String> {
    let mut decoder = table.new_stream_decoder();
    let mut buf = [0u8; 1024];

    while !should_exit() {
        match source.read(&mut buf) {
            Ok(read_bytes) => {
                if read_bytes > 0 {
                    decoder.received(&buf[..read_bytes]);
                    loop {
                        match decoder.decode() {
                            Ok(frame) => {
                                if let Err(e) = writeln!(writer, "{}", frame.display(false)) {
                                    return Err(format!("Failed to write log: {:?}", e));
                                }
                                if let Err(e) = writer.flush() {
                                    return Err(format!("Failed to flush log: {:?}", e));
                                }
                            }
                            Err(defmt_decoder::DecodeError::UnexpectedEof) => break,
                            Err(defmt_decoder::DecodeError::Malformed) => continue,
                        }
                    }
                } else {
                    std::thread::sleep(poll_interval);
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
