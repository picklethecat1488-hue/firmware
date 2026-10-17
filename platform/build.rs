const LOG_LEVEL: &str = "trace";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let board_toml = code_gen::find_board_toml();
    println!("cargo:rerun-if-changed={}", board_toml.display());

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let content = std::fs::read_to_string(&board_toml).expect("Failed to read board.toml");
    let generated = code_gen::generate_gpio_definitions(&content, "cat_detector");

    let dest_path = std::path::Path::new(&out_dir).join("generated_gpio.rs");
    std::fs::write(&dest_path, generated).expect("Failed to write generated_gpio.rs");

    // If the tracing feature is enabled, override compile-time DEFMT_LOG level for this crate
    if std::env::var("CARGO_FEATURE_TRACING").is_ok() {
        println!("cargo:rustc-env=DEFMT_LOG={}", LOG_LEVEL);
    }

    // Ensure GIT_HASH is defined
    if std::env::var("GIT_HASH").is_err() {
        let git_hash = std::process::Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout).ok()
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "unknown".to_string());
        println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
    }
}
