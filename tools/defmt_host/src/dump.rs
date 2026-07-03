use crate::RttLogSource;
use defmt_decoder::Table;
use std::io::Write;

/// Drains all currently buffered logs from the RTT source and writes them to the writer.
/// Exits as soon as a read returns 0 (source is empty).
pub fn dump_logs<S: RttLogSource, W: Write>(
    mut source: S,
    table: &Table,
    mut writer: W,
) -> Result<(), String> {
    let mut decoder = table.new_stream_decoder();
    let mut buf = [0u8; 1024];

    loop {
        match source.read(&mut buf) {
            Ok(read_bytes) => {
                if read_bytes == 0 {
                    break;
                }
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
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
