use std::fs;
use tool_common::decode_project_info;

#[test]
fn test_decode_project_info_mock() {
    let temp_dir = std::env::temp_dir().join("tool_common_test_workspace");
    let _ = fs::remove_dir_all(&temp_dir);
    let projects_dir = temp_dir.join("projects");
    let proj_dir = projects_dir.join("dummy_project");
    let cargo_dir = proj_dir.join(".cargo");
    fs::create_dir_all(&cargo_dir).unwrap();

    let config_path = cargo_dir.join("config.toml");
    fs::write(
        &config_path,
        r#"
[target.thumbv6m-none-eabi]
runner = "probe-rs run --chip RP2040"
"#,
    )
    .unwrap();

    let memory_path = proj_dir.join("memory.x");
    fs::write(
        &memory_path,
        r#"
MEMORY
{
  FLASH : ORIGIN = 0x10000000, LENGTH = 1024K
}
"#,
    )
    .unwrap();

    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    let res = decode_project_info("dummy_project");
    std::env::set_current_dir(old_dir).unwrap();
    let _ = fs::remove_dir_all(&temp_dir);

    let (chip, partition_addr, partition_size) = res.unwrap();
    assert_eq!(chip, "RP2040");
    assert_eq!(partition_addr, 0x10100000);
    assert_eq!(partition_size, 1024 * 1024);
}
