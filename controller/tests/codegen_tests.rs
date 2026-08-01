use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Param {
    name: String,
    r#type: String,
}

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

    fn extra_params_inferred(&self) -> Vec<Param> {
        let mut params = Vec::new();

        let cap = self.receiver_capacity.as_deref().unwrap_or("4");
        params.push(Param {
            name: "r".to_string(),
            r#type: format!(
                "$crate::{}Receiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, {}>",
                self.name, cap
            ),
        });

        if self.is_system.unwrap_or(false) {
            params.push(Param {
                name: "r2".to_string(),
                r#type: "platform::gesture_detector::GestureReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>".to_string(),
            });
            params.push(Param {
                name: "r3".to_string(),
                r#type: "embassy_sync::channel::Receiver<'static, embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, $crate::ThermalUpdateAction, 4>".to_string(),
            });
        }

        if self.has_telemetry.unwrap_or(true) {
            params.push(Param {
                name: "t".to_string(),
                r#type: "$crate::TelemetrySender<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, { $crate::telemetry_controller::CHANNEL_CAPACITY }>".to_string(),
            });
        }

        params
    }
}

#[derive(Deserialize, Debug)]
struct ControllerConfig {
    controllers: Vec<Controller>,
}

#[test]
fn test_controllers_toml_deserialization_and_inference() {
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
    let led_params = led.extra_params_inferred();
    assert_eq!(led_params.len(), 2);
    assert_eq!(led_params[0].name, "r");
    assert_eq!(
        led_params[0].r#type,
        "$crate::LedReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>"
    );
    assert_eq!(led_params[1].name, "t");
    assert_eq!(led_params[1].r#type, "$crate::TelemetrySender<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, { $crate::telemetry_controller::CHANNEL_CAPACITY }>");

    // Verify System controller properties (is_system = true, has_telemetry = false)
    let system = config
        .controllers
        .iter()
        .find(|c| c.name == "System")
        .unwrap();
    assert_eq!(system.is_system, Some(true));
    assert_eq!(system.has_telemetry, Some(false));
    let system_params = system.extra_params_inferred();
    assert_eq!(system_params.len(), 3);
    assert_eq!(system_params[0].name, "r");
    assert_eq!(system_params[1].name, "r2");
    assert_eq!(system_params[2].name, "r3");

    // Verify Filesystem controller properties (receiver_capacity = "16", has_telemetry = false)
    let fs = config
        .controllers
        .iter()
        .find(|c| c.name == "Filesystem")
        .unwrap();
    assert_eq!(fs.receiver_capacity, Some("16".to_string()));
    assert_eq!(fs.has_telemetry, Some(false));
    let fs_params = fs.extra_params_inferred();
    assert_eq!(fs_params.len(), 1);
    assert_eq!(fs_params[0].name, "r");
    assert_eq!(fs_params[0].r#type, "$crate::FilesystemReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 16>");

    // Verify Sensor controller properties (has_telemetry = false)
    let sensor = config
        .controllers
        .iter()
        .find(|c| c.name == "Sensor")
        .unwrap();
    assert_eq!(sensor.has_telemetry, Some(false));
    let sensor_params = sensor.extra_params_inferred();
    assert_eq!(sensor_params.len(), 1);
    assert_eq!(sensor_params[0].name, "r");

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
    let telemetry_params = telemetry.extra_params_inferred();
    assert_eq!(telemetry_params.len(), 1);
    assert_eq!(telemetry_params[0].name, "r");
    assert_eq!(telemetry_params[0].r#type, "$crate::TelemetryReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, $channel_size>");

    // Verify spawn_extra_rxs empty default check
    assert_eq!(led.spawn_extra_rxs_slice().len(), 0);
    assert_eq!(system.spawn_extra_rxs_slice().len(), 2);
}
