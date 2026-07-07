use defmt_decoder::Table;
use host_cli::{dump_logs, DefmtLogSource};
use probe_rs::probe::list::Lister;
use probe_rs::MemoryInterface as _;
use std::io;
use std::time::Duration;
use tool_common::GdbClient;

pub trait TargetAccess {
    fn read_mem(&mut self, addr: u64, buf: &mut [u8]) -> Result<(), String>;
    fn write_mem(&mut self, addr: u64, buf: &[u8]) -> Result<(), String>;
    fn reset(&mut self) -> Result<(), String>;
    fn run(&mut self) -> Result<(), String>;
}

struct ProbeRsTarget<'a, 'b> {
    core: &'a mut probe_rs::Core<'b>,
}

impl<'a, 'b> TargetAccess for ProbeRsTarget<'a, 'b> {
    fn read_mem(&mut self, addr: u64, buf: &mut [u8]) -> Result<(), String> {
        self.core.read_8(addr, buf).map_err(|e| format!("{:?}", e))
    }
    fn write_mem(&mut self, addr: u64, buf: &[u8]) -> Result<(), String> {
        self.core.write_8(addr, buf).map_err(|e| format!("{:?}", e))
    }
    fn reset(&mut self) -> Result<(), String> {
        self.core.reset().map_err(|e| format!("{:?}", e))
    }
    fn run(&mut self) -> Result<(), String> {
        self.core.run().map_err(|e| format!("{:?}", e))
    }
}

impl TargetAccess for GdbClient {
    fn read_mem(&mut self, addr: u64, buf: &mut [u8]) -> Result<(), String> {
        self.read_mem(addr, buf)
    }
    fn write_mem(&mut self, addr: u64, buf: &[u8]) -> Result<(), String> {
        self.write_mem(addr, buf)
    }
    fn reset(&mut self) -> Result<(), String> {
        self.reset()
    }
    fn run(&mut self) -> Result<(), String> {
        self.run()
    }
}

pub struct RttChannel {
    name: String,
    buffer_ptr: u64,
    size: usize,
    write_offset_ptr: u64,
    read_offset_ptr: u64,
}

impl RttChannel {
    pub fn name(&self) -> Option<&str> {
        if self.name.is_empty() {
            None
        } else {
            Some(&self.name)
        }
    }

    pub fn read<T: TargetAccess + ?Sized>(
        &self,
        target: &mut T,
        buf: &mut [u8],
    ) -> Result<usize, String> {
        let mut wr_bytes = [0u8; 4];
        let mut rd_bytes = [0u8; 4];
        target.read_mem(self.write_offset_ptr, &mut wr_bytes)?;
        target.read_mem(self.read_offset_ptr, &mut rd_bytes)?;
        let write = u32::from_le_bytes(wr_bytes) as usize;
        let read = u32::from_le_bytes(rd_bytes) as usize;

        if write == read {
            return Ok(0);
        }

        let buf_len = buf.len();
        let len = if write > read {
            let bytes_to_read = std::cmp::min(buf_len, write - read);
            target.read_mem(self.buffer_ptr + read as u64, &mut buf[..bytes_to_read])?;
            bytes_to_read
        } else {
            let first_part = self.size - read;
            if buf_len <= first_part {
                target.read_mem(self.buffer_ptr + read as u64, &mut buf[..buf_len])?;
                buf_len
            } else {
                target.read_mem(self.buffer_ptr + read as u64, &mut buf[..first_part])?;
                let second_part = std::cmp::min(buf_len - first_part, write);
                target.read_mem(
                    self.buffer_ptr,
                    &mut buf[first_part..first_part + second_part],
                )?;
                first_part + second_part
            }
        };

        let new_read = (read + len) % self.size;
        target.write_mem(self.read_offset_ptr, &u32::to_le_bytes(new_read as u32))?;
        Ok(len)
    }

    pub fn write<T: TargetAccess + ?Sized>(
        &self,
        target: &mut T,
        buf: &[u8],
    ) -> Result<usize, String> {
        let mut wr_bytes = [0u8; 4];
        let mut rd_bytes = [0u8; 4];
        target.read_mem(self.write_offset_ptr, &mut wr_bytes)?;
        target.read_mem(self.read_offset_ptr, &mut rd_bytes)?;
        let write = u32::from_le_bytes(wr_bytes) as usize;
        let read = u32::from_le_bytes(rd_bytes) as usize;

        let available = if read > write {
            read - write - 1
        } else {
            self.size - 1 - (write - read)
        };

        if available == 0 {
            return Ok(0);
        }

        let len = std::cmp::min(buf.len(), available);
        if write + len <= self.size {
            target.write_mem(self.buffer_ptr + write as u64, &buf[..len])?;
        } else {
            let first_part = self.size - write;
            target.write_mem(self.buffer_ptr + write as u64, &buf[..first_part])?;
            let second_part = len - first_part;
            target.write_mem(self.buffer_ptr, &buf[first_part..first_part + second_part])?;
        }

        let new_write = (write + len) % self.size;
        target.write_mem(self.write_offset_ptr, &u32::to_le_bytes(new_write as u32))?;
        Ok(len)
    }
}

pub fn parse_rtt_channels<T: TargetAccess + ?Sized>(
    target: &mut T,
    rtt_symbol_addr: u64,
) -> Result<(Vec<RttChannel>, Vec<RttChannel>), String> {
    let mut header = [0u8; 128];
    target.read_mem(rtt_symbol_addr, &mut header)?;

    let max_up = u32::from_le_bytes(header[16..20].try_into().unwrap()) as usize;
    let max_down = u32::from_le_bytes(header[20..24].try_into().unwrap()) as usize;

    let mut up_channels = Vec::new();
    let mut down_channels = Vec::new();

    let mut offset = 24;
    for _ in 0..max_up {
        let chunk = &header[offset..offset + 24];
        let name_ptr = u32::from_le_bytes(chunk[0..4].try_into().unwrap()) as u64;
        let buffer_ptr = u32::from_le_bytes(chunk[4..8].try_into().unwrap()) as u64;
        let size = u32::from_le_bytes(chunk[8..12].try_into().unwrap()) as usize;
        let write_offset_ptr = rtt_symbol_addr + offset as u64 + 12;
        let read_offset_ptr = rtt_symbol_addr + offset as u64 + 16;
        offset += 24;

        let mut name = String::new();
        if name_ptr != 0 {
            let mut name_buf = [0u8; 32];
            if target.read_mem(name_ptr, &mut name_buf).is_ok() {
                if let Some(pos) = name_buf.iter().position(|&b| b == 0) {
                    name = String::from_utf8_lossy(&name_buf[..pos]).into_owned();
                }
            }
        }
        up_channels.push(RttChannel {
            name,
            buffer_ptr,
            size,
            write_offset_ptr,
            read_offset_ptr,
        });
    }

    for _ in 0..max_down {
        let chunk = &header[offset..offset + 24];
        let name_ptr = u32::from_le_bytes(chunk[0..4].try_into().unwrap()) as u64;
        let buffer_ptr = u32::from_le_bytes(chunk[4..8].try_into().unwrap()) as u64;
        let size = u32::from_le_bytes(chunk[8..12].try_into().unwrap()) as usize;
        let write_offset_ptr = rtt_symbol_addr + offset as u64 + 12;
        let read_offset_ptr = rtt_symbol_addr + offset as u64 + 16;
        offset += 24;

        let mut name = String::new();
        if name_ptr != 0 {
            let mut name_buf = [0u8; 32];
            if target.read_mem(name_ptr, &mut name_buf).is_ok() {
                if let Some(pos) = name_buf.iter().position(|&b| b == 0) {
                    name = String::from_utf8_lossy(&name_buf[..pos]).into_owned();
                }
            }
        }
        down_channels.push(RttChannel {
            name,
            buffer_ptr,
            size,
            write_offset_ptr,
            read_offset_ptr,
        });
    }

    Ok((up_channels, down_channels))
}

struct TargetRttSource<'a, T: TargetAccess + ?Sized> {
    channel: &'a RttChannel,
    target: &'a mut T,
}

impl<'a, T: TargetAccess + ?Sized> DefmtLogSource for TargetRttSource<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        self.channel.read(self.target, buf)
    }
}

pub fn run_rtt(
    chip: &str,
    table: Option<&Table>,
    elf_path: &std::path::Path,
    dump: bool,
    raw: bool,
    dump_mem: bool,
    no_reset: bool,
    openocd_host: Option<&str>,
    show_raw_cli: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Locate Symbol Address first
    let rtt_symbol_addr = match tool_common::find_symbol_address(elf_path, "_SEGGER_RTT") {
        Ok(Some(addr)) => {
            println!("Found RTT control block at symbol address 0x{:08X}", addr);
            addr
        }
        _ => return Err("Could not locate symbol address of _SEGGER_RTT in ELF".into()),
    };

    // 1. Connection selection
    let mut gdb_client_store = None;
    let mut probe_rs_session_store = None;

    if let Some(host) = openocd_host {
        let addr = if host.contains(':') {
            host.to_string()
        } else {
            format!("{}:3333", host)
        };
        println!("Connecting to existing OpenOCD GDB session at {}...", addr);
        let client = GdbClient::connect(&addr)?;
        gdb_client_store = Some(client);
    } else {
        println!("Connecting to probe for chip '{}'...", chip);
        let lister = Lister::new();
        let probes = lister.list_all();
        let probe_info = probes.first().ok_or("No debug probes connected")?;
        let probe = probe_info.open()?;

        let mut session = probe.attach(chip, probe_rs::Permissions::default())?;
        {
            let mut core = session.core(0)?;
            if !no_reset {
                println!("Resetting target CPU...");
                core.reset()?;
                std::thread::sleep(Duration::from_millis(200));
            } else {
                let status = core.status()?;
                println!("Core status: {:?}", status);
                if status.is_halted() {
                    println!("Core is halted. Resuming execution...");
                    core.run()?;
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
        probe_rs_session_store = Some(session);
    }

    // Set up unified interface
    let mut target: Box<dyn TargetAccess> = if let Some(ref mut client) = gdb_client_store {
        Box::new(ProbeRsTargetWrapper { client })
    } else if let Some(ref mut session) = probe_rs_session_store {
        let core = session.core(0)?;
        // Storing session/core references presents a lifetime challenge, so we wrap it
        // using raw pointer to core in ProbeRsWrapper, safe because session is kept alive in probe_rs_session_store
        Box::new(ProbeRsWrapper::new(core))
    } else {
        unreachable!();
    };

    if dump_mem {
        println!("\n=== Dumping raw _SEGGER_RTT control block ===");
        let mut rtt_header_mem = vec![0u8; 128];
        target.read_mem(rtt_symbol_addr, &mut rtt_header_mem)?;

        println!(
            "Header memory dump (128 bytes at 0x{:08X}):",
            rtt_symbol_addr
        );
        print_hex_ascii_dump(rtt_symbol_addr, &rtt_header_mem);
        return Ok(());
    }

    let (up_channels, down_channels) = parse_rtt_channels(target.as_mut(), rtt_symbol_addr)?;

    // Identify channels
    let mut defmt_channel = None;
    let mut cli_up_channel = None;
    let mut cli_down_channel = None;

    for chan in &up_channels {
        if chan.name() == Some("defmt") {
            defmt_channel = Some(chan);
        } else if chan.name() == Some("cli") {
            cli_up_channel = Some(chan);
        }
    }

    for chan in &down_channels {
        if chan.name() == Some("cli") {
            cli_down_channel = Some(chan);
        }
    }

    if raw {
        if cli_up_channel.is_none() && !up_channels.is_empty() {
            cli_up_channel = Some(&up_channels[0]);
        }
        if cli_down_channel.is_none() && !down_channels.is_empty() {
            cli_down_channel = Some(&down_channels[0]);
        }
    }

    println!("RTT connected. Active channels:");
    if defmt_channel.is_some() {
        println!("  - Up Channel: \"defmt\" (logs)");
    }
    if cli_up_channel.is_some() {
        println!("  - Up Channel: \"cli\" (raw output)");
    }
    if cli_down_channel.is_some() {
        println!("  - Down Channel: \"cli\" (raw input)");
    }

    // Set up defmt decoder
    let mut decoder = if let Some(table) = table {
        Some(table.new_stream_decoder())
    } else {
        None
    };

    let (stdin_tx, stdin_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    if cli_down_channel.is_some() {
        use std::io::Read;
        std::thread::spawn(move || {
            let mut stdin = std::io::stdin();
            let mut buf = [0u8; 128];
            loop {
                match stdin.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        let _ = stdin_tx.send(buf[..n].to_vec());
                    }
                    _ => break,
                }
            }
        });
    }

    // Dump initial CLI output
    if let Some(cli_up) = cli_up_channel {
        let mut initial_buf = [0u8; 1024];
        match cli_up.read(target.as_mut(), &mut initial_buf) {
            Ok(n) if n > 0 => {
                use std::io::Write;
                if show_raw_cli {
                    println!("\n--- Raw CLI Buffer Dump ({} bytes cached) ---", n);
                    for (i, chunk) in initial_buf[..n].chunks(16).enumerate() {
                        let hex_string: Vec<String> =
                            chunk.iter().map(|b| format!("{:02X}", b)).collect();
                        let hex_part = hex_string.join(" ");
                        let ascii_part: String = chunk
                            .iter()
                            .map(|&b| {
                                if (32..=126).contains(&b) {
                                    b as char
                                } else {
                                    '.'
                                }
                            })
                            .collect();
                        println!("0x{:04X}: {:48} | {}", i * 16, hex_part, ascii_part);
                    }
                    println!("--- End of Dump ---\n");
                }

                let _ = io::stdout().write_all(&initial_buf[..n]);
                let _ = io::stdout().flush();
            }
            _ => {}
        }
    }

    println!("\nRTT Session Started (Press Ctrl+C to exit):");
    if cli_up_channel.is_some() && cli_down_channel.is_some() {
        println!("Interactive RTT console active. Type commands and press Enter.");
    }
    println!();

    if dump {
        if let Some(chan) = defmt_channel {
            let source = TargetRttSource {
                channel: chan,
                target: target.as_mut(),
            };
            let table_ref = table.ok_or("Logging mode requires a valid ELF defmt table")?;
            println!("Draining buffered defmt logs:\n");
            dump_logs(source, table_ref, io::stdout())?;
        }
        return Ok(());
    }

    let mut rtt_buf = [0u8; 1024];
    loop {
        let mut did_work = false;

        // 1. Poll defmt logs
        if let Some(chan) = defmt_channel {
            match chan.read(target.as_mut(), &mut rtt_buf) {
                Ok(n) if n > 0 => {
                    did_work = true;
                    if let Some(ref mut dec) = decoder {
                        dec.received(&rtt_buf[..n]);
                        loop {
                            match dec.decode() {
                                Ok(frame) => {
                                    println!("{}", frame.display(true));
                                }
                                Err(defmt_decoder::DecodeError::UnexpectedEof) => break,
                                Err(defmt_decoder::DecodeError::Malformed) => {
                                    eprintln!("Error: malformed defmt frame");
                                    continue;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("defmt RTT read error: {:?}", e);
                    break;
                }
                _ => {}
            }
        }

        // 2. Poll CLI output
        if let Some(cli_up) = cli_up_channel {
            match cli_up.read(target.as_mut(), &mut rtt_buf) {
                Ok(n) if n > 0 => {
                    use std::io::Write;
                    did_work = true;
                    let _ = std::io::stdout().write_all(&rtt_buf[..n]);
                    let _ = std::io::stdout().flush();
                }
                Err(e) => {
                    eprintln!("CLI RTT read error: {:?}", e);
                    break;
                }
                _ => {}
            }
        }

        // 3. Poll CLI input
        if let Some(cli_down) = cli_down_channel {
            if let Ok(input_bytes) = stdin_rx.try_recv() {
                did_work = true;
                let mut written = 0;
                while written < input_bytes.len() {
                    match cli_down.write(target.as_mut(), &input_bytes[written..]) {
                        Ok(n) => written += n,
                        Err(e) => {
                            eprintln!("CLI RTT write error: {:?}", e);
                            break;
                        }
                    }
                }
            }
        }

        if !did_work {
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    Ok(())
}

struct ProbeRsTargetWrapper<'a> {
    client: &'a mut GdbClient,
}

impl<'a> TargetAccess for ProbeRsTargetWrapper<'a> {
    fn read_mem(&mut self, addr: u64, buf: &mut [u8]) -> Result<(), String> {
        self.client.read_mem(addr, buf)
    }
    fn write_mem(&mut self, addr: u64, buf: &[u8]) -> Result<(), String> {
        self.client.write_mem(addr, buf)
    }
    fn reset(&mut self) -> Result<(), String> {
        self.client.reset()
    }
    fn run(&mut self) -> Result<(), String> {
        self.client.run()
    }
}

struct ProbeRsWrapper<'b> {
    core_ptr: *mut probe_rs::Core<'b>,
    _phantom: std::marker::PhantomData<&'b mut probe_rs::Core<'b>>,
}

impl<'b> ProbeRsWrapper<'b> {
    fn new(core: probe_rs::Core<'b>) -> Self {
        // Keep a pointer to core, safe because the session owns the core and is kept alive in run_rtt
        let core_box = Box::new(core);
        let core_ptr = Box::into_raw(core_box);
        Self {
            core_ptr,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'b> Drop for ProbeRsWrapper<'b> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.core_ptr);
        }
    }
}

impl<'b> TargetAccess for ProbeRsWrapper<'b> {
    fn read_mem(&mut self, addr: u64, buf: &mut [u8]) -> Result<(), String> {
        let core = unsafe { &mut *self.core_ptr };
        core.read_8(addr, buf).map_err(|e| format!("{:?}", e))
    }
    fn write_mem(&mut self, addr: u64, buf: &[u8]) -> Result<(), String> {
        let core = unsafe { &mut *self.core_ptr };
        core.write_8(addr, buf).map_err(|e| format!("{:?}", e))
    }
    fn reset(&mut self) -> Result<(), String> {
        let core = unsafe { &mut *self.core_ptr };
        core.reset().map_err(|e| format!("{:?}", e))
    }
    fn run(&mut self) -> Result<(), String> {
        let core = unsafe { &mut *self.core_ptr };
        core.run().map_err(|e| format!("{:?}", e))
    }
}

fn print_hex_ascii_dump(start_addr: u64, data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        let hex_string: Vec<String> = chunk.iter().map(|b| format!("{:02X}", b)).collect();
        let hex_part = hex_string.join(" ");
        let ascii_part: String = chunk
            .iter()
            .map(|&b| {
                if (32..=126).contains(&b) {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();
        println!(
            "0x{:08X}: {:48} | {}",
            start_addr + (i * 16) as u64,
            hex_part,
            ascii_part
        );
    }
}
