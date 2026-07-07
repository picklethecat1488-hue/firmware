use host_cli::{dump_logs, stream_logs, DefmtLogSource};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

struct MockLogSource {
    data: Vec<u8>,
    read_index: usize,
}

impl DefmtLogSource for MockLogSource {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        if self.read_index >= self.data.len() {
            return Ok(0);
        }
        let chunk_size = std::cmp::min(buf.len(), self.data.len() - self.read_index);
        buf[..chunk_size]
            .copy_from_slice(&self.data[self.read_index..self.read_index + chunk_size]);
        self.read_index += chunk_size;
        Ok(chunk_size)
    }
}

#[test]
fn test_stream_logs_poll_loop() {
    let source = MockLogSource {
        data: vec![],
        read_index: 0,
    };

    let elf_candidates = [
        "target/thumbv6m-none-eabi/debug/cat_detector/app",
        "target/thumbv6m-none-eabi/release/cat_detector/app",
        "../../target/thumbv6m-none-eabi/debug/cat_detector/app",
    ];
    let mut elf_path = None;
    for &c in &elf_candidates {
        if PathBuf::from(c).is_file() {
            elf_path = Some(PathBuf::from(c));
            break;
        }
    }

    if let Some(path) = elf_path {
        let elf_data = std::fs::read(path).unwrap();
        if let Ok(Some(table)) = defmt_decoder::Table::parse(&elf_data) {
            let mut writer = Vec::new();
            let res = stream_logs(
                source,
                &table,
                &mut writer,
                Duration::from_millis(1),
                || true, // exit immediately
            );
            assert!(res.is_ok());
            assert!(writer.is_empty());
        }
    }
}

#[test]
fn test_cli_argument_validation() {
    let bin_path = env!("CARGO_BIN_EXE_host_cli");

    // 1. Non-existent ELF file (failing to load metadata / autodetect chip)
    let output = Command::new(bin_path)
        .arg("--elf")
        .arg("non_existent_file.elf")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Failed to read ELF file") || stderr.contains("error"));
}

#[test]
fn test_dump_logs_empty() {
    let source = MockLogSource {
        data: vec![],
        read_index: 0,
    };

    let elf_candidates = [
        "target/thumbv6m-none-eabi/debug/cat_detector/app",
        "target/thumbv6m-none-eabi/release/cat_detector/app",
        "../../target/thumbv6m-none-eabi/debug/cat_detector/app",
    ];
    let mut elf_path = None;
    for &c in &elf_candidates {
        if PathBuf::from(c).is_file() {
            elf_path = Some(PathBuf::from(c));
            break;
        }
    }

    if let Some(path) = elf_path {
        let elf_data = std::fs::read(path).unwrap();
        if let Ok(Some(table)) = defmt_decoder::Table::parse(&elf_data) {
            let mut writer = Vec::new();
            let res = dump_logs(source, &table, &mut writer);
            assert!(res.is_ok());
            assert!(writer.is_empty());
        }
    }
}
