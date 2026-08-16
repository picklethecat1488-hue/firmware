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
