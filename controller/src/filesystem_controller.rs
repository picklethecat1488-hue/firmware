//! Project-agnostic flat filesystem controller built over sequential-storage.

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
extern crate std;

use crate::{Sender, TelemetrySender};
use core::fmt::Write as _;
use core::ops::Range;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embedded_storage_async::nor_flash::{MultiwriteNorFlash, NorFlash};

// =========================================================================
// Filesystem Capacity & Buffer Constants
// =========================================================================

pub use platform::directory::{string_to_key, DIR_BUF_SIZE, KEY_SIZE};

pub use platform::flash::ProfilingFlash;

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

    /// Stores/overwrites a file with the given name (key) and contents (value).
    #[crate::tracing::instrument(
        name = "filesystem_controller::write_file",
        level = "info",
        skip(content)
    )]
    pub async fn write_file(&mut self, name: &str, content: &[u8]) -> Result<(), ()> {
        let mut cache = sequential_storage::cache::NoCache::new();
        let key = string_to_key(name);

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

        match res {
            Err(_e) => {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::error!("store_item failed: {:?}", defmt::Debug2Format(&_e));
                #[cfg(not(all(target_arch = "arm", target_os = "none")))]
                std::eprintln!("store_item failed: {:?}", _e);
                Err(())
            }
            Ok(()) => {
                // If this is not the directory index itself, add it to the directory index
                if name != ".dir" {
                    let mut dir_buf = [0u8; DIR_BUF_SIZE];
                    let mut existing_dir_str = "";
                    if let Ok(Some(existing_dir)) = self.read_file(".dir", &mut dir_buf).await {
                        if let Ok(s) = core::str::from_utf8(existing_dir) {
                            existing_dir_str = s;
                        }
                    }

                    if let Some(new_dir) =
                        platform::directory::add_to_directory(existing_dir_str, name)
                    {
                        // Write directory directly to avoid async recursion cycle
                        let dir_key = string_to_key(".dir");
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
                }
                Ok(())
            }
        }
    }

    /// Fetches a file's content.
    #[crate::tracing::instrument(
        name = "filesystem_controller::read_file",
        level = "info",
        skip(out_buf)
    )]
    pub async fn read_file<'a>(
        &mut self,
        name: &str,
        out_buf: &'a mut [u8],
    ) -> Result<Option<&'a [u8]>, ()> {
        self.read_file_raw(name, out_buf).await
    }

    /// Uninstrumented raw file read for boot/init contexts.
    pub async fn read_file_raw<'a>(
        &mut self,
        name: &str,
        out_buf: &'a mut [u8],
    ) -> Result<Option<&'a [u8]>, ()> {
        let mut cache = sequential_storage::cache::NoCache::new();
        let key = string_to_key(name);

        let res = sequential_storage::map::fetch_item::<[u8; KEY_SIZE], &[u8], _>(
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
    #[crate::tracing::instrument(name = "filesystem_controller::remove_file", level = "info")]
    pub async fn remove_file(&mut self, name: &str) -> Result<(), ()> {
        let mut cache = sequential_storage::cache::NoCache::new();
        let key = string_to_key(name);

        // Remove from map
        let res = sequential_storage::map::remove_item::<[u8; KEY_SIZE], _>(
            &mut self.flash,
            self.range.clone(),
            &mut cache,
            self.buf,
            &key,
        )
        .await;

        if res.is_err() {
            Err(())
        } else {
            // If this is not the directory index itself, remove it from the index
            if name != ".dir" {
                let mut dir_buf = [0u8; DIR_BUF_SIZE];
                let mut existing_dir_str = "";
                if let Ok(Some(existing_dir)) = self.read_file(".dir", &mut dir_buf).await {
                    if let Ok(s) = core::str::from_utf8(existing_dir) {
                        existing_dir_str = s;
                    }
                }

                let new_dir = platform::directory::remove_from_directory(existing_dir_str, name);

                // Write directory directly to avoid async recursion cycle
                let dir_key = string_to_key(".dir");
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
        let key = string_to_key(".dir");
        let res = sequential_storage::map::fetch_item::<[u8; KEY_SIZE], &[u8], _>(
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
                Err(())
            } else if sequential_storage::map::store_item(
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
                Err(())
            } else {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::info!("Filesystem partition successfully reformatted.");
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

impl<F: NorFlash + MultiwriteNorFlash> FilesystemController<ProfilingFlash<F>> {
    /// Set telemetry sender for flash erase profiling.
    pub fn set_telemetry(&mut self, telemetry_tx: TelemetrySender<CriticalSectionRawMutex, 64>) {
        self.flash.set_telemetry(telemetry_tx);
    }
}

/// Request command for pipelining filesystem operations from different runloops.
#[allow(clippy::type_complexity)]
pub enum FsRequest {
    /// Write file request
    WriteFile {
        /// File name
        name: heapless::String<{ platform::MAX_FILE_NAME_LEN }>,
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
        name: heapless::String<{ platform::MAX_FILE_NAME_LEN }>,
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
    sender: Sender<'static, CriticalSectionRawMutex, FsRequest, 16>,
}

impl FilesystemClient {
    /// Create a new FilesystemClient.
    pub fn new(sender: Sender<'static, CriticalSectionRawMutex, FsRequest, 16>) -> Self {
        Self { sender }
    }

    /// Stores/overwrites a file with the given name and contents asynchronously.
    pub async fn write_file(&self, name: &str, content: &[u8]) -> Result<(), ()> {
        let signal = Signal::new();
        let name_str = heapless::String::try_from(name).map_err(|_| ())?;
        let request = FsRequest::WriteFile {
            name: name_str,
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
        name: &str,
        content: &[u8],
        signal: &'static Signal<CriticalSectionRawMutex, Result<(), ()>>,
    ) {
        signal.reset();
        let name_str = heapless::String::try_from(name).unwrap_or_default();
        let request = FsRequest::WriteFile {
            name: name_str,
            content_ptr: content.as_ptr(),
            content_len: content.len(),
            signal: signal as *const _,
        };
        self.sender.send(request).await;
    }

    /// Fetches a file's content asynchronously.
    pub async fn read_file<'a>(
        &self,
        name: &str,
        out_buf: &'a mut [u8],
    ) -> Result<Option<&'a [u8]>, ()> {
        let signal = Signal::new();
        let name_str = heapless::String::try_from(name).map_err(|_| ())?;
        let request = FsRequest::ReadFile {
            name: name_str,
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

impl<F: NorFlash + MultiwriteNorFlash> FilesystemController<F> {
    /// Task loop for the filesystem pipeline.
    pub async fn run(&mut self, rx: crate::FilesystemReceiver<CriticalSectionRawMutex, 16>) -> ! {
        let _ = self.verify_and_repair().await;
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
                    let res = self.write_file(name.as_str(), content).await;
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
                    let res = self.read_file(name.as_str(), buf).await;
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
}

use platform::subcommand_enum;

subcommand_enum! {
    /// Filesystem subcommands for CLI processing.
    pub enum FilesystemSubcommand {
        /// Format filesystem partition
        Format,
        /// List files in directory
        Ls,
    }
    "Invalid fs subcommand. Expected: format, ls"
}

subcommand_enum! {
    /// Format target for partition erasure.
    pub enum FormatTarget {
        /// Erase only the filesystem partition
        Fs,
        /// Erase only the telemetry partition
        Telemetry,
        /// Erase both partitions
        All,
    }
    "Invalid format target. Expected: fs, telemetry, all"
}

/// Processes filesystem-specific CLI subcommands.
pub fn handle_fs_cli<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    C: crate::ShellConfig,
>(
    resolver: &impl crate::ShellDeviceResolver<C>,
    subcommand: Option<FilesystemSubcommand>,
    target: Option<FormatTarget>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let partition = resolver.resolve_partition(None)?;
    let mut fs_buf = resolver.lock_fs_buffer()?;
    let fs_buf_static = unsafe { fs_buf.as_static_mut() };

    let cmd = subcommand.ok_or("Missing fs subcommand")?;
    let target = target.unwrap_or(FormatTarget::Fs);

    match cmd {
        FilesystemSubcommand::Format => {
            let flash_ref = unsafe { &mut *partition.flash_ptr };
            let mut async_flash = platform::BlockingAsyncFlash(flash_ref);

            match target {
                FormatTarget::Fs => {
                    let _ = core::writeln!(writer, "\r\nFormatting filesystem partition...");
                    let res = embassy_futures::block_on(
                        async_flash
                            .erase(partition.start_address, partition.start_address + 64 * 1024),
                    );
                    match res {
                        Ok(()) => {
                            let _ = core::writeln!(
                                writer,
                                "Formatting successful! Rebooting target system..."
                            );
                            #[cfg(all(target_arch = "arm", target_os = "none"))]
                            {
                                embassy_time::block_for(embassy_time::Duration::from_secs(2));
                                cortex_m::peripheral::SCB::sys_reset();
                            }
                            #[allow(unreachable_code)]
                            Ok(())
                        }
                        Err(_) => Err("Formatting failed!"),
                    }
                }
                FormatTarget::Telemetry => {
                    let _ = core::writeln!(writer, "\r\nFormatting telemetry partition...");
                    let res = embassy_futures::block_on(
                        async_flash
                            .erase(partition.start_address + 64 * 1024, partition.end_address),
                    );
                    match res {
                        Ok(()) => {
                            let _ =
                                core::writeln!(writer, "Formatting successful! Telemetry cleared.");
                            Ok(())
                        }
                        Err(_) => Err("Formatting failed!"),
                    }
                }
                FormatTarget::All => {
                    let _ = core::writeln!(writer, "\r\nFormatting all partitions...");
                    let res = embassy_futures::block_on(
                        async_flash.erase(partition.start_address, partition.end_address),
                    );
                    match res {
                        Ok(()) => {
                            let _ = core::writeln!(
                                writer,
                                "Formatting successful! Rebooting target system..."
                            );
                            #[cfg(all(target_arch = "arm", target_os = "none"))]
                            {
                                embassy_time::block_for(embassy_time::Duration::from_secs(2));
                                cortex_m::peripheral::SCB::sys_reset();
                            }
                            #[allow(unreachable_code)]
                            Ok(())
                        }
                        Err(_) => Err("Formatting failed!"),
                    }
                }
            }
        }
        FilesystemSubcommand::Ls => {
            let flash_ref = unsafe { &mut *partition.flash_ptr };
            let async_flash = platform::BlockingAsyncFlash(flash_ref);
            let mut fs = crate::filesystem_controller::FilesystemController::new(
                async_flash,
                partition.start_address..partition.end_address,
                fs_buf_static,
            );

            let _ = core::writeln!(writer, "\r\nListing directory...");
            let mut dir_buf = [0u8; DIR_BUF_SIZE];
            let res = embassy_futures::block_on(fs.read_file(".dir", &mut dir_buf));
            match res {
                Ok(Some(list)) => {
                    if let Ok(s) = core::str::from_utf8(list) {
                        let _ = core::writeln!(writer, "Filename");
                        let _ = core::writeln!(
                            writer,
                            "--------------------------------------------------"
                        );
                        for line in s.split('\n') {
                            if !line.is_empty() {
                                let _ = core::writeln!(writer, "{}", line);
                            }
                        }
                    } else {
                        let _ =
                            core::writeln!(writer, "Error: Directory list contains invalid UTF-8");
                    }
                    Ok(())
                }
                Ok(None) => {
                    let _ = core::writeln!(writer, "No files found (directory empty).");
                    Ok(())
                }
                Err(()) => {
                    let _ = core::writeln!(writer, "Error: Directory file is corrupted.");
                    Err("Failed to read directory")
                }
            }
        }
    }
}
