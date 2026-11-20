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
        "rp2040_flash_2mb",
        "cat_detector_shell",
        false,
        platform::types::FirmwareTraceMode::None,
        4,
        4096,
        2048,
        0x10000000,
        0x101C0000,
        64 * 1024,
        0x101D0000,
        192 * 1024,
        0x101B0000,
        64 * 1024,
    );
    let release_writer = platform::types::ProjectMetadata::serialize(
        "rp2040",
        "rp2040_flash_2mb",
        "cat_detector_shell",
        true,
        platform::types::FirmwareTraceMode::None,
        4,
        4096,
        2048,
        0x10000000,
        0x101C0000,
        64 * 1024,
        0x101D0000,
        192 * 1024,
        0x101B0000,
        64 * 1024,
    );
    let perf_writer = platform::types::ProjectMetadata::serialize(
        "rp2040",
        "rp2040_flash_2mb",
        "cat_detector_shell",
        true,
        platform::types::FirmwareTraceMode::Perf,
        4,
        4096,
        2048,
        0x10000000,
        0x101C0000,
        64 * 1024,
        0x101D0000,
        192 * 1024,
        0x101B0000,
        64 * 1024,
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
    assert_eq!(info.board_type, "rp2040_flash_2mb");
    assert_eq!(info.bin_name.as_deref(), Some("cat_detector_shell"));
    assert_eq!(info.release_build, Some(false));
    assert_eq!(info.trace_mode.as_deref(), Some("none"));
    assert_eq!(
        writer.len, release_writer.len,
        "1-byte CBOR boolean release_build must have identical compact length for debug & release"
    );
    assert!(
        release_writer.len <= 98,
        "Expected compact .program_metadata <= 98 bytes after release_build: Option<bool>, got {}",
        release_writer.len
    );
    let release_info = tool_common::ProjectInfo::decode_bytes(
        &release_writer.buf[..release_writer.len],
        0,
        release_writer.len,
    )
    .unwrap();
    assert_eq!(release_info.release_build, Some(true));
    assert_eq!(release_info.trace_mode.as_deref(), Some("none"));
    assert_ne!(
        info.checksum, release_info.checksum,
        "debug vs release builds must produce distinct .program_metadata CRCs"
    );
    let perf_info = tool_common::ProjectInfo::decode_bytes(
        &perf_writer.buf[..perf_writer.len],
        0,
        perf_writer.len,
    )
    .unwrap();
    assert_eq!(perf_info.trace_mode.as_deref(), Some("perf"));
    assert_ne!(
        release_info.checksum, perf_info.checksum,
        "different trace modes must produce distinct .program_metadata CRCs"
    );
    assert_eq!(info.flash_write_size, 4);
    assert_eq!(info.flash_erase_size, 4096);
    assert_eq!(info.stack_scan_limit, 2048);
    assert_eq!(info.flash_start, 0x10000000);
    assert_eq!(info.partitions.len(), 3);
    assert_eq!(info.partitions[0].kind, 0);
    assert_eq!(info.partitions[0].address, 0x101C0000);
    assert_eq!(info.partitions[0].size, 64 * 1024);
    assert_eq!(info.partitions[1].kind, 1);
    assert_eq!(info.partitions[1].address, 0x101D0000);
    assert_eq!(info.partitions[1].size, 192 * 1024);
    assert_eq!(info.partitions[2].kind, 2);
    assert_eq!(info.partitions[2].address, 0x101B0000);
    assert_eq!(info.partitions[2].size, 64 * 1024);
    assert_ne!(info.checksum, 0);
    assert_eq!(info.metadata_partition().unwrap().address, 0x101B0000);
    assert_eq!(info.metadata_partition().unwrap().size, 64 * 1024);

    // Corrupting a single byte must fail 16-bit CRC-16 verification
    let mut corrupted = writer.buf[..len].to_vec();
    corrupted[len - 1] ^= 0x01;
    let err = tool_common::ProjectInfo::decode_bytes(&corrupted, 0, len).unwrap_err();
    assert!(
        err.contains("checksum mismatch"),
        "Unexpected error: {}",
        err
    );

    let in_use_report = tool_common::InterfaceStatusReport {
        interface: tool_common::TargetInterfaceKind::SwdUsb,
        availability: tool_common::InterfaceAvailability::InUse,
        devices: vec![tool_common::EnumeratedInterfaceDevice {
            identifier: "J-Link".to_string(),
            vid_pid: Some("1366:0105".to_string()),
            serial_number: Some("000123456".to_string()),
        }],
        error: Some("USB probe 'J-Link' is currently in use".to_string()),
    };
    let in_use_json = in_use_report.to_json();
    assert!(in_use_json.contains("\"interface\":\"swd_usb\""));
    assert!(in_use_json.contains("\"availability\":\"in_use\""));
    assert!(in_use_json.contains("\"in_use\":true"));
    assert!(in_use_json.contains("\"retryable\":true"));
}
