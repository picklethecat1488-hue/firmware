use rinja::Template;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

/// Config mapping board name keys to individual board target configurations.
#[derive(Deserialize, Debug, Clone)]
pub struct BoardsConfig {
    /// Map of board configurations.
    pub boards: HashMap<String, BoardConfig>,
}

/// Target-specific parameters, memory layouts, and GPIO mappings for a board configuration.
#[derive(Deserialize, Debug, Clone)]
pub struct BoardConfig {
    /// Name of the MCU chip (e.g. `"rp2040"`).
    pub chip: String,
    /// Base address of target's flash memory.
    pub flash_base: u32,
    /// Total size of flash memory in bytes.
    pub flash_size: u32,
    /// Total size of target SRAM memory in bytes.
    pub sram_size: u32,
    /// Stack size allocated for Core 0 execution.
    pub core0_stack_size: usize,
    /// Stack size allocated for Core 1 execution.
    pub core1_stack_size: usize,
    /// Top address limit of SRAM/stack.
    pub stack_top: u32,
    /// Hardware bottom limit (guard address) for Core 0 stack.
    pub core0_stack_bottom: u32,
    /// Core watchdog monitor timeout limit in milliseconds.
    pub core_monitor_timeout_ms: u32,
    /// Core monitor warning threshold percentage.
    pub core_monitor_warn_pct: u32,
    /// Mappings of named pins to their numeric GPIO identifiers.
    pub pins: HashMap<String, u32>,
    /// Mappings of named communication buses to bus controller settings.
    pub buses: HashMap<String, BoardBusConfig>,
    /// Mappings of custom hardware resources to typed key-value settings.
    pub hardware_resources: HashMap<String, TypedResource>,
    /// Mappings of named memory layout partitions.
    pub partitions: HashMap<String, BoardPartitionConfig>,
}

/// A custom board configuration resource containing a value and an explicit Rust type.
#[derive(Deserialize, Debug, Clone)]
pub struct TypedResource {
    /// TOML value associated with the resource.
    pub value: toml::Value,
    /// Desired Rust type of the resource (e.g. `"u8"`, `"u16"`, `"usize"`).
    pub r#type: String,
}

/// Settings configuration for a target hardware serial bus controller.
#[derive(Deserialize, Debug, Clone)]
pub struct BoardBusConfig {
    /// SDA data pin identifier.
    pub sda: String,
    /// SCL clock pin identifier.
    pub scl: String,
    /// Operating frequency of the bus in Hz.
    pub frequency: u32,
}

/// Memory partition boundary coordinates relative to storage space boundaries.
#[derive(Deserialize, Debug, Clone)]
pub struct BoardPartitionConfig {
    /// Starting offset relative to flash base.
    pub start: u32,
    /// Ending offset relative to flash base.
    pub end: u32,
}

/// A key-value specification for a generated constant mapping.
pub struct BoardConstant {
    /// Name identifier of the generated constant.
    pub name: String,
    /// Target Rust type of the constant.
    pub const_type: String,
    /// Code representation string of the constant value.
    pub val_str: String,
    /// Documentation comment for the constant.
    pub doc: String,
}

#[derive(Template)]
#[template(path = "generated_board.rs.jinja", escape = "none")]
pub struct GeneratedBoardTemplate {
    pub name: String,
    pub chip: String,
    pub flash_base: u32,
    pub constants: Vec<BoardConstant>,
}

impl GeneratedBoardTemplate {
    pub fn flash_base_hex(&self) -> String {
        format!("0x{:08X}", self.flash_base)
    }
}

pub fn parse_memory_map(root: &Path) -> (u32, u32) {
    let path = root.join("platform/src/rp2040/layouts/xip/memory.x");
    if !path.exists() {
        return (0x10000000, 0x20042000);
    }
    let content = std::fs::read_to_string(path).expect("Failed to read memory.x");

    let mut flash_start = 0x10000000;
    let mut ram_start = 0x20000000;
    let mut ram_length = 264 * 1024;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("/*")
            || line.is_empty()
            || line.starts_with("MEMORY")
            || line.starts_with("{")
            || line.starts_with("}")
        {
            continue;
        }
        if let Some(colon_idx) = line.find(':') {
            let name = line[..colon_idx].trim();
            let rest = &line[colon_idx + 1..];

            let mut origin = None;
            let mut length = None;

            for part in rest.split(',') {
                let part = part.trim();
                if let Some(eq_idx) = part.find('=') {
                    let key = part[..eq_idx].trim();
                    let val = part[eq_idx + 1..].trim();
                    if key.eq_ignore_ascii_case("ORIGIN") {
                        origin = parse_numeric_value(val);
                    } else if key.eq_ignore_ascii_case("LENGTH") {
                        length = parse_numeric_value(val);
                    }
                }
            }

            if name.eq_ignore_ascii_case("BOOT2") || name.eq_ignore_ascii_case("FLASH") {
                if let Some(orig) = origin {
                    if name.eq_ignore_ascii_case("BOOT2") || flash_start == 0x10000000 {
                        flash_start = orig;
                    }
                }
            } else if name.eq_ignore_ascii_case("RAM") {
                if let Some(orig) = origin {
                    ram_start = orig;
                }
                if let Some(len) = length {
                    ram_length = len;
                }
            }
        }
    }

    (flash_start, ram_start + ram_length)
}

fn parse_numeric_value(val: &str) -> Option<u32> {
    let val = val.trim();
    if val.contains('-') {
        let parts: Vec<&str> = val.split('-').collect();
        if parts.len() == 2 {
            let left = parse_single_numeric(parts[0].trim())?;
            let right = parse_single_numeric(parts[1].trim())?;
            return Some(left - right);
        }
    }
    parse_single_numeric(val)
}

fn parse_single_numeric(val: &str) -> Option<u32> {
    let mut s = val.to_string();
    let mut multiplier = 1;
    if s.ends_with('K') || s.ends_with('k') {
        multiplier = 1024;
        s.pop();
    } else if s.ends_with('M') || s.ends_with('m') {
        multiplier = 1024 * 1024;
        s.pop();
    }
    let s = s.trim();
    let parsed = if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(&s[2..], 16).ok()?
    } else {
        s.parse::<u32>().ok()?
    };
    Some(parsed * multiplier)
}

pub fn generate_board_definitions(toml_content: &str, board_name: &str) -> String {
    let config: BoardsConfig =
        toml::from_str(toml_content).expect("Failed to parse boards TOML content");

    let board_config = config
        .boards
        .get(board_name)
        .unwrap_or_else(|| panic!("Board '{}' not found in TOML", board_name));

    let mut constants = Vec::new();

    // 1. Validate GPIO Pin Uniqueness & valid range for RP2040 (0..=29)
    let mut seen_pins = HashMap::new();
    for (k, v) in &board_config.pins {
        if let Some(other_pin) = seen_pins.insert(*v, k) {
            panic!(
                "Validation Error: GPIO Pin {} is assigned to multiple resources: '{}' and '{}' in board '{}'",
                v, k, other_pin, board_name
            );
        }
        if board_config.chip == "rp2040" && *v > 29 {
            panic!(
                "Validation Error: Pin '{}' has invalid GPIO index {} (RP2040 only supports GPIO 0 to 29) in board '{}'",
                k, v, board_name
            );
        }
    }

    // 2. Validate I2C Address ranges based on their declared type (7-bit u8 vs 16-bit u16 address spaces)
    for (k, v) in &board_config.hardware_resources {
        if k.contains("I2C_ADDR") {
            if let toml::Value::Integer(addr) = &v.value {
                if v.r#type == "u8" && (*addr < 0x08 || *addr > 0x7F) {
                    panic!(
                        "Validation Error: Resource '{}' has invalid 7-bit I2C address 0x{:02X} (must be between 0x08 and 0x7F) in board '{}'",
                        k, addr, board_name
                    );
                } else if v.r#type == "u16" && (*addr < 0 || *addr > 0xFFFF) {
                    panic!(
                        "Validation Error: Resource '{}' has invalid 16-bit I2C address 0x{:04X} (must be between 0x0000 and 0xFFFF) in board '{}'",
                        k, addr, board_name
                    );
                }
            }
        }
    }

    // 3. Validate Core Monitor threshold percentage (<= 100)
    if board_config.core_monitor_warn_pct > 100 {
        panic!(
            "Validation Error: core_monitor_warn_pct must be <= 100, found {} in board '{}'",
            board_config.core_monitor_warn_pct, board_name
        );
    }

    // 4. Validate Partition boundaries against Flash size
    for (k, v) in &board_config.partitions {
        if v.end > board_config.flash_size {
            panic!(
                "Validation Error: Partition '{}' end offset (0x{:X}) exceeds flash size (0x{:X}) in board '{}'",
                k, v.end, board_config.flash_size, board_name
            );
        }
        if v.start >= v.end {
            panic!(
                "Validation Error: Partition '{}' has invalid bounds [0x{:X}..0x{:X}] in board '{}'",
                k, v.start, v.end, board_name
            );
        }
    }

    // 5. Validate SRAM stack boundaries
    if board_config.stack_top > 0x20000000 + board_config.sram_size {
        panic!(
            "Validation Error: stack_top (0x{:X}) exceeds SRAM boundaries (up to 0x{:X}) in board '{}'",
            board_config.stack_top, 0x20000000 + board_config.sram_size, board_name
        );
    }
    if (board_config.stack_top - board_config.core0_stack_size as u32) < 0x20000000 {
        panic!(
            "Validation Error: Derived stack bottom (0x{:X}) is below SRAM start address (0x20000000) in board '{}'",
            board_config.stack_top - board_config.core0_stack_size as u32, board_name
        );
    }

    // Determine dual core feature flag from environment or default to chip capability
    let env_dual_core = std::env::var("CARGO_FEATURE_DUAL_CORE").is_ok()
        || std::env::var("CARGO_FEATURE_CORE1").is_ok()
        || std::env::var("CARGO_CFG_TARGET_FEATURE")
            .map(|v| v.contains("dual-core") || v.contains("multicore"))
            .unwrap_or(false)
        || board_config.chip == "rp2040";

    constants.push(BoardConstant {
        name: "DUAL_CORE_ENABLED".to_string(),
        const_type: "bool".to_string(),
        val_str: env_dual_core.to_string(),
        doc: "Whether dual core support is active for the current target/board configuration."
            .to_string(),
    });

    // Pins
    for (k, v) in &board_config.pins {
        constants.push(BoardConstant {
            name: k.clone(),
            const_type: "u32".to_string(),
            val_str: v.to_string(),
            doc: format!("GPIO Pin for {}", k),
        });
    }

    // Hardware Resources
    for (k, v) in &board_config.hardware_resources {
        let val_str = match &v.value {
            toml::Value::Integer(i) => {
                if v.r#type == "u8" {
                    format!("0x{:02X}", i)
                } else {
                    i.to_string()
                }
            }
            toml::Value::Float(f) => f.to_string(),
            toml::Value::Boolean(b) => b.to_string(),
            toml::Value::String(s) => format!("\"{}\"", s),
            _ => panic!("Unsupported type for hardware resource: {:?}", v.value),
        };
        constants.push(BoardConstant {
            name: k.clone(),
            const_type: v.r#type.clone(),
            val_str,
            doc: format!("Hardware resource constant for {}", k),
        });
    }

    // Partitions
    for (k, v) in &board_config.partitions {
        let prefix = k.to_uppercase();
        constants.push(BoardConstant {
            name: format!("{}_PARTITION_START", prefix),
            const_type: "u32".to_string(),
            val_str: format!("0x{:08X}", v.start),
            doc: format!("Start address of {} partition", k),
        });
        constants.push(BoardConstant {
            name: format!("{}_PARTITION_END", prefix),
            const_type: "u32".to_string(),
            val_str: format!("0x{:08X}", v.end),
            doc: format!("End address of {} partition", k),
        });
    }

    // Stack top and core sizes
    constants.push(BoardConstant {
        name: "CORE0_STACK_TOP".to_string(),
        const_type: "u32".to_string(),
        val_str: format!("0x{:08X}", board_config.stack_top),
        doc: "Top address of the stack/SRAM for Core 0.".to_string(),
    });
    constants.push(BoardConstant {
        name: "CORE0_STACK_SIZE".to_string(),
        const_type: "usize".to_string(),
        val_str: board_config.core0_stack_size.to_string(),
        doc: "Core 0 stack size in bytes.".to_string(),
    });
    constants.push(BoardConstant {
        name: "CORE1_STACK_SIZE".to_string(),
        const_type: "usize".to_string(),
        val_str: board_config.core1_stack_size.to_string(),
        doc: "Core 1 stack size in bytes.".to_string(),
    });

    // Core 1 enablement constant
    let core1_enabled = board_config.core1_stack_size > 0;
    constants.push(BoardConstant {
        name: "CORE1_ENABLED".to_string(),
        const_type: "bool".to_string(),
        val_str: core1_enabled.to_string(),
        doc: "Whether Core 1 (multicore) is enabled.".to_string(),
    });

    // Default target-independent flash geometry constants
    constants.push(BoardConstant {
        name: "FLASH_SIZE".to_string(),
        const_type: "usize".to_string(),
        val_str: board_config.flash_size.to_string(),
        doc: "Total size of target flash memory in bytes.".to_string(),
    });
    constants.push(BoardConstant {
        name: "FLASH_START".to_string(),
        const_type: "u32".to_string(),
        val_str: format!("0x{:08X}", board_config.flash_base),
        doc: "Starting address of target flash memory.".to_string(),
    });
    constants.push(BoardConstant {
        name: "FLASH_END".to_string(),
        const_type: "u32".to_string(),
        val_str: format!(
            "0x{:08X}",
            board_config.flash_base + board_config.flash_size
        ),
        doc: "Ending address of target flash memory.".to_string(),
    });

    let (write_size, erase_size) = match board_config.chip.as_str() {
        "rp2040" => (256, 4096),
        _ => (256, 4096),
    };
    constants.push(BoardConstant {
        name: "FLASH_WRITE_SIZE".to_string(),
        const_type: "usize".to_string(),
        val_str: write_size.to_string(),
        doc: "Minimum flash write block size in bytes.".to_string(),
    });
    constants.push(BoardConstant {
        name: "FLASH_ERASE_SIZE".to_string(),
        const_type: "usize".to_string(),
        val_str: erase_size.to_string(),
        doc: "Minimum flash erase sector size in bytes.".to_string(),
    });

    // Default FS_BUF_SIZE constant if not defined in hardware_resources
    let fs_buf_size = board_config
        .hardware_resources
        .get("FS_BUF_SIZE")
        .and_then(|r| match &r.value {
            toml::Value::Integer(i) => Some(*i as usize),
            _ => None,
        })
        .unwrap_or(8192);
    constants.push(BoardConstant {
        name: "FS_BUF_SIZE".to_string(),
        const_type: "usize".to_string(),
        val_str: fs_buf_size.to_string(),
        doc: "Size of the static working filesystem buffer in bytes.".to_string(),
    });

    // Board parameters
    constants.push(BoardConstant {
        name: "CORE_MONITOR_TIMEOUT_MS".to_string(),
        const_type: "u32".to_string(),
        val_str: board_config.core_monitor_timeout_ms.to_string(),
        doc: "Default core monitor timeout in milliseconds.".to_string(),
    });
    constants.push(BoardConstant {
        name: "CORE_MONITOR_WARN_PCT".to_string(),
        const_type: "u32".to_string(),
        val_str: board_config.core_monitor_warn_pct.to_string(),
        doc: "Default core monitor warning threshold percentage.".to_string(),
    });

    constants.sort_by(|a, b| a.name.cmp(&b.name));

    let template = GeneratedBoardTemplate {
        name: board_name.to_string(),
        chip: board_config.chip.clone(),
        flash_base: board_config.flash_base,
        constants,
    };

    template.render().expect("Failed to render board template")
}

/// Rinja template for rendering a compilable Board Support Package (BSP) skeleton.
#[derive(Template)]
#[template(path = "bsp.rs.jinja", escape = "none")]
pub struct BspSkeletonTemplate {
    /// The PascalCase name of the board.
    pub name_pascal: String,
    /// The target microcontroller chip type (e.g. `"rp2040"`).
    pub chip: String,
    /// Whether Core 1 (multicore) is enabled.
    pub core1_enabled: bool,
    /// I2C SDA pin name/identifier.
    pub i2c_sda_pin: String,
    /// I2C SCL pin name/identifier.
    pub i2c_scl_pin: String,
    /// I2C speed/frequency in Hz.
    pub i2c_frequency: u32,
    /// Filesystem buffer size in bytes.
    pub fs_buf_size: usize,
}

/// Renders a compilable Board Support Package (BSP) skeleton.
pub fn render_board_skeleton(name_pascal: &str) -> String {
    let board_toml_path = crate::find_board_toml();
    let content = std::fs::read_to_string(&board_toml_path).expect("Failed to read board.toml");
    let boards_config: BoardsConfig = toml::from_str(&content).expect("Failed to parse board.toml");

    let matched_board = boards_config
        .boards
        .iter()
        .find(|(k, _)| {
            k.eq_ignore_ascii_case(name_pascal)
                || name_pascal.to_lowercase().contains(&k.to_lowercase())
        })
        .map(|(_, v)| v)
        .or_else(|| boards_config.boards.values().next())
        .expect("No boards found in board.toml");

    let core1_enabled = matched_board.core1_stack_size > 0;

    let mut i2c_sda_pin = "12".to_string();
    let mut i2c_scl_pin = "13".to_string();
    let mut i2c_frequency = 400000;
    if let Some(i2c_bus) = matched_board.buses.get("i2c") {
        i2c_sda_pin = i2c_bus.sda.clone();
        i2c_scl_pin = i2c_bus.scl.clone();
        i2c_frequency = i2c_bus.frequency;
    }

    let fs_buf_size = matched_board
        .hardware_resources
        .get("FS_BUF_SIZE")
        .and_then(|r| match &r.value {
            toml::Value::Integer(i) => Some(*i as usize),
            _ => None,
        })
        .unwrap_or(8192);

    let template = BspSkeletonTemplate {
        name_pascal: name_pascal.to_string(),
        chip: matched_board.chip.clone(),
        core1_enabled,
        i2c_sda_pin,
        i2c_scl_pin,
        i2c_frequency,
        fs_buf_size,
    };
    template
        .render()
        .expect("Failed to render board skeleton template")
}
