use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=peripheral.toml");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-check-cfg=cfg(target_has_embassy_rp)");

    if env::var("CARGO_FEATURE_TRACING").is_ok() {
        println!("cargo:rustc-env=DEFMT_LOG=trace");
    }

    let target = env::var("TARGET").unwrap_or_default();
    if target == "thumbv6m-none-eabi" {
        println!("cargo:rustc-cfg=target_has_embassy_rp");
    } else if target.contains("none") {
        eprintln!(
            "Error: Target '{}' does not support the RP2040-specific embassy_rp crate. \
            The TMC6300 PIO driver requires RP2040 hardware support (thumbv6m-none-eabi). \
            For non-RP2040 ARM targets, use the standard GPIO-based Tmc6300 driver.",
            target
        );
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_initializers.rs");

    let toml_str = fs::read_to_string("peripheral.toml").expect("Failed to read peripheral.toml");
    let code = code_gen::generate_peripheral_initializers(&toml_str);

    fs::write(&dest_path, code).unwrap();
}
