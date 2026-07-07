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
    if offset + 60 > section_data.len() {
        return Err("Symbol address is out of section bounds".to_string());
    }

    let data = &section_data[offset..offset + 60];

    let magic = &data[0..8];
    if magic != b"PROJMET\0" {
        return Err("Invalid project metadata magic signature".to_string());
    }

    let version = u32::from_le_bytes(data[8..12].try_into().unwrap());
    if version != 1 {
        return Err(format!("Unsupported project metadata version: {}", version));
    }

    let chip_bytes = &data[12..44];
    let chip_len = chip_bytes.iter().position(|&b| b == 0).unwrap_or(32);
    let chip = std::str::from_utf8(&chip_bytes[..chip_len])
        .map_err(|e| format!("Invalid UTF-8 in chip name metadata: {:?}", e))?
        .to_string();

    let partition_address = u32::from_le_bytes(data[44..48].try_into().unwrap());
    let partition_size = u32::from_le_bytes(data[48..52].try_into().unwrap()) as usize;
    let flash_write_size = u32::from_le_bytes(data[52..56].try_into().unwrap());
    let flash_erase_size = u32::from_le_bytes(data[56..60].try_into().unwrap());

    Ok(ProjectInfo {
        chip,
        partition_address,
        partition_size,
        flash_write_size,
        flash_erase_size,
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
