//! Cargo Build Script for configuring app-specific tracing and generating active topology.
//!
//! This script resolves and parses the topology defined in `app.toml`, then calls the
//! code generator to output `generated_app.rs` inside the build `OUT_DIR`.

const LOG_LEVEL: &str = "trace";

/// Main entry point for the app package build script.
///
/// Steps:
/// 1. Configure the defmt log level from environment settings.
/// 2. Tell Cargo to rerun this build script if `app.toml` changes.
/// 3. Read the application topology configuration from `app.toml`.
/// 4. Generate the active spawning macros and dynamic `get_shell_pointers` implementation.
/// 5. Write the resulting code to `generated_app.rs` in `OUT_DIR`.
fn main() {
    if std::env::var("CARGO_FEATURE_TRACING").is_ok() {
        println!("cargo:rustc-env=DEFMT_LOG={}", LOG_LEVEL);
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let toml_path = code_gen::find_app_toml();
    println!("cargo:rerun-if-changed={}", toml_path.display());

    let content = std::fs::read_to_string(&toml_path).expect("Failed to read app.toml");
    let app_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let generated = code_gen::generate_app_topology(&content, &app_name);

    let dest_path = std::path::Path::new(&out_dir).join("generated_app.rs");
    std::fs::write(&dest_path, generated).expect("Failed to write generated_app.rs");
}
