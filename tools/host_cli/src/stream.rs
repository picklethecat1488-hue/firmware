use crate::DefmtLogSource;
use defmt_decoder::Table;
use std::io::Write;
use std::time::Duration;

/// Generic log streaming engine that reads raw defmt frames from a DefmtLogSource,
/// decodes them using the Table, and writes the plaintext logs to the writer.
pub fn stream_logs<S: DefmtLogSource, W: Write>(
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
                            Err(defmt_decoder::DecodeError::UnexpectedEof) => {
                                break;
                            }
                            Err(defmt_decoder::DecodeError::Malformed) => {
                                eprintln!("Error: malformed frame received (possible target/ELF mismatch or log buffer corruption)");
                                continue;
                            }
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

/// Cancels echoed input characters from target RTT buffer against typed input buffer.
///
/// Strips matching characters and line-ending sequences from `rtt_bytes` when they match
/// pending bytes in `sent_buffer`, preventing host-transmitted keystrokes from echoing back
/// into the interactive console stream.
pub fn cancel_echo(rtt_bytes: &[u8], sent_buffer: &mut Vec<u8>) -> Vec<u8> {
    let mut write_buf = Vec::with_capacity(rtt_bytes.len());
    for &b in rtt_bytes {
        if !sent_buffer.is_empty()
            && (sent_buffer[0] == b
                || ((b == b'\r' || b == b'\n')
                    && (sent_buffer[0] == b'\r' || sent_buffer[0] == b'\n')))
        {
            sent_buffer.remove(0);
        } else {
            write_buf.push(b);
        }
    }
    write_buf
}
