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

    let writer = platform::types::ProjectMetadata::serialize(
        "rp2040",
        4,
        4096,
        2048,
        0x10000000,
        0x101C0000,
        64 * 1024,
        0x101D0000,
        192 * 1024,
    );
    let len = writer.len;
    let mut bytes_str = String::new();
    for i in 0..len {
        bytes_str.push_str(&format!("0x{:02x}, ", writer.buf[i]));
    }

    fs::write(
        &src_path,
        format!(
            r#"
#[used]
#[no_mangle]
pub static PROJECT_METADATA: [u8; {}] = [
    {}
];

fn main() {{}}
"#,
            len, bytes_str
        ),
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
    assert_eq!(info.flash_write_size, 4);
    assert_eq!(info.flash_erase_size, 4096);
    assert_eq!(info.stack_scan_limit, 2048);
    assert_eq!(info.flash_start, 0x10000000);
    assert_eq!(info.partitions.len(), 2);
    assert_eq!(info.partitions[0].kind, 0);
    assert_eq!(info.partitions[0].address, 0x101C0000);
    assert_eq!(info.partitions[0].size, 64 * 1024);
    assert_eq!(info.partitions[1].kind, 1);
    assert_eq!(info.partitions[1].address, 0x101D0000);
    assert_eq!(info.partitions[1].size, 192 * 1024);
}
