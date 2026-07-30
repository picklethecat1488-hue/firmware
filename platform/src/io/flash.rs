//! NOR Flash driver adapters.

#![deny(missing_docs)]

use core::ops::Range;
use embedded_storage_async::nor_flash::{MultiwriteNorFlash, NorFlash};

/// Adapter exposing a blocking nor-flash driver as an asynchronous nor-flash driver.
pub struct BlockingAsyncFlash<F>(pub F);

impl<F: embedded_storage::nor_flash::ErrorType> embedded_storage_async::nor_flash::ErrorType
    for BlockingAsyncFlash<F>
{
    type Error = F::Error;
}

impl<F: embedded_storage::nor_flash::ReadNorFlash> embedded_storage_async::nor_flash::ReadNorFlash
    for BlockingAsyncFlash<F>
{
    const READ_SIZE: usize = F::READ_SIZE;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let mut inner = &mut self.0;
        embedded_storage::nor_flash::ReadNorFlash::read(&mut inner, offset, bytes)
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<F: embedded_storage::nor_flash::NorFlash> embedded_storage_async::nor_flash::NorFlash
    for BlockingAsyncFlash<F>
{
    const WRITE_SIZE: usize = F::WRITE_SIZE;
    const ERASE_SIZE: usize = F::ERASE_SIZE;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let mut inner = &mut self.0;
        embedded_storage::nor_flash::NorFlash::write(&mut inner, offset, bytes)
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let mut inner = &mut self.0;
        embedded_storage::nor_flash::NorFlash::erase(&mut inner, from, to)
    }
}

impl<F: embedded_storage::nor_flash::NorFlash> embedded_storage_async::nor_flash::MultiwriteNorFlash
    for BlockingAsyncFlash<F>
{
}

/// Fetches a file's content directly from flash using sequential-storage without a running controller task.
pub async fn read_file_direct<F: NorFlash + MultiwriteNorFlash>(
    flash: &mut F,
    range: Range<u32>,
    buf: &mut [u8],
    name: &str,
    out_buf: &mut [u8],
) -> Result<Option<usize>, ()> {
    let mut cache = sequential_storage::cache::NoCache::new();
    let key = crate::directory::string_to_key(name);
    let res = sequential_storage::map::fetch_item::<[u8; crate::directory::KEY_SIZE], &[u8], _>(
        flash, range, &mut cache, buf, &key,
    )
    .await;
    match res {
        Ok(Some(val)) => {
            if val.len() <= out_buf.len() {
                out_buf[..val.len()].copy_from_slice(val);
                Ok(Some(val.len()))
            } else {
                Err(())
            }
        }
        Ok(None) => Ok(None),
        Err(_) => Err(()),
    }
}

/// Stores/overwrites a file directly in flash using sequential-storage, updating the directory listing.
pub async fn write_file_direct<F: NorFlash + MultiwriteNorFlash>(
    flash: &mut F,
    range: Range<u32>,
    buf: &mut [u8],
    name: &str,
    content: &[u8],
) -> Result<(), ()> {
    let mut cache = sequential_storage::cache::NoCache::new();
    let key = crate::directory::string_to_key(name);
    let res =
        sequential_storage::map::store_item(flash, range.clone(), &mut cache, buf, &key, &content)
            .await;
    if res.is_err() {
        return Err(());
    }

    if name != ".dir" {
        let mut dir_buf = [0u8; crate::directory::DIR_BUF_SIZE];
        let mut existing_dir_str = "";
        let read_res = read_file_direct(flash, range.clone(), buf, ".dir", &mut dir_buf).await;
        if let Ok(Some(len)) = read_res {
            if let Ok(s) = core::str::from_utf8(&dir_buf[..len]) {
                existing_dir_str = s;
            }
        }

        if let Some(new_dir) = crate::directory::add_to_directory(existing_dir_str, name) {
            let dir_key = crate::directory::string_to_key(".dir");
            let _ = sequential_storage::map::store_item(
                flash,
                range,
                &mut cache,
                buf,
                &dir_key,
                &new_dir.as_bytes(),
            )
            .await;
        }
    }
    Ok(())
}

/// Writes a telemetry record directly to flash, maintaining index headers and chunking.
pub async fn write_telemetry_record_direct<F: NorFlash + MultiwriteNorFlash>(
    flash: &mut F,
    range: Range<u32>,
    buf: &mut [u8],
    record: &model::telemetry::TelemetryRecord,
    max_records: usize,
) -> Result<(), ()> {
    // 1. Read header index from "telemetry.rrd"
    let mut header_buf = [0u8; model::telemetry::TELEMETRY_HEADER_SIZE];
    let (mut count, mut next_idx, exists) = match read_file_direct(
        flash,
        range.clone(),
        buf,
        model::telemetry::TELEMETRY_HEADER_FILE,
        &mut header_buf,
    )
    .await
    {
        Ok(Some(model::telemetry::TELEMETRY_HEADER_SIZE)) => {
            let mut val_count = 0;
            let mut val_next = 0;
            let mut ok = false;
            let mut decoder = minicbor::Decoder::new(&header_buf[1..]);
            if let Ok(Some(2)) = decoder.array() {
                if let (Ok(c), Ok(n)) = (decoder.u32(), decoder.u32()) {
                    val_count = c as usize;
                    val_next = n as usize;
                    ok = true;
                }
            }
            if ok {
                (val_count, val_next, true)
            } else {
                (0, 0, false)
            }
        }
        _ => (0, 0, false),
    };

    if !exists {
        count = 0;
        next_idx = 0;
    }

    // 2. Determine chunk and slot indices
    let chunk_idx = next_idx / model::telemetry::CHUNK_SIZE;
    let slot_idx = next_idx % model::telemetry::CHUNK_SIZE;
    let mut name_buf = [0u8; crate::MAX_FILE_NAME_LEN];
    let chunk_name = model::telemetry::chunk_name(chunk_idx, &mut name_buf);

    // 3. Read the chunk file or use a zeroed buffer
    let mut chunk_buf = [0u8; model::telemetry::CHUNK_FILE_SIZE];
    let _ = read_file_direct(flash, range.clone(), buf, chunk_name, &mut chunk_buf).await;

    // 4. Serialize the telemetry record
    #[cfg(any(test, not(all(target_arch = "arm", target_os = "none"))))]
    let timestamp_us = 0;
    #[cfg(not(any(test, not(all(target_arch = "arm", target_os = "none")))))]
    let timestamp_us = embassy_time::Instant::now().as_micros();

    let serialized = record.serialize(timestamp_us);

    // 5. Copy serialized record to chunk slot
    let offset = slot_idx * model::telemetry::TELEMETRY_RECORD_SIZE;
    chunk_buf[offset..offset + model::telemetry::TELEMETRY_RECORD_SIZE]
        .copy_from_slice(&serialized);

    // 6. Write chunk file back to flash
    write_file_direct(flash, range.clone(), buf, chunk_name, &chunk_buf).await?;

    // 7. Update metadata
    next_idx = (next_idx + 1) % max_records;
    count = core::cmp::min(count + 1, max_records);

    // 8. Serialize and write the updated header index to "telemetry.rrd"
    let mut new_header = [0u8; model::telemetry::TELEMETRY_HEADER_SIZE];
    let cursor = minicbor::encode::write::Cursor::new(&mut new_header[1..]);
    let mut encoder = minicbor::Encoder::new(cursor);
    if encoder.array(2).is_ok()
        && encoder.u32(count as u32).is_ok()
        && encoder.u32(next_idx as u32).is_ok()
    {
        let len = encoder.into_writer().position();
        if len < model::telemetry::TELEMETRY_HEADER_SIZE {
            new_header[0] = len as u8;
        }
    }
    write_file_direct(
        flash,
        range,
        buf,
        model::telemetry::TELEMETRY_HEADER_FILE,
        &new_header,
    )
    .await?;

    Ok(())
}

/// A boot status recorder that writes telemetry records directly to flash.
pub struct DirectFlashBootStatus<
    'a,
    F: embedded_storage::nor_flash::NorFlash + embedded_storage::nor_flash::MultiwriteNorFlash,
> {
    flash: &'a mut F,
    storage_range: Range<u32>,
    max_records: usize,
}

impl<
        'a,
        F: embedded_storage::nor_flash::NorFlash + embedded_storage::nor_flash::MultiwriteNorFlash,
    > DirectFlashBootStatus<'a, F>
{
    /// Create a new direct flash boot status recorder.
    pub fn new(flash: &'a mut F, storage_range: Range<u32>, max_records: usize) -> Self {
        Self {
            flash,
            storage_range,
            max_records,
        }
    }
}

impl<
        'a,
        F: embedded_storage::nor_flash::NorFlash + embedded_storage::nor_flash::MultiwriteNorFlash,
    > model::interfaces::BootStatus for DirectFlashBootStatus<'a, F>
{
    fn record_error(&mut self, error: model::types::PeripheralError) {
        let record = model::telemetry::TelemetryRecord::PeripheralError(error);
        let mut fs_buf = [0u8; 4096];
        let mut async_flash = BlockingAsyncFlash(&mut *self.flash);
        let _ = embassy_futures::block_on(write_telemetry_record_direct(
            &mut async_flash,
            self.storage_range.clone(),
            &mut fs_buf,
            &record,
            self.max_records,
        ));
        crate::tracing::trace_telemetry_record(&record);
    }
}
