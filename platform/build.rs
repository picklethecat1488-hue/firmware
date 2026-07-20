const LOG_LEVEL: &str = "trace";

fn main() {
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
