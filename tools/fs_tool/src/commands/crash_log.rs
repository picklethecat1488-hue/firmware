use crate::flash::EitherFlash;
use crate::string_to_key;
use std::io;

pub async fn run<R>(
    flash: &mut EitherFlash,
    flash_range: std::ops::Range<u32>,
    cache: &mut sequential_storage::cache::NoCache,
    spinner: &indicatif::ProgressBar,
    context: &Option<addr2line::Context<R>>,
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
                    if filename.starts_with("crash_") && filename.ends_with(".log") {
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
                                if let Ok(text) = std::str::from_utf8(content) {
                                    let mut in_backtrace = false;
                                    for line in text.lines() {
                                        if line.starts_with("Backtrace:") {
                                            in_backtrace = true;
                                            println!("{}", line);
                                            continue;
                                        }
                                        if in_backtrace {
                                            if line.trim().is_empty()
                                                || line.starts_with("System Logs:")
                                            {
                                                in_backtrace = false;
                                            } else if line.trim().starts_with("0x") {
                                                let addr_str = line.trim().trim_start_matches("0x");
                                                if let Ok(addr) = u64::from_str_radix(addr_str, 16)
                                                {
                                                    if let Some(ctx) = context {
                                                        match ctx.find_frames(addr).skip_all_loads()
                                                        {
                                                            Ok(mut frames) => {
                                                                let mut found = false;
                                                                while let Ok(Some(frame)) =
                                                                    frames.next()
                                                                {
                                                                    found = true;
                                                                    let func_name = if let Some(f) =
                                                                        &frame.function
                                                                    {
                                                                        let raw = f.raw_name().unwrap_or(std::borrow::Cow::Borrowed("??"));
                                                                        format!("{:#}", rustc_demangle::demangle(&raw))
                                                                    } else {
                                                                        "??".to_string()
                                                                    };
                                                                    if let Some(loc) =
                                                                        frame.location
                                                                    {
                                                                        println!(
                                                                            "  0x{:08X} - {} ({}:{})",
                                                                            addr,
                                                                            func_name,
                                                                            loc.file.unwrap_or("??"),
                                                                            loc.line.unwrap_or(0)
                                                                        );
                                                                    } else {
                                                                        println!("  0x{:08X} - {} (??:0)", addr, func_name);
                                                                    }
                                                                }
                                                                if !found {
                                                                    println!("  0x{:08X} - (no symbol found)", addr);
                                                                }
                                                            }
                                                            Err(_) => {
                                                                println!("  0x{:08X} - (symbolication error)", addr);
                                                            }
                                                        }
                                                    } else {
                                                        println!("{}", line);
                                                    }
                                                    continue;
                                                }
                                            }
                                        }
                                        println!("{}", line);
                                    }
                                } else {
                                    println!("(Binary crash log, {} bytes)", content.len());
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
