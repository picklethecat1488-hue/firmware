use object::{Object, ObjectSection, ObjectSymbol};

/// Parsed project information from the ELF metadata section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectInfo {
    /// Chip name (e.g. "rp2040")
    pub chip: String,
    /// The virtual memory flash address of the storage partition
    pub partition_address: u32,
    /// The size of the storage partition in bytes
    pub partition_size: usize,
    /// Flash write alignment/size in bytes
    pub flash_write_size: u32,
    /// Flash erase sector size in bytes
    pub flash_erase_size: u32,
    /// Stack scan limit in words
    pub stack_scan_limit: u32,
}

/// Autodetects chip and layout parameters from an ELF file's project metadata section.
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

    // Find the PROJECT_METADATA symbol in the ELF symbol table
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

    // Find the section containing the symbol address
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
    let max_len = if remaining > 512 { 512 } else { remaining };
    let data = &section_data[offset..offset + max_len];

    let metadata: platform::types::ProjectMetadata<'_> =
        minicbor::decode(data).map_err(|e| format!("Failed to decode CBOR metadata: {:?}", e))?;

    Ok(ProjectInfo {
        chip: metadata.chip.to_string(),
        partition_address: metadata.partition_address,
        partition_size: metadata.partition_size as usize,
        flash_write_size: metadata.flash_write_size,
        flash_erase_size: metadata.flash_erase_size,
        stack_scan_limit: metadata.stack_scan_limit,
    })
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
