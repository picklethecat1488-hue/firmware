//! Project-agnostic flat filesystem controller built over sequential-storage.

use core::cmp;
use core::ops::Range;
use embedded_storage_async::nor_flash::{MultiwriteNorFlash, NorFlash};
use heapless::String;

/// A profiling wrapper around a flash driver that counts and times page erases.
pub struct ProfilingFlash<F: NorFlash> {
    inner: F,
    erase_count: u32,
}

impl<F: NorFlash> ProfilingFlash<F> {
    /// Create a new ProfilingFlash wrapper.
    pub fn new(inner: F) -> Self {
        Self { inner, erase_count: 0 }
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
        defmt::info!("[Profile] Flash erase starting at 0x{:X} to 0x{:X}", from, to);
        
        let res = self.inner.erase(from, to).await;
        
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            let duration = start.elapsed();
            defmt::info!("[Profile] Flash erase completed in {} ms (Total erases: {})", duration.as_millis(), self.erase_count);
        }
        
        res
    }
}

impl<F: NorFlash + MultiwriteNorFlash> MultiwriteNorFlash for ProfilingFlash<F> {}



/// File Controller managing raw files/telemetry in flash using sequential-storage map.
pub struct FilesystemController<F: NorFlash + MultiwriteNorFlash> {
    flash: F,
    range: Range<u32>,
}

impl<F: NorFlash + MultiwriteNorFlash> FilesystemController<F> {
    /// Creates a new FilesystemController.
    pub fn new(flash: F, range: Range<u32>) -> Self {
        Self { flash, range }
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
        let mut buf = [0u8; 1024];
        let mut cache = sequential_storage::cache::NoCache::new();
        let key = Self::string_to_key(name);

        // Write the file content
        let res = sequential_storage::map::store_item(
            &mut self.flash,
            self.range.clone(),
            &mut cache,
            &mut buf,
            &key,
            &content,
        )
        .await;

        if res.is_err() {
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
                let mut dir_write_buf = [0u8; 1024];
                let _ = sequential_storage::map::store_item(
                    &mut self.flash,
                    self.range.clone(),
                    &mut cache,
                    &mut dir_write_buf,
                    &dir_key,
                    &current_dir.as_bytes(),
                )
                .await;
            }
        }

        Ok(())
    }

    /// Fetches a file's content.
    pub async fn read_file<'a>(&mut self, name: &str, out_buf: &'a mut [u8]) -> Result<Option<&'a [u8]>, ()> {
        let mut cache = sequential_storage::cache::NoCache::new();
        let key = Self::string_to_key(name);
        let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
            &mut self.flash,
            self.range.clone(),
            &mut cache,
            out_buf,
            &key,
        )
        .await;
        match res {
            Ok(val) => Ok(val),
            Err(_) => Err(()),
        }
    }

    /// Removes a file from storage.
    pub async fn remove_file(&mut self, name: &str) -> Result<(), ()> {
        let mut buf = [0u8; 1024];
        let mut cache = sequential_storage::cache::NoCache::new();
        let key = Self::string_to_key(name);

        // Remove from map
        let res = sequential_storage::map::remove_item::<[u8; 32], _>(
            &mut self.flash,
            self.range.clone(),
            &mut cache,
            &mut buf,
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
            let mut dir_write_buf = [0u8; 1024];
            let _ = sequential_storage::map::store_item(
                &mut self.flash,
                self.range.clone(),
                &mut cache,
                &mut dir_write_buf,
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

}
