use std::process::Command;

fn main() {
    // Retrieve short git commit hash; fallback to package version if git is unavailable
    let git_hash = Command::new("git")
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
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());

    // Tell cargo to set the GIT_HASH environment variable for compile time
    println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
}
