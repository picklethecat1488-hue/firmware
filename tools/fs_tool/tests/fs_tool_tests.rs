use embedded_storage_async::nor_flash::{NorFlash, ReadNorFlash};
use fs_tool::{string_to_key, DataType, HostFlash};

#[test]
fn test_string_to_key() {
    // Exact 32 bytes
    let key32 = string_to_key("abcdefghijklmnopqrstuvwxyz123456");
    assert_eq!(&key32[..], b"abcdefghijklmnopqrstuvwxyz123456");

    // Short string (should pad with 0)
    let key_short = string_to_key("hello");
    assert_eq!(&key_short[..5], b"hello");
    assert_eq!(key_short[5], 0);

    // Truncated string
    let key_long = string_to_key("abcdefghijklmnopqrstuvwxyz123456789");
    assert_eq!(&key_long[..], b"abcdefghijklmnopqrstuvwxyz123456");
}

#[test]
fn test_data_type_decoding() {
    assert_eq!(
        DataType::from_filename("calibration.bin"),
        DataType::Calibration
    );
    assert_eq!(
        DataType::from_filename("telemetry.rrd"),
        DataType::Telemetry
    );
    assert_eq!(DataType::from_filename("crash.log"), DataType::CrashLog);
    assert_eq!(DataType::from_filename("unknown.data"), DataType::Unknown);

    assert_eq!(DataType::Calibration.to_str(), "Protobuf Calibration Data");
    assert_eq!(DataType::Telemetry.to_str(), "CBOR Telemetry Data");
}

#[test]
fn test_host_flash_driver() {
    futures::executor::block_on(async {
        let mut flash = HostFlash::new(vec![0xFF; 2048]);
        assert_eq!(flash.capacity(), 2048);

        // Write some bytes
        let write_data = [0xAA, 0xBB, 0xCC, 0xDD];
        assert!(flash.write(1024, &write_data).await.is_ok());

        // Read back
        let mut read_data = [0u8; 4];
        assert!(flash.read(1024, &mut read_data).await.is_ok());
        assert_eq!(read_data, write_data);

        // Erase sector
        assert!(flash.erase(1024, 2048).await.is_ok());
        assert!(flash.read(1024, &mut read_data).await.is_ok());
        assert_eq!(read_data, [0xFF; 4]);

        // Out of bounds checks
        assert!(flash.read(2047, &mut read_data).await.is_err());
        assert!(flash.write(2047, &write_data).await.is_err());
    });
}

#[test]
fn test_cli_cp() {
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    let bin_path = env!("CARGO_BIN_EXE_fs_tool");

    // Clean up from previous run just in case
    let dump_path = std::env::temp_dir().join("test_cli_cp_flash_dump.bin");
    let src_path = std::env::temp_dir().join("test_cli_cp_source.txt");
    let dest_path = std::env::temp_dir().join("test_cli_cp_dest.txt");
    let _ = std::fs::remove_file(&dump_path);
    let _ = std::fs::remove_file(&src_path);
    let _ = std::fs::remove_file(&dest_path);

    // 1. Create a dummy flash dump file of 256KB initialized to 0xFF
    let mut dump_file = File::create(&dump_path).unwrap();
    dump_file.write_all(&vec![0xFF; 262144]).unwrap();
    drop(dump_file);

    // 2. Create a dummy local host file to copy from
    let mut src_file = File::create(&src_path).unwrap();
    src_file.write_all(b"Hello from host!").unwrap();
    drop(src_file);

    // 3. Run cp command: copy from host to device
    let status = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("cp")
        .arg(&src_path)
        .arg("dev:test_file.txt")
        .status()
        .unwrap();
    assert!(status.success());

    // 4. Verify directory listing has the new file
    let output = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("ls")
        .output()
        .unwrap();
    assert!(output.status.success());
    let ls_text = String::from_utf8(output.stdout).unwrap();
    assert!(ls_text.contains("test_file.txt"));

    // 5. Run cp command: copy from device back to host
    let status2 = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("cp")
        .arg("dev:test_file.txt")
        .arg(&dest_path)
        .status()
        .unwrap();
    assert!(status2.success());

    // 6. Verify dest file matches source content
    let dest_content = std::fs::read(&dest_path).unwrap();
    assert_eq!(dest_content, b"Hello from host!");

    // Clean up files
    let _ = std::fs::remove_file(&dump_path);
    let _ = std::fs::remove_file(&src_path);
    let _ = std::fs::remove_file(&dest_path);
}

#[test]
fn test_decode_all_projects() {
    let mut dir = std::env::current_dir().unwrap();
    let mut projects_path = None;
    for _ in 0..5 {
        let candidate = dir.join("projects");
        if candidate.is_dir() {
            projects_path = Some(candidate);
            break;
        }
        if let Some(parent) = dir.parent() {
            dir = parent.to_path_buf();
        } else {
            break;
        }
    }

    let projects_path = projects_path.expect("Could not find projects/ directory");
    let mut tested_count = 0;

    for entry in std::fs::read_dir(projects_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let config_toml = path.join(".cargo/config.toml");
            let memory_x = path.join("memory.x");
            if config_toml.exists() && memory_x.exists() {
                let project_name = path.file_name().unwrap().to_str().unwrap();
                println!("Testing decode_project_info for '{}'...", project_name);

                let res = fs_tool::flash::decode_project_info(project_name);
                assert!(
                    res.is_ok(),
                    "Failed to decode project '{}': {:?}",
                    project_name,
                    res.err()
                );

                let (chip, base_addr, size) = res.unwrap();
                assert!(!chip.is_empty(), "Chip name should not be empty");
                assert!(base_addr > 0, "Base address should be positive");
                assert!(size > 0, "Size should be positive");

                tested_count += 1;
            }
        }
    }

    assert!(
        tested_count > 0,
        "Expected at least one valid firmware project to be tested"
    );
}
