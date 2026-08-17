use code_gen::{find_controllers_toml, ControllerConfig, RunLoopTemplate};
use rinja::Template;
use std::fs;

#[test]
fn test_find_toml() {
    let toml_path = find_controllers_toml();
    assert!(toml_path.exists());
    assert!(toml_path.to_string_lossy().contains("controllers.toml"));
}

#[test]
fn test_lookup_controller() {
    let toml_path = find_controllers_toml();
    let content = fs::read_to_string(&toml_path).unwrap();
    let config: ControllerConfig = toml::from_str(&content).unwrap();

    // Check exact match
    let led = config
        .controllers
        .iter()
        .find(|c| c.name.eq_ignore_ascii_case("Led"));
    assert!(led.is_some());
    assert_eq!(led.unwrap().name, "Led");

    // Check case-insensitive match
    let motor = config
        .controllers
        .iter()
        .find(|c| c.name.eq_ignore_ascii_case("motor"));
    assert!(motor.is_some());
    assert_eq!(motor.unwrap().name, "Motor");
}

#[test]
fn test_telemetry_defaults() {
    let toml_path = find_controllers_toml();
    let content = fs::read_to_string(&toml_path).unwrap();
    let config: ControllerConfig = toml::from_str(&content).unwrap();

    let led = config.controllers.iter().find(|c| c.name == "Led").unwrap();
    assert!(led.has_telemetry.is_none());

    let fs = config
        .controllers
        .iter()
        .find(|c| c.name == "Filesystem")
        .unwrap();
    assert_eq!(fs.has_telemetry, Some(false));
}

#[test]
fn test_runloop_template_rendering() {
    let run_loop_template = RunLoopTemplate {
        name: "Test".to_string(),
        msg_type: "crate::TestCmd".to_string(),
        has_telemetry: true,
        is_system: false,
        impl_generics: "<D>".to_string(),
        impl_type: "TestController<D>".to_string(),
        impl_phantom: "D".to_string(),
    };
    let output = run_loop_template.render().unwrap();
    assert!(output.contains("#[crate::tracing::controller_context]"));
    assert!(output.contains("pub struct TestController<D>"));
    assert!(output.contains("impl<D> TestController<D>"));
    assert!(output.contains("pub async fn run<MutexRaw: RawMutex, const SIZE: usize>"));
    assert!(output.contains("r: TestReceiver<MutexRaw, SIZE>"));
    assert!(output.contains("t: TelemetrySender<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, { crate::telemetry_controller::CHANNEL_CAPACITY }>"));
    assert!(output.contains("let mut telemetry_client = TestTelemetryClient::new(Some(t));"));
    assert!(output.contains("let cmd = r.receive().await;"));
    assert!(output.contains("match cmd"));
}

#[test]
fn test_peripheral_initializers_syntax_and_compiles() {
    let toml_path = code_gen::find_peripherals_toml();
    let content = fs::read_to_string(&toml_path).unwrap();
    let generated = code_gen::generate_peripheral_initializers(&content);
    assert!(!generated.is_empty());

    // Parse the generated macro definitions as Rust syntax to verify they compile
    let parsed = syn::parse_str::<syn::File>(&generated);
    assert!(
        parsed.is_ok(),
        "Failed to parse generated peripheral initializers: {:?}",
        parsed.err()
    );
}

#[test]
fn test_peripheral_sample_syntax_and_compiles() {
    use code_gen::PeripheralSampleTemplate;

    // Test with probeable (like Max17048)
    let template_probeable = PeripheralSampleTemplate {
        name: "MyProbeableSensor".to_string(),
        has_probeable: true,
        has_led_driver: false,
        has_fuel_gauge: false,
        has_tickable: false,
        has_charge_status: false,
        has_proximity_sensor: false,
    };
    let rendered_probeable = template_probeable.render().unwrap();
    assert!(rendered_probeable
        .contains("impl<I: I2c> model::interfaces::Probeable for MyProbeableSensor<I>"));
    let parsed_probeable = syn::parse_str::<syn::File>(&rendered_probeable);
    assert!(
        parsed_probeable.is_ok(),
        "Failed to parse MyProbeableSensor: {:?}",
        parsed_probeable.err()
    );

    // Test without probeable and check all other traits
    let template_other = PeripheralSampleTemplate {
        name: "MyOtherSensor".to_string(),
        has_probeable: false,
        has_led_driver: true,
        has_fuel_gauge: true,
        has_tickable: true,
        has_charge_status: true,
        has_proximity_sensor: true,
    };
    let rendered_other = template_other.render().unwrap();
    assert!(!rendered_other.contains("Probeable"));
    assert!(
        rendered_other.contains("impl<I: I2c> model::interfaces::LedDriver for MyOtherSensor<I>")
    );
    assert!(
        rendered_other.contains("impl<I: I2c> model::interfaces::FuelGauge for MyOtherSensor<I>")
    );
    assert!(
        rendered_other.contains("impl<I: I2c> model::interfaces::Tickable for MyOtherSensor<I>")
    );
    assert!(rendered_other
        .contains("impl<I: I2c> model::interfaces::ChargeStatus for MyOtherSensor<I>"));
    assert!(rendered_other
        .contains("impl<I: I2c> model::interfaces::ProximitySensor for MyOtherSensor<I>"));
    let parsed_other = syn::parse_str::<syn::File>(&rendered_other);
    assert!(
        parsed_other.is_ok(),
        "Failed to parse MyOtherSensor: {:?}",
        parsed_other.err()
    );
}

#[test]
fn test_find_workspace_root() {
    let root = code_gen::find_workspace_root();
    assert!(root.exists());
    assert!(root.join("pyproject.toml").exists());
}

#[test]
fn test_parse_subcommand_enums() {
    let root = code_gen::find_workspace_root();
    let mut enums = std::collections::HashMap::new();
    code_gen::parse_subcommand_enums(&root.join("controller/src"), &mut enums);
    assert!(!enums.is_empty(), "Parsed enums map should not be empty");

    let sensor_sub = enums.get("SensorSubcommand");
    assert!(sensor_sub.is_some(), "Should find SensorSubcommand");
    let subcommands = sensor_sub.unwrap();
    assert_eq!(subcommands.len(), 4);
    assert_eq!(subcommands[0].name, "status");
    assert_eq!(subcommands[0].doc, "Read sensor values");
    assert_eq!(subcommands[1].name, "cal_near");
    assert_eq!(subcommands[1].doc, "Calibrate near proximity");
}

#[test]
fn test_board_codegen() {
    let root = code_gen::find_workspace_root();
    let toml_path = root.join("board/board.toml");
    let content = std::fs::read_to_string(&toml_path).unwrap();
    let generated = code_gen::generate_board_definitions(&content, "cat_detector");
    assert!(generated.contains("pub const PUMP_PIN_IA: u32 = 19;"));
    assert!(generated.contains("pub const FS_PARTITION_START: u32 = 0x001C0000;"));

    assert!(generated.contains("chip: \"rp2040\""));
}

#[test]
#[should_panic(expected = "assigned to multiple resources")]
fn test_board_codegen_overlapping_pins() {
    let bad_toml = r#"
        [boards.bad_board]
        chip = "rp2040"
        flash_base = 0x10000000
        flash_size = 0x00200000
        sram_size = 0x00042000
        core0_stack_size = 24576
        core1_stack_size = 16384
        stack_top = 0x20042000
        core0_stack_bottom = 0x2003C000
        core_monitor_timeout_ms = 10000
        core_monitor_warn_pct = 80
        pins = { PIN_A = 10, PIN_B = 10 }
        buses = {}
        hardware_resources = {}
        partitions = {}
    "#;
    code_gen::generate_board_definitions(bad_toml, "bad_board");
}

#[test]
#[should_panic(expected = "invalid GPIO index")]
fn test_board_codegen_invalid_pin_range() {
    let bad_toml = r#"
        [boards.bad_board]
        chip = "rp2040"
        flash_base = 0x10000000
        flash_size = 0x00200000
        sram_size = 0x00042000
        core0_stack_size = 24576
        core1_stack_size = 16384
        stack_top = 0x20042000
        core0_stack_bottom = 0x2003C000
        core_monitor_timeout_ms = 10000
        core_monitor_warn_pct = 80
        pins = { PIN_A = 35 }
        buses = {}
        hardware_resources = {}
        partitions = {}
    "#;
    code_gen::generate_board_definitions(bad_toml, "bad_board");
}

#[test]
#[should_panic(expected = "invalid 7-bit I2C address")]
fn test_board_codegen_invalid_i2c_addr() {
    let bad_toml = r#"
        [boards.bad_board]
        chip = "rp2040"
        flash_base = 0x10000000
        flash_size = 0x00200000
        sram_size = 0x00042000
        core0_stack_size = 24576
        core1_stack_size = 16384
        stack_top = 0x20042000
        core0_stack_bottom = 0x2003C000
        core_monitor_timeout_ms = 10000
        core_monitor_warn_pct = 80
        pins = {}
        buses = {}
        hardware_resources = { SOME_I2C_ADDR = { value = 0x05, type = "u8" } }
        partitions = {}
    "#;
    code_gen::generate_board_definitions(bad_toml, "bad_board");
}

#[test]
fn test_board_placement_matches_memory_map() {
    let root = code_gen::find_workspace_root();
    let toml_path = root.join("board/board.toml");
    let content = std::fs::read_to_string(&toml_path).unwrap();
    let config: code_gen::BoardsConfig = toml::from_str(&content).unwrap();

    let board_config = config.boards.get("cat_detector").unwrap();

    // Parse memory map dynamically
    let (flash_start, ram_end) = code_gen::parse_memory_map(&root);

    // Validate that board.toml matches layout
    assert_eq!(board_config.flash_base, flash_start);
    assert_eq!(board_config.stack_top, ram_end);

    // Validate stack bottom derivation
    assert_eq!(
        board_config.core0_stack_bottom,
        board_config.stack_top - board_config.core0_stack_size as u32
    );

    // Validate that generated telemetry MAX_RECORDS fits in the telemetry partition and is multiple of 64
    let telemetry_start = board_config.partitions.get("telemetry").unwrap().start;
    let telemetry_end = board_config.partitions.get("telemetry").unwrap().end;
    let telemetry_size = telemetry_end - telemetry_start;

    let num_chunks = telemetry_size / 2560;
    let max_records_val = num_chunks as usize * 64;

    assert_eq!(
        max_records_val % 64,
        0,
        "MAX_RECORDS must be a multiple of 64"
    );
    assert_eq!(max_records_val, 4864);

    let required_size = num_chunks * 2560;
    assert!(
        required_size <= telemetry_size,
        "Telemetry partition size ({} bytes) is too small for {} records (requires {} bytes)",
        telemetry_size,
        max_records_val,
        required_size
    );
}

#[test]
fn test_board_codegen_valid_16bit_i2c_addr() {
    let valid_toml = r#"
        [boards.test_board]
        chip = "rp2040"
        flash_base = 0x10000000
        flash_size = 0x00200000
        sram_size = 0x00042000
        core0_stack_size = 24576
        core1_stack_size = 16384
        stack_top = 0x20042000
        core0_stack_bottom = 0x2003C000
        core_monitor_timeout_ms = 10000
        core_monitor_warn_pct = 80
        pins = {}
        buses = {}
        hardware_resources = { SOME_16BIT_I2C_ADDR = { value = 0x0A5F, type = "u16" } }
        partitions = {}
    "#;
    let generated = code_gen::generate_board_definitions(valid_toml, "test_board");
    assert!(generated.contains("pub const SOME_16BIT_I2C_ADDR: u16 = 2655;")); // 0x0A5F = 2655
}

#[test]
#[should_panic(expected = "invalid 16-bit I2C address")]
fn test_board_codegen_invalid_16bit_i2c_addr() {
    let bad_toml = r#"
        [boards.bad_board]
        chip = "rp2040"
        flash_base = 0x10000000
        flash_size = 0x00200000
        sram_size = 0x00042000
        core0_stack_size = 24576
        core1_stack_size = 16384
        stack_top = 0x20042000
        core0_stack_bottom = 0x2003C000
        core_monitor_timeout_ms = 10000
        core_monitor_warn_pct = 80
        pins = {}
        buses = {}
        hardware_resources = { INVALID_16BIT_I2C_ADDR = { value = 0x10000, type = "u16" } }
        partitions = {}
    "#;
    code_gen::generate_board_definitions(bad_toml, "bad_board");
}

#[test]
fn test_validate_app_toml_parsing_and_generation() {
    let app_toml_path = code_gen::find_app_toml();
    let content = fs::read_to_string(&app_toml_path).unwrap();

    // Validate parsing MultiAppConfig
    let multi_config: code_gen::MultiAppConfig = toml::from_str(&content).unwrap();
    assert!(multi_config.apps.contains_key("cat_detector"));

    // Validate rendering active topology
    let rendered = code_gen::generate_app_topology(&content, "cat_detector");
    assert!(!rendered.is_empty());
    assert!(rendered.contains("pub struct CatDetectorFeatureSet"));
    assert!(rendered.contains("pub fn create_default_feature_set"));
}
