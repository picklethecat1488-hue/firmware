use code_gen::{CliHandlerSkeletonTemplate, SampleCliTemplate, ShellConfigToml};
use indicatif::{ProgressBar, ProgressStyle};
use rinja::Template;
use std::fs;
use std::path::Path;

/// Handles generating CLI samples and writing them to files under `out_dir`.
pub fn handle(targets: &[String], out_dir: &Path, shell_config: &ShellConfigToml) {
    if !targets.is_empty() {
        let mut commands = Vec::new();
        // Validate all targets first
        for target in targets {
            let found = shell_config.cli_commands.iter().find(|cmd| {
                cmd.group.eq_ignore_ascii_case(target) || cmd.cmd_name.eq_ignore_ascii_case(target)
            });
            match found {
                Some(cmd) => commands.push(cmd.clone()),
                None => {
                    eprintln!("Error: Unknown CLI command/group '{}'", target);
                    std::process::exit(1);
                }
            }
        }

        // Render and write each validated target
        for cmd in commands {
            let file_name = format!("handle_{}_cli.rs", cmd.group.to_lowercase());
            let spinner = create_spinner(&file_name);

            let template = CliHandlerSkeletonTemplate { cmd };
            let output = template
                .render()
                .expect("Failed to render skeleton template");

            write_file(out_dir, &file_name, &output, &spinner);
        }
    } else {
        let file_name = "sample_cli.rs";
        let spinner = create_spinner(file_name);

        let template = SampleCliTemplate {
            cli_commands: shell_config.cli_commands.clone(),
        };
        let output = template
            .render()
            .expect("Failed to render sample CLI template");

        write_file(out_dir, file_name, &output, &spinner);
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
