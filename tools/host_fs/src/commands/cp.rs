use crate::flash::EitherFlash;
use crate::string_to_key;
use std::fs::File;
use std::io::{self, Write};

/// Arguments for copying files between device and host.
pub struct CpArgs<'a> {
    /// Source file path (e.g. dev:log.txt or host path)
    pub src: &'a str,
    /// Destination file path (e.g. dev:log.txt or host path)
    pub dest: &'a str,
    /// Optional ELF file path for decoding dump logs
    pub dump_option: &'a Option<String>,
}

pub async fn run(
    flash: &mut EitherFlash,
    flash_range: std::ops::Range<u32>,
    cache: &mut sequential_storage::cache::NoCache,
    spinner: &indicatif::ProgressBar,
    args: CpArgs<'_>,
    buf: &mut [u8],
) -> io::Result<()> {
    let CpArgs { src, dest, dump_option } = args;
    let (dir_buf, file_buf) = buf.split_at_mut(1024 * 8);
    let src_is_dev = src.starts_with("dev:");
    let dest_is_dev = dest.starts_with("dev:");

    match (src_is_dev, dest_is_dev) {
        (true, false) => {
            // Copy from device to host
            let dev_filename = src.trim_start_matches("dev:");
            spinner.set_message(format!("Reading {} from device...", dev_filename));
            let key = string_to_key(dev_filename);

            let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                flash,
                flash_range.clone(),
                cache,
                file_buf,
                &key,
            )
            .await;

            spinner.finish_and_clear();

            match res {
                Ok(Some(content)) => {
                    std::fs::write(dest, content)?;
                    println!("Successfully copied dev:{} to {}", dev_filename, dest);
                }
                Ok(None) => {
                    eprintln!("Error: File '{}' not found on device.", dev_filename);
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error reading file from device: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
        (false, true) => {
            // Copy from host to device
            let dev_filename = dest.trim_start_matches("dev:");
            spinner.set_message(format!("Reading local file {}...", src));
            let file_content = std::fs::read(src)?;
            let file_content_slice: &[u8] = &file_content;

            spinner.set_message(format!("Writing {} to device...", dev_filename));
            let key = string_to_key(dev_filename);

            let res = sequential_storage::map::store_item(
                flash,
                flash_range.clone(),
                cache,
                file_buf,
                &key,
                &file_content_slice,
            )
            .await;

            if let Err(e) = res {
                spinner.finish_and_clear();
                eprintln!("Error writing file to device: {:?}", e);
                std::process::exit(1);
            }

            // Update the directory index (.dir)
            spinner.set_message("Updating directory index...");
            let dir_key = string_to_key(".dir");
            let dir_res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                flash,
                flash_range.clone(),
                cache,
                dir_buf,
                &dir_key,
            )
            .await;

            let mut current_dir = String::new();
            if let Ok(Some(existing_dir)) = dir_res {
                if let Ok(s) = std::str::from_utf8(existing_dir) {
                    current_dir.push_str(s);
                }
            }

            // Check if name is already in the list
            let mut found = false;
            for entry in current_dir.split('\n') {
                if entry == dev_filename {
                    found = true;
                    break;
                }
            }

            if !found {
                if !current_dir.is_empty() {
                    current_dir.push('\n');
                }
                current_dir.push_str(dev_filename);

                let dir_bytes = current_dir.as_bytes();
                let store_dir_res = sequential_storage::map::store_item(
                    flash,
                    flash_range.clone(),
                    cache,
                    file_buf,
                    &dir_key,
                    &dir_bytes,
                )
                .await;

                if let Err(e) = store_dir_res {
                    spinner.finish_and_clear();
                    eprintln!("Error updating directory index on device: {:?}", e);
                    std::process::exit(1);
                }
            }

            // Save updated flash content back to the host file or device!
            match flash {
                EitherFlash::Host(f) => {
                    spinner.set_message("Saving updated flash dump...");
                    let mut dump_file = File::create(dump_option.as_ref().unwrap())?;
                    dump_file.write_all(&f.data)?;
                }
                EitherFlash::Probe(f) => {
                    spinner
                        .set_message("Writing updated filesystem back to device via probe-rs...");
                    f.commit().map_err(io::Error::other)?;
                }
                EitherFlash::Gdb(f) => {
                    spinner.set_message(
                        "Writing updated filesystem back to device via OpenOCD GDB...",
                    );
                    f.commit().map_err(io::Error::other)?;
                }
            }

            spinner.finish_and_clear();
            println!("Successfully copied {} to dev:{}", src, dev_filename);
        }
        _ => {
            spinner.finish_and_clear();
            eprintln!("Error: One path must be local and the other must be prefixed with 'dev:'");
            std::process::exit(1);
        }
    }
    Ok(())
}
