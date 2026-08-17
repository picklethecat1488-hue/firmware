use std::path::PathBuf;

/// Searches upward from the current directory to locate the path of `controllers.toml`.
pub fn find_controllers_toml() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    loop {
        let toml_path = path.join("controller/controllers.toml");
        if toml_path.exists() {
            return toml_path;
        }
        let direct_toml_path = path.join("controllers.toml");
        if direct_toml_path.exists() {
            return direct_toml_path;
        }
        if !path.pop() {
            panic!(
                "Could not locate controllers.toml in current directory or any parent directories!"
            );
        }
    }
}

/// Searches upward from the current directory to locate the path of `shell.toml`.
pub fn find_shell_toml() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    loop {
        let toml_path = path.join("controller/shell.toml");
        if toml_path.exists() {
            return toml_path;
        }
        let direct_toml_path = path.join("shell.toml");
        if direct_toml_path.exists() {
            return direct_toml_path;
        }
        if !path.pop() {
            panic!("Could not locate shell.toml in current directory or any parent directories!");
        }
    }
}

/// Searches upward from the current directory to locate the path of `peripheral.toml`.
pub fn find_peripherals_toml() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    loop {
        let toml_path = path.join("peripheral/peripheral.toml");
        if toml_path.exists() {
            return toml_path;
        }
        let direct_toml_path = path.join("peripheral.toml");
        if direct_toml_path.exists() {
            return direct_toml_path;
        }
        if !path.pop() {
            panic!(
                "Could not locate peripheral.toml in current directory or any parent directories!"
            );
        }
    }
}

/// Searches upward from the current directory to locate the path of `board.toml`.
pub fn find_board_toml() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    loop {
        let toml_path = path.join("board/board.toml");
        if toml_path.exists() {
            return toml_path;
        }
        let direct_toml_path = path.join("board.toml");
        if direct_toml_path.exists() {
            return direct_toml_path;
        }
        if !path.pop() {
            panic!("Could not locate board.toml in current directory or any parent directories!");
        }
    }
}

/// Searches upward from the current directory to locate the path of `app.toml`.
///
/// Returns:
/// A `PathBuf` pointing to the located `app.toml` file.
///
/// Panics:
/// If the file cannot be located in the current directory or any parent directories.
pub fn find_app_toml() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    loop {
        let toml_path = path.join("app/app.toml");
        if toml_path.exists() {
            return toml_path;
        }
        let direct_toml_path = path.join("app.toml");
        if direct_toml_path.exists() {
            return direct_toml_path;
        }
        if !path.pop() {
            panic!("Could not locate app.toml in current directory or any parent directories!");
        }
    }
}

/// A controller configuration in the topology
#[derive(serde::Deserialize, Debug, Clone)]
pub struct TopologyController {
    /// Name of the domain controller.
    pub name: String,
    /// Flag indicating if the controller is enabled.
    pub enabled: Option<bool>,
    /// Execution core processor ID (e.g. 0 or 1).
    pub core: Option<u32>,
    /// Path variable identifier of the controller instance.
    pub instance: String,
    /// Messaging channel variable name.
    pub channel: Option<String>,
    /// Type generic parameter constraints for task initialization.
    pub generics: String,
    /// Optional field name mapped in the shell controller pointers.
    pub shell_field: Option<String>,
    /// Optional type representing pointer target inside the shell pointers.
    pub shell_type: Option<String>,
    /// Optional name key assigned to the device inside the shell pointers.
    pub shell_name: Option<String>,
    /// Pointer resolution logic or expression.
    pub ptr: Option<String>,
    /// Optional global OnceLock variable name for runtime pointer retrieval.
    pub register: Option<String>,
    /// Optional field mapping the controller's underlying raw hardware device.
    pub device_shell_field: Option<String>,
    /// Generic type signature of the raw hardware device.
    pub device_shell_type: Option<String>,
    /// Name key assigned to raw hardware device inside the shell pointers.
    pub device_shell_name: Option<String>,
    /// Pointer expression for the raw hardware device.
    pub device_ptr: Option<String>,
}

/// A channel configuration in the topology
#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppChannel {
    /// Type signature of the channel (e.g. "controller::ThermalChannel").
    pub r#type: String,
    /// Capacity count of the channel.
    pub capacity: Option<toml::Value>,
    /// Custom initialization value block.
    pub val: Option<String>,
}

/// A partition layout configuration in the topology
#[derive(serde::Deserialize, Debug, Clone)]
pub struct TopologyPartition {
    /// Name of the storage partition.
    pub name: String,
    /// The associated board partition key in board.toml.
    pub board_partition: String,
    /// Memory block partition strategy type.
    pub kind: String,
}

/// Configured system features parsed from `app.toml`.
#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppFeatures {
    /// Inactivity timeout seconds.
    pub inactivity_timeout_seconds: Option<u32>,
    /// Map of custom feature configurations.
    #[serde(flatten)]
    pub list: std::collections::BTreeMap<String, std::collections::BTreeMap<String, toml::Value>>,
}

/// Single application active topology structure parsed from `app.toml`.
#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppTopology {
    /// Associated target CLI shell configuration struct name.
    pub shell_config: String,
    /// Configured messaging channels.
    pub channels: std::collections::BTreeMap<String, AppChannel>,
    /// Configured topology controller devices.
    pub controllers: Vec<TopologyController>,
    /// Configured flash layout partition segments.
    pub partitions: Vec<TopologyPartition>,
    /// Configured active CLI command handlers.
    pub cli_handlers: Vec<String>,
    /// Configured system features.
    pub features: Option<AppFeatures>,
}

/// Root structure of `app.toml` supporting multiple applications.
#[derive(serde::Deserialize, Debug, Clone)]
pub struct MultiAppConfig {
    /// HashMap of individual application topologies.
    pub apps: std::collections::HashMap<String, AppTopology>,
}

/// Legacy AppConfig wrapper keeping compatibility with features/cli checks in controller crate
#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppConfig {
    /// Features (controllers) enablement map.
    pub features: std::collections::HashMap<String, bool>,
    /// CLI handlers enablement map.
    pub cli_handlers: std::collections::HashMap<String, bool>,
}
