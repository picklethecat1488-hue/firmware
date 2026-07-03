use crate::flash::EitherFlash;
use crate::string_to_key;
use std::io;

pub async fn run(
    flash: &mut EitherFlash,
    flash_range: std::ops::Range<u32>,
    cache: &mut sequential_storage::cache::NoCache,
    spinner: &indicatif::ProgressBar,
    filename: &str,
) -> io::Result<()> {
    spinner.set_message(format!("Reading {}...", filename));
    let key = string_to_key(filename);
    let mut out_buf = vec![0u8; 1024 * 16]; // support up to 16KB files

    let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
        flash,
        flash_range,
        cache,
        &mut out_buf,
        &key,
    )
    .await;

    spinner.finish_and_clear();

    match res {
        Ok(Some(content)) => {
            // Check if content is UTF-8 text or binary
            if let Ok(text) = std::str::from_utf8(content) {
                print!("{}", text);
            } else {
                // Print hex dump for binary contents
                println!("(Binary content, {} bytes)", content.len());
                for chunk in content.chunks(16) {
                    for byte in chunk {
                        print!("{:02X} ", byte);
                    }
                    println!();
                }
            }
        }
        Ok(None) => {
            eprintln!("File not found: {}", filename);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error reading file: {:?}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}
