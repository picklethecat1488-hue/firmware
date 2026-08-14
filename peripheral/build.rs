use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=peripheral.toml");
    println!("cargo:rerun-if-changed=build.rs");

    if env::var("CARGO_FEATURE_TRACING").is_ok() {
        println!("cargo:rustc-env=DEFMT_LOG=trace");
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_initializers.rs");

    let toml_str = fs::read_to_string("peripheral.toml").expect("Failed to read peripheral.toml");
    let code = code_gen::generate_peripheral_initializers(&toml_str);

    fs::write(&dest_path, code).unwrap();
}
