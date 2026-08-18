//! Code generation logic for application active topology, spawning macros, and shell pointers.

use rinja::Template;
use std::collections::BTreeMap;

/// A single resolved pointer item representing a registered device in the interactive shell pointers.
#[derive(Debug, Clone)]
pub struct ShellItem {
    /// Name key of the device in the array mapping.
    pub shell_name: String,
    /// Concrete type of the resolved pointer target.
    pub shell_type: String,
    /// Cast or retrieval pointer expression.
    pub ptr: String,
}

/// A resolved channel details struct parsed and prepared in Rust for the Rinja template.
#[derive(Debug, Clone)]
pub struct ResolvedChannel {
    /// Variable name identifier of the channel (lowercase).
    pub name: String,
    /// Type signature string of the channel.
    pub channel_type: String,
    /// Resolved capacity limit.
    pub capacity: String,
    /// Resolved initialization value expression.
    pub val_expr: String,
    /// Flag indicating if the type is a direct embassy_sync::channel::Channel.
    pub is_embassy_channel: bool,
}

/// A resolved flash partition containing absolute address coordinates resolved from board.toml.
#[derive(Debug, Clone)]
pub struct ResolvedPartition {
    /// Name of the partition.
    pub name: String,
    /// Hexadecimal string representation of the absolute start address in memory.
    pub start_address: String,
    /// Hexadecimal string representation of the absolute end address in memory.
    pub end_address: String,
    /// Memory block partition strategy type.
    pub kind: String,
}

/// A resolved controller containing the hygiene-safe macro instance path mapping.
#[derive(Debug, Clone)]
pub struct ResolvedController {
    /// Name of the domain controller.
    pub name: String,
    /// Flag indicating if the controller is enabled.
    pub enabled: Option<bool>,
    /// Execution core processor ID (e.g. 0 or 1).
    pub core: Option<u32>,
    /// Path variable identifier of the controller instance.
    pub instance: String,
    /// Macro-safe hygienic representation of the instance.
    pub macro_instance: String,
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

/// Rinja template context for generating `generated_app.rs`.
#[derive(Template)]
#[template(path = "generated_app.rs.jinja", escape = "none")]
pub struct GeneratedAppTemplate {
    /// Associated target CLI shell configuration struct name.
    pub shell_config: String,
    /// Configured messaging channels.
    pub channels: Vec<ResolvedChannel>,
    /// Resolved telemetry channel static name (if present).
    pub telemetry_channel: Option<String>,
    /// Configured controllers in the topology.
    pub controllers: Vec<ResolvedController>,
    /// Configured absolute flash layout partition segments.
    pub partitions: Vec<ResolvedPartition>,
    /// BTreeMap of grouped ShellItems mapped by their shell field keys.
    pub shell_groups: BTreeMap<String, Vec<ShellItem>>,
    /// Active feature types list.
    pub feature_types: Vec<String>,
    /// Active feature initialization expressions list.
    pub feature_inits: Vec<String>,
    /// Inactivity timeout seconds.
    pub inactivity_timeout_seconds: u32,
    /// Derived name of the FeatureSet struct.
    pub feature_set_name: String,
    /// Number of active features in the FeatureSet.
    pub feature_count: usize,
}

/// Generates the active application topology, spawning macros, and get_shell_pointers implementation.
///
/// Parameters:
/// - `app_toml_content`: String slice of the `app.toml` contents.
/// - `app_name`: Identifier of the target application (e.g. "cat_detector").
///
/// Returns:
/// A `String` containing the rendered Rust code.
pub fn generate_app_topology(app_toml_content: &str, app_name: &str) -> String {
    let multi_config: crate::utils::MultiAppConfig = toml::from_str(app_toml_content)
        .expect("Failed to parse app.toml inside generate_app_topology");

    let app_topology = multi_config
        .apps
        .get(app_name)
        .unwrap_or_else(|| panic!("Could not find topology for app '{}' in app.toml", app_name));

    // Resolve partition coordinates by reading board.toml
    let board_toml_path = crate::utils::find_board_toml();
    let board_toml_content =
        std::fs::read_to_string(&board_toml_path).expect("Failed to read board.toml");
    let boards_config: crate::board::BoardsConfig = toml::from_str(&board_toml_content)
        .expect("Failed to parse board.toml inside app topology generation");
    let board_config = boards_config
        .boards
        .get("cat_detector")
        .expect("Failed to find board config for 'cat_detector' in board.toml");

    let mut resolved_partitions = Vec::new();
    for p in &app_topology.partitions {
        let bp = board_config
            .partitions
            .get(&p.board_partition)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find partition '{}' in board.toml referenced by app partition '{}'",
                    p.board_partition, p.name
                )
            });
        let start = bp.start;
        let end = bp.end;
        resolved_partitions.push(ResolvedPartition {
            name: p.name.clone(),
            start_address: format!("0x{:08X}", start),
            end_address: format!("0x{:08X}", end),
            kind: p.kind.clone(),
        });
    }

    // Resolve channels
    let mut resolved_channels = Vec::new();
    for (name, ch) in &app_topology.channels {
        let is_embassy_channel = ch.r#type.starts_with("embassy_sync");
        let capacity_str = match &ch.capacity {
            Some(toml::Value::Integer(i)) => i.to_string(),
            Some(toml::Value::String(s)) => s.clone(),
            _ => "4".to_string(),
        };
        let val_expr = ch
            .val
            .clone()
            .unwrap_or_else(|| format!("{}::new()", ch.r#type));
        resolved_channels.push(ResolvedChannel {
            name: name.clone(),
            channel_type: ch.r#type.clone(),
            capacity: capacity_str,
            val_expr,
            is_embassy_channel,
        });
    }

    // Resolve telemetry channel static name dynamically from configured channels
    let mut telemetry_channel = None;
    for ch in &resolved_channels {
        if ch.channel_type.contains("TelemetryChannel") {
            telemetry_channel = Some(format!("{}_CHANNEL", ch.name.to_uppercase()));
            break;
        }
    }

    // Resolve missing channel names and hygiene-safe macro instance mappings
    let mut resolved_controllers = Vec::new();
    for ctrl in &app_topology.controllers {
        let channel_name = ctrl.channel.clone().unwrap_or_else(|| {
            if ctrl.name == "Sensor" {
                format!(
                    "SENSOR_{}_CHANNEL",
                    ctrl.shell_name.as_ref().unwrap().to_uppercase()
                )
            } else {
                format!("{}_CHANNEL", ctrl.name.to_uppercase())
            }
        });

        let macro_instance = if ctrl.instance.starts_with("controllers.") {
            ctrl.instance.replacen("controllers.", "$controllers.", 1)
        } else {
            format!("${}", ctrl.instance)
        };

        resolved_controllers.push(ResolvedController {
            name: ctrl.name.clone(),
            enabled: ctrl.enabled,
            core: ctrl.core,
            instance: ctrl.instance.clone(),
            macro_instance,
            channel: Some(channel_name),
            generics: ctrl.generics.clone(),
            shell_field: ctrl.shell_field.clone(),
            shell_type: ctrl.shell_type.clone(),
            shell_name: ctrl.shell_name.clone(),
            ptr: ctrl.ptr.clone(),
            register: ctrl.register.clone(),
            device_shell_field: ctrl.device_shell_field.clone(),
            device_shell_type: ctrl.device_shell_type.clone(),
            device_shell_name: ctrl.device_shell_name.clone(),
            device_ptr: ctrl.device_ptr.clone(),
        });
    }

    // Group controllers and devices into shell pointer registry groups
    let mut shell_groups: BTreeMap<String, Vec<ShellItem>> = BTreeMap::new();

    // 1. Group controllers
    for ctrl in &resolved_controllers {
        let enabled = ctrl.enabled.unwrap_or(true);
        if !enabled {
            continue;
        }

        let name_lower = ctrl.name.to_lowercase();
        let derived_field_and_type = match name_lower.as_str() {
            "thermal" => Some(("thermals", "ThermalControllerType")),
            "battery" => Some(("batteries", "BatteryControllerType")),
            "led" => Some(("leds", "LedControllerType")),
            "system" => Some(("system_ctrls", "SystemControllerType")),
            "motor" => Some(("motor_ctrls", "MotorControllerType")),
            "sensor" => Some(("sensors", "SensorControllerType")),
            _ => None,
        };

        let shell_field = ctrl
            .shell_field
            .clone()
            .or_else(|| derived_field_and_type.map(|(f, _)| f.to_string()));
        let shell_type = ctrl
            .shell_type
            .clone()
            .or_else(|| derived_field_and_type.map(|(_, t)| t.to_string()));

        if let Some(ref field) = shell_field {
            let name = ctrl
                .shell_name
                .clone()
                .unwrap_or_else(|| "default".to_string());
            let ty = shell_type.expect("Missing shell_type");
            let mut ptr = ctrl.ptr.clone().unwrap_or_else(|| {
                if ctrl.core.unwrap_or(0) != 0 && ctrl.name == "Motor" {
                    format!("&mut controllers.core0.{}_proxy", name_lower)
                } else {
                    format!("&mut {}", ctrl.instance)
                }
            });
            if ptr.contains("controllers.") {
                ptr = ptr.replacen("controllers.", "$controllers.", 1);
            }
            shell_groups
                .entry(field.clone())
                .or_default()
                .push(ShellItem {
                    shell_name: name,
                    shell_type: ty,
                    ptr,
                });
        }

        let mut device_shell_field = ctrl.device_shell_field.clone();
        if device_shell_field.is_none() {
            if let Some(ref ty) = ctrl.device_shell_type {
                let dev_name = ty.replace("Device", "").to_lowercase();
                device_shell_field = Some(format!("{}s", dev_name));
            }
        }

        if let Some(ref field) = device_shell_field {
            let name = ctrl
                .device_shell_name
                .clone()
                .unwrap_or_else(|| "default".to_string());
            let ty = ctrl
                .device_shell_type
                .clone()
                .expect("Missing device_shell_type for controller mapped to shell");
            let mut ptr = ctrl.device_ptr.clone().unwrap_or_else(|| {
                if ctrl.core.unwrap_or(0) != 0 {
                    "core::ptr::null_mut()".to_string()
                } else {
                    let dev_name = ty.replace("Device", "").to_lowercase();
                    format!("&mut {}.{}", ctrl.instance, dev_name)
                }
            });
            if ptr.contains("controllers.") {
                ptr = ptr.replacen("controllers.", "$controllers.", 1);
            }
            shell_groups
                .entry(field.clone())
                .or_default()
                .push(ShellItem {
                    shell_name: name,
                    shell_type: ty,
                    ptr,
                });
        }
    }

    let mut feature_types = Vec::new();
    let mut feature_inits = Vec::new();
    let mut inactivity_timeout_seconds = 30;

    let name_pascal = app_name
        .split('_')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>();
    let feature_set_name = format!("{}FeatureSet", name_pascal);

    struct FeatureSchema {
        name: &'static str,
        type_generator: fn(&std::collections::BTreeMap<String, toml::Value>) -> String,
        init_generator: fn(&std::collections::BTreeMap<String, toml::Value>) -> String,
    }

    if let Some(ref features) = app_topology.features {
        if let Some(val) = features.inactivity_timeout_seconds {
            inactivity_timeout_seconds = val;
        }

        let schemas = [
            FeatureSchema {
                name: "motor",
                type_generator: |_| "controller::MotorFeatureConfig<MutexRaw>".to_string(),
                init_generator: |params| {
                    let channel_raw = params
                        .get("channel")
                        .and_then(|v| v.as_str())
                        .expect("Missing channel for motor feature");
                    let channel = format!("{}_CHANNEL", channel_raw.to_uppercase());
                    let max_speed = params
                        .get("max_speed")
                        .and_then(|v| v.as_str())
                        .unwrap_or("model::types::MotorSpeed::MAX");
                    format!(
                        "controller::MotorFeatureConfig::new(Some({}.sender()), {})",
                        channel, max_speed
                    )
                },
            },
            FeatureSchema {
                name: "battery",
                type_generator: |_| "controller::BatteryFeatureConfig<MutexRaw>".to_string(),
                init_generator: |params| {
                    let channel_raw = params
                        .get("channel")
                        .and_then(|v| v.as_str())
                        .expect("Missing channel for battery feature");
                    let channel = format!("{}_CHANNEL", channel_raw.to_uppercase());
                    let critical_soc = get_param_str(params, "critical_soc")
                        .expect("Missing critical_soc for battery feature");
                    let hysteresis = get_param_str(params, "hysteresis")
                        .expect("Missing hysteresis for battery feature");
                    let low_soc = get_param_str(params, "low_soc")
                        .expect("Missing low_soc for battery feature");
                    let mid_soc = get_param_str(params, "mid_soc")
                        .expect("Missing mid_soc for battery feature");
                    let high_soc = get_param_str(params, "high_soc")
                        .expect("Missing high_soc for battery feature");
                    format!(
                        "controller::BatteryFeatureConfig::new(Some({}.sender()), platform::BatteryManager::new({}, {}, {}, {}, {}))",
                        channel, critical_soc, hysteresis, low_soc, mid_soc, high_soc
                    )
                },
            },
            FeatureSchema {
                name: "proximity",
                type_generator: |params| {
                    let channels_val = params
                        .get("channels")
                        .expect("Missing channels for proximity feature");
                    let len = channels_val
                        .as_array()
                        .expect("proximity channels must be an array")
                        .len();
                    format!("controller::ProximityFeatureConfig<MutexRaw, {}>", len)
                },
                init_generator: |params| {
                    let channels_val = params
                        .get("channels")
                        .expect("Missing channels for proximity feature");
                    let channels: Vec<String> = channels_val
                        .as_array()
                        .expect("proximity channels must be an array")
                        .iter()
                        .map(|v| {
                            let ch = v.as_str().expect("proximity channel must be a string");
                            format!("{}_CHANNEL", ch.to_uppercase())
                        })
                        .collect();
                    let press_threshold = get_param_str(params, "press_threshold")
                        .expect("Missing press_threshold for proximity feature");
                    let near_threshold = get_param_str(params, "near_threshold")
                        .expect("Missing near_threshold for proximity feature");
                    let wake_threshold = get_param_str(params, "wake_threshold")
                        .expect("Missing wake_threshold for proximity feature");
                    let gesture_action = get_param_str(params, "gesture_action")
                        .expect("Missing gesture_action for proximity feature");
                    let telemetry_raw = get_param_str(params, "telemetry");

                    let channel_senders: Vec<String> = channels
                        .iter()
                        .map(|ch| format!("{}.sender()", ch))
                        .collect();
                    let telemetry_sender = match telemetry_raw {
                        Some(ref tel) => format!("Some({}_CHANNEL.sender())", tel.to_uppercase()),
                        None => "None".to_string(),
                    };
                    format!(
                        "controller::ProximityFeatureConfig::new(&[{}], {}, {}, {}, {}, {})",
                        channel_senders.join(", "),
                        press_threshold,
                        near_threshold,
                        wake_threshold,
                        gesture_action,
                        telemetry_sender
                    )
                },
            },
            FeatureSchema {
                name: "led",
                type_generator: |_| "controller::LedFeatureConfig<MutexRaw>".to_string(),
                init_generator: |params| {
                    let channel_raw = params
                        .get("channel")
                        .and_then(|v| v.as_str())
                        .expect("Missing channel for led feature");
                    let channel = format!("{}_CHANNEL", channel_raw.to_uppercase());
                    let brightness = params
                        .get("brightness")
                        .and_then(|v| v.as_integer())
                        .unwrap_or(100);
                    format!(
                        "controller::LedFeatureConfig::new(Some({}.sender()), {})",
                        channel, brightness
                    )
                },
            },
            FeatureSchema {
                name: "thermal",
                type_generator: |_| "controller::ThermalFeatureConfig<MutexRaw>".to_string(),
                init_generator: |params| {
                    let channel_raw = params
                        .get("channel")
                        .and_then(|v| v.as_str())
                        .expect("Missing channel for thermal feature");
                    let channel = format!("{}_CHANNEL", channel_raw.to_uppercase());
                    let overheating_threshold = get_param_str(params, "overheating_threshold")
                        .expect("Missing overheating_threshold for thermal feature");
                    let critical_threshold = get_param_str(params, "critical_threshold")
                        .expect("Missing critical_threshold for thermal feature");
                    format!(
                        "controller::ThermalFeatureConfig::new_with_thresholds(Some({}.sender()), {}, {})",
                        channel, overheating_threshold, critical_threshold
                    )
                },
            },
        ];

        // 1. Detect typos
        for feat_name in features.list.keys() {
            if !schemas.iter().any(|s| s.name == feat_name.as_str()) {
                panic!("Unknown system feature type '{}' in app.toml", feat_name);
            }
        }

        // 2. Loop through active features in deterministic order
        for schema in &schemas {
            if let Some(feat_params) = features.list.get(schema.name) {
                feature_types.push((schema.type_generator)(feat_params));
                feature_inits.push((schema.init_generator)(feat_params));
            }
        }
    }

    let feature_count = feature_types.len();

    let template = GeneratedAppTemplate {
        shell_config: app_topology.shell_config.clone(),
        channels: resolved_channels,
        telemetry_channel,
        controllers: resolved_controllers,
        partitions: resolved_partitions,
        shell_groups,
        feature_types,
        feature_inits,
        inactivity_timeout_seconds,
        feature_set_name,
        feature_count,
    };

    template
        .render()
        .expect("Failed to render GeneratedAppTemplate")
}

/// Rinja template for rendering a minimal application library logic skeleton.
#[derive(Template)]
#[template(path = "lib.rs.jinja", escape = "none")]
pub struct AppSkeletonTemplate {
    /// PascalCase name of the application.
    pub name_pascal: String,
    /// Whether Core 1 is enabled.
    pub core1_enabled: bool,
}

/// Renders a minimal application library logic skeleton.
pub fn render_app_skeleton(name_pascal: &str) -> String {
    let core1_enabled = is_core1_enabled_for_app(name_pascal);
    let template = AppSkeletonTemplate {
        name_pascal: name_pascal.to_string(),
        core1_enabled,
    };
    template
        .render()
        .expect("Failed to render app skeleton template")
}

/// Rinja template for rendering a production firmware binary main entry skeleton.
#[derive(Template)]
#[template(path = "app.rs.jinja", escape = "none")]
pub struct AppRunnerSkeletonTemplate {
    /// SnakeCase name of the application module/crate.
    pub name_snake: String,
    /// PascalCase name of the application.
    pub name_pascal: String,
    /// Whether Core 1 is enabled.
    pub core1_enabled: bool,
}

/// Renders a production firmware binary main entry skeleton.
pub fn render_app_runner_skeleton(name_snake: &str, name_pascal: &str) -> String {
    let core1_enabled = is_core1_enabled_for_app(name_pascal);
    let template = AppRunnerSkeletonTemplate {
        name_snake: name_snake.to_string(),
        name_pascal: name_pascal.to_string(),
        core1_enabled,
    };
    template
        .render()
        .expect("Failed to render app runner skeleton template")
}

/// Rinja template for rendering an interactive CLI console shell runner skeleton.
#[derive(Template)]
#[template(path = "shell.rs.jinja", escape = "none")]
pub struct AppShellSkeletonTemplate {
    /// SnakeCase name of the application module/crate.
    pub name_snake: String,
    /// PascalCase name of the application.
    pub name_pascal: String,
    /// Name of the shell configuration struct.
    pub shell_config: String,
    /// Whether Core 1 is enabled.
    pub core1_enabled: bool,
}

/// Renders an interactive CLI console shell runner skeleton.
pub fn render_app_shell_skeleton(
    name_snake: &str,
    name_pascal: &str,
    shell_config: &str,
) -> String {
    let core1_enabled = is_core1_enabled_for_app(name_pascal);
    let template = AppShellSkeletonTemplate {
        name_snake: name_snake.to_string(),
        name_pascal: name_pascal.to_string(),
        shell_config: shell_config.to_string(),
        core1_enabled,
    };
    template
        .render()
        .expect("Failed to render app shell skeleton template")
}

fn is_core1_enabled_for_app(app_name_pascal: &str) -> bool {
    let app_toml_path = crate::find_app_toml();
    let content = std::fs::read_to_string(&app_toml_path).expect("Failed to read app.toml");
    let app_config: crate::MultiAppConfig =
        toml::from_str(&content).expect("Failed to parse app.toml");

    let matched_app = app_config
        .apps
        .iter()
        .find(|(k, _)| {
            k.eq_ignore_ascii_case(app_name_pascal)
                || app_name_pascal.to_lowercase().contains(&k.to_lowercase())
        })
        .map(|(_, v)| v);

    if let Some(app_topology) = matched_app {
        app_topology
            .core1_enabled
            .unwrap_or_else(|| app_topology.controllers.iter().any(|c| c.core == Some(1)))
    } else {
        false
    }
}

fn get_param_str(
    params: &std::collections::BTreeMap<String, toml::Value>,
    key: &str,
) -> Option<String> {
    params.get(key).map(|v| match v {
        toml::Value::String(s) => s.clone(),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => f.to_string(),
        toml::Value::Boolean(b) => b.to_string(),
        _ => panic!("Unsupported TOML value type for feature parameter {}", key),
    })
}
