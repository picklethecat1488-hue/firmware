use object::{Object, ObjectSection, ObjectSymbol};

/// Byte width of the big-endian 16-bit CRC-16-CCITT prefix on `.program_metadata` payloads.
pub const CHECKSUM_PREFIX_LEN: usize = 2;

/// Partition kind discriminant for the non-volatile `metadata` `sequential_storage::map` partition.
pub const METADATA_PARTITION_KIND: u32 = 2;

/// Parsed partition information extracted from `.program_metadata`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionInfo {
    /// The kind/type of the partition (0 = Fs, 1 = Telemetry, 2 = Metadata).
    pub kind: u32,
    /// The virtual memory flash address of the partition.
    pub address: u32,
    /// The size of the partition in bytes.
    pub size: usize,
}

/// Parsed project information and ELF section geometry from `.program_metadata`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectInfo {
    /// Chip name (e.g. "rp2040", "MCXN947").
    pub chip: String,
    /// Canonical board type identifier (e.g. "rp2040_flash_2mb", "carrier_board").
    pub board_type: String,
    /// Optional compiled Cargo binary target name (e.g. "cat_detector_shell", "cat_detector_app").
    pub bin_name: Option<String>,
    /// Optional compiled Cargo profile identifier ("debug" or "release").
    pub build_profile: Option<String>,
    /// Flash write alignment/size in bytes.
    pub flash_write_size: u32,
    /// Flash erase sector size in bytes.
    pub flash_erase_size: u32,
    /// Stack scan limit in words.
    pub stack_scan_limit: u32,
    /// Flash base address (e.g. `0x1000_0000` or `0x0000_0000`).
    pub flash_start: u32,
    /// List of flash partitions (`fs`, `telemetry`, `metadata`).
    pub partitions: Vec<PartitionInfo>,
    /// Verified 16-bit CRC-16-CCITT checksum of the serialized CBOR metadata.
    pub checksum: u16,
    /// Virtual memory address of `.program_metadata` in the ELF binary.
    pub program_metadata_addr: u64,
    /// Byte length of `.program_metadata` in the ELF binary.
    pub program_metadata_size: usize,
}

impl ProjectInfo {
    /// Decode a raw `.program_metadata` byte slice prefixed by its 16-bit CRC-16-CCITT checksum.
    pub fn decode_bytes(
        data: &[u8],
        program_metadata_addr: u64,
        program_metadata_size: usize,
    ) -> Result<Self, String> {
        if data.len() <= CHECKSUM_PREFIX_LEN {
            return Err("Program metadata buffer is too short for 16-bit checksum".to_string());
        }

        let expected_crc = u16::from_be_bytes([data[0], data[1]]);
        let cbor_slice = &data[CHECKSUM_PREFIX_LEN..];
        let mut decoder = minicbor::Decoder::new(cbor_slice);
        let metadata: platform::types::ProjectMetadata<'_> = decoder
            .decode()
            .map_err(|e| format!("Failed to decode CBOR metadata: {:?}", e))?;
        let cbor_end = decoder.position();
        let actual_crc = platform::cbor::crc16_ccitt(&cbor_slice[..cbor_end]);
        if actual_crc != expected_crc {
            return Err(format!(
                "Program metadata 16-bit checksum mismatch: expected 0x{:04X}, computed 0x{:04X}",
                expected_crc, actual_crc
            ));
        }

        let partitions = metadata
            .partitions
            .iter()
            .map(|p| PartitionInfo {
                kind: p.kind,
                address: p.address,
                size: p.size as usize,
            })
            .collect();

        let effective_size = if program_metadata_size > 0 {
            program_metadata_size
        } else {
            CHECKSUM_PREFIX_LEN + cbor_end
        };

        Ok(Self {
            chip: metadata.chip.to_string(),
            board_type: metadata.board_type.to_string(),
            bin_name: metadata.bin_name.map(str::to_string),
            build_profile: metadata.build_profile.map(str::to_string),
            flash_write_size: metadata.flash_write_size,
            flash_erase_size: metadata.flash_erase_size,
            stack_scan_limit: metadata.stack_scan_limit,
            flash_start: metadata.flash_start,
            partitions,
            checksum: expected_crc,
            program_metadata_addr,
            program_metadata_size: effective_size,
        })
    }

    /// Return the `metadata` (`sequential_storage::map`) partition descriptor resolved from the ELF.
    pub fn metadata_partition(&self) -> Option<&PartitionInfo> {
        self.partitions
            .iter()
            .find(|p| p.kind == METADATA_PARTITION_KIND)
    }
}

/// Autodetects chip, board type, checksum, and partition layout from an ELF file's `.program_metadata` section.
pub fn autodetect_project_info(elf_path: &std::path::Path) -> Result<ProjectInfo, String> {
    let elf_data = std::fs::read(elf_path).map_err(|e| {
        format!(
            "Failed to read ELF file at '{}': {:?}",
            elf_path.display(),
            e
        )
    })?;
    let file = object::File::parse(&*elf_data)
        .map_err(|e| format!("Failed to parse ELF file: {:?}", e))?;

    if let Some(sec) = file.section_by_name(".program_metadata") {
        let sec_bytes = sec
            .data()
            .map_err(|e| format!("Failed to read .program_metadata section: {:?}", e))?;
        if !sec_bytes.is_empty() {
            return ProjectInfo::decode_bytes(sec_bytes, sec.address(), sec_bytes.len());
        }
    }

    let mut symbol = None;
    for sym in file.symbols() {
        let name = sym.name();
        if name == Ok("PROJECT_METADATA") || name == Ok("_PROJECT_METADATA") {
            symbol = Some(sym);
            break;
        }
    }

    let symbol = symbol.ok_or_else(|| {
        "Could not find 'PROJECT_METADATA' symbol in ELF. Make sure the target binary includes the PROJECT_METADATA static.".to_string()
    })?;

    let address = symbol.address();
    let sym_size = symbol.size() as usize;

    let mut section_data = None;
    let mut section_address = 0;
    for sec in file.sections() {
        let start = sec.address();
        let size = sec.size();
        if address >= start && address < start + size {
            section_data = Some(
                sec.data()
                    .map_err(|e| format!("Failed to read section data: {:?}", e))?,
            );
            section_address = start;
            break;
        }
    }

    let section_data = section_data.ok_or_else(|| {
        format!(
            "Could not find ELF section containing symbol address 0x{:08X}",
            address
        )
    })?;

    let offset = (address - section_address) as usize;
    if offset >= section_data.len() {
        return Err("Symbol address is out of section bounds".to_string());
    }

    let remaining = section_data.len() - offset;
    let slice_len = if sym_size > 0 && sym_size <= remaining {
        sym_size
    } else {
        remaining.min(512)
    };
    ProjectInfo::decode_bytes(
        &section_data[offset..offset + slice_len],
        address,
        slice_len,
    )
}

/// Finds the address of a symbol in the ELF file.
pub fn find_symbol_address(
    elf_path: &std::path::Path,
    symbol_name: &str,
) -> Result<Option<u64>, String> {
    let elf_data = std::fs::read(elf_path).map_err(|e| {
        format!(
            "Failed to read ELF file at '{}': {:?}",
            elf_path.display(),
            e
        )
    })?;
    let file = object::File::parse(&*elf_data)
        .map_err(|e| format!("Failed to parse ELF file: {:?}", e))?;

    for sym in file.symbols() {
        if sym.name() == Ok(symbol_name) {
            return Ok(Some(sym.address()));
        }
    }
    Ok(None)
}
