use embedded_storage::nor_flash::NorFlashErrorKind;
use probe_rs::probe::list::Lister;
use probe_rs::MemoryInterface;
pub use tool_common::autodetect_project_info;

/// A mock flash driver that implements the embedded-storage-async traits
/// over an in-memory buffer containing the pulled raw flash binary image.
pub struct HostFlash {
    /// In-memory buffer representing the flash contents
    pub data: Vec<u8>,
}

impl HostFlash {
    /// Creates a new HostFlash instance with the provided byte buffer.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl embedded_storage_async::nor_flash::ErrorType for HostFlash {
    type Error = NorFlashErrorKind;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for HostFlash {
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        if end <= self.data.len() {
            bytes.copy_from_slice(&self.data[start..end]);
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for HostFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 1024;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        if end <= self.data.len() {
            self.data[start..end].copy_from_slice(bytes);
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let start = from as usize;
        let end = to as usize;
        if end <= self.data.len() {
            self.data[start..end].fill(0xFF);
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for HostFlash {}

/// A target-attached flash driver that implements the embedded-storage-async traits
/// directly over an active probe-rs debug session. Uses a local buffer cache
/// to enable fast in-memory reads and tracks dirty sectors for efficient write-back.
pub struct ProbeFlash {
    session: probe_rs::Session,
    base_address: u32,
    /// Local copy of the flash contents to avoid slow small USB reads
    pub data: Vec<u8>,
    /// Track dirty sectors (each sector is 4096 bytes)
    dirty_sectors: Vec<bool>,
}

impl ProbeFlash {
    /// Creates a new ProbeFlash instance by attaching to the target chip and bulk-reading the partition.
    pub fn new(chip: &str, base_address: u32, capacity: usize) -> Result<Self, String> {
        let lister = Lister::new();
        let probes = lister.list_all();
        let probe_info = probes
            .first()
            .ok_or_else(|| "No debug probes connected".to_string())?;
        let probe = probe_info
            .open()
            .map_err(|e| format!("Failed to open probe: {:?}", e))?;

        let mut session = probe
            .attach(chip, probe_rs::Permissions::default())
            .map_err(|e| format!("Failed to attach to chip {}: {:?}", chip, e))?;

        // Bulk read the entire filesystem partition from the device
        let mut data = vec![0u8; capacity];
        {
            let mut core = session
                .core(0)
                .map_err(|e| format!("Failed to access core: {:?}", e))?;
            core.read_8(base_address as u64, &mut data)
                .map_err(|e| format!("Failed to read memory from target: {:?}", e))?;
        }

        let sector_size = 4096;
        let num_sectors = capacity.div_ceil(sector_size);
        let dirty_sectors = vec![false; num_sectors];

        Ok(Self {
            session,
            base_address,
            data,
            dirty_sectors,
        })
    }

    /// Commit the modified dirty sectors back to the target's flash
    pub fn commit(&mut self) -> Result<(), String> {
        let sector_size = 4096;
        let mut has_dirty = false;
        let mut loader = self.session.target().flash_loader();

        for (sec_idx, &dirty) in self.dirty_sectors.iter().enumerate() {
            if dirty {
                has_dirty = true;
                let start_offset = sec_idx * sector_size;
                let end_offset = std::cmp::min(start_offset + sector_size, self.data.len());
                let addr = self.base_address as u64 + start_offset as u64;
                loader
                    .add_data(addr, &self.data[start_offset..end_offset])
                    .map_err(|e| format!("Failed to add data to loader: {:?}", e))?;
            }
        }

        if has_dirty {
            let options = probe_rs::flashing::DownloadOptions::default();
            loader
                .commit(&mut self.session, options)
                .map_err(|e| format!("Failed to commit flash data: {:?}", e))?;
        }

        // Reset dirty tracking
        self.dirty_sectors.fill(false);
        Ok(())
    }
}

impl embedded_storage_async::nor_flash::ErrorType for ProbeFlash {
    type Error = NorFlashErrorKind;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for ProbeFlash {
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        if end <= self.data.len() {
            bytes.copy_from_slice(&self.data[start..end]);
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for ProbeFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 1024;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        if end <= self.data.len() {
            self.data[start..end].copy_from_slice(bytes);

            // Mark affected sectors as dirty
            let sector_size = 4096;
            let start_sec = start / sector_size;
            let end_sec = (end - 1) / sector_size;
            for sec in start_sec..=end_sec {
                if sec < self.dirty_sectors.len() {
                    self.dirty_sectors[sec] = true;
                }
            }
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let start = from as usize;
        let end = to as usize;
        if end <= self.data.len() {
            self.data[start..end].fill(0xFF);

            // Mark affected sectors as dirty
            let sector_size = 4096;
            let start_sec = start / sector_size;
            let end_sec = (end - 1) / sector_size;
            for sec in start_sec..=end_sec {
                if sec < self.dirty_sectors.len() {
                    self.dirty_sectors[sec] = true;
                }
            }
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for ProbeFlash {}

pub struct GdbFlash {
    gdb: tool_common::GdbClient,
    base_address: u32,
    pub data: Vec<u8>,
    dirty_sectors: Vec<bool>,
}

impl GdbFlash {
    pub fn new(addr: &str, base_address: u32, capacity: usize) -> Result<Self, String> {
        let mut gdb = tool_common::GdbClient::connect(addr)?;

        let mut data = vec![0u8; capacity];
        gdb.read_mem(base_address as u64, &mut data)?;

        let sector_size = 4096;
        let num_sectors = capacity.div_ceil(sector_size);
        let dirty_sectors = vec![false; num_sectors];

        Ok(Self {
            gdb,
            base_address,
            data,
            dirty_sectors,
        })
    }

    pub fn commit(&mut self) -> Result<(), String> {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("flash_partition_update.bin");
        std::fs::write(&temp_file, &self.data)
            .map_err(|e| format!("Failed to write temp binary: {:?}", e))?;

        // Halt target before writing/flashing
        let _ = self.gdb.halt();

        let cmd = format!(
            "monitor flash write_image erase {} 0x{:08x} bin",
            temp_file.display(),
            self.base_address
        );
        let send_res = self.gdb.run_monitor_cmd(&cmd);

        let _ = std::fs::remove_file(temp_file);

        // Resume execution after flashing
        let _ = self.gdb.run();

        send_res?;

        self.dirty_sectors.fill(false);
        Ok(())
    }
}

impl embedded_storage_async::nor_flash::ErrorType for GdbFlash {
    type Error = NorFlashErrorKind;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for GdbFlash {
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        if end <= self.data.len() {
            bytes.copy_from_slice(&self.data[start..end]);
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for GdbFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 1024;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        if end <= self.data.len() {
            self.data[start..end].copy_from_slice(bytes);

            let sector_size = 4096;
            let start_sec = start / sector_size;
            let end_sec = (end - 1) / sector_size;
            for sec in start_sec..=end_sec {
                if sec < self.dirty_sectors.len() {
                    self.dirty_sectors[sec] = true;
                }
            }
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let start = from as usize;
        let end = to as usize;
        if end <= self.data.len() {
            self.data[start..end].fill(0xFF);

            let sector_size = 4096;
            let start_sec = start / sector_size;
            let end_sec = (end - 1) / sector_size;
            for sec in start_sec..=end_sec {
                if sec < self.dirty_sectors.len() {
                    self.dirty_sectors[sec] = true;
                }
            }
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for GdbFlash {}

pub enum EitherFlash {
    Host(HostFlash),
    Probe(Box<ProbeFlash>),
    Gdb(Box<GdbFlash>),
}

impl EitherFlash {
    pub fn capacity(&self) -> usize {
        match self {
            EitherFlash::Host(f) => f.data.len(),
            EitherFlash::Probe(f) => f.data.len(),
            EitherFlash::Gdb(f) => f.data.len(),
        }
    }
}

impl embedded_storage_async::nor_flash::ErrorType for EitherFlash {
    type Error = NorFlashErrorKind;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for EitherFlash {
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        match self {
            EitherFlash::Host(f) => f.read(offset, bytes).await,
            EitherFlash::Probe(f) => f.read(offset, bytes).await,
            EitherFlash::Gdb(f) => f.read(offset, bytes).await,
        }
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for EitherFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 1024;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        match self {
            EitherFlash::Host(f) => f.write(offset, bytes).await,
            EitherFlash::Probe(f) => f.write(offset, bytes).await,
            EitherFlash::Gdb(f) => f.write(offset, bytes).await,
        }
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        match self {
            EitherFlash::Host(f) => f.erase(from, to).await,
            EitherFlash::Probe(f) => f.erase(from, to).await,
            EitherFlash::Gdb(f) => f.erase(from, to).await,
        }
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for EitherFlash {}
