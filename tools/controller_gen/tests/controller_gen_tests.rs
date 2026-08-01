use controller_gen::{find_controllers_toml, ControllerConfig, RunLoopTemplate};
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
    // has_telemetry is None, should default to true on inference
    let led_params = led.extra_params_inferred();
    assert_eq!(led_params.len(), 2);
    assert_eq!(led_params[1].name, "t");

    let fs = config
        .controllers
        .iter()
        .find(|c| c.name == "Filesystem")
        .unwrap();
    // has_telemetry is Some(false), should not generate 't'
    let fs_params = fs.extra_params_inferred();
    assert_eq!(fs_params.len(), 1);
    assert_ne!(fs_params[0].name, "t");
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
    };
    let output = run_loop_template.render().unwrap();
    assert!(output.contains("impl<D> TestController<D>"));
    assert!(output.contains("pub async fn run<MutexRaw: RawMutex, const SIZE: usize>"));
    assert!(output.contains("r: TestReceiver<MutexRaw, SIZE>"));
    assert!(output.contains("t: TelemetrySender<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, { crate::telemetry_controller::CHANNEL_CAPACITY }>"));
    assert!(output.contains("let mut telemetry_client = TestTelemetryClient::new(Some(t));"));
    assert!(output.contains("let cmd = r.receive().await;"));
    assert!(output.contains("match cmd"));
}
