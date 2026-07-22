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
    buf: &mut [u8],
) -> io::Result<()>
where
    R: addr2line::gimli::Reader<Offset = usize>,
{
    let (dir_buf, file_buf) = buf.split_at_mut(1024 * 8);
    spinner.set_message("Fetching directory list (.dir)...");
    let key = string_to_key(".dir");
    let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
        flash,
        flash_range.clone(),
        cache,
        dir_buf,
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
                        let content_res =
                            sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                                flash,
                                flash_range.clone(),
                                cache,
                                file_buf,
                                &log_key,
                            )
                            .await;

                        match content_res {
                            Ok(Some(content)) => {
                                // CBOR serialized crash dump
                                let mut decoder = minicbor::Decoder::new(content);
                                let dump_res: Result<platform::types::CrashDump, _> =
                                    decoder.decode();
                                match dump_res {
                                    Ok(dump) => {
                                        tool_common::print_crash_dump(
                                            "--- PANIC (CBOR) ---",
                                            &dump,
                                            context,
                                            defmt_table.as_ref(),
                                        );
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "Failed to decode CBOR crash dump for {}: {:?}",
                                            filename, e
                                        );
                                    }
                                }
                            }
                            other => {
                                eprintln!(
                                    "Failed to read crash log content for {}: {:?}",
                                    filename, other
                                );
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
