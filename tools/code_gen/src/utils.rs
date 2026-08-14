use std::path::PathBuf;

/// Searches upward from the current directory to locate the path of `controllers.toml`.
pub fn find_controllers_toml() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    loop {
        let toml_path = path.join("controller/controllers.toml");
        if toml_path.exists() {
            return toml_path;
        }
        let direct_toml_path = path.join("controllers.toml");
        if direct_toml_path.exists() {
            return direct_toml_path;
        }
        if !path.pop() {
            panic!(
                "Could not locate controllers.toml in current directory or any parent directories!"
            );
        }
    }
}

/// Searches upward from the current directory to locate the path of `shell.toml`.
pub fn find_shell_toml() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    loop {
        let toml_path = path.join("controller/shell.toml");
        if toml_path.exists() {
            return toml_path;
        }
        let direct_toml_path = path.join("shell.toml");
        if direct_toml_path.exists() {
            return direct_toml_path;
        }
        if !path.pop() {
            panic!("Could not locate shell.toml in current directory or any parent directories!");
        }
    }
}

/// Searches upward from the current directory to locate the path of `peripheral.toml`.
pub fn find_peripherals_toml() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    loop {
        let toml_path = path.join("peripheral/peripheral.toml");
        if toml_path.exists() {
            return toml_path;
        }
        let direct_toml_path = path.join("peripheral.toml");
        if direct_toml_path.exists() {
            return direct_toml_path;
        }
        if !path.pop() {
            panic!(
                "Could not locate peripheral.toml in current directory or any parent directories!"
            );
        }
    }
}
