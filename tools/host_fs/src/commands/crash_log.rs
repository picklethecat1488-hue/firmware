use crate::flash::EitherFlash;
use crate::string_to_key;
use std::io;

pub async fn run<R>(
    flash: &mut EitherFlash,
    flash_range: std::ops::Range<u32>,
    cache: &mut sequential_storage::cache::NoCache,
    spinner: &indicatif::ProgressBar,
    context: &Option<addr2line::Context<R>>,
    defmt_table: &Option<defmt_decoder::Table>,
) -> io::Result<()>
where
    R: addr2line::gimli::Reader<Offset = usize>,
{
    spinner.set_message("Fetching directory list (.dir)...");
    let mut dir_buf = [0u8; 512];
    let key = string_to_key(".dir");
    let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
        flash,
        flash_range.clone(),
        cache,
        &mut dir_buf,
        &key,
    )
    .await;

    spinner.finish_and_clear();

    match res {
        Ok(Some(list)) => {
            if let Ok(s) = std::str::from_utf8(list) {
                let mut found_crash = false;
                for filename in s.split('\n') {
                    if filename.starts_with("crash_") && filename.ends_with(".cbor") {
                        found_crash = true;
                        let log_key = string_to_key(filename);
                        let mut out_buf = vec![0u8; 1024 * 16];
                        let content_res =
                            sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                                flash,
                                flash_range.clone(),
                                cache,
                                &mut out_buf,
                                &log_key,
                            )
                            .await;

                        match content_res {
                            Ok(Some(content)) => {
                                // CBOR serialized crash dump
                                let mut decoder = minicbor::Decoder::new(content);
                                let dump_res: Result<firmware_lib::types::CrashDump, _> =
                                    decoder.decode();
                                match dump_res {
                                    Ok(dump) => {
                                        println!("--- PANIC (CBOR) ---");
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
                                        for &pc in dump.backtrace.iter().take(pc_count) {
                                            let addr = pc as u64;
                                            if let Some(ctx) = context {
                                                match ctx.find_frames(addr).skip_all_loads() {
                                                    Ok(mut frames) => {
                                                        let mut found = false;
                                                        while let Ok(Some(frame)) = frames.next() {
                                                            found = true;
                                                            let func_name = if let Some(f) =
                                                                &frame.function
                                                            {
                                                                let raw = f.raw_name().unwrap_or(
                                                                    std::borrow::Cow::Borrowed(
                                                                        "??",
                                                                    ),
                                                                );
                                                                format!(
                                                                    "{:#}",
                                                                    rustc_demangle::demangle(&raw)
                                                                )
                                                            } else {
                                                                "??".to_string()
                                                            };
                                                            if let Some(loc) = frame.location {
                                                                println!(
                                                                    "  0x{:08X} - {} ({}:{})",
                                                                    addr,
                                                                    func_name,
                                                                    loc.file.unwrap_or("??"),
                                                                    loc.line.unwrap_or(0)
                                                                );
                                                            } else {
                                                                println!(
                                                                    "  0x{:08X} - {} (??:0)",
                                                                    addr, func_name
                                                                );
                                                            }
                                                        }
                                                        if !found {
                                                            println!(
                                                                "  0x{:08X} - (no symbol found)",
                                                                addr
                                                            );
                                                        }
                                                    }
                                                    Err(_) => {
                                                        println!(
                                                            "  0x{:08X} - (symbolication error)",
                                                            addr
                                                        );
                                                    }
                                                }
                                            } else {
                                                println!("  0x{:08X}", addr);
                                            }
                                        }

                                        println!("\nSystem Logs (defmt):");
                                        if let Some(table) = defmt_table {
                                            let mut decoder = table.new_stream_decoder();
                                            decoder.received(dump.system_logs);
                                            loop {
                                                match decoder.decode() {
                                                    Ok(frame) => {
                                                        let display = frame.display(false);
                                                        println!("{}", display);
                                                    }
                                                    Err(
                                                        defmt_decoder::DecodeError::UnexpectedEof,
                                                    ) => break,
                                                    Err(defmt_decoder::DecodeError::Malformed) => {
                                                        continue;
                                                    }
                                                }
                                            }
                                        } else {
                                            println!("(No ELF provided to decode {} bytes of binary logs)", dump.system_logs.len());
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "Failed to decode CBOR crash dump for {}: {:?}",
                                            filename, e
                                        );
                                    }
                                }
                            }
                            _ => {
                                eprintln!("Failed to read crash log content for {}", filename);
                            }
                        }
                    }
                }
                if !found_crash {
                    println!("No stored crash logs found in filesystem.");
                }
            }
        }
        Ok(None) => {
            println!("No files found (directory empty).");
        }
        Err(e) => {
            eprintln!("Error reading directory: {:?}", e);
        }
    }
    Ok(())
}
