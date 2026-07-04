//! Common utilities shared across target-attached host CLI tools.

use object::{Object, ObjectSection, ObjectSymbol};

/// Autodetects chip and layout parameters from an ELF file's project metadata section.
pub fn autodetect_project_info(elf_path: &std::path::Path) -> Result<(String, u32, usize), String> {
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
    if offset + 52 > section_data.len() {
        return Err("Symbol address is out of section bounds".to_string());
    }

    let data = &section_data[offset..offset + 52];

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

    Ok((chip, partition_address, partition_size))
}
