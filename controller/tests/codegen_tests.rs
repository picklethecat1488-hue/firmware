use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Controller {
    name: String,
    msg_type: String,
    macro_params: Vec<String>,
    task_call_args: Vec<String>,
    controller_type: String,
    spawn_call_args: Vec<String>,
    spawn_extra_rxs: Option<Vec<String>>,
    macro_generics: Option<Vec<String>>,
    spawn_generics: Option<Vec<String>>,
    attributes: Option<Vec<String>>,

    has_telemetry: Option<bool>,
    receiver_capacity: Option<String>,
    is_system: Option<bool>,
}

impl Controller {
    fn spawn_extra_rxs_slice(&self) -> &[String] {
        self.spawn_extra_rxs.as_deref().unwrap_or(&[])
    }
}

#[derive(Deserialize, Debug)]
struct ControllerConfig {
    controllers: Vec<Controller>,
}

#[test]
fn test_controllers_toml_deserialization() {
    // Read controllers.toml from the package root directory
    let toml_content = std::fs::read_to_string("controllers.toml")
        .expect("Failed to read controllers.toml from package root");

    let config: ControllerConfig =
        toml::from_str(&toml_content).expect("Failed to deserialize controllers.toml");

    // We expect exactly 8 controllers
    assert_eq!(config.controllers.len(), 8);

    // Verify Led controller properties
    let led = config.controllers.iter().find(|c| c.name == "Led").unwrap();
    assert!(led.has_telemetry.is_none()); // should default to true on unwrap

    // Verify System controller properties (is_system = true, has_telemetry = false)
    let system = config
        .controllers
        .iter()
        .find(|c| c.name == "System")
        .unwrap();
    assert_eq!(system.is_system, Some(true));
    assert_eq!(system.has_telemetry, Some(false));

    // Verify Filesystem controller properties (receiver_capacity = "16", has_telemetry = false)
    let fs = config
        .controllers
        .iter()
        .find(|c| c.name == "Filesystem")
        .unwrap();
    assert_eq!(fs.receiver_capacity, Some("16".to_string()));
    assert_eq!(fs.has_telemetry, Some(false));

    // Verify Sensor controller properties (has_telemetry = false)
    let sensor = config
        .controllers
        .iter()
        .find(|c| c.name == "Sensor")
        .unwrap();
    assert_eq!(sensor.has_telemetry, Some(false));

    // Verify Telemetry controller properties (receiver_capacity = "$channel_size")
    let telemetry = config
        .controllers
        .iter()
        .find(|c| c.name == "Telemetry")
        .unwrap();
    assert_eq!(
        telemetry.receiver_capacity,
        Some("$channel_size".to_string())
    );
    assert_eq!(telemetry.has_telemetry, Some(false));

    // Verify spawn_extra_rxs empty default check
    assert_eq!(led.spawn_extra_rxs_slice().len(), 0);
    assert_eq!(system.spawn_extra_rxs_slice().len(), 2);
}
