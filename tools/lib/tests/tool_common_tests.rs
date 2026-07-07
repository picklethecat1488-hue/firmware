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
#[repr(C)]
pub struct ProjectMetadata {
    pub magic: [u8; 8],
    pub version: u32,
    pub chip: [u8; 32],
    pub partition_address: u32,
    pub partition_size: u32,
    pub flash_write_size: u32,
    pub flash_erase_size: u32,
    pub stack_scan_limit: u32,
}

#[used]
#[no_mangle]
pub static PROJECT_METADATA: ProjectMetadata = ProjectMetadata {
    magic: *b"PROJMET\0",
    version: 1,
    chip: [
        b'r', b'p', b'2', b'0', b'4', b'0', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    partition_address: 0x101C0000,
    partition_size: 256 * 1024,
    flash_write_size: 4,
    flash_erase_size: 4096,
    stack_scan_limit: 2048,
};

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
