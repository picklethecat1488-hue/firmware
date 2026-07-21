use crate::tracing::handle_tracing_line;
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
}

impl TargetAccess for GdbClient {
    fn read_mem(&mut self, addr: u64, buf: &mut [u8]) -> Result<(), String> {
        self.read_mem(addr, buf)
    }
    fn write_mem(&mut self, addr: u64, buf: &[u8]) -> Result<(), String> {
        self.write_mem(addr, buf)
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

fn read_elf_addr(file: &object::File, addr: u64, len: usize) -> Option<Vec<u8>> {
    use object::{Object, ObjectSection};
    for section in file.sections() {
        let start = section.address();
        let size = section.size();
        if addr >= start && addr < start + size {
            if let Ok(data) = section.data() {
                let offset = (addr - start) as usize;
                let end = (offset + len).min(data.len());
                return Some(data[offset..end].to_vec());
            }
        }
    }
    None
}

pub fn parse_rtt_channels<T: TargetAccess + ?Sized>(
    target: &mut T,
    rtt_symbol_addr: u64,
    elf_file: Option<&object::File>,
) -> Result<(Vec<RttChannel>, Vec<RttChannel>), String> {
    let mut initial_header = [0u8; 24];
    target.read_mem(rtt_symbol_addr, &mut initial_header)?;

    let max_up = u32::from_le_bytes(initial_header[16..20].try_into().unwrap()) as usize;
    let max_down = u32::from_le_bytes(initial_header[20..24].try_into().unwrap()) as usize;

    if max_up > 32 || max_down > 32 {
        return Err(format!(
            "RTT control block has invalid channel counts: up={}, down={}",
            max_up, max_down
        ));
    }

    let full_size = 24 + 24 * max_up + 24 * max_down;
    let mut header = vec![0u8; full_size];
    target.read_mem(rtt_symbol_addr, &mut header)?;

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
            let mut name_read = false;
            if let Some(elf) = elf_file {
                if let Some(bytes) = read_elf_addr(elf, name_ptr, 32) {
                    if let Some(pos) = bytes.iter().position(|&b| b == 0) {
                        name = String::from_utf8_lossy(&bytes[..pos]).into_owned();
                        name_read = true;
                    }
                }
            }

            if !name_read {
                let mut name_buf = [0u8; 32];
                if target.read_mem(name_ptr, &mut name_buf).is_ok() {
                    if let Some(pos) = name_buf.iter().position(|&b| b == 0) {
                        name = String::from_utf8_lossy(&name_buf[..pos]).into_owned();
                    }
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
            let mut name_read = false;
            if let Some(elf) = elf_file {
                if let Some(bytes) = read_elf_addr(elf, name_ptr, 32) {
                    if let Some(pos) = bytes.iter().position(|&b| b == 0) {
                        name = String::from_utf8_lossy(&bytes[..pos]).into_owned();
                        name_read = true;
                    }
                }
            }

            if !name_read {
                let mut name_buf = [0u8; 32];
                if target.read_mem(name_ptr, &mut name_buf).is_ok() {
                    if let Some(pos) = name_buf.iter().position(|&b| b == 0) {
                        name = String::from_utf8_lossy(&name_buf[..pos]).into_owned();
                    }
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

/// Options for configuring RTT session runner.
pub struct RttOptions<'a> {
    /// Target chip identifier (e.g. RP2040)
    pub chip: &'a str,
    /// Parsed defmt decoder table
    pub table: Option<&'a Table>,
    /// Path to target ELF binary
    pub elf_path: &'a std::path::Path,
    /// Mode to dump logs in non-interactive batch
    pub dump: bool,
    /// Output raw bytes instead of formatted logs
    pub raw: bool,
    /// Dump logs from memory partition
    pub dump_mem: bool,
    /// Reset target on connection
    pub reset: bool,
    /// Host for openocd connection
    pub openocd_host: Option<&'a str>,
    /// Output raw console telemetry
    pub show_raw_cli: bool,
    /// Terminal progress bar spinner
    pub spinner: &'a indicatif::ProgressBar,
    /// Channel mode mapping for RTT
    pub channel_mode: crate::ChannelMode,
    /// Enable tracing reconstruction
    pub trace: bool,
    /// Trace collection duration in seconds
    pub duration: Option<u64>,
}

pub fn run_rtt(opts: RttOptions<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let mut running = true;

    let start_time = std::time::Instant::now();

    let RttOptions {
        chip,
        table,
        elf_path,
        dump,
        raw,
        dump_mem,
        reset,
        openocd_host,
        show_raw_cli,
        spinner,
        channel_mode,
        trace,
        duration,
    } = opts;
    // Locate Symbol Address first
    let rtt_symbol_addr = match tool_common::find_symbol_address(elf_path, "_SEGGER_RTT") {
        Ok(Some(addr)) => {
            spinner.set_message(format!(
                "Found RTT control block at symbol address 0x{:08X}",
                addr
            ));
            addr
        }
        _ => return Err("Could not locate symbol address of _SEGGER_RTT in ELF".into()),
    };

    // Load debug symbols for on-the-fly backtrace symbolication
    let elf_data = std::fs::read(elf_path)?;
    let object_file = object::File::parse(&*elf_data).ok();
    let addr2line_ctx = object_file
        .as_ref()
        .and_then(|obj| addr2line::Context::new(obj).ok());

    // Spawn the stdin reader thread once (if interactive CLI is enabled)
    let (stdin_tx, stdin_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let stream_cli =
        channel_mode == crate::ChannelMode::Cli || channel_mode == crate::ChannelMode::Both;
    if stream_cli && !dump {
        use std::io::Read;
        std::thread::spawn(move || {
            let mut stdin = std::io::stdin();
            let mut buf = [0u8; 128];
            loop {
                match stdin.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        if stdin_tx.send(buf[..n].to_vec()).is_err() {
                            break;
                        }
                    }
                    _ => break,
                }
            }
        });
    }

    let mut is_reconnecting = false;
    loop {
        if !running {
            break;
        }
        // 1. Connection selection
        let mut gdb_client_store = None;
        let mut probe_rs_session_store = None;

        // Reset spinner state if we are retrying
        spinner.reset();
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let connection_res = if let Some(host) = openocd_host {
            let addr = if host.contains(':') {
                host.to_string()
            } else {
                format!("{}:3333", host)
            };
            spinner.set_message(format!("Connecting to OpenOCD GDB session at {}...", addr));
            match GdbClient::connect(&addr) {
                Ok(client) => {
                    gdb_client_store = Some(client);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            spinner.set_message(format!("Connecting to probe for chip '{}'...", chip));
            let lister = Lister::new();
            let probes = lister.list_all();
            match probes
                .first()
                .ok_or_else(|| "No debug probes connected".to_string())
                .and_then(|info| info.open().map_err(|e| format!("{:?}", e)))
            {
                Ok(probe) => match probe.attach(chip, probe_rs::Permissions::default()) {
                    Ok(mut session) => {
                        {
                            let mut core = session.core(0)?;
                            if reset && !is_reconnecting {
                                spinner.set_message("Resetting target CPU...");
                                let _ = core.reset();
                                std::thread::sleep(Duration::from_millis(100));
                                let _ = core.run();
                                std::thread::sleep(Duration::from_millis(100));
                            } else {
                                let status = core.status()?;
                                spinner.set_message(format!("Core status: {:?}", status));
                                if status.is_halted() {
                                    spinner.set_message("Core is halted. Resuming execution...");
                                    let _ = core.run();
                                    std::thread::sleep(Duration::from_millis(100));
                                }
                            }
                            is_reconnecting = true;
                        }
                        probe_rs_session_store = Some(session);
                        Ok(())
                    }
                    Err(e) => Err(format!("{:?}", e)),
                },
                Err(e) => Err(e),
            }
        };

        if let Err(err) = connection_res {
            if dump {
                return Err(format!("Connection failed: {}", err).into());
            }
            spinner.set_message(format!("Connection failed: {}. Retrying in 1s...", err));
            std::thread::sleep(Duration::from_secs(1));
            continue;
        }

        // Set up unified interface
        let mut target: Box<dyn TargetAccess> = if let Some(ref mut client) = gdb_client_store {
            Box::new(ProbeRsTargetWrapper { client })
        } else if let Some(ref mut session) = probe_rs_session_store {
            let core = match session.core(0) {
                Ok(c) => c,
                Err(e) => {
                    spinner
                        .set_message(format!("Failed to access core: {:?}. Retrying in 1s...", e));
                    std::thread::sleep(Duration::from_secs(1));
                    continue;
                }
            };
            Box::new(ProbeRsWrapper::new(core))
        } else {
            unreachable!();
        };

        if dump_mem {
            spinner.finish_and_clear();
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

        spinner.set_message("Locating RTT channels...");
        let (up_channels, down_channels) =
            match parse_rtt_channels(target.as_mut(), rtt_symbol_addr, object_file.as_ref()) {
                Ok(chans) => chans,
                Err(e) => {
                    spinner.set_message(format!(
                        "Failed to parse RTT channels: {}. Retrying in 1s...",
                        e
                    ));
                    std::thread::sleep(Duration::from_secs(1));
                    continue;
                }
            };

        // Identify channels based on selected mode
        let mut defmt_channel = None;
        let mut cli_up_channel = None;
        let mut cli_down_channel = None;

        let stream_defmt =
            channel_mode == crate::ChannelMode::Defmt || channel_mode == crate::ChannelMode::Both;

        for chan in &up_channels {
            if chan.name() == Some("defmt") && stream_defmt {
                defmt_channel = Some(chan);
            } else if chan.name() == Some("cli") && stream_cli {
                cli_up_channel = Some(chan);
            }
        }

        for chan in &down_channels {
            if chan.name() == Some("cli") && stream_cli {
                cli_down_channel = Some(chan);
            }
        }

        if raw {
            if cli_up_channel.is_none() && !up_channels.is_empty() && stream_cli {
                cli_up_channel = Some(&up_channels[0]);
            }
            if cli_down_channel.is_none() && !down_channels.is_empty() && stream_cli {
                cli_down_channel = Some(&down_channels[0]);
            }
        }

        spinner.finish_and_clear();

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
        let mut decoder = table.map(|table| table.new_stream_decoder());

        let locations = if let Some(t) = table {
            t.get_locations(&elf_data).ok()
        } else {
            None
        };

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

        println!("\nRTT Session Started (Press Ctrl+C to exit):");
        if cli_up_channel.is_some() && cli_down_channel.is_some() {
            println!("Interactive RTT console active. Type commands and press Enter.");
        }
        println!();

        // Run the poll loop
        let mut rtt_buf = [0u8; 1024];
        let mut sent_buffer = Vec::<u8>::new();
        let mut run_error = None;
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
                                        let plain_line = frame.display(false).to_string();
                                        let display = frame.display(true);
                                        let line_str = display.to_string();

                                        let module_path = locations
                                            .as_ref()
                                            .and_then(|locs| locs.get(&frame.index()))
                                            .map(|loc| loc.module.as_str());

                                        match handle_tracing_line(&line_str, module_path) {
                                            Ok(true) => continue,
                                            Err(e) => {
                                                eprintln!("Error parsing trace line: {}", e);
                                                continue;
                                            }
                                            Ok(false) => {}
                                        }
                                        match handle_intercepted_crash_dump(
                                            &plain_line,
                                            &addr2line_ctx,
                                            table,
                                        ) {
                                            Ok(true) => continue,
                                            Err(e) => {
                                                eprintln!("Error parsing crash dump: {}", e);
                                                continue;
                                            }
                                            Ok(false) => {}
                                        }

                                        // If tracing is enabled, manually log the device log event with target timestamp
                                        if trace {
                                            let msg = frame.display_message().to_string();
                                            let device_ts = frame
                                                .display_timestamp()
                                                .map(|ts| ts.to_string())
                                                .and_then(|s| s.trim().parse::<f64>().ok())
                                                .map(|ts_sec| ts_sec * 1_000_000.0);

                                            let (file, line, ns) = if let Some(loc) = locations
                                                .as_ref()
                                                .and_then(|locs| locs.get(&frame.index()))
                                            {
                                                (
                                                    loc.file.display().to_string(),
                                                    loc.line as i64,
                                                    loc.module.clone(),
                                                )
                                            } else {
                                                (String::new(), 0i64, "rp_pico".to_string())
                                            };
                                            tracing::info!(
                                                target: "device_log",
                                                code_filepath = file,
                                                code_lineno = line,
                                                code_namespace = ns,
                                                device_ts = device_ts.unwrap_or(0.0),
                                                "{}",
                                                msg
                                            );
                                        }

                                        let mut line_str = line_str.clone();

                                        let mut module_context = String::new();
                                        if let Some(ref locs) = locations {
                                            if let Some(loc) = locs.get(&frame.index()) {
                                                module_context =
                                                    format!("\x1b[36m[{}]\x1b[0m ", loc.module);
                                            }
                                        }

                                        if !module_context.is_empty() {
                                            let mut inserted = false;
                                            for lvl in &["TRACE", "DEBUG", "INFO", "WARN", "ERROR"]
                                            {
                                                if let Some(pos) = line_str.find(lvl) {
                                                    let rest = &line_str[pos..];
                                                    if let Some(space_pos) = rest.find(' ') {
                                                        let insert_idx = pos + space_pos + 1;
                                                        line_str.insert_str(
                                                            insert_idx,
                                                            &module_context,
                                                        );
                                                        inserted = true;
                                                        break;
                                                    }
                                                }
                                            }
                                            if !inserted {
                                                line_str =
                                                    format!("{}{}", module_context, line_str);
                                            }
                                        }
                                        if !line_str.contains("Device Telemetry: ") {
                                            println!("{}", line_str);
                                        }
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
                        run_error = Some(format!("defmt read error: {:?}", e));
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
                        use std::io::IsTerminal;
                        let use_echo_canceller = std::io::stdout().is_terminal();
                        let mut write_buf = Vec::with_capacity(n);
                        for &b in &rtt_buf[..n] {
                            if use_echo_canceller
                                && !sent_buffer.is_empty()
                                && (sent_buffer[0] == b
                                    || ((b == b'\r' || b == b'\n')
                                        && (sent_buffer[0] == b'\r' || sent_buffer[0] == b'\n')))
                            {
                                sent_buffer.remove(0);
                            } else {
                                write_buf.push(b);
                            }
                        }
                        if !write_buf.is_empty() {
                            let _ = std::io::stdout().write_all(&write_buf);
                            let _ = std::io::stdout().flush();
                        }
                    }
                    Err(e) => {
                        run_error = Some(format!("CLI read error: {:?}", e));
                        break;
                    }
                    _ => {}
                }
            }

            // 3. Poll CLI input
            if let Ok(input_bytes) = stdin_rx.try_recv() {
                did_work = true;
                if let Some(cli_down) = cli_down_channel {
                    use std::io::IsTerminal;
                    if std::io::stdout().is_terminal() {
                        // Queue typed characters for echo cancellation
                        for &b in &input_bytes {
                            if b == b'\n' || b == b'\r' {
                                sent_buffer.push(b'\r');
                                sent_buffer.push(b'\n');
                            } else {
                                sent_buffer.push(b);
                            }
                        }
                        if sent_buffer.len() > 1024 {
                            sent_buffer.clear();
                        }
                    }

                    let mut written = 0;
                    while written < input_bytes.len() {
                        match cli_down.write(target.as_mut(), &input_bytes[written..]) {
                            Ok(n) => written += n,
                            Err(e) => {
                                run_error = Some(format!("CLI write error: {:?}", e));
                                break;
                            }
                        }
                    }
                }
            }

            if let Some(secs) = duration {
                if start_time.elapsed().as_secs() >= secs {
                    running = false;
                }
            }

            if !running {
                break;
            }

            if run_error.is_some() {
                break;
            }

            if !did_work {
                std::thread::sleep(Duration::from_millis(10));
            }
        }

        if let Some(err) = run_error {
            if !running {
                break;
            }
            eprintln!("\nConnection lost: {}. Reconnecting...", err);
            for _ in 0..10 {
                if !running {
                    break;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        } else {
            break;
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

fn split_cbor_display(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut bracket_level = 0;

    let s = s.trim();
    let s = if s.starts_with('[') && s.ends_with(']') {
        &s[1..s.len() - 1]
    } else {
        s
    };

    for c in s.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                current.push(c);
            }
            '[' => {
                if !in_quotes {
                    bracket_level += 1;
                }
                current.push(c);
            }
            ']' => {
                if !in_quotes {
                    bracket_level -= 1;
                }
                current.push(c);
            }
            ',' => {
                if !in_quotes && bracket_level == 0 {
                    parts.push(current.trim().to_string());
                    current.clear();
                } else {
                    current.push(c);
                }
            }
            _ => {
                current.push(c);
            }
        }
    }
    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }
    parts
}

fn parse_u32(s: &str) -> u32 {
    let s = s.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(&s[2..], 16).unwrap_or(0)
    } else {
        s.parse::<u32>().unwrap_or(0)
    }
}

fn decode_hex(s: &str) -> Vec<u8> {
    let s = s.trim().trim_start_matches("h'").trim_end_matches('\'');
    let mut bytes = Vec::new();
    let mut chars = s.chars().filter(|c| c.is_ascii_hexdigit());
    while let (Some(c1), Some(c2)) = (chars.next(), chars.next()) {
        let hex_str: String = [c1, c2].iter().collect();
        if let Ok(b) = u8::from_str_radix(&hex_str, 16) {
            bytes.push(b);
        }
    }
    bytes
}

fn handle_intercepted_crash_dump<R>(
    log_line: &str,
    context: &Option<addr2line::Context<R>>,
    defmt_table: Option<&defmt_decoder::Table>,
) -> Result<bool, &'static str>
where
    R: addr2line::gimli::Reader<Offset = usize>,
{
    const MAX_CORES: usize = 2;

    let start_idx = match log_line.find("Crash Dump: ") {
        Some(idx) => idx + "Crash Dump: ".len(),
        None => return Ok(false),
    };

    let array_str = &log_line[start_idx..];
    let parts = split_cbor_display(array_str);
    if parts.len() < 4 {
        return Err("Malformed crash dump: CBOR array has fewer than 4 elements");
    }

    let revision_hash = parts[0].trim_matches('"').to_string();
    let system_logs = decode_hex(&parts[1]);
    let uuid_bytes = decode_hex(&parts[2]);
    let mut uuid = [0u8; 16];
    if uuid_bytes.len() == 16 {
        uuid.copy_from_slice(&uuid_bytes);
    }

    let cores_str = &parts[3];
    let core_parts = split_cbor_display(cores_str);
    if core_parts.is_empty() {
        return Err("Malformed crash dump: no cores in CBOR cores array");
    }

    let parse_core = |core_str: &str| -> Result<platform::types::CoreDump, &'static str> {
        let fields = split_cbor_display(core_str);
        if fields.len() < 10 {
            return Err("Malformed core dump: fewer than 10 elements in CoreDump array");
        }
        let r0 = parse_u32(&fields[0]);
        let r1 = parse_u32(&fields[1]);
        let r2 = parse_u32(&fields[2]);
        let r3 = parse_u32(&fields[3]);
        let sp = parse_u32(&fields[4]);
        let lr = parse_u32(&fields[5]);
        let pc = parse_u32(&fields[6]);

        let bt_str = fields[7].trim_matches(|c| c == '[' || c == ']');
        let backtrace: Vec<u32> = if bt_str.trim().is_empty() {
            Vec::new()
        } else {
            bt_str.split(',').map(parse_u32).collect()
        };
        let backtrace_len = parse_u32(&fields[8]) as usize;
        let panicked = fields[9].trim().parse::<bool>().unwrap_or(false);

        let mut bt = [0u32; 32];
        for (i, &val) in backtrace.iter().enumerate().take(32) {
            bt[i] = val;
        }

        Ok(platform::types::CoreDump {
            r0,
            r1,
            r2,
            r3,
            sp,
            lr,
            pc,
            backtrace: bt,
            backtrace_len: backtrace_len as u32,
            panicked,
        })
    };

    let mut cores = [platform::types::CoreDump::new(); MAX_CORES];

    for (i, part) in core_parts.iter().enumerate().take(MAX_CORES) {
        cores[i] = parse_core(part)?;
    }

    let dump = platform::types::CrashDump {
        revision_hash: &revision_hash,
        system_logs: &system_logs,
        uuid,
        cores,
    };

    tool_common::print_crash_dump(
        "🔥 TARGET DEVICE CRASH DETECTED 🔥",
        &dump,
        context,
        defmt_table,
    );

    Ok(true)
}
