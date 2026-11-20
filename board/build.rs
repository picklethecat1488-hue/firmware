//! Cargo Build Script for generating board-specific configuration constants.
//!
//! This script automatically loads the active board configuration from `board.toml`
//! and runs the code generator to produce the output `generated_board.rs` in `OUT_DIR`.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Main entry point for the build script.
///
/// This function:
/// 1. Tells Cargo to rerun this script if `board.toml` changes.
/// 2. Resolves the board configuration file.
/// 3. Runs code generation for the `"cat_detector"` target board.
/// 4. Writes the generated constants module to `generated_board.rs` inside the compilation `OUT_DIR`.
fn main() {
    println!("cargo:rerun-if-changed=board.toml");
    if [
        "CARGO_FEATURE_TRACING",
        "CARGO_FEATURE_TRACE_PERF",
        "CARGO_FEATURE_TRACE_SENSORS",
        "CARGO_FEATURE_TRACE_SENSOR_SPANS",
        "CARGO_FEATURE_TRACE_AUDIO",
    ]
    .iter()
    .any(|var| env::var(var).is_ok())
    {
        println!("cargo:rustc-env=DEFMT_LOG=trace");
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let toml_path = code_gen::find_board_toml();
    let content = std::fs::read_to_string(&toml_path).expect("Failed to read board.toml");

    let generated_cat = code_gen::generate_board_definitions(&content, "cat_detector");
    let mut f_cat = File::create(Path::new(&out_dir).join("generated_board.rs")).unwrap();
    f_cat.write_all(generated_cat.as_bytes()).unwrap();

    if content.contains("[boards.carrier_board]") {
        let generated_cb = code_gen::generate_board_definitions(&content, "carrier_board");
        let mut f_cb =
            File::create(Path::new(&out_dir).join("generated_carrier_board.rs")).unwrap();
        f_cb.write_all(generated_cb.as_bytes()).unwrap();
    }
}
