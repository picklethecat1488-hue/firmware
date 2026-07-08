/// A human-readable representation of a resolved stack frame.
#[derive(Debug, Clone)]
pub struct SymbolicatedFrame {
    /// The original program counter address
    pub addr: u64,
    /// Demangled name of the function
    pub func_name: String,
    /// Source file path (if available)
    pub file: Option<String>,
    /// Source line number (if available)
    pub line: Option<u32>,
}

/// Symbolicates a program counter (PC) address using an addr2line context.
pub fn symbolicate_addr<R>(
    context: &addr2line::Context<R>,
    addr: u64,
) -> Result<Vec<SymbolicatedFrame>, String>
where
    R: addr2line::gimli::Reader<Offset = usize>,
{
    let mut resolved_frames = Vec::new();
    match context.find_frames(addr).skip_all_loads() {
        Ok(mut frames) => {
            while let Ok(Some(frame)) = frames.next() {
                let func_name = if let Some(f) = &frame.function {
                    let raw = f.raw_name().unwrap_or(std::borrow::Cow::Borrowed("??"));
                    format!("{:#}", rustc_demangle::demangle(&raw))
                } else {
                    "??".to_string()
                };
                let (file, line) = if let Some(loc) = frame.location {
                    (loc.file.map(|s| s.to_string()), loc.line)
                } else {
                    (None, None)
                };
                resolved_frames.push(SymbolicatedFrame {
                    addr,
                    func_name,
                    file,
                    line,
                });
            }
        }
        Err(e) => return Err(format!("Symbolication error: {:?}", e)),
    }
    Ok(resolved_frames)
}

/// Prints a formatted crash dump, symbolicating backtrace program counters and decoding defmt system logs.
pub fn print_crash_dump<R>(
    header: &str,
    dump: &firmware_lib::types::CrashDump,
    context: &Option<addr2line::Context<R>>,
    defmt_table: Option<&defmt_decoder::Table>,
) where
    R: addr2line::gimli::Reader<Offset = usize>,
{
    println!("\n\n========================================================");
    println!("{}", header);
    println!("========================================================");
    let u = dump.uuid;
    println!("UUID: {:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        u[0], u[1], u[2], u[3], u[4], u[5], u[6], u[7],
        u[8], u[9], u[10], u[11], u[12], u[13], u[14], u[15]);
    println!("Revision Hash: {}", dump.revision_hash);
    println!("Registers:");
    println!("  R0: 0x{:08X}", dump.r0);
    println!("  R1: 0x{:08X}", dump.r1);
    println!("  R2: 0x{:08X}", dump.r2);
    println!("  R3: 0x{:08X}", dump.r3);

    println!("\nBacktrace:");
    let pc_count = dump.backtrace_len as usize;
    if pc_count == 0 {
        println!("  (No backtrace frames captured. Check compiler optimization or stack pointer alignment)");
    } else {
        println!("  Raw PCs: {:x?}", &dump.backtrace[..pc_count]);
        let mut frames_list = Vec::new();
        for &pc in dump.backtrace.iter().take(pc_count) {
            let addr = pc as u64;
            let lookup_addr = addr.saturating_sub(1);
            if let Some(ctx) = context {
                if let Ok(frames) = symbolicate_addr(ctx, lookup_addr) {
                    frames_list.push((addr, frames));
                } else {
                    frames_list.push((addr, Vec::new()));
                }
            } else {
                frames_list.push((addr, Vec::new()));
            }
        }

        // Detect the index of the panic entry point (e.g. rust_begin_unwind or panic_fmt)
        let mut panic_entry_idx = None;
        for (i, (_addr, frames)) in frames_list.iter().enumerate() {
            for frame in frames {
                let name = frame.func_name.to_lowercase();
                if name.contains("rust_begin_unwind")
                    || name.contains("panic_fmt")
                    || name.contains("panic_impl")
                    || name.contains("begin_panic")
                {
                    panic_entry_idx = Some(i);
                }
            }
        }

        // If a panic entry point is found, filter out all frames up to and including it.
        // This removes the stale logs / defmt buffer write history and panic internals.
        let skip_count = if let Some(idx) = panic_entry_idx {
            idx + 1
        } else {
            0
        };

        for (addr, frames) in frames_list.into_iter().skip(skip_count) {
            if frames.is_empty() {
                println!("  0x{:08X} - (no symbol found)", addr);
            } else {
                let filtered_frames: Vec<_> = frames
                    .into_iter()
                    .filter(|f| !should_filter_frame(&f.func_name))
                    .collect();

                for frame in filtered_frames {
                    if let Some(file) = frame.file {
                        println!(
                            "  0x{:08X} - {} ({}:{})",
                            addr,
                            frame.func_name,
                            file,
                            frame.line.unwrap_or(0)
                        );
                    } else {
                        println!("  0x{:08X} - {} (??:0)", addr, frame.func_name);
                    }
                }
            }
        }
    }

    println!("\nCrash Context System Logs:");
    if let Some(table) = defmt_table {
        let mut decoder = table.new_stream_decoder();
        decoder.received(dump.system_logs);
        loop {
            match decoder.decode() {
                Ok(frame) => {
                    println!("  {}", frame.display(true));
                }
                Err(defmt_decoder::DecodeError::UnexpectedEof) => break,
                Err(defmt_decoder::DecodeError::Malformed) => {
                    println!("  [Malformed log frame]");
                    break;
                }
            }
        }
    } else {
        println!("  (No defmt table loaded to decode system logs)");
    }
    println!("========================================================\n\n");
}

fn should_filter_frame(name: &str) -> bool {
    let name_lower = name.to_lowercase();
    name_lower.contains("core::")
        || name_lower.contains("alloc::")
        || name_lower.contains("std::")
        || name_lower.contains("compiler_builtins::")
        || name_lower.contains("embassy_executor::")
        || name_lower.contains("__udivmod")
        || name_lower.contains("__aeabi_")
}
