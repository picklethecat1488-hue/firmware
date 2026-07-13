use std::fs;
use std::process::Command;
use tool_common::autodetect_project_info;

#[test]
fn test_autodetect_project_info() {
    let temp_dir = std::env::temp_dir().join("autodetect_test_workspace");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let src_path = temp_dir.join("dummy.rs");
    let bin_path = temp_dir.join("dummy_bin");

    fs::write(
        &src_path,
        r#"
#[used]
#[no_mangle]
pub static PROJECT_METADATA: [u8; 25] = [
    0x86, 0x66, 0x72, 0x70, 0x32, 0x30, 0x34, 0x30, 0x1a, 0x10, 0x1c, 0x00,
    0x00, 0x1a, 0x00, 0x04, 0x00, 0x00, 0x04, 0x19, 0x10, 0x00, 0x19, 0x08,
    0x00
];

fn main() {}
"#,
    )
    .unwrap();

    let status = Command::new("rustc")
        .arg(&src_path)
        .arg("-o")
        .arg(&bin_path)
        .status()
        .expect("Failed to execute rustc compiler");

    assert!(status.success(), "Dummy compilation failed");

    let res = autodetect_project_info(&bin_path);

    // Clean up before asserting so we don't leave temp files behind
    let _ = fs::remove_dir_all(&temp_dir);

    assert!(
        res.is_ok(),
        "Failed to autodetect metadata: {:?}",
        res.err()
    );
    let info = res.unwrap();
    assert_eq!(info.chip, "rp2040");
    assert_eq!(info.partition_address, 0x101C0000);
    assert_eq!(info.partition_size, 256 * 1024);
    assert_eq!(info.flash_write_size, 4);
    assert_eq!(info.flash_erase_size, 4096);
    assert_eq!(info.stack_scan_limit, 2048);
}
