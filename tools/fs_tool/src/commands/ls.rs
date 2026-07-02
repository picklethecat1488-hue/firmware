use crate::flash::EitherFlash;
use crate::{string_to_key, DataType};
use std::io;

pub async fn run(
    flash: &mut EitherFlash,
    flash_range: std::ops::Range<u32>,
    cache: &mut sequential_storage::cache::NoCache,
    spinner: &indicatif::ProgressBar,
) -> io::Result<()> {
    spinner.set_message("Reading directory (.dir)...");
    let mut dir_buf = [0u8; 512];
    let key = string_to_key(".dir");
    let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
        flash,
        flash_range,
        cache,
        &mut dir_buf,
        &key,
    )
    .await;

    spinner.finish_and_clear();

    match res {
        Ok(Some(list)) => {
            if let Ok(s) = std::str::from_utf8(list) {
                println!("{:<24} | Data Type / Format", "Filename");
                println!("{}", "-".repeat(50));
                for line in s.split('\n') {
                    if !line.is_empty() {
                        let dt = DataType::from_filename(line);
                        println!("{:<24} | {}", line, dt.to_str());
                    }
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
