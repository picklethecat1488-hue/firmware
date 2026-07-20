use std::io::Read;

/// A minimal GDB Remote Serial Protocol client for connecting to an existing OpenOCD GDB server session.
pub struct GdbClient {
    stream: std::net::TcpStream,
}

impl GdbClient {
    /// Connects to a GDB server at the specified address (e.g. "localhost:3333").
    pub fn connect(addr: &str) -> Result<Self, String> {
        let stream = std::net::TcpStream::connect(addr)
            .map_err(|e| format!("Failed to connect to GDB server at {}: {:?}", addr, e))?;
        let mut client = Self { stream };
        // Perform initial handshake query
        let _ = client.send_packet("qSupported");
        let _ = client.read_packet();
        Ok(client)
    }

    /// Sends a formatted RSP packet and reads the ACK.
    pub fn send_packet(&mut self, payload: &str) -> Result<(), String> {
        use std::io::Write as _;
        let mut checksum: u8 = 0;
        for &b in payload.as_bytes() {
            checksum = checksum.wrapping_add(b);
        }
        let packet = format!("${}#{:02x}", payload, checksum);
        self.stream
            .write_all(packet.as_bytes())
            .map_err(|e| format!("GDB write failed: {:?}", e))?;

        // Read ACK '+'
        let mut ack = [0u8; 1];
        self.stream
            .read_exact(&mut ack)
            .map_err(|e| format!("GDB ACK read failed: {:?}", e))?;
        if ack[0] != b'+' {
            return Err(format!(
                "GDB server rejected packet: expected '+' but got {:?}",
                ack[0] as char
            ));
        }
        Ok(())
    }

    /// Reads a formatted RSP packet payload and sends an ACK.
    pub fn read_packet(&mut self) -> Result<String, String> {
        use std::io::Read as _;
        use std::io::Write as _;
        let mut buf = [0u8; 1];
        // Wait for start of packet '$'
        loop {
            self.stream
                .read_exact(&mut buf)
                .map_err(|e| format!("GDB start read failed: {:?}", e))?;
            if buf[0] == b'$' {
                break;
            }
        }
        // Read payload until '#'
        let mut payload = Vec::new();
        loop {
            self.stream
                .read_exact(&mut buf)
                .map_err(|e| format!("GDB payload read failed: {:?}", e))?;
            if buf[0] == b'#' {
                break;
            }
            payload.push(buf[0]);
        }
        // Read 2 checksum bytes
        let mut cksum = [0u8; 2];
        self.stream
            .read_exact(&mut cksum)
            .map_err(|e| format!("GDB checksum read failed: {:?}", e))?;

        // Send ACK '+'
        self.stream
            .write_all(b"+")
            .map_err(|e| format!("GDB ACK write failed: {:?}", e))?;

        String::from_utf8(payload).map_err(|e| format!("Invalid UTF-8 in GDB response: {:?}", e))
    }

    /// Reads memory from target RAM.
    pub fn read_mem(&mut self, mut addr: u64, mut buf: &mut [u8]) -> Result<(), String> {
        let chunk_size = 1024;
        while !buf.is_empty() {
            let read_len = std::cmp::min(buf.len(), chunk_size);
            let (chunk, rest) = buf.split_at_mut(read_len);

            let payload = format!("m{:x},{:x}", addr, read_len);
            self.send_packet(&payload)?;
            let response = self.read_packet()?;
            if response.starts_with('E') {
                return Err(format!(
                    "GDB memory read failed at 0x{:08X}: {}",
                    addr, response
                ));
            }
            if response.len() != read_len * 2 {
                return Err(format!(
                    "GDB memory read size mismatch at 0x{:08X}: expected {} hex chars, got {} (response payload: {:?})",
                    addr,
                    read_len * 2,
                    response.len(),
                    response
                ));
            }
            for i in 0..read_len {
                let byte_str = &response[i * 2..i * 2 + 2];
                chunk[i] = u8::from_str_radix(byte_str, 16)
                    .map_err(|e| format!("Failed to parse GDB hex byte: {:?}", e))?;
            }
            addr += read_len as u64;
            buf = rest;
        }
        Ok(())
    }

    /// Writes memory to target RAM/Flash.
    pub fn write_mem(&mut self, mut addr: u64, mut data: &[u8]) -> Result<(), String> {
        let chunk_size = 1024;
        while !data.is_empty() {
            let write_len = std::cmp::min(data.len(), chunk_size);
            let chunk = &data[..write_len];

            let mut hex_data = String::new();
            for &b in chunk {
                hex_data.push_str(&format!("{:02x}", b));
            }
            let payload = format!("M{:x},{:x}:{}", addr, write_len, hex_data);
            self.send_packet(&payload)?;
            let response = self.read_packet()?;
            if response != "OK" {
                return Err(format!(
                    "GDB memory write failed at 0x{:08X}: {}",
                    addr, response
                ));
            }
            addr += write_len as u64;
            data = &data[write_len..];
        }
        Ok(())
    }

    /// Halts the target CPU.
    pub fn halt(&mut self) -> Result<(), String> {
        use std::io::Write as _;
        // Send break byte (0x03 / Ctrl-C)
        self.stream
            .write_all(&[0x03])
            .map_err(|e| format!("GDB break send failed: {:?}", e))?;
        let _ = self.read_packet(); // Read stop reply
        Ok(())
    }

    /// Resumes target CPU execution.
    pub fn run(&mut self) -> Result<(), String> {
        self.send_packet("c")
    }

    /// Executes a target monitor command (via qRcmd) and drains all console output packets,
    /// returning the command's text output response.
    pub fn run_monitor_cmd(&mut self, cmd: &str) -> Result<String, String> {
        let mut hex_cmd = String::new();
        for &b in cmd.as_bytes() {
            hex_cmd.push_str(&format!("{:02x}", b));
        }
        self.send_packet(&format!("qRcmd,{}", hex_cmd))?;

        let mut console_output = String::new();
        loop {
            let response = self.read_packet()?;
            if response == "OK" || response.is_empty() {
                break;
            }
            if response.starts_with('E') {
                return Err(format!("Monitor command '{}' failed: {}", cmd, response));
            }
            if let Some(hex_part) = response.strip_prefix('O') {
                let mut bytes = Vec::new();
                for chunk in hex_part.as_bytes().chunks(2) {
                    if chunk.len() == 2 {
                        let byte_str = std::str::from_utf8(chunk).unwrap();
                        if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
                            bytes.push(byte);
                        }
                    }
                }
                console_output.push_str(&String::from_utf8_lossy(&bytes));
            } else {
                break;
            }
        }
        Ok(console_output)
    }

    /// Resets the target CPU using OpenOCD monitor reset commands.
    pub fn reset(&mut self) -> Result<(), String> {
        self.run_monitor_cmd("monitor reset halt")?;
        self.run_monitor_cmd("monitor reset run")?;
        Ok(())
    }
}
