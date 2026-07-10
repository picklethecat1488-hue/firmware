//! Project-agnostic flat filesystem controller built over sequential-storage.

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
extern crate std;

use core::cmp;
use core::fmt::Write as _;
use core::ops::Range;
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embedded_storage_async::nor_flash::{MultiwriteNorFlash, NorFlash};
use heapless::String;

static TELEMETRY_ENABLED: AtomicBool = AtomicBool::new(true);

/// A profiling wrapper around a flash driver that counts and times page erases.
pub struct ProfilingFlash<F: NorFlash> {
    /// The inner flash driver instance being profiled
    inner: F,
    /// Total number of page erases performed since system boot
    erase_count: u32,
    /// Optional telemetry sender to log erase operations
    telemetry_tx: Option<
        embassy_sync::channel::Sender<
            'static,
            CriticalSectionRawMutex,
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
            CriticalSectionRawMutex,
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

        if TELEMETRY_ENABLED.load(Ordering::Relaxed) {
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

/// File Controller managing raw files/telemetry in flash using sequential-storage map.
pub struct FilesystemController<F: NorFlash + MultiwriteNorFlash> {
    /// The underlying flash driver instance (possibly wrapped in profiling)
    pub flash: F,
    /// The physical partition address range in flash (start..end byte offsets)
    range: Range<u32>,
    /// Reference to a statically allocated buffer for sequential-storage operations
    buf: &'static mut [u8],
}

impl<F: NorFlash + MultiwriteNorFlash> FilesystemController<F> {
    /// Creates a new FilesystemController.
    pub fn new(flash: F, range: Range<u32>, buf: &'static mut [u8]) -> Self {
        Self { flash, range, buf }
    }

    /// Helper to convert a string path into a fixed-size 32-byte key.
    fn string_to_key(name: &str) -> [u8; 32] {
        let mut key = [0u8; 32];
        let bytes = name.as_bytes();
        let len = cmp::min(bytes.len(), 32);
        key[..len].copy_from_slice(&bytes[..len]);
        key
    }

    /// Stores/overwrites a file with the given name (key) and contents (value).
    pub async fn write_file(&mut self, name: &str, content: &[u8]) -> Result<(), ()> {
        let is_telemetry = name.starts_with("telemetry");
        if is_telemetry {
            TELEMETRY_ENABLED.store(false, Ordering::Relaxed);
        }

        let mut cache = sequential_storage::cache::NoCache::new();
        let key = Self::string_to_key(name);

        // Store item in map
        let res = sequential_storage::map::store_item(
            &mut self.flash,
            self.range.clone(),
            &mut cache,
            self.buf,
            &key,
            &content,
        )
        .await;

        if is_telemetry {
            TELEMETRY_ENABLED.store(true, Ordering::Relaxed);
        }

        if let Err(_e) = res {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::error!("store_item failed: {:?}", defmt::Debug2Format(&_e));
            #[cfg(not(all(target_arch = "arm", target_os = "none")))]
            std::eprintln!("store_item failed: {:?}", _e);
            return Err(());
        }

        // If this is not the directory index itself, add it to the directory index
        if name != ".dir" {
            let mut dir_buf = [0u8; 512];
            let mut current_dir = String::<512>::new();
            if let Ok(Some(existing_dir)) = self.read_file(".dir", &mut dir_buf).await {
                if let Ok(s) = core::str::from_utf8(existing_dir) {
                    let _ = current_dir.push_str(s);
                }
            }

            // Check if name is already in the list
            let mut found = false;
            for entry in current_dir.split('\n') {
                if entry == name {
                    found = true;
                    break;
                }
            }

            if !found {
                if !current_dir.is_empty() {
                    let _ = current_dir.push('\n');
                }
                let _ = current_dir.push_str(name);

                // Write directory directly to avoid async recursion cycle
                let dir_key = Self::string_to_key(".dir");
                let _ = sequential_storage::map::store_item(
                    &mut self.flash,
                    self.range.clone(),
                    &mut cache,
                    self.buf,
                    &dir_key,
                    &current_dir.as_bytes(),
                )
                .await;
            }
        }

        Ok(())
    }

    /// Fetches a file's content.
    pub async fn read_file<'a>(
        &mut self,
        name: &str,
        out_buf: &'a mut [u8],
    ) -> Result<Option<&'a [u8]>, ()> {
        let mut cache = sequential_storage::cache::NoCache::new();
        let key = Self::string_to_key(name);

        let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
            &mut self.flash,
            self.range.clone(),
            &mut cache,
            self.buf,
            &key,
        )
        .await;
        match res {
            Ok(Some(val)) => {
                if val.len() <= out_buf.len() {
                    out_buf[..val.len()].copy_from_slice(val);
                    Ok(Some(&out_buf[..val.len()]))
                } else {
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    defmt::error!(
                        "read_file: output buffer too small ({} bytes) for file of size {} bytes",
                        out_buf.len(),
                        val.len()
                    );
                    Err(())
                }
            }
            Ok(None) => Ok(None),
            Err(_e) => {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::error!("fetch_item failed: {:?}", defmt::Debug2Format(&_e));
                #[cfg(not(all(target_arch = "arm", target_os = "none")))]
                std::eprintln!("fetch_item failed: {:?}", _e);
                Err(())
            }
        }
    }

    /// Removes a file from storage.
    pub async fn remove_file(&mut self, name: &str) -> Result<(), ()> {
        let mut cache = sequential_storage::cache::NoCache::new();
        let key = Self::string_to_key(name);

        // Remove from map
        let res = sequential_storage::map::remove_item::<[u8; 32], _>(
            &mut self.flash,
            self.range.clone(),
            &mut cache,
            self.buf,
            &key,
        )
        .await;

        if res.is_err() {
            return Err(());
        }

        // If this is not the directory index itself, remove it from the index
        if name != ".dir" {
            let mut dir_buf = [0u8; 512];
            let mut current_dir = String::<512>::new();
            if let Ok(Some(existing_dir)) = self.read_file(".dir", &mut dir_buf).await {
                if let Ok(s) = core::str::from_utf8(existing_dir) {
                    let _ = current_dir.push_str(s);
                }
            }

            let mut new_dir = String::<512>::new();
            for entry in current_dir.split('\n') {
                if entry != name && !entry.is_empty() {
                    if !new_dir.is_empty() {
                        let _ = new_dir.push('\n');
                    }
                    let _ = new_dir.push_str(entry);
                }
            }

            // Write directory directly to avoid async recursion cycle
            let dir_key = Self::string_to_key(".dir");
            let _ = sequential_storage::map::store_item(
                &mut self.flash,
                self.range.clone(),
                &mut cache,
                self.buf,
                &dir_key,
                &new_dir.as_bytes(),
            )
            .await;
        }

        Ok(())
    }

    /// Returns a newline-separated string listing all files currently stored.
    pub async fn list_files<'a>(&mut self, out_buf: &'a mut [u8]) -> Result<Option<&'a [u8]>, ()> {
        self.read_file(".dir", out_buf).await
    }

    /// Erases the entire filesystem partition.
    pub async fn format(&mut self) -> Result<(), ()> {
        self.flash
            .erase(self.range.start, self.range.end)
            .await
            .map_err(|_| ())
    }

    /// Verifies the filesystem health by trying to read the directory index.
    /// If it returns a Corrupted or InvalidValue error, it formats/erases the entire partition.
    pub async fn verify_and_repair(&mut self) -> Result<(), ()> {
        let mut cache = sequential_storage::cache::NoCache::new();
        let key = Self::string_to_key(".dir");
        let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
            &mut self.flash,
            self.range.clone(),
            &mut cache,
            self.buf,
            &key,
        )
        .await;

        if res.is_err() {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::error!("Filesystem corrupted or invalid! Reformatting partition...");

            // Erase the entire range
            if self
                .flash
                .erase(self.range.start, self.range.end)
                .await
                .is_err()
            {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::error!("Failed to erase corrupted partition!");
                return Err(());
            }

            // Re-write an empty directory index
            if sequential_storage::map::store_item(
                &mut self.flash,
                self.range.clone(),
                &mut cache,
                self.buf,
                &key,
                &[0u8; 0],
            )
            .await
            .is_err()
            {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::error!("Failed to write empty directory after format!");
                return Err(());
            }

            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::info!("Filesystem partition successfully reformatted.");
        }

        Ok(())
    }
}

impl<F: NorFlash + MultiwriteNorFlash> FilesystemController<ProfilingFlash<F>> {
    /// Set telemetry sender for flash erase profiling.
    pub fn set_telemetry(
        &mut self,
        telemetry_tx: embassy_sync::channel::Sender<
            'static,
            CriticalSectionRawMutex,
            model::telemetry::TelemetryRecord,
            64,
        >,
    ) {
        self.flash.set_telemetry(telemetry_tx);
    }
}

/// Request command for pipelining filesystem operations from different runloops.
#[allow(clippy::type_complexity)]
pub enum FsRequest {
    /// Write file request
    WriteFile {
        /// File name
        name: &'static str,
        /// Raw pointer to the content buffer
        content_ptr: *const u8,
        /// Length of the content buffer
        content_len: usize,
        /// Raw pointer to the signal for response notification
        signal: *const Signal<CriticalSectionRawMutex, Result<(), ()>>,
    },
    /// Read file request
    ReadFile {
        /// File name
        name: &'static str,
        /// Raw pointer to the output buffer
        buf_ptr: *mut u8,
        /// Length of the output buffer
        buf_len: usize,
        /// Raw pointer to the signal for response notification
        signal: *const Signal<CriticalSectionRawMutex, Result<Option<(usize, usize)>, ()>>,
    },
}

unsafe impl Send for FsRequest {}
unsafe impl Sync for FsRequest {}

/// Client interface for interacting with the pipelined filesystem.
#[derive(Clone, Copy)]
pub struct FilesystemClient {
    sender: embassy_sync::channel::Sender<'static, CriticalSectionRawMutex, FsRequest, 16>,
}

impl FilesystemClient {
    /// Create a new FilesystemClient.
    pub fn new(
        sender: embassy_sync::channel::Sender<'static, CriticalSectionRawMutex, FsRequest, 16>,
    ) -> Self {
        Self { sender }
    }

    /// Stores/overwrites a file with the given name and contents asynchronously.
    pub async fn write_file(&self, name: &'static str, content: &[u8]) -> Result<(), ()> {
        let signal = Signal::new();
        let request = FsRequest::WriteFile {
            name,
            content_ptr: content.as_ptr(),
            content_len: content.len(),
            signal: &signal as *const _,
        };
        self.sender.send(request).await;
        signal.wait().await
    }

    /// Starts a file write operation asynchronously without waiting for completion.
    /// The caller must ensure that the content buffer remains valid until the write completes.
    pub async fn start_write_file(
        &self,
        name: &'static str,
        content: &[u8],
        signal: &'static Signal<CriticalSectionRawMutex, Result<(), ()>>,
    ) {
        signal.reset();
        let request = FsRequest::WriteFile {
            name,
            content_ptr: content.as_ptr(),
            content_len: content.len(),
            signal: signal as *const _,
        };
        self.sender.send(request).await;
    }

    /// Fetches a file's content asynchronously.
    pub async fn read_file<'a>(
        &self,
        name: &'static str,
        out_buf: &'a mut [u8],
    ) -> Result<Option<&'a [u8]>, ()> {
        let signal = Signal::new();
        let request = FsRequest::ReadFile {
            name,
            buf_ptr: out_buf.as_mut_ptr(),
            buf_len: out_buf.len(),
            signal: &signal as *const _,
        };
        self.sender.send(request).await;
        match signal.wait().await {
            Ok(Some((start, len))) => Ok(Some(&out_buf[start..start + len])),
            Ok(None) => Ok(None),
            Err(()) => Err(()),
        }
    }
}

/// Task loop for the filesystem pipeline.
pub async fn run_filesystem_task<F: NorFlash + MultiwriteNorFlash>(
    mut fs: FilesystemController<F>,
    rx: embassy_sync::channel::Receiver<'static, CriticalSectionRawMutex, FsRequest, 16>,
) -> ! {
    let _ = fs.verify_and_repair().await;
    loop {
        let req = rx.receive().await;
        match req {
            FsRequest::WriteFile {
                name,
                content_ptr,
                content_len,
                signal,
            } => {
                let content = unsafe { core::slice::from_raw_parts(content_ptr, content_len) };
                let res = fs.write_file(name, content).await;
                unsafe { &*signal }.signal(res);
            }
            FsRequest::ReadFile {
                name,
                buf_ptr,
                buf_len,
                signal,
            } => {
                let buf = unsafe { core::slice::from_raw_parts_mut(buf_ptr, buf_len) };
                let base_ptr = buf.as_ptr() as usize;
                let res = fs.read_file(name, buf).await;
                let mapped_res = res.map(|opt| {
                    opt.map(|slice| {
                        let start = slice.as_ptr() as usize - base_ptr;
                        (start, slice.len())
                    })
                });
                unsafe { &*signal }.signal(mapped_res);
            }
        }
    }
}

/// Filesystem-specific CLI commands
#[derive(Debug, embedded_cli::Command, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemCliCommand {
    /// Format/erase the filesystem partition
    Format,
}

/// Processes filesystem-specific CLI commands
pub fn process_filesystem_command<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    F: embedded_storage::nor_flash::NorFlash + 'static,
>(
    flash_ptr: Option<*mut F>,
    storage_start: u32,
    storage_end: u32,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
    cmd: FilesystemCliCommand,
) -> Result<(), &'static str> {
    match cmd {
        FilesystemCliCommand::Format => {
            if let Some(flash_raw) = flash_ptr {
                static mut SHELL_FS_BUF_3: [u8; 2048] = [0u8; 2048];
                let flash_ref = unsafe { &mut *flash_raw };
                let async_flash = firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
                let fs_buf = unsafe { &mut *core::ptr::addr_of_mut!(SHELL_FS_BUF_3) };
                let mut fs = crate::filesystem_controller::FilesystemController::new(
                    async_flash,
                    storage_start..storage_end,
                    fs_buf,
                );

                let _ = core::writeln!(writer, "\r\nFormatting filesystem...");
                let res = embassy_futures::block_on(fs.format());
                match res {
                    Ok(()) => {
                        let _ = core::writeln!(
                            writer,
                            "Formatting successful! Rebooting target system..."
                        );
                        #[cfg(all(target_arch = "arm", target_os = "none"))]
                        cortex_m::peripheral::SCB::sys_reset();
                        #[allow(unreachable_code)]
                        Ok(())
                    }
                    Err(()) => Err("Formatting failed!"),
                }
            } else {
                Err("Flash peripheral not available")
            }
        }
    }
}
