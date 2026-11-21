use embedded_storage_async::nor_flash::{NorFlash, ReadNorFlash};
use host_fs::{string_to_key, DataType, HostFlash};

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
        DataType::from_filename(model::telemetry::TELEMETRY_HEADER_FILE),
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

    let bin_path = env!("CARGO_BIN_EXE_host_fs");

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
fn test_crash_log_decoding_integration() {
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    let bin_path = env!("CARGO_BIN_EXE_host_fs");

    let dump_path = std::env::temp_dir().join("test_crash_log_flash_dump.bin");
    let _ = std::fs::remove_file(&dump_path);

    // 1. Create a dummy flash driver and sequential-storage partition
    let mut flash = HostFlash::new(vec![0xFF; 262144]); // 256KB partition
    let flash_range = 0..262144;
    let mut cache = sequential_storage::cache::NoCache::new();

    // 2. Synthesize a mock CrashDump payload
    let backtrace = [
        0x10000234, 0x10000456, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let dump = platform::types::CrashDump {
        revision_hash: "abcd123",
        system_logs: b"mock log data",
        uuid: [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc,
            0xde, 0xf0,
        ],
        cores: [
            platform::types::CoreDump {
                r0: 0x11111111,
                r1: 0x22222222,
                r2: 0x33333333,
                r3: 0x44444444,
                sp: 0x20042000,
                lr: 0x10000100,
                pc: 0x10000200,
                backtrace,
                backtrace_len: 2,
                panicked: true,
            },
            platform::types::CoreDump {
                r0: 0,
                r1: 0,
                r2: 0,
                r3: 0,
                sp: 0,
                lr: 0,
                pc: 0,
                backtrace: [0u32; 32],
                backtrace_len: 0,
                panicked: false,
            },
        ],
    };

    // Serialize it via the shared panic_handler serialization logic
    let mut cbor_buf = vec![0u8; 1024];
    let encoded_len = platform::panic_handler::serialize_crash_dump(&dump, &mut cbor_buf).unwrap();
    let encoded_bytes = &cbor_buf[..encoded_len];

    // 3. Write items to the flash
    futures::executor::block_on(async {
        // Write the crash_0.cbor file
        let file_key = string_to_key("crash_0.cbor");
        sequential_storage::map::store_item(
            &mut flash,
            flash_range.clone(),
            &mut cache,
            &mut [0u8; 1024],
            &file_key,
            &encoded_bytes,
        )
        .await
        .unwrap();

        // Write the .dir file listing "crash_0.cbor"
        let dir_key = string_to_key(".dir");
        sequential_storage::map::store_item(
            &mut flash,
            flash_range.clone(),
            &mut cache,
            &mut [0u8; 1024],
            &dir_key,
            &"crash_0.cbor".as_bytes(),
        )
        .await
        .unwrap();
    });

    // 4. Save host flash bytes to the temporary file
    let mut dump_file = File::create(&dump_path).unwrap();
    dump_file.write_all(&flash.data).unwrap();
    drop(dump_file);

    // 5. Run the crash-log command using the CLI binary
    let output = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("crash-log")
        .output()
        .unwrap();

    let stdout_text = String::from_utf8(output.stdout).unwrap();
    let stderr_text = String::from_utf8(output.stderr).unwrap();
    println!("stdout: {}", stdout_text);
    println!("stderr: {}", stderr_text);

    assert!(output.status.success());

    // 6. Verify output fields contain our synthesized data
    assert!(stdout_text.contains("--- PANIC (CBOR) ---"));
    assert!(stdout_text.contains("UUID: 12345678-9abc-def0-1234-56789abcdef0"));
    assert!(stdout_text.contains("Revision Hash: abcd123"));
    assert!(stdout_text.contains("Registers:"));
    assert!(stdout_text.contains("R0: 0x11111111"));
    assert!(stdout_text.contains("R1: 0x22222222"));
    assert!(stdout_text.contains("R2: 0x33333333"));
    assert!(stdout_text.contains("R3: 0x44444444"));
    assert!(stdout_text.contains("Backtrace:"));
    assert!(stdout_text.contains("0x10000234"));
    assert!(stdout_text.contains("0x10000456"));
    assert!(stdout_text.contains("Crash Context System Logs:"));
    assert!(stdout_text.contains("No defmt table loaded to decode system logs"));

    // Clean up
    let _ = std::fs::remove_file(&dump_path);
}

#[test]
fn test_cli_rm() {
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    let bin_path = env!("CARGO_BIN_EXE_host_fs");

    let dump_path = std::env::temp_dir().join("test_cli_rm_flash_dump.bin");
    let src1_path = std::env::temp_dir().join("test_cli_rm_src1.txt");
    let src2_path = std::env::temp_dir().join("test_cli_rm_src2.txt");
    let _ = std::fs::remove_file(&dump_path);
    let _ = std::fs::remove_file(&src1_path);
    let _ = std::fs::remove_file(&src2_path);

    // 1. Create a dummy flash dump file of 256KB
    let mut dump_file = File::create(&dump_path).unwrap();
    dump_file.write_all(&vec![0xFF; 262144]).unwrap();
    drop(dump_file);

    // 2. Create two dummy local files
    File::create(&src1_path)
        .unwrap()
        .write_all(b"File 1 content")
        .unwrap();
    File::create(&src2_path)
        .unwrap()
        .write_all(b"File 2 content")
        .unwrap();

    // 3. Copy both to device
    let status = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("cp")
        .arg(&src1_path)
        .arg("dev:file1.txt")
        .status()
        .unwrap();
    assert!(status.success());

    let status = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("cp")
        .arg(&src2_path)
        .arg("dev:file2.txt")
        .status()
        .unwrap();
    assert!(status.success());

    // 4. Verify directory listing lists both
    let output = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("ls")
        .output()
        .unwrap();
    assert!(output.status.success());
    let ls_text = String::from_utf8(output.stdout).unwrap();
    assert!(ls_text.contains("file1.txt"));
    assert!(ls_text.contains("file2.txt"));

    // 5. Remove file1.txt
    let status = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("rm")
        .arg("file1.txt")
        .status()
        .unwrap();
    assert!(status.success());

    // 6. Verify directory listing only lists file2.txt
    let output = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("ls")
        .output()
        .unwrap();
    assert!(output.status.success());
    let ls_text = String::from_utf8(output.stdout).unwrap();
    assert!(!ls_text.contains("file1.txt"));
    assert!(ls_text.contains("file2.txt"));

    // 7. Clear all remaining files
    let status = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("rm")
        .status()
        .unwrap();
    assert!(status.success());

    // 8. Verify directory empty
    let output = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("ls")
        .output()
        .unwrap();
    assert!(output.status.success());
    let ls_text = String::from_utf8(output.stdout).unwrap();
    assert!(ls_text.contains("No files found"));

    // Clean up
    let _ = std::fs::remove_file(&dump_path);
    let _ = std::fs::remove_file(&src1_path);
    let _ = std::fs::remove_file(&src2_path);
}

#[test]
fn test_cli_rm_small_dump() {
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    let bin_path = env!("CARGO_BIN_EXE_host_fs");

    let dump_path = std::env::temp_dir().join("test_cli_rm_small_dump_flash_dump.bin");
    let src_path = std::env::temp_dir().join("test_cli_rm_small_dump_src.txt");
    let _ = std::fs::remove_file(&dump_path);
    let _ = std::fs::remove_file(&src_path);

    // 1. Create a dummy flash dump file of 8KB (exactly two 4KB sectors)
    let mut dump_file = File::create(&dump_path).unwrap();
    dump_file.write_all(&vec![0xFF; 8192]).unwrap();
    drop(dump_file);

    // 2. Create a source file to copy
    let mut src_file = File::create(&src_path).unwrap();
    src_file
        .write_all(b"Hello from small buffer test!")
        .unwrap();
    drop(src_file);

    // 3. Copy file to device
    let status = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("cp")
        .arg(&src_path)
        .arg("dev:test_small.txt")
        .status()
        .unwrap();
    assert!(status.success(), "Failed to copy file into small dump");

    // 4. Remove file from device
    let status = Command::new(bin_path)
        .arg("--dump")
        .arg(&dump_path)
        .arg("rm")
        .arg("test_small.txt")
        .status()
        .unwrap();
    assert!(status.success(), "Failed to remove file from small dump");

    // Clean up
    let _ = std::fs::remove_file(&dump_path);
    let _ = std::fs::remove_file(&src_path);
}

#[test]
fn test_query_metadata_elf_geometry_checksum_and_metadata_override() {
    use std::process::Command;

    let bin_path = env!("CARGO_BIN_EXE_host_fs");
    let temp_dir = std::env::temp_dir().join("test_query_metadata_elf_geometry_ws");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    let rp_writer = platform::types::ProjectMetadata::serialize(
        "rp2040",
        "rp2040_flash_2mb",
        "cat_detector_shell",
        true,
        platform::types::FirmwareTraceMode::Sensor,
        1,
        4096,
        2048,
        0x1000_0000,
        0x101C_0000,
        64 * 1024,
        0x101D_0000,
        192 * 1024,
        0x101B_0000,
        64 * 1024,
    );

    let src_path = temp_dir.join("dummy_elf.rs");
    let elf_bin_path = temp_dir.join("dummy_elf_bin");
    let mut bytes_str = String::new();
    for i in 0..rp_writer.len {
        bytes_str.push_str(&format!("0x{:02x}, ", rp_writer.buf[i]));
    }
    std::fs::write(
        &src_path,
        format!(
            "#[used]\n#[no_mangle]\npub static PROJECT_METADATA: [u8; {}] = [{}];\nfn main() {{}}\n",
            rp_writer.len, bytes_str
        ),
    )
    .unwrap();
    let status = Command::new("rustc")
        .arg(&src_path)
        .arg("-o")
        .arg(&elf_bin_path)
        .status()
        .unwrap();
    assert!(status.success());

    let elf_info = tool_common::autodetect_project_info(&elf_bin_path).unwrap();
    let standalone_meta_path = temp_dir.join("standalone_prog_meta.bin");
    std::fs::write(&standalone_meta_path, &rp_writer.buf[..rp_writer.len]).unwrap();

    // 1. Verify `--elf-only` dumps the entire target ELF `.program_metadata` specification
    let elf_only_out = Command::new(bin_path)
        .args([
            "--elf",
            elf_bin_path.to_str().unwrap(),
            "query_metadata",
            "--elf-only",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(elf_only_out.status.success());
    let elf_stdout = String::from_utf8(elf_only_out.stdout).unwrap();
    assert!(elf_stdout.contains("\"board_type\":\"rp2040_flash_2mb\""));
    assert!(elf_stdout.contains("\"source\":\"elf\""));
    assert!(elf_stdout.contains("\"partitions\":["));

    // 2. Verify standalone `.program_metadata` dump validates 16-bit CRC-16 and dumps full section
    let out = Command::new(bin_path)
        .args([
            "--elf",
            elf_bin_path.to_str().unwrap(),
            "--dump",
            standalone_meta_path.to_str().unwrap(),
            "query_metadata",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "query_metadata failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("\"board_type\":\"rp2040_flash_2mb\""));
    assert!(stdout.contains("\"chip\":\"rp2040\""));
    assert!(stdout.contains("\"bin_name\":\"cat_detector_shell\""));
    assert!(stdout.contains("\"release_build\":true"));
    assert!(stdout.contains("\"trace_mode\":\"sensor\""));
    assert!(stdout.contains("\"source\":\"program_metadata\""));
    assert!(stdout.contains("\"stack_scan_limit\":2048"));
    assert!(stdout.contains(&format!("\"0x{:04X}\"", elf_info.checksum)));

    // 3. Provision an explicit `"board_type"` key in a 64 KB `metadata` map partition dump
    // and verify it overrides compiled `.program_metadata`!
    let meta_part_dump_path = temp_dir.join("metadata_part_dump.bin");
    futures::executor::block_on(async {
        let mut meta_flash = HostFlash::new(vec![0xFFu8; 64 * 1024]);
        let mut cache = sequential_storage::cache::NoCache::new();
        let mut scratch = vec![0u8; 512];
        let key = string_to_key("board_type");
        let board_val: &[u8] = b"rp2040_flash_4mb";
        sequential_storage::map::store_item::<[u8; 32], &[u8], _>(
            &mut meta_flash,
            0..(64 * 1024),
            &mut cache,
            &mut scratch,
            &key,
            &board_val,
        )
        .await
        .unwrap();
        std::fs::write(&meta_part_dump_path, &meta_flash.data).unwrap();
    });

    let out_override = Command::new(bin_path)
        .args([
            "--elf",
            elf_bin_path.to_str().unwrap(),
            "--dump",
            meta_part_dump_path.to_str().unwrap(),
            "query_metadata",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(out_override.status.success());
    let stdout_override = String::from_utf8(out_override.stdout).unwrap();
    assert!(stdout_override.contains("\"board_type\":\"rp2040_flash_4mb\""));
    assert!(stdout_override.contains("\"program_metadata_board_type\":\"rp2040_flash_2mb\""));
    assert!(stdout_override.contains("\"source\":\"metadata_partition\""));

    // 4. Verify Carrier Board standalone `.program_metadata` human-readable full section dump
    let carrier_dump_path = temp_dir.join("carrier_prog_meta.bin");
    let cb_writer = platform::types::ProjectMetadata::serialize(
        "MCXN947",
        "carrier_board",
        "carrier_board_shell",
        false,
        platform::types::FirmwareTraceMode::Perf,
        128,
        8192,
        2048,
        0x0000_0000,
        0x001E_8000,
        32 * 1024,
        0x001F_0000,
        64 * 1024,
        0x001E_0000,
        32 * 1024,
    );
    std::fs::write(&carrier_dump_path, &cb_writer.buf[..cb_writer.len]).unwrap();

    let out_cb = Command::new(bin_path)
        .args([
            "--dump",
            carrier_dump_path.to_str().unwrap(),
            "query_metadata",
        ])
        .output()
        .unwrap();
    assert!(out_cb.status.success());
    let stdout_cb = String::from_utf8(out_cb.stdout).unwrap();
    assert!(stdout_cb.contains("board_type: carrier_board"));
    assert!(stdout_cb.contains("chip: MCXN947"));
    assert!(stdout_cb.contains("bin_name: carrier_board_shell"));
    assert!(stdout_cb.contains("release_build: false"));
    assert!(stdout_cb.contains("trace_mode: perf"));
    assert!(stdout_cb.contains("partition[2] (metadata): address=0x001E0000 size=32768"));

    let _ = std::fs::remove_dir_all(&temp_dir);
}
