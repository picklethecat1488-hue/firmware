use object::{Object, ObjectSection, ObjectSymbol};
use std::path::Path;

/// Byte width of the big-endian 16-bit CRC-16-CCITT prefix on `.program_metadata` payloads.
pub const CHECKSUM_PREFIX_LEN: usize = 2;

/// Partition kind discriminant for the non-volatile `metadata` `sequential_storage::map` partition.
pub const METADATA_PARTITION_KIND: u32 = 2;

/// Maximum symbol read window when resolving `PROJECT_METADATA_ELF` from host test binaries.
const MAX_ELF_TABLE_SYMBOL_LEN: usize = 512;

/// Parsed partition information deferred-loaded from `.program_metadata_elf`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionInfo {
    /// The kind/type of the partition (`0 = Fs`, `1 = Telemetry`, `2 = Metadata`).
    pub kind: u32,
    /// The virtual memory flash address of the partition.
    pub address: u32,
    /// The size of the partition in bytes.
    pub size: usize,
}

/// Decoded compact `<= 12`-byte on-target `.program_metadata` header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactTargetMetadata {
    /// Verified 16-bit CRC-16-CCITT checksum of the serialized compact CBOR header.
    pub checksum: u16,
    /// Decoded compact fields (`elf_meta_crc16`, `board_crc16`, `release_build`, `trace_mode`).
    pub header: platform::types::ProgramMetadataCompact,
    /// Exact serialized byte length of the compact header (`CHECKSUM_PREFIX_LEN + cbor_len`).
    pub byte_len: usize,
}

impl CompactTargetMetadata {
    /// Decode and CRC-verify a raw compact `.program_metadata` byte slice.
    pub fn decode_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() <= CHECKSUM_PREFIX_LEN {
            return Err("Program metadata buffer is too short for 16-bit checksum".to_string());
        }
        let expected_crc = u16::from_be_bytes([data[0], data[1]]);
        let cbor_slice = &data[CHECKSUM_PREFIX_LEN..];
        let mut decoder = minicbor::Decoder::new(cbor_slice);
        let header: platform::types::ProgramMetadataCompact = decoder
            .decode()
            .map_err(|e| format!("Failed to decode compact CBOR metadata: {:?}", e))?;
        let cbor_end = decoder.position();
        let actual_crc = platform::cbor::crc16_ccitt(&cbor_slice[..cbor_end]);
        if actual_crc != expected_crc {
            return Err(format!(
                "Program metadata 16-bit checksum mismatch: expected 0x{:04X}, computed 0x{:04X}",
                expected_crc, actual_crc
            ));
        }
        Ok(Self {
            checksum: expected_crc,
            header,
            byte_len: CHECKSUM_PREFIX_LEN + cbor_end,
        })
    }
}

/// Deferred string and partition metadata loaded from `.program_metadata_elf` in an `.elf` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredElfTable {
    /// Verified 16-bit CRC-16-CCITT checksum of the `.program_metadata_elf` CBOR payload.
    pub elf_meta_crc16: u16,
    /// 16-bit CRC-16-CCITT digest of `board_type.as_bytes()`.
    pub board_crc16: u16,
    /// Chip name (e.g. `"rp2040"`, `"MCXN947"`).
    pub chip: String,
    /// Canonical board type identifier (e.g. `"rp2040_flash_2mb"`, `"carrier_board"`).
    pub board_type: String,
    /// Compiled Cargo binary target name (e.g. `"cat_detector_shell"`, `"cat_detector_app"`).
    pub bin_name: Option<String>,
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
}

impl DeferredElfTable {
    /// Decode and CRC-verify a `.program_metadata_elf` byte slice from an `.elf` file.
    pub fn decode_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() <= CHECKSUM_PREFIX_LEN {
            return Err("ELF metadata table buffer is too short for 16-bit checksum".to_string());
        }
        let expected_crc = u16::from_be_bytes([data[0], data[1]]);
        let cbor_slice = &data[CHECKSUM_PREFIX_LEN..];
        let mut decoder = minicbor::Decoder::new(cbor_slice);
        let metadata: platform::types::ProjectMetadata<'_> = decoder
            .decode()
            .map_err(|e| format!("Failed to decode deferred ELF CBOR metadata: {:?}", e))?;
        let cbor_end = decoder.position();
        let actual_crc = platform::cbor::crc16_ccitt(&cbor_slice[..cbor_end]);
        if actual_crc != expected_crc {
            return Err(format!(
                "ELF metadata table 16-bit checksum mismatch: expected 0x{:04X}, computed 0x{:04X}",
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
        let board_crc16 = platform::cbor::crc16_ccitt(metadata.board_type.as_bytes());
        Ok(Self {
            elf_meta_crc16: expected_crc,
            board_crc16,
            chip: metadata.chip.to_string(),
            board_type: metadata.board_type.to_string(),
            bin_name: metadata.bin_name.map(str::to_string),
            flash_write_size: metadata.flash_write_size,
            flash_erase_size: metadata.flash_erase_size,
            stack_scan_limit: metadata.stack_scan_limit,
            flash_start: metadata.flash_start,
            partitions,
        })
    }
}

/// Parsed project information combining the compact `.program_metadata` header with deferred `.elf` strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectInfo {
    /// Chip name (e.g. `"rp2040"`, `"MCXN947"`).
    pub chip: String,
    /// Canonical board type identifier (e.g. `"rp2040_flash_2mb"`, `"carrier_board"`).
    pub board_type: String,
    /// Optional compiled Cargo binary target name (e.g. `"cat_detector_shell"`, `"cat_detector_app"`).
    pub bin_name: Option<String>,
    /// Optional compiled Cargo release flag (`true` = release, `false` = debug).
    pub release_build: Option<bool>,
    /// Optional compile-time tracing mode identifier (`"none"`, `"sensor"`, `"perf"`, `"audio"`).
    pub trace_mode: Option<String>,
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
    /// Verified 16-bit CRC-16-CCITT checksum of the compact `.program_metadata` header.
    pub checksum: u16,
    /// 16-bit CRC-16-CCITT digest of the deferred `.program_metadata_elf` table.
    pub elf_meta_crc16: u16,
    /// 16-bit CRC-16-CCITT digest of `board_type`.
    pub board_crc16: u16,
    /// Virtual memory address of `.program_metadata` in the ELF binary.
    pub program_metadata_addr: u64,
    /// Byte length of the compact `.program_metadata` header in target flash (`<= 12` bytes).
    pub program_metadata_size: usize,
}

impl ProjectInfo {
    /// Combine a verified compact `.program_metadata` header with its deferred `.program_metadata_elf` table.
    pub fn from_compact_and_elf(
        compact: &CompactTargetMetadata,
        elf_table: &DeferredElfTable,
        program_metadata_addr: u64,
    ) -> Result<Self, String> {
        if compact.header.elf_meta_crc16 != elf_table.elf_meta_crc16 {
            return Err(format!(
                "Compact .program_metadata elf_meta_crc16 (0x{:04X}) does not match .program_metadata_elf (0x{:04X})",
                compact.header.elf_meta_crc16, elf_table.elf_meta_crc16
            ));
        }
        Ok(Self {
            chip: elf_table.chip.clone(),
            board_type: elf_table.board_type.clone(),
            bin_name: elf_table.bin_name.clone(),
            release_build: Some(compact.header.release_build),
            trace_mode: Some(compact.header.trace_mode.as_str().to_string()),
            flash_write_size: elf_table.flash_write_size,
            flash_erase_size: elf_table.flash_erase_size,
            stack_scan_limit: elf_table.stack_scan_limit,
            flash_start: elf_table.flash_start,
            partitions: elf_table.partitions.clone(),
            checksum: compact.checksum,
            elf_meta_crc16: elf_table.elf_meta_crc16,
            board_crc16: elf_table.board_crc16,
            program_metadata_addr,
            program_metadata_size: compact.byte_len,
        })
    }

    /// Decode a raw on-device compact `.program_metadata` slice and deferred-load strings from `self`
    /// (or a matching entry in `sibling_tables`).
    pub fn decode_target_bytes(
        &self,
        data: &[u8],
        sibling_tables: &[DeferredElfTable],
    ) -> Result<Self, String> {
        let compact = CompactTargetMetadata::decode_bytes(data)?;
        if compact.header.elf_meta_crc16 == self.elf_meta_crc16 {
            let mut resolved = self.clone();
            resolved.checksum = compact.checksum;
            resolved.release_build = Some(compact.header.release_build);
            resolved.trace_mode = Some(compact.header.trace_mode.as_str().to_string());
            resolved.program_metadata_size = compact.byte_len;
            return Ok(resolved);
        }

        if let Some(sib) = sibling_tables.iter().find(|t| {
            t.chip.eq_ignore_ascii_case(&self.chip)
                && t.elf_meta_crc16 == compact.header.elf_meta_crc16
        }) {
            return Self::from_compact_and_elf(&compact, sib, self.program_metadata_addr);
        }

        if compact.header.board_crc16 == self.board_crc16 {
            let mut board_resolved = self.clone();
            board_resolved.bin_name = None;
            board_resolved.checksum = compact.checksum;
            board_resolved.elf_meta_crc16 = compact.header.elf_meta_crc16;
            board_resolved.release_build = Some(compact.header.release_build);
            board_resolved.trace_mode = Some(compact.header.trace_mode.as_str().to_string());
            board_resolved.program_metadata_size = compact.byte_len;
            return Ok(board_resolved);
        }

        Err(format!(
            "Target .program_metadata ELF table digest mismatch (device elf_crc=0x{:04X}, board_crc=0x{:04X}; ELF elf_crc=0x{:04X}, board_crc=0x{:04X})",
            compact.header.elf_meta_crc16,
            compact.header.board_crc16,
            self.elf_meta_crc16,
            self.board_crc16
        ))
    }

    /// Return the `metadata` (`sequential_storage::map`) partition descriptor resolved from the ELF.
    pub fn metadata_partition(&self) -> Option<&PartitionInfo> {
        self.partitions
            .iter()
            .find(|p| p.kind == METADATA_PARTITION_KIND)
    }
}

/// Extract raw bytes, virtual address, and declared size for a named section or fallback symbol in `file`.
fn read_section_or_symbol<'a>(
    file: &'a object::File<'a>,
    section_name: &str,
    symbol_names: &[&str],
    max_fallback_len: usize,
) -> Result<(&'a [u8], u64, usize), String> {
    if let Some(sec) = file.section_by_name(section_name) {
        let sec_bytes = sec
            .data()
            .map_err(|e| format!("Failed to read {} section: {:?}", section_name, e))?;
        if !sec_bytes.is_empty() {
            return Ok((sec_bytes, sec.address(), sec_bytes.len()));
        }
    }

    let mut matched_sym = None;
    for sym in file.symbols() {
        if let Ok(name) = sym.name() {
            if symbol_names.contains(&name) {
                matched_sym = Some(sym);
                break;
            }
        }
    }

    let symbol = matched_sym.ok_or_else(|| {
        format!(
            "Could not find section '{}' or symbol {:?} in ELF.",
            section_name, symbol_names
        )
    })?;

    if let Some(sec_idx) = symbol.section_index() {
        if let Ok(sec) = file.section_by_index(sec_idx) {
            let sec_data = sec
                .data()
                .map_err(|e| format!("Failed to read symbol section data: {:?}", e))?;
            let sym_addr = symbol.address();
            let sec_addr = sec.address();
            let offset = sym_addr.saturating_sub(sec_addr) as usize;
            if offset < sec_data.len() {
                let sym_size = symbol.size() as usize;
                let remaining = sec_data.len() - offset;
                let slice_len = if sym_size > 0 && sym_size <= remaining {
                    sym_size
                } else {
                    remaining.min(max_fallback_len)
                };
                return Ok((&sec_data[offset..offset + slice_len], sym_addr, slice_len));
            }
        }
    }

    let address = symbol.address();
    let sym_size = symbol.size() as usize;
    for sec in file.sections() {
        let start = sec.address();
        let size = sec.size();
        if address >= start && address < start + size {
            let section_data = sec
                .data()
                .map_err(|e| format!("Failed to read section data: {:?}", e))?;
            let offset = (address - start) as usize;
            if offset >= section_data.len() {
                return Err("Symbol address is out of section bounds".to_string());
            }
            let remaining = section_data.len() - offset;
            let slice_len = if sym_size > 0 && sym_size <= remaining {
                sym_size
            } else {
                remaining.min(max_fallback_len)
            };
            return Ok((
                &section_data[offset..offset + slice_len],
                address,
                slice_len,
            ));
        }
    }

    Err(format!(
        "Could not find ELF section containing symbol {:?} at 0x{:08X}",
        symbol_names, address
    ))
}

/// Extract the deferred `.program_metadata_elf` table from an `.elf` file.
pub fn read_deferred_elf_table(elf_path: &Path) -> Result<DeferredElfTable, String> {
    let elf_data = std::fs::read(elf_path).map_err(|e| {
        format!(
            "Failed to read ELF file at '{}': {:?}",
            elf_path.display(),
            e
        )
    })?;
    let file = object::File::parse(&*elf_data)
        .map_err(|e| format!("Failed to parse ELF file: {:?}", e))?;
    let (elf_bytes, _, _) = read_section_or_symbol(
        &file,
        ".program_metadata_elf",
        &["PROJECT_METADATA_ELF", "_PROJECT_METADATA_ELF"],
        MAX_ELF_TABLE_SYMBOL_LEN,
    )?;
    DeferredElfTable::decode_bytes(elf_bytes)
}

/// Discover deferred `.program_metadata_elf` tables in sibling compiled ELFs near `elf_path`.
pub fn discover_sibling_elf_tables(elf_path: &Path) -> Vec<DeferredElfTable> {
    let mut tables = Vec::new();
    let mut candidate_dirs = Vec::new();
    if let Some(parent) = elf_path.parent() {
        candidate_dirs.push(parent.to_path_buf());
        if let Some(target_triple_dir) = parent.parent() {
            for profile in ["debug", "release"] {
                let sibling_profile = target_triple_dir.join(profile);
                if sibling_profile != parent && sibling_profile.is_dir() {
                    candidate_dirs.push(sibling_profile);
                }
            }
        }
    }

    for dir in candidate_dirs {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path == elf_path || !path.is_file() {
                continue;
            }
            if path.extension().is_some() {
                continue;
            }
            if let Ok(tbl) = read_deferred_elf_table(&path) {
                if !tables.iter().any(|existing: &DeferredElfTable| {
                    existing.elf_meta_crc16 == tbl.elf_meta_crc16
                }) {
                    tables.push(tbl);
                }
            }
        }
    }
    tables
}

/// Autodetects compact `.program_metadata` (`<= 12` bytes) and deferred-loads strings & partitions
/// from `.program_metadata_elf` in `elf_path`.
pub fn autodetect_project_info(elf_path: &Path) -> Result<ProjectInfo, String> {
    let elf_data = std::fs::read(elf_path).map_err(|e| {
        format!(
            "Failed to read ELF file at '{}': {:?}",
            elf_path.display(),
            e
        )
    })?;
    let file = object::File::parse(&*elf_data)
        .map_err(|e| format!("Failed to parse ELF file: {:?}", e))?;

    let (compact_bytes, addr, _) = read_section_or_symbol(
        &file,
        ".program_metadata",
        &["PROJECT_METADATA", "_PROJECT_METADATA"],
        platform::types::COMPACT_PROGRAM_METADATA_CAP,
    )?;
    let (elf_bytes, _, _) = read_section_or_symbol(
        &file,
        ".program_metadata_elf",
        &["PROJECT_METADATA_ELF", "_PROJECT_METADATA_ELF"],
        MAX_ELF_TABLE_SYMBOL_LEN,
    )?;

    let compact = CompactTargetMetadata::decode_bytes(compact_bytes)?;
    let elf_table = DeferredElfTable::decode_bytes(elf_bytes)?;
    ProjectInfo::from_compact_and_elf(&compact, &elf_table, addr)
}

/// Finds the address of a symbol in the ELF file.
pub fn find_symbol_address(elf_path: &Path, symbol_name: &str) -> Result<Option<u64>, String> {
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
