//! Common utilities shared across target-attached host CLI tools.

use object::{Object, ObjectSection, ObjectSymbol};
use std::io::Read;

/// Autodetects chip and layout parameters from an ELF file's project metadata section.
pub fn autodetect_project_info(elf_path: &std::path::Path) -> Result<(String, u32, usize), String> {
    let elf_data = std::fs::read(elf_path).map_err(|e| {
        format!(
            "Failed to read ELF file at '{}': {:?}",
            elf_path.display(),
            e
        )
    })?;
    let file = object::File::parse(&*elf_data)
        .map_err(|e| format!("Failed to parse ELF file: {:?}", e))?;

    // Find the PROJECT_METADATA symbol in the ELF symbol table
    let mut symbol = None;
    for sym in file.symbols() {
        let name = sym.name();
        if name == Ok("PROJECT_METADATA") || name == Ok("_PROJECT_METADATA") {
            symbol = Some(sym);
            break;
        }
    }

    let symbol = symbol.ok_or_else(|| {
        "Could not find 'PROJECT_METADATA' symbol in ELF. Make sure the target binary includes the PROJECT_METADATA static.".to_string()
    })?;

    let address = symbol.address();

    // Find the section containing the symbol address
    let mut section_data = None;
    let mut section_address = 0;
    for sec in file.sections() {
        let start = sec.address();
        let size = sec.size();
        if address >= start && address < start + size {
            section_data = Some(
                sec.data()
                    .map_err(|e| format!("Failed to read section data: {:?}", e))?,
            );
            section_address = start;
            break;
        }
    }

    let section_data = section_data.ok_or_else(|| {
        format!(
            "Could not find ELF section containing symbol address 0x{:08X}",
            address
        )
    })?;

    let offset = (address - section_address) as usize;
    if offset + 52 > section_data.len() {
        return Err("Symbol address is out of section bounds".to_string());
    }

    let data = &section_data[offset..offset + 52];

    let magic = &data[0..8];
    if magic != b"PROJMET\0" {
        return Err("Invalid project metadata magic signature".to_string());
    }

    let version = u32::from_le_bytes(data[8..12].try_into().unwrap());
    if version != 1 {
        return Err(format!("Unsupported project metadata version: {}", version));
    }

    let chip_bytes = &data[12..44];
    let chip_len = chip_bytes.iter().position(|&b| b == 0).unwrap_or(32);
    let chip = std::str::from_utf8(&chip_bytes[..chip_len])
        .map_err(|e| format!("Invalid UTF-8 in chip name metadata: {:?}", e))?
        .to_string();

    let partition_address = u32::from_le_bytes(data[44..48].try_into().unwrap());
    let partition_size = u32::from_le_bytes(data[48..52].try_into().unwrap()) as usize;

    Ok((chip, partition_address, partition_size))
}

/// Finds the address of a symbol in the ELF file.
pub fn find_symbol_address(
    elf_path: &std::path::Path,
    symbol_name: &str,
) -> Result<Option<u64>, String> {
    let elf_data = std::fs::read(elf_path).map_err(|e| {
        format!(
            "Failed to read ELF file at '{}': {:?}",
            elf_path.display(),
            e
        )
    })?;
    let file = object::File::parse(&*elf_data)
        .map_err(|e| format!("Failed to parse ELF file: {:?}", e))?;

    for sym in file.symbols() {
        if sym.name() == Ok(symbol_name) {
            return Ok(Some(sym.address()));
        }
    }
    Ok(None)
}

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
            if response.starts_with('O') {
                let hex_part = &response[1..];
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
