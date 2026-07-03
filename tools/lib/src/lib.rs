//! Common utilities shared across target-attached host CLI tools.

/// Parses the project name to get the target chip name from `.cargo/config.toml`
/// and layout parameters from `memory.x`.
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
