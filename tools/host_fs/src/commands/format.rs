use crate::flash::EitherFlash;
use crate::string_to_key;
use std::io;

pub async fn run(
    flash: &mut EitherFlash,
    flash_range: std::ops::Range<u32>,
    cache: &mut sequential_storage::cache::NoCache,
    spinner: &indicatif::ProgressBar,
    buf: &mut [u8],
) -> io::Result<()> {
    spinner.set_message("Formatting filesystem partition on device...");

    use embedded_storage_async::nor_flash::NorFlash;
    // Erase the entire partition range
    flash
        .erase(flash_range.start, flash_range.end)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Erase failed: {:?}", e)))?;

    // Write an empty directory index (.dir)
    let key = string_to_key(".dir");
    sequential_storage::map::store_item(flash, flash_range.clone(), cache, buf, &key, &[0u8; 0])
        .await
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to write directory index: {:?}", e),
            )
        })?;

    // Commit changes back to target flash memory
    match flash {
        EitherFlash::Probe(f) => f.commit().map_err(io::Error::other)?,
        EitherFlash::Gdb(f) => f.commit().map_err(io::Error::other)?,
        _ => {}
    }

    spinner.finish_and_clear();
    println!("Filesystem partition formatted successfully.");
    Ok(())
}
