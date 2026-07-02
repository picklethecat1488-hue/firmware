use embedded_storage::nor_flash::NorFlashErrorKind;
use probe_rs::probe::list::Lister;
use probe_rs::MemoryInterface;

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

pub enum EitherFlash {
    Host(HostFlash),
    Probe(Box<ProbeFlash>),
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
        }
    }

    fn capacity(&self) -> usize {
        match self {
            EitherFlash::Host(f) => f.capacity(),
            EitherFlash::Probe(f) => f.capacity(),
        }
    }
}

impl embedded_storage_async::nor_flash::NorFlash for EitherFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 1024;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        match self {
            EitherFlash::Host(f) => f.write(offset, bytes).await,
            EitherFlash::Probe(f) => f.write(offset, bytes).await,
        }
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        match self {
            EitherFlash::Host(f) => f.erase(from, to).await,
            EitherFlash::Probe(f) => f.erase(from, to).await,
        }
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for EitherFlash {}

pub fn decode_project_info(project_name: &str) -> Result<(String, u32, usize), String> {
    // 1. Find projects directory by searching parent directories
    let mut dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {:?}", e))?;
    let mut projects_dir = None;
    for _ in 0..5 {
        let candidate = dir.join("projects");
        if candidate.is_dir() {
            projects_dir = Some(candidate);
            break;
        }
        if let Some(parent) = dir.parent() {
            dir = parent.to_path_buf();
        } else {
            break;
        }
    }

    let projects = projects_dir.ok_or_else(|| {
        "Could not find 'projects' directory in current directory or parent directories".to_string()
    })?;

    let project_dir = projects.join(project_name);
    if !project_dir.exists() {
        return Err(format!(
            "Project directory '{}' not found.",
            project_dir.display()
        ));
    }

    // 2. Read .cargo/config.toml to get the chip name
    let config_path = project_dir.join(".cargo/config.toml");
    let config_content = std::fs::read_to_string(&config_path)
        .map_err(|_| format!("Failed to read config file at '{}'", config_path.display()))?;

    // Find chip name by looking for "--chip"
    let chip = if let Some(idx) = config_content.find("--chip") {
        let after_chip = config_content[idx + 6..].trim();
        let end_idx = after_chip
            .find(|c: char| c.is_whitespace() || c == '"' || c == '\'')
            .unwrap_or(after_chip.len());
        after_chip[..end_idx].to_string()
    } else {
        return Err(
            "Could not find chip name in .cargo/config.toml (expected '--chip <CHIP>')".to_string(),
        );
    };

    // 3. Read memory.x to get partition layout
    let memory_path = project_dir.join("memory.x");
    let memory_content = std::fs::read_to_string(&memory_path)
        .map_err(|_| format!("Failed to read memory.x at '{}'", memory_path.display()))?;

    // Clean whitespaces for easier parsing
    let clean_mem: String = memory_content
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect();

    // Find FLASH:ORIGIN=
    let origin_marker = "FLASH:ORIGIN=";
    let flash_origin = if let Some(idx) = clean_mem.find(origin_marker) {
        let val_part = &clean_mem[idx + origin_marker.len()..];
        let end_idx = val_part.find(',').unwrap_or(val_part.len());
        let val_str = &val_part[..end_idx];
        parse_int_str(val_str)?
    } else {
        return Err("Could not find FLASH ORIGIN in memory.x".to_string());
    };

    // Align flash origin to sector/page boundary (usually 0x10000000)
    let flash_base = flash_origin & 0xFFFF0000;

    // Find LENGTH=
    let len_marker = "LENGTH=";
    let flash_len = if let Some(idx) = clean_mem.find("FLASH:ORIGIN=") {
        let after_origin = &clean_mem[idx..];
        if let Some(l_idx) = after_origin.find(len_marker) {
            let val_part = &after_origin[l_idx + len_marker.len()..];
            let end_idx = val_part.find('}').unwrap_or(val_part.len());
            let val_str = &val_part[..end_idx];
            parse_length_str(val_str)?
        } else {
            return Err("Could not find FLASH LENGTH in memory.x".to_string());
        }
    } else {
        return Err("Could not find FLASH definition in memory.x".to_string());
    };

    // Calculate partition offset
    let partition_address = flash_base + flash_len;

    // Partition size is the remaining flash. RP2040 flash is typically 2MB (2048K)
    let total_flash = 2048 * 1024;
    let partition_size = if flash_len < total_flash {
        (total_flash - flash_len) as usize
    } else {
        256 * 1024 // Default fallback
    };

    Ok((chip, partition_address, partition_size))
}

fn parse_int_str(s: &str) -> Result<u32, String> {
    let s = s.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), 16)
            .map_err(|e| format!("Failed to parse hex integer {}: {:?}", s, e))
    } else {
        s.parse::<u32>()
            .map_err(|e| format!("Failed to parse integer {}: {:?}", s, e))
    }
}

fn parse_length_str(s: &str) -> Result<u32, String> {
    let s = s.trim();
    // It might contain math expression like "1792K-0x100" or just "1792K"
    let clean = s.split('-').next().unwrap().trim(); // Take part before '-' if any

    // Parse number and multiplier
    let num_str: String = clean.chars().take_while(|c| c.is_ascii_digit()).collect();
    let num = num_str
        .parse::<u32>()
        .map_err(|e| format!("Failed to parse number in length {}: {:?}", clean, e))?;

    let mult_part = &clean[num_str.len()..];
    let mult = if mult_part.starts_with('k') || mult_part.starts_with('K') {
        1024
    } else if mult_part.starts_with('m') || mult_part.starts_with('M') {
        1024 * 1024
    } else if mult_part.starts_with('g') || mult_part.starts_with('G') {
        1024 * 1024 * 1024
    } else {
        1
    };

    Ok(num * mult)
}
