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
    dump: &platform::types::CrashDump,
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

    for (cpu_id, core) in dump.cores.iter().enumerate() {
        println!("\n--------------------------------------------------------");
        println!("CPU Core: Core {}", cpu_id);
        println!("Panicked: {}", core.panicked);
        println!("Registers:");
        println!("  R0: 0x{:08X}", core.r0);
        println!("  R1: 0x{:08X}", core.r1);
        println!("  R2: 0x{:08X}", core.r2);
        println!("  R3: 0x{:08X}", core.r3);
        println!("  SP: 0x{:08X}", core.sp);
        println!("  LR: 0x{:08X}", core.lr);
        println!("  PC: 0x{:08X}", core.pc);

        println!("\nBacktrace:");
        let pc_count = core.backtrace_len as usize;
        if pc_count == 0 {
            println!("  (No backtrace frames captured. Check compiler optimization or stack pointer alignment)");
        } else {
            println!("  Raw PCs: {:x?}", &core.backtrace[..pc_count]);
            let mut frames_list = Vec::new();
            for &pc in core.backtrace.iter().take(pc_count) {
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
    let _ = std::io::Write::flush(&mut std::io::stdout());
}

fn is_crate_frame(name: &str, crate_name: &str) -> bool {
    let name_lower = name.to_lowercase();
    let pattern = format!("{}::", crate_name);
    if name_lower.starts_with(&pattern) {
        return true;
    }
    for prefix in &["<", "::", " ", "&", "[", "("] {
        if name_lower.contains(&format!("{}{}", prefix, pattern)) {
            return true;
        }
    }
    false
}

fn should_filter_frame(name: &str) -> bool {
    let name_lower = name.to_lowercase();
    is_crate_frame(name, "core")
        || is_crate_frame(name, "alloc")
        || is_crate_frame(name, "std")
        || name_lower.contains("compiler_builtins::")
        || name_lower.contains("embassy_executor::")
        || name_lower.contains("__udivmod")
        || name_lower.contains("__aeabi_")
}

const MAX_DWARF_LINE_SLIP: u32 = 25;

/// Resolve a symbol name (e.g. `"Reset_Handler"`, `"Core1_Reset_Handler"`) or DWARF source
/// `file:line` target (e.g. `"cat_detector_shell.rs:102"`) in `elf_path` to its instruction address.
pub fn resolve_breakpoint_target(elf_path: &std::path::Path, spec: &str) -> Result<u64, String> {
    use object::{Object, ObjectSymbol};

    let data = std::fs::read(elf_path)
        .map_err(|e| format!("Failed to read ELF '{}': {}", elf_path.display(), e))?;
    let obj = object::File::parse(&*data)
        .map_err(|e| format!("Failed to parse ELF '{}': {}", elf_path.display(), e))?;

    if let Some((raw_file, raw_line)) = spec.rsplit_once(':') {
        if !raw_file.is_empty() && raw_line.chars().all(|c| c.is_ascii_digit()) {
            let target_line: u32 = raw_line
                .parse()
                .map_err(|e| format!("Invalid line number in '{}': {}", spec, e))?;
            let normalized_suffix = raw_file.replace('\\', "/");
            let ctx = addr2line::Context::new(&obj)
                .map_err(|e| format!("Failed to load DWARF line table: {:?}", e))?;
            let iter = ctx
                .find_location_range(0, u64::MAX)
                .map_err(|e| format!("Failed to scan DWARF locations: {:?}", e))?;

            let mut best_match: Option<(u32, u64)> = None;
            for (start_addr, len, loc) in iter {
                if len == 0 {
                    continue;
                }
                let Some(file_str) = loc.file else {
                    continue;
                };
                let Some(line_num) = loc.line else {
                    continue;
                };
                if line_num < target_line
                    || line_num > target_line.saturating_add(MAX_DWARF_LINE_SLIP)
                {
                    continue;
                }
                let file_norm = file_str.replace('\\', "/");
                if file_norm == normalized_suffix
                    || file_norm.ends_with(&format!("/{}", normalized_suffix))
                {
                    let delta = line_num - target_line;
                    let candidate = (delta, start_addr & !1);
                    if best_match.is_none_or(|prev| candidate < prev) {
                        best_match = Some(candidate);
                    }
                }
            }

            return best_match
                .map(|(_, addr)| addr)
                .ok_or_else(|| format!("No DWARF instruction found near '{}'", spec));
        }
    }

    // Symbol lookup: exact raw name first, then exact demangled name, then unique demangled suffix.
    let mut demangled_exact: Option<u64> = None;
    let mut demangled_contains: Option<u64> = None;
    for sym in obj.symbols() {
        let Ok(name) = sym.name() else {
            continue;
        };
        if name.is_empty() || sym.address() == 0 {
            continue;
        }
        let addr = sym.address() & !1;
        if name == spec {
            return Ok(addr);
        }
        let demangled = format!("{:#}", rustc_demangle::demangle(name));
        if demangled == spec {
            demangled_exact = Some(addr);
        } else if demangled.ends_with(&format!("::{}", spec)) || demangled.contains(spec) {
            if demangled_contains.is_none_or(|prev| addr < prev) {
                demangled_contains = Some(addr);
            }
        }
    }

    demangled_exact
        .or(demangled_contains)
        .ok_or_else(|| format!("Symbol '{}' not found in '{}'", spec, elf_path.display()))
}
