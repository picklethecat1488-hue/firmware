use code_gen::ControllerConfig;

/// Handles listing all configured controllers.
pub fn handle(config: &ControllerConfig) {
    for ctrl in &config.controllers {
        println!("{}", ctrl.name);
    }
}
