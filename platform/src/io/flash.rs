//! NOR Flash driver adapters.

#![deny(missing_docs)]

use embedded_storage_async::nor_flash::{MultiwriteNorFlash, NorFlash};

/// Adapter exposing a blocking nor-flash driver as an asynchronous nor-flash driver.
pub struct BlockingAsyncFlash<F>(pub F);

use core::sync::atomic::{AtomicUsize, Ordering};

/// A thread-safe wrapper around a Mutex-protected flash device that implements NorFlash.
pub struct SharedFlashMutex<S: 'static> {
    mutex: &'static embassy_sync::mutex::Mutex<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        S,
    >,
    capacity: AtomicUsize,
}

impl<S: 'static> Clone for SharedFlashMutex<S> {
    fn clone(&self) -> Self {
        Self {
            mutex: self.mutex,
            capacity: AtomicUsize::new(self.capacity.load(Ordering::Relaxed)),
        }
    }
}

impl<S: 'static> SharedFlashMutex<S> {
    /// Creates a new SharedFlashMutex.
    pub const fn new(
        mutex: &'static embassy_sync::mutex::Mutex<
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            S,
        >,
    ) -> Self {
        Self {
            mutex,
            capacity: AtomicUsize::new(0),
        }
    }
}

impl<S: embedded_storage_async::nor_flash::ErrorType> embedded_storage_async::nor_flash::ErrorType
    for SharedFlashMutex<S>
{
    type Error = S::Error;
}

impl<S: embedded_storage_async::nor_flash::ReadNorFlash>
    embedded_storage_async::nor_flash::ReadNorFlash for SharedFlashMutex<S>
{
    const READ_SIZE: usize = S::READ_SIZE;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let mut flash = self.mutex.lock().await;
        if self.capacity.load(Ordering::Relaxed) == 0 {
            self.capacity.store(flash.capacity(), Ordering::Relaxed);
        }
        flash.read(offset, bytes).await
    }

    fn capacity(&self) -> usize {
        let cached = self.capacity.load(Ordering::Relaxed);
        if cached != 0 {
            cached
        } else if let Ok(flash) = self.mutex.try_lock() {
            let cap = flash.capacity();
            self.capacity.store(cap, Ordering::Relaxed);
            cap
        } else {
            cached
        }
    }
}

impl<S: embedded_storage_async::nor_flash::NorFlash> embedded_storage_async::nor_flash::NorFlash
    for SharedFlashMutex<S>
{
    const WRITE_SIZE: usize = S::WRITE_SIZE;
    const ERASE_SIZE: usize = S::ERASE_SIZE;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let mut flash = self.mutex.lock().await;
        if self.capacity.load(Ordering::Relaxed) == 0 {
            self.capacity.store(flash.capacity(), Ordering::Relaxed);
        }
        flash.write(offset, bytes).await
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let mut flash = self.mutex.lock().await;
        if self.capacity.load(Ordering::Relaxed) == 0 {
            self.capacity.store(flash.capacity(), Ordering::Relaxed);
        }
        flash.erase(from, to).await
    }
}

impl<S: embedded_storage_async::nor_flash::MultiwriteNorFlash>
    embedded_storage_async::nor_flash::MultiwriteNorFlash for SharedFlashMutex<S>
{
}

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
    range: crate::types::MapFilesystem,
    buf: &mut [u8],
    name: &str,
    out_buf: &mut [u8],
) -> Result<Option<usize>, ()> {
    let mut cache = sequential_storage::cache::NoCache::new();
    let key = crate::directory::string_to_key(name);
    let res = sequential_storage::map::fetch_item::<[u8; crate::directory::KEY_SIZE], &[u8], _>(
        flash, range.0, &mut cache, buf, &key,
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
    range: crate::types::MapFilesystem,
    buf: &mut [u8],
    name: &str,
    content: &[u8],
) -> Result<(), ()> {
    let mut cache = sequential_storage::cache::NoCache::new();
    let key = crate::directory::string_to_key(name);
    let res = sequential_storage::map::store_item(
        flash,
        range.0.clone(),
        &mut cache,
        buf,
        &key,
        &content,
    )
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
                range.0,
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

/// Writes a telemetry record directly to flash queue storage.
pub async fn write_telemetry_record_direct<F: NorFlash + MultiwriteNorFlash>(
    flash: &mut F,
    telemetry_range: crate::types::QueueFilesystem,
    record: &model::telemetry::TelemetryRecord,
) -> Result<(), ()> {
    #[cfg(any(test, not(all(target_arch = "arm", target_os = "none"))))]
    let timestamp_us = 0;
    #[cfg(not(any(test, not(all(target_arch = "arm", target_os = "none")))))]
    let timestamp_us = embassy_time::Instant::now().as_micros();

    let slot = record.serialize(timestamp_us);
    let len = slot[0] as usize;
    if len > 0 && len < model::telemetry::TELEMETRY_MAX_SIZE {
        let mut cache = sequential_storage::cache::NoCache::new();
        let push_res = sequential_storage::queue::push(
            flash,
            telemetry_range.0.clone(),
            &mut cache,
            &slot[..1 + len],
            true, // allow_overwrite_old_data = true
        )
        .await;
        if push_res.is_err() {
            return Err(());
        }
    }

    Ok(())
}

/// A boot status recorder that writes telemetry records directly to flash.
pub struct DirectFlashBootStatus<
    'a,
    F: embedded_storage::nor_flash::NorFlash + embedded_storage::nor_flash::MultiwriteNorFlash,
> {
    flash: &'a mut F,
    telemetry_range: crate::types::QueueFilesystem,
}

impl<
        'a,
        F: embedded_storage::nor_flash::NorFlash + embedded_storage::nor_flash::MultiwriteNorFlash,
    > DirectFlashBootStatus<'a, F>
{
    /// Create a new direct flash boot status recorder.
    pub fn new(flash: &'a mut F, telemetry_range: crate::types::QueueFilesystem) -> Self {
        Self {
            flash,
            telemetry_range,
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
        let mut async_flash = BlockingAsyncFlash(&mut *self.flash);
        TELEMETRY_ENABLED.store(false, core::sync::atomic::Ordering::Relaxed);
        let _ = embassy_futures::block_on(write_telemetry_record_direct(
            &mut async_flash,
            self.telemetry_range.clone(),
            &record,
        ));
        TELEMETRY_ENABLED.store(true, core::sync::atomic::Ordering::Relaxed);
        crate::tracing::trace_telemetry_record(&record);
    }
}

/// Global flag indicating if flash profiling telemetry logging is enabled.
pub static TELEMETRY_ENABLED: core::sync::atomic::AtomicBool =
    core::sync::atomic::AtomicBool::new(true);

/// A profiling wrapper around a flash driver that counts and times page erases.
pub struct ProfilingFlash<F: NorFlash> {
    inner: F,
    erase_count: u32,
    telemetry_tx: Option<
        embassy_sync::channel::Sender<
            'static,
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            model::telemetry::TelemetryRecord,
            64,
        >,
    >,
}

impl<F: NorFlash> ProfilingFlash<F> {
    /// Create a new ProfilingFlash wrapper.
    pub fn new(inner: F) -> Self {
        Self {
            inner,
            erase_count: 0,
            telemetry_tx: None,
        }
    }

    /// Set telemetry sender for flash erase profiling.
    pub fn set_telemetry(
        &mut self,
        telemetry_tx: embassy_sync::channel::Sender<
            'static,
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            model::telemetry::TelemetryRecord,
            64,
        >,
    ) {
        self.telemetry_tx = Some(telemetry_tx);
    }

    /// Get total page erases performed since boot.
    pub fn erase_count(&self) -> u32 {
        self.erase_count
    }
}

impl<F: NorFlash> embedded_storage_async::nor_flash::ErrorType for ProfilingFlash<F> {
    type Error = F::Error;
}

impl<F: NorFlash> embedded_storage_async::nor_flash::ReadNorFlash for ProfilingFlash<F> {
    const READ_SIZE: usize = F::READ_SIZE;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.inner.read(offset, bytes).await
    }

    fn capacity(&self) -> usize {
        self.inner.capacity()
    }
}

impl<F: NorFlash> embedded_storage_async::nor_flash::NorFlash for ProfilingFlash<F> {
    const WRITE_SIZE: usize = F::WRITE_SIZE;
    const ERASE_SIZE: usize = F::ERASE_SIZE;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.inner.write(offset, bytes).await
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        self.erase_count += 1;

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let start = embassy_time::Instant::now();

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::debug!(
            "[Profile] Flash erase starting at 0x{:X} to 0x{:X}",
            from,
            to
        );

        let res = self.inner.erase(from, to).await;

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        let duration_ms = {
            let duration = start.elapsed();
            let ms = duration.as_millis() as u32;
            defmt::debug!(
                "[Profile] Flash erase completed in {} ms (Total erases: {})",
                ms,
                self.erase_count
            );
            ms
        };

        #[cfg(not(all(target_arch = "arm", target_os = "none")))]
        let duration_ms = 0;

        if TELEMETRY_ENABLED.load(core::sync::atomic::Ordering::Relaxed) {
            if let Some(tx) = &self.telemetry_tx {
                let sector = from / F::ERASE_SIZE as u32;
                let details = model::types::FlashEraseTelemetry {
                    sector,
                    duration_ms,
                    erase_count: self.erase_count,
                };
                let _ = tx.try_send(model::telemetry::TelemetryRecord::FlashTelemetry(details));
            }
        }

        res
    }
}

impl<F: NorFlash + MultiwriteNorFlash> MultiwriteNorFlash for ProfilingFlash<F> {}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Target concrete flash device type alias.
pub type TargetFlash<const FLASH_SIZE: usize> = ProfilingFlash<
    SharedFlashMutex<
        BlockingAsyncFlash<
            embassy_rp::flash::Flash<
                'static,
                embassy_rp::peripherals::FLASH,
                embassy_rp::flash::Blocking,
                FLASH_SIZE,
            >,
        >,
    >,
>;
