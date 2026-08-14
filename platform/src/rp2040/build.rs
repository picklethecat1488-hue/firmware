use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Copy layouts/xip/memory.x to Cargo's output directory
    fs::copy("layouts/xip/memory.x", out_dir.join("memory.x")).unwrap();

    // Propagate link search path transitively to whatever binary links against us
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=layouts/xip/memory.x");
}
