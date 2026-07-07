use crate::flash::EitherFlash;
use crate::string_to_key;
use std::fs::File;
use std::io::{self, Write};

pub async fn run(
    flash: &mut EitherFlash,
    flash_range: std::ops::Range<u32>,
    cache: &mut sequential_storage::cache::NoCache,
    spinner: &indicatif::ProgressBar,
    filename: &Option<String>,
    dump_option: &Option<String>,
    buf: &mut [u8],
) -> io::Result<()> {
    let (dir_buf, file_buf) = buf.split_at_mut(1024 * 8);
    spinner.set_message("Reading directory index (.dir)...");
    let dir_key = string_to_key(".dir");
    let dir_res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
        flash,
        flash_range.clone(),
        cache,
        dir_buf,
        &dir_key,
    )
    .await;

    let current_dir = match dir_res {
        Ok(Some(existing_dir)) => {
            if let Ok(s) = std::str::from_utf8(existing_dir) {
                s.to_string()
            } else {
                String::new()
            }
        }
        _ => String::new(),
    };

    let files_on_dev: Vec<&str> = current_dir.split('\n').filter(|s| !s.is_empty()).collect();

    // Determine files to remove
    let files_to_remove: Vec<&str> = match filename {
        None => files_on_dev.clone(),
        Some(name) if name == "*" => files_on_dev.clone(),
        Some(name) => {
            if files_on_dev.contains(&name.as_str()) {
                vec![name.as_str()]
            } else {
                spinner.finish_and_clear();
                eprintln!("Error: File '{}' not found on device.", name);
                std::process::exit(1);
            }
        }
    };

    if files_to_remove.is_empty() {
        spinner.finish_and_clear();
        println!("No files found to remove.");
        return Ok(());
    }

    for name in &files_to_remove {
        spinner.set_message(format!("Removing {}...", name));
        let key = string_to_key(name);
        let res = sequential_storage::map::remove_item::<[u8; 32], _>(
            flash,
            flash_range.clone(),
            cache,
            file_buf,
            &key,
        )
        .await;

        if let Err(e) = res {
            spinner.finish_and_clear();
            eprintln!("Error removing file '{}': {:?}", name, e);
            std::process::exit(1);
        }
    }

    // Update or remove directory index (.dir)
    spinner.set_message("Updating directory index...");
    if filename.is_none() || filename.as_deref() == Some("*") {
        // Remove the directory index item entirely
        let res = sequential_storage::map::remove_item::<[u8; 32], _>(
            flash,
            flash_range.clone(),
            cache,
            file_buf,
            &dir_key,
        )
        .await;
        if let Err(e) = res {
            spinner.finish_and_clear();
            eprintln!("Error removing directory index: {:?}", e);
            std::process::exit(1);
        }
    } else {
        // Rebuild directory string excluding the deleted file
        let new_dir_files: Vec<&str> = files_on_dev
            .into_iter()
            .filter(|&f| f != filename.as_ref().unwrap())
            .collect();

        if new_dir_files.is_empty() {
            // Remove the directory index item entirely since it is now empty
            let res = sequential_storage::map::remove_item::<[u8; 32], _>(
                flash,
                flash_range.clone(),
                cache,
                file_buf,
                &dir_key,
            )
            .await;
            if let Err(e) = res {
                spinner.finish_and_clear();
                eprintln!("Error removing directory index: {:?}", e);
                std::process::exit(1);
            }
        } else {
            let new_dir_str = new_dir_files.join("\n");
            let res = sequential_storage::map::store_item(
                flash,
                flash_range.clone(),
                cache,
                file_buf,
                &dir_key,
                &new_dir_str.as_bytes(),
            )
            .await;
            if let Err(e) = res {
                spinner.finish_and_clear();
                eprintln!("Error writing directory index: {:?}", e);
                std::process::exit(1);
            }
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
            spinner.set_message("Writing updated filesystem back to device via probe-rs...");
            f.commit().map_err(io::Error::other)?;
        }
        EitherFlash::Gdb(f) => {
            spinner.set_message("Writing updated filesystem back to device via OpenOCD GDB...");
            f.commit().map_err(io::Error::other)?;
        }
    }

    spinner.finish_and_clear();
    if filename.is_none() || filename.as_deref() == Some("*") {
        println!("Successfully cleared all files.");
    } else {
        println!("Successfully removed file: {}", filename.as_ref().unwrap());
    }

    Ok(())
}
