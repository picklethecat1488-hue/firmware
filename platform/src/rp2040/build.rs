use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap_or_default();

    let (mem_src, meta_src) = if target.starts_with("thumbv8m") {
        (
            "../mcxn947/layouts/flash/memory.x",
            "../mcxn947/layouts/flash/program_metadata.x",
        )
    } else {
        ("layouts/xip/memory.x", "layouts/xip/program_metadata.x")
    };

    // Stage target-appropriate memory.x and program_metadata.x to Cargo's output directory
    fs::copy(mem_src, out_dir.join("memory.x")).unwrap();
    fs::copy(meta_src, out_dir.join("program_metadata.x")).unwrap();

    // Propagate link search path transitively to whatever binary links against us
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=layouts/xip/memory.x");
    println!("cargo:rerun-if-changed=layouts/xip/program_metadata.x");
    println!("cargo:rerun-if-changed=../mcxn947/layouts/flash/memory.x");
    println!("cargo:rerun-if-changed=../mcxn947/layouts/flash/program_metadata.x");
}
