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
    /// Optional motor configuration settings.
    #[serde(default)]
    pub motor: Option<BoardMotorConfig>,
}

/// Target-specific motor timing, current, and commutation parameters.
#[derive(Deserialize, Debug, Clone)]
pub struct BoardMotorConfig {
    /// Startup blanking period in ticks.
    #[serde(default = "default_startup_blanking_ticks")]
    pub startup_blanking_ticks: u32,
    /// Default maximum current limit in milliamperes.
    #[serde(default = "default_max_current_ma")]
    pub default_max_current_ma: i32,
    /// Initial commutation step interval in ticks during startup ramp.
    #[serde(default = "default_startup_commutation_step_ticks")]
    pub startup_commutation_step_ticks: u32,
    /// Base speed percentage divisor for calculating target commutation step interval ticks.
    #[serde(default = "default_commutation_speed_divisor")]
    pub commutation_speed_divisor: u32,
    /// Minimum commutation step interval in ticks.
    #[serde(default = "default_min_commutation_step_ticks")]
    pub min_commutation_step_ticks: u32,
    /// Maximum commutation step interval in ticks.
    #[serde(default = "default_max_commutation_step_ticks")]
    pub max_commutation_step_ticks: u32,
}

fn default_startup_blanking_ticks() -> u32 {
    50
}
fn default_max_current_ma() -> i32 {
    2500
}
fn default_startup_commutation_step_ticks() -> u32 {
    10
}
fn default_commutation_speed_divisor() -> u32 {
    100
}
fn default_min_commutation_step_ticks() -> u32 {
    1
}
fn default_max_commutation_step_ticks() -> u32 {
    100
}

impl Default for BoardMotorConfig {
    fn default() -> Self {
        Self {
            startup_blanking_ticks: default_startup_blanking_ticks(),
            default_max_current_ma: default_max_current_ma(),
            startup_commutation_step_ticks: default_startup_commutation_step_ticks(),
            commutation_speed_divisor: default_commutation_speed_divisor(),
            min_commutation_step_ticks: default_min_commutation_step_ticks(),
            max_commutation_step_ticks: default_max_commutation_step_ticks(),
        }
    }
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
#[derive(Deserialize, Debug, Clone, Default)]
pub struct BoardBusConfig {
    /// SDA data pin identifier.
    #[serde(default)]
    pub sda: Option<String>,
    /// SCL clock pin identifier.
    #[serde(default)]
    pub scl: Option<String>,
    /// Operating frequency of the bus in Hz.
    #[serde(default)]
    pub frequency: Option<u32>,
    /// TX pin identifier.
    #[serde(default)]
    pub tx: Option<String>,
    /// RX pin identifier.
    #[serde(default)]
    pub rx: Option<String>,
    /// Generic or PIO pin identifier.
    #[serde(default)]
    pub pin: Option<String>,
    /// Multiple pins assigned to this bus.
    #[serde(default)]
    pub pins: Option<Vec<String>>,
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
    pub bus_assigned_pins: Vec<u32>,
}

impl GeneratedBoardTemplate {
    pub fn flash_base_hex(&self) -> String {
        format!("0x{:08X}", self.flash_base)
    }

    pub fn bus_assigned_match(&self) -> String {
        if self.bus_assigned_pins.is_empty() {
            "u32::MAX".to_string()
        } else {
            self.bus_assigned_pins
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        }
    }
}

/// Rinja template for generating GPIO bus-assignment and alternate-function mappings.
#[derive(Template)]
#[template(path = "generated_gpio.rs.jinja", escape = "none")]
pub struct GeneratedGpioTemplate {
    /// List of numeric GPIO pin IDs assigned to buses.
    pub bus_assigned_pins: Vec<u32>,
    /// Total number of GPIO pins.
    pub num_gpio_pins: u32,
}

impl GeneratedGpioTemplate {
    /// Returns the pipe-delimited pattern string for match statements.
    pub fn bus_assigned_match(&self) -> String {
        if self.bus_assigned_pins.is_empty() {
            "u32::MAX".to_string()
        } else {
            self.bus_assigned_pins
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        }
    }
}

/// Resolves all unique numeric GPIO pin IDs assigned to communication buses.
pub fn resolve_bus_assigned_pins(board_config: &BoardConfig) -> Vec<u32> {
    let mut assigned_pins = std::collections::BTreeSet::new();

    let resolve_pin = |entry: &str| -> Option<u32> {
        board_config
            .pins
            .get(entry)
            .copied()
            .or_else(|| entry.parse::<u32>().ok())
    };

    for bus in board_config.buses.values() {
        if let Some(ref sda) = bus.sda {
            if let Some(pin) = resolve_pin(sda) {
                assigned_pins.insert(pin);
            }
        }
        if let Some(ref scl) = bus.scl {
            if let Some(pin) = resolve_pin(scl) {
                assigned_pins.insert(pin);
            }
        }
        if let Some(ref tx) = bus.tx {
            if let Some(pin) = resolve_pin(tx) {
                assigned_pins.insert(pin);
            }
        }
        if let Some(ref rx) = bus.rx {
            if let Some(pin) = resolve_pin(rx) {
                assigned_pins.insert(pin);
            }
        }
        if let Some(ref pin_name) = bus.pin {
            if let Some(pin) = resolve_pin(pin_name) {
                assigned_pins.insert(pin);
            }
        }
        if let Some(ref pin_list) = bus.pins {
            for pin_name in pin_list {
                if let Some(pin) = resolve_pin(pin_name) {
                    assigned_pins.insert(pin);
                }
            }
        }
    }

    assigned_pins.into_iter().collect()
}

/// Generates GPIO diagnostic mapping definitions from board TOML content.
pub fn generate_gpio_definitions(toml_content: &str, board_name: &str) -> String {
    let config: BoardsConfig =
        toml::from_str(toml_content).expect("Failed to parse boards TOML content");

    let board_config = config
        .boards
        .get(board_name)
        .unwrap_or_else(|| panic!("Board '{}' not found in TOML", board_name));

    let bus_assigned_pins = resolve_bus_assigned_pins(board_config);

    let template = GeneratedGpioTemplate {
        bus_assigned_pins,
        num_gpio_pins: 30,
    };

    template.render().expect("Failed to render GPIO template")
}

pub fn parse_memory_map(root: &Path) -> (u32, u32) {
    let path = root.join("platform/src/rp2040/layouts/xip/memory.x");
    if !path.exists() {
        return (0x10000000, 0x20042000);
    }
    let content = std::fs::read_to_string(path).expect("Failed to read memory.x");

    let mut flash_start = None;
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

            if name.eq_ignore_ascii_case("BOOT2") {
                if let Some(orig) = origin {
                    flash_start = Some(orig);
                }
            } else if name.eq_ignore_ascii_case("FLASH") {
                if let Some(orig) = origin {
                    if flash_start.is_none() {
                        flash_start = Some(orig);
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

    (flash_start.unwrap_or(0x10000000), ram_start + ram_length)
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
        "rp2040" => (1, 4096),
        _ => (1, 4096),
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

    // Motor configuration
    let motor_config = board_config.motor.clone().unwrap_or_default();
    constants.push(BoardConstant {
        name: "MOTOR_STARTUP_BLANKING_TICKS".to_string(),
        const_type: "u32".to_string(),
        val_str: motor_config.startup_blanking_ticks.to_string(),
        doc: "Startup blanking period in ticks during which safety limits/cutoff checks are bypassed.".to_string(),
    });
    constants.push(BoardConstant {
        name: "MOTOR_DEFAULT_MAX_CURRENT_MA".to_string(),
        const_type: "i32".to_string(),
        val_str: motor_config.default_max_current_ma.to_string(),
        doc: "Default maximum current limit for the motor controller in milliamperes.".to_string(),
    });
    constants.push(BoardConstant {
        name: "MOTOR_STARTUP_COMMUTATION_STEP_TICKS".to_string(),
        const_type: "u32".to_string(),
        val_str: motor_config.startup_commutation_step_ticks.to_string(),
        doc: "Initial commutation step interval in 10ms ticks during startup soft-start ramp."
            .to_string(),
    });
    constants.push(BoardConstant {
        name: "MOTOR_COMMUTATION_SPEED_DIVISOR".to_string(),
        const_type: "u32".to_string(),
        val_str: motor_config.commutation_speed_divisor.to_string(),
        doc:
            "Base speed percentage divisor for calculating target commutation step interval ticks."
                .to_string(),
    });
    constants.push(BoardConstant {
        name: "MOTOR_MIN_COMMUTATION_STEP_TICKS".to_string(),
        const_type: "u32".to_string(),
        val_str: motor_config.min_commutation_step_ticks.to_string(),
        doc: "Minimum commutation step interval in 10ms ticks at full speed.".to_string(),
    });
    constants.push(BoardConstant {
        name: "MOTOR_MAX_COMMUTATION_STEP_TICKS".to_string(),
        const_type: "u32".to_string(),
        val_str: motor_config.max_commutation_step_ticks.to_string(),
        doc: "Maximum commutation step interval in 10ms ticks at minimum speed.".to_string(),
    });
    constants.push(BoardConstant {
        name: "MOTOR_CONFIG".to_string(),
        const_type: "model::types::MotorConfig".to_string(),
        val_str: format!(
            "model::types::MotorConfig {{\n        startup_blanking_ticks: {},\n        default_max_current_ma: {},\n        startup_commutation_step_ticks: {},\n        commutation_speed_divisor: {},\n        min_commutation_step_ticks: {},\n        max_commutation_step_ticks: {},\n    }}",
            motor_config.startup_blanking_ticks,
            motor_config.default_max_current_ma,
            motor_config.startup_commutation_step_ticks,
            motor_config.commutation_speed_divisor,
            motor_config.min_commutation_step_ticks,
            motor_config.max_commutation_step_ticks,
        ),
        doc: "Canonical MotorConfig model instance configured from board.toml.".to_string(),
    });

    constants.sort_by(|a, b| a.name.cmp(&b.name));

    let bus_assigned_pins = resolve_bus_assigned_pins(board_config);

    let template = GeneratedBoardTemplate {
        name: board_name.to_string(),
        chip: board_config.chip.clone(),
        flash_base: board_config.flash_base,
        constants,
        bus_assigned_pins,
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
        if let Some(ref sda) = i2c_bus.sda {
            i2c_sda_pin = sda.clone();
        }
        if let Some(ref scl) = i2c_bus.scl {
            i2c_scl_pin = scl.clone();
        }
        if let Some(freq) = i2c_bus.frequency {
            i2c_frequency = freq;
        }
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
