use code_gen::{ControllerConfig, RunLoopTemplate};
use indicatif::{ProgressBar, ProgressStyle};
use rinja::Template;
use std::fs;
use std::path::Path;

/// Handles generating boilerplate runloop implementations.
pub fn handle(targets: &[String], out_dir: &Path, config: &ControllerConfig) {
    if !targets.is_empty() {
        let mut controllers = Vec::new();
        // Validate all targets first
        for target in targets {
            let chosen = config
                .controllers
                .iter()
                .find(|c| c.name.eq_ignore_ascii_case(target));
            match chosen {
                Some(ctrl) => controllers.push(ctrl.clone()),
                None => {
                    eprintln!("Error: Unknown controller '{}'", target);
                    std::process::exit(1);
                }
            }
        }

        // Render and write each validated target
        for ctrl in controllers {
            let file_name = format!("{}_runloop.rs", ctrl.name.to_lowercase());
            let spinner = create_spinner(&file_name);

            let run_loop_template = RunLoopTemplate {
                name: ctrl.name.clone(),
                msg_type: ctrl.msg_type.clone(),
                has_telemetry: ctrl.has_telemetry.unwrap_or(true),
                is_system: ctrl.is_system.unwrap_or(false),
                impl_generics: ctrl.impl_generics.clone(),
                impl_type: ctrl.impl_type.clone(),
                impl_phantom: ctrl.impl_phantom_str().to_string(),
            };
            let output = run_loop_template
                .render()
                .expect("Failed to render runloop template");

            write_file(out_dir, &file_name, &output, &spinner);
        }
    } else {
        let file_name = "sample_runloops.rs";
        let spinner = create_spinner(file_name);

        let mut runloops_content = String::new();
        for ctrl in &config.controllers {
            runloops_content.push_str(&format!(
                "// --- Boilerplate runloop implementation for {} ---\n",
                ctrl.name
            ));
            let run_loop_template = RunLoopTemplate {
                name: ctrl.name.clone(),
                msg_type: ctrl.msg_type.clone(),
                has_telemetry: ctrl.has_telemetry.unwrap_or(true),
                is_system: ctrl.is_system.unwrap_or(false),
                impl_generics: ctrl.impl_generics.clone(),
                impl_type: ctrl.impl_type.clone(),
                impl_phantom: ctrl.impl_phantom_str().to_string(),
            };
            let run_loop_output = run_loop_template
                .render()
                .expect("Failed to render runloop template");
            runloops_content.push_str(&run_loop_output);
            runloops_content.push_str("\n\n");
        }

        write_file(out_dir, file_name, &runloops_content, &spinner);
    }
}

fn create_spinner(file_name: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
            .template("{spinner:.green} {msg}")
            .expect("Failed to create progress template"),
    );
    spinner.set_message(format!("Generating {}...", file_name));
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
    spinner
}

fn write_file(out_dir: &Path, file_name: &str, content: &str, spinner: &ProgressBar) {
    fs::create_dir_all(out_dir).expect("Failed to create output directory");
    let dest_path = out_dir.join(file_name);
    fs::write(&dest_path, content).expect("Failed to write file");
    spinner.finish_with_message(format!("Generated {}", dest_path.display()));
}
