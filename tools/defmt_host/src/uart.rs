use defmt_decoder::Table;
use defmt_host::{dump_logs, stream_logs, DefmtLogSource};
use std::io;
use std::time::Duration;

struct SerialLogSource {
    port: Box<dyn serialport::SerialPort>,
}

impl DefmtLogSource for SerialLogSource {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        match self.port.read(buf) {
            Ok(n) => Ok(n),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(0),
            Err(e) if e.kind() == io::ErrorKind::TimedOut => Ok(0),
            Err(e) => Err(format!("Serial read failed: {:?}", e)),
        }
    }
}

pub fn run_uart(
    port_path: &str,
    baud: u32,
    table: &Table,
    dump: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Connecting to serial port '{}' at {} baud...",
        port_path, baud
    );
    let port = serialport::new(port_path, baud)
        .timeout(Duration::from_millis(10))
        .open()?;
    let source = SerialLogSource { port };

    if dump {
        println!("Draining buffered defmt logs from serial port:\n");
        dump_logs(source, table, io::stdout())?;
    } else {
        println!("Streaming defmt logs from serial port (Ctrl+C to stop):\n");
        stream_logs(
            source,
            table,
            io::stdout(),
            Duration::from_millis(10),
            || false, // Keep running forever
        )?;
    }

    Ok(())
}
