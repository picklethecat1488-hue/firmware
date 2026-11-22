//! Read and dump the entire `.program_metadata` section (and provisioned `metadata`
//! partition overrides) to stdout using section/partition geometry from the target ELF.

use crate::flash::{read_probe_memory_fast, HostFlash};
use crate::string_to_key;
use probe_rs::probe::list::Lister;
use std::io;
use std::path::Path;
use tool_common::metadata::PartitionInfo;
use tool_common::{
    autodetect_project_info, discover_sibling_elf_tables, normalize_rtt_bridge_addr,
    DeferredElfTable, EnumeratedInterfaceDevice, InterfaceAvailability, InterfaceStatusReport,
    ProjectInfo, TargetInterfaceKind, DEFAULT_SWD_SPEED_KHZ,
};

/// Map key inside the `metadata` partition that can override compiled `board_type`.
pub const BOARD_TYPE_MAP_KEY: &str = "board_type";

/// Partition kind discriminant for the `fs` (`sequential_storage::map`) partition.
const FS_PARTITION_KIND: u32 = 0;

/// Partition kind discriminant for the `telemetry` (`sequential_storage::queue`) partition.
const TELEMETRY_PARTITION_KIND: u32 = 1;

/// Partition kind discriminant for the `metadata` (`sequential_storage::map`) partition.
const METADATA_PARTITION_KIND: u32 = 2;

/// Maximum serialized compact `.program_metadata` read window in bytes (`16` bytes = 4 AHB-AP words).
const MAX_PROGRAM_METADATA_READ_LEN: usize = platform::types::COMPACT_PROGRAM_METADATA_CAP;

/// Complete decoded `.program_metadata` dump returned by `host_fs query_metadata`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueriedProgramMetadata {
    /// Decoded `.program_metadata` payload (checksum, chip, flash parameters, and partitions).
    pub info: ProjectInfo,
    /// Effective `board_type` (`provisioned_board_type` if set in `metadata`, else `info.board_type`).
    pub effective_board_type: String,
    /// Optional provisioned `"board_type"` value read from the `metadata` map partition.
    pub provisioned_board_type: Option<String>,
    /// Source of `effective_board_type` (`"program_metadata"`, `"metadata_partition"`, or `"elf"`).
    pub source: &'static str,
}

/// Map partition kind discriminant to its canonical human-readable partition name.
fn partition_kind_name(kind: u32) -> &'static str {
    match kind {
        FS_PARTITION_KIND => "fs",
        TELEMETRY_PARTITION_KIND => "telemetry",
        METADATA_PARTITION_KIND => "metadata",
        _ => "unknown",
    }
}

/// Escape double quotes and backslashes for JSON string serialization.
fn json_escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

/// Format a `PartitionInfo` entry as a JSON object string.
fn format_partition_json(part: &PartitionInfo) -> String {
    format!(
        "{{\"kind\":{},\"name\":\"{}\",\"address\":{},\"size\":{}}}",
        part.kind,
        partition_kind_name(part.kind),
        part.address,
        part.size
    )
}

impl QueriedProgramMetadata {
    /// Serialize the entire decoded program metadata section as a single-line JSON object.
    pub fn to_json(&self) -> String {
        let provisioned_json = match &self.provisioned_board_type {
            Some(val) => format!("\"{}\"", json_escape(val)),
            None => "null".to_string(),
        };
        let bin_json = match &self.info.bin_name {
            Some(val) => format!("\"{}\"", json_escape(val)),
            None => "null".to_string(),
        };
        let release_json = match self.info.release_build {
            Some(true) => "true",
            Some(false) => "false",
            None => "null",
        };
        let trace_mode_json = match &self.info.trace_mode {
            Some(val) => format!("\"{}\"", json_escape(val)),
            None => "null".to_string(),
        };
        let parts_json: Vec<String> = self
            .info
            .partitions
            .iter()
            .map(format_partition_json)
            .collect();
        format!(
            "{{\"checksum\":\"0x{:04X}\",\"chip\":\"{}\",\"board_type\":\"{}\",\
             \"program_metadata_board_type\":\"{}\",\"provisioned_board_type\":{},\
             \"bin_name\":{},\"release_build\":{},\"trace_mode\":{},\
             \"source\":\"{}\",\"flash_start\":{},\"flash_write_size\":{},\
             \"flash_erase_size\":{},\"stack_scan_limit\":{},\
             \"program_metadata_addr\":{},\"program_metadata_size\":{},\
             \"partitions\":[{}]}}",
            self.info.checksum,
            json_escape(&self.info.chip),
            json_escape(&self.effective_board_type),
            json_escape(&self.info.board_type),
            provisioned_json,
            bin_json,
            release_json,
            trace_mode_json,
            self.source,
            self.info.flash_start,
            self.info.flash_write_size,
            self.info.flash_erase_size,
            self.info.stack_scan_limit,
            self.info.program_metadata_addr,
            self.info.program_metadata_size,
            parts_json.join(",")
        )
    }

    /// Format the entire decoded program metadata section as human-readable key-value lines.
    pub fn to_text(&self) -> String {
        let mut lines = vec![
            format!("checksum: 0x{:04X}", self.info.checksum),
            format!("chip: {}", self.info.chip),
            format!("board_type: {}", self.effective_board_type),
            format!("program_metadata_board_type: {}", self.info.board_type),
        ];
        if let Some(ref bin) = self.info.bin_name {
            lines.push(format!("bin_name: {}", bin));
        }
        if let Some(rel) = self.info.release_build {
            lines.push(format!("release_build: {}", rel));
        }
        if let Some(ref tmode) = self.info.trace_mode {
            lines.push(format!("trace_mode: {}", tmode));
        }
        if let Some(ref prov) = self.provisioned_board_type {
            lines.push(format!("provisioned_board_type: {}", prov));
        }
        lines.push(format!("source: {}", self.source));
        lines.push(format!("flash_start: 0x{:08X}", self.info.flash_start));
        lines.push(format!("flash_write_size: {}", self.info.flash_write_size));
        lines.push(format!("flash_erase_size: {}", self.info.flash_erase_size));
        lines.push(format!("stack_scan_limit: {}", self.info.stack_scan_limit));
        lines.push(format!(
            "program_metadata_addr: 0x{:08X}",
            self.info.program_metadata_addr
        ));
        lines.push(format!(
            "program_metadata_size: {}",
            self.info.program_metadata_size
        ));
        for (idx, part) in self.info.partitions.iter().enumerate() {
            lines.push(format!(
                "partition[{}] ({}): address=0x{:08X} size={}",
                idx,
                partition_kind_name(part.kind),
                part.address,
                part.size
            ));
        }
        lines.join("\n")
    }
}

/// Attempt to fetch a provisioned `"board_type"` item from a `metadata` map partition slice
/// whose erase geometry matches `erase_size`.
async fn read_board_type_from_metadata_slice(slice: &[u8], erase_size: usize) -> Option<String> {
    if erase_size == 0 || slice.len() < erase_size.saturating_mul(2) {
        return None;
    }
    if slice.iter().all(|&b| b == 0xFF) {
        return None;
    }
    let cap = slice.len() as u32;
    let mut flash = HostFlash::new(slice.to_vec());
    let mut cache = sequential_storage::cache::NoCache::new();
    let mut scratch = vec![0u8; 512];
    let key = string_to_key(BOARD_TYPE_MAP_KEY);
    if let Ok(Some(raw_bytes)) = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
        &mut flash,
        0..cap,
        &mut cache,
        &mut scratch,
        &key,
    )
    .await
    {
        if let Ok(val) = std::str::from_utf8(raw_bytes) {
            let trimmed = val.trim_matches(char::from(0)).trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

/// Decode `.program_metadata` (and optional `metadata` partition override) from an offline dump.
pub async fn query_from_dump_bytes(
    dump: &[u8],
    elf_info: Option<&ProjectInfo>,
    sibling_tables: &[DeferredElfTable],
) -> Result<QueriedProgramMetadata, String> {
    let spec = elf_info.ok_or_else(|| {
        "Offline dump query_metadata requires --elf to deferred-load .program_metadata_elf strings and partition offsets"
            .to_string()
    })?;

    // 1. Standalone compact `.program_metadata` section dump starting at byte 0 (`<= 16` bytes)
    if dump.len() <= MAX_PROGRAM_METADATA_READ_LEN {
        if let Ok(info) = spec.decode_target_bytes(dump, sibling_tables) {
            let board_type = info.board_type.clone();
            return Ok(QueriedProgramMetadata {
                info,
                effective_board_type: board_type,
                provisioned_board_type: None,
                source: "program_metadata",
            });
        }
    }

    // 2. Standalone `metadata` partition dump matching the ELF's `metadata` partition size
    if let Some(meta_part) = spec.metadata_partition() {
        if dump.len() == meta_part.size {
            if let Some(override_bt) =
                read_board_type_from_metadata_slice(dump, spec.flash_erase_size as usize).await
            {
                return Ok(QueriedProgramMetadata {
                    info: spec.clone(),
                    effective_board_type: override_bt.clone(),
                    provisioned_board_type: Some(override_bt),
                    source: "metadata_partition",
                });
            }
        }
    }

    // 3. Full flash dump indexed strictly by ELF `.program_metadata` and `metadata` partition offsets
    let prog_offset =
        (spec.program_metadata_addr as usize).saturating_sub(spec.flash_start as usize);
    let prog_size = spec.program_metadata_size;
    if prog_size == 0 || dump.len() < prog_offset + prog_size {
        return Err(format!(
            "Dump length ({} bytes) is smaller than ELF .program_metadata range (offset 0x{:X}, size {})",
            dump.len(),
            prog_offset,
            prog_size
        ));
    }

    let dump_window = prog_size
        .max(MAX_PROGRAM_METADATA_READ_LEN)
        .min(dump.len().saturating_sub(prog_offset));
    let decoded_info = spec.decode_target_bytes(
        &dump[prog_offset..prog_offset + dump_window],
        sibling_tables,
    )?;

    if let Some(meta_part) = spec.metadata_partition() {
        let meta_offset = (meta_part.address as usize).saturating_sub(spec.flash_start as usize);
        if dump.len() >= meta_offset + meta_part.size {
            let part_slice = &dump[meta_offset..meta_offset + meta_part.size];
            if let Some(override_bt) =
                read_board_type_from_metadata_slice(part_slice, spec.flash_erase_size as usize)
                    .await
            {
                return Ok(QueriedProgramMetadata {
                    info: decoded_info,
                    effective_board_type: override_bt.clone(),
                    provisioned_board_type: Some(override_bt),
                    source: "metadata_partition",
                });
            }
        }
    }

    let board_type = decoded_info.board_type.clone();
    Ok(QueriedProgramMetadata {
        info: decoded_info,
        effective_board_type: board_type,
        provisioned_board_type: None,
        source: "program_metadata",
    })
}

/// Structured target metadata query failure carrying typed USB/SWD availability state.
#[derive(Debug, Clone)]
pub struct TargetQueryFailure {
    /// Structured interface status report (`Disconnected`, `InUse`, `TransientIo`, or `InvalidMetadata`).
    pub status: InterfaceStatusReport,
    /// Human-readable error message.
    pub message: String,
}

/// Read and decode compact `.program_metadata` (and optional `metadata` override) from a live target,
/// deferred-loading strings & partition geometry from `elf_info` / `sibling_tables`.
pub async fn query_from_target(
    elf_info: &ProjectInfo,
    sibling_tables: &[DeferredElfTable],
    gdb_bridge: Option<&str>,
) -> Result<QueriedProgramMetadata, TargetQueryFailure> {
    if elf_info.program_metadata_size == 0 {
        let msg = "ELF .program_metadata section size is zero".to_string();
        return Err(TargetQueryFailure {
            status: InterfaceStatusReport {
                interface: TargetInterfaceKind::SwdUsb,
                availability: InterfaceAvailability::InvalidMetadata,
                devices: Vec::new(),
                error: Some(msg.clone()),
            },
            message: msg,
        });
    }
    let prog_read_len = elf_info
        .program_metadata_size
        .max(MAX_PROGRAM_METADATA_READ_LEN);

    if let Some(host) = gdb_bridge {
        let addr = normalize_rtt_bridge_addr(host);
        let mut gdb = tool_common::GdbClient::connect(&addr).map_err(|msg| TargetQueryFailure {
            status: InterfaceStatusReport {
                interface: TargetInterfaceKind::SwdUsb,
                availability: InterfaceAvailability::TransientIo,
                devices: Vec::new(),
                error: Some(msg.clone()),
            },
            message: msg,
        })?;
        let mut meta_buf = vec![0u8; prog_read_len];
        gdb.read_mem(elf_info.program_metadata_addr, &mut meta_buf)
            .map_err(|msg| TargetQueryFailure {
                status: InterfaceStatusReport {
                    interface: TargetInterfaceKind::SwdUsb,
                    availability: InterfaceAvailability::TransientIo,
                    devices: Vec::new(),
                    error: Some(msg.clone()),
                },
                message: msg,
            })?;
        let decoded = elf_info
            .decode_target_bytes(&meta_buf, sibling_tables)
            .map_err(|e| {
                let msg = format!("AttachedTargetMetadataInvalid ({}): {}", elf_info.chip, e);
                TargetQueryFailure {
                    status: InterfaceStatusReport {
                        interface: TargetInterfaceKind::SwdUsb,
                        availability: InterfaceAvailability::InvalidMetadata,
                        devices: Vec::new(),
                        error: Some(msg.clone()),
                    },
                    message: msg,
                }
            })?;
        let bt = decoded.board_type.clone();
        return Ok(QueriedProgramMetadata {
            info: decoded,
            effective_board_type: bt,
            provisioned_board_type: None,
            source: "program_metadata",
        });
    }

    let lister = Lister::new();
    let probes = lister.list_all();
    let devices: Vec<EnumeratedInterfaceDevice> = probes
        .iter()
        .map(|info| EnumeratedInterfaceDevice {
            identifier: info.identifier.clone(),
            vid_pid: Some(format!("{:04x}:{:04x}", info.vendor_id, info.product_id)),
            serial_number: info.serial_number.clone(),
        })
        .collect();
    let probe_info = probes.first().ok_or_else(|| {
        let msg = "No debug probes connected".to_string();
        TargetQueryFailure {
            status: InterfaceStatusReport {
                interface: TargetInterfaceKind::SwdUsb,
                availability: InterfaceAvailability::Disconnected,
                devices: Vec::new(),
                error: Some(msg.clone()),
            },
            message: msg,
        }
    })?;
    let mut probe = probe_info.open().map_err(|e| {
        let msg = format!("Failed to open probe: {:?}", e);
        TargetQueryFailure {
            status: InterfaceStatusReport {
                interface: TargetInterfaceKind::SwdUsb,
                availability: InterfaceAvailability::InUse,
                devices: devices.clone(),
                error: Some(msg.clone()),
            },
            message: msg,
        }
    })?;
    let _ = probe.set_speed(DEFAULT_SWD_SPEED_KHZ);
    let mut session = probe
        .attach(&elf_info.chip, probe_rs::Permissions::default())
        .map_err(|e| {
            let msg = format!("Failed to attach as {}: {:?}", elf_info.chip, e);
            TargetQueryFailure {
                status: InterfaceStatusReport {
                    interface: TargetInterfaceKind::SwdUsb,
                    availability: InterfaceAvailability::TransientIo,
                    devices: devices.clone(),
                    error: Some(msg.clone()),
                },
                message: msg,
            }
        })?;

    let mut prog_meta_buf = vec![0u8; prog_read_len];
    let meta_part_spec = elf_info.metadata_partition().cloned();
    let mut meta_part_buf = vec![0u8; meta_part_spec.as_ref().map(|p| p.size).unwrap_or(0)];

    let (decoded, part_read_ok) = {
        let mut core = session.core(0).map_err(|e| {
            let msg = format!("Failed to access core 0 on {}: {:?}", elf_info.chip, e);
            TargetQueryFailure {
                status: InterfaceStatusReport {
                    interface: TargetInterfaceKind::SwdUsb,
                    availability: InterfaceAvailability::TransientIo,
                    devices: devices.clone(),
                    error: Some(msg.clone()),
                },
                message: msg,
            }
        })?;
        // Read non-intrusively over AHB-AP without halting Core 0 so an active
        // SRAM NOR flash routine (which temporarily disables XIP on RP2040) can
        // complete and re-enable XIP naturally.
        read_probe_memory_fast(
            &mut core,
            elf_info.program_metadata_addr,
            &mut prog_meta_buf,
        )
        .map_err(|e| {
            let msg = format!(
                "Failed to read .program_metadata at 0x{:08X}: {:?}",
                elf_info.program_metadata_addr, e
            );
            TargetQueryFailure {
                status: InterfaceStatusReport {
                    interface: TargetInterfaceKind::SwdUsb,
                    availability: InterfaceAvailability::TransientIo,
                    devices: devices.clone(),
                    error: Some(msg.clone()),
                },
                message: msg,
            }
        })?;

        let decoded = match elf_info.decode_target_bytes(&prog_meta_buf, sibling_tables) {
            Ok(info) => info,
            Err(_) => {
                // An active SRAM NOR-flash transaction (e.g. `sequential_storage` during boot)
                // temporarily disables XIP on RP2040 (`0x00` reads) for a few milliseconds.
                // Poll non-destructively first before falling back to a brief reset + boot2 XIP init.
                let mut recovered: Option<ProjectInfo> = None;
                for attempt in 0..6 {
                    if attempt == 3 {
                        let _ = core.reset();
                        let _ = core.run();
                    }
                    std::thread::sleep(std::time::Duration::from_millis(25));
                    if read_probe_memory_fast(
                        &mut core,
                        elf_info.program_metadata_addr,
                        &mut prog_meta_buf,
                    )
                    .is_ok()
                    {
                        if let Ok(info) =
                            elf_info.decode_target_bytes(&prog_meta_buf, sibling_tables)
                        {
                            recovered = Some(info);
                            break;
                        }
                    }
                }
                match recovered {
                    Some(info) => info,
                    None => elf_info
                        .decode_target_bytes(&prog_meta_buf, sibling_tables)
                        .map_err(|e| {
                            let msg =
                                format!("AttachedTargetMetadataInvalid ({}): {}", elf_info.chip, e);
                            TargetQueryFailure {
                                status: InterfaceStatusReport {
                                    interface: TargetInterfaceKind::SwdUsb,
                                    availability: InterfaceAvailability::InvalidMetadata,
                                    devices: devices.clone(),
                                    error: Some(msg.clone()),
                                },
                                message: msg,
                            }
                        })?,
                }
            }
        };

        let r2 = if let Some(ref mp) = meta_part_spec {
            let erase_sz = (elf_info.flash_erase_size as usize).max(4096);
            let mut has_active_sector = false;
            let mut sec_off = 0usize;
            while sec_off + 8 <= mp.size {
                let mut hdr = [0u8; 8];
                if read_probe_memory_fast(&mut core, mp.address as u64 + sec_off as u64, &mut hdr)
                    .is_ok()
                    && hdr != [0xFF; 8]
                    && hdr != [0x00; 8]
                {
                    has_active_sector = true;
                    break;
                }
                sec_off += erase_sz;
            }
            if has_active_sector && !meta_part_buf.is_empty() {
                read_probe_memory_fast(&mut core, mp.address as u64, &mut meta_part_buf).is_ok()
            } else {
                false
            }
        } else {
            false
        };
        (decoded, r2)
    };

    if part_read_ok {
        if let Some(override_bt) =
            read_board_type_from_metadata_slice(&meta_part_buf, elf_info.flash_erase_size as usize)
                .await
        {
            return Ok(QueriedProgramMetadata {
                info: decoded,
                effective_board_type: override_bt.clone(),
                provisioned_board_type: Some(override_bt),
                source: "metadata_partition",
            });
        }
    }

    let bt = decoded.board_type.clone();
    Ok(QueriedProgramMetadata {
        info: decoded,
        effective_board_type: bt,
        provisioned_board_type: None,
        source: "program_metadata",
    })
}

/// Execute the `query_metadata` CLI subcommand and dump the full `.program_metadata` section to stdout.
pub async fn run(
    explicit_elf: Option<&Path>,
    dump_path: Option<&str>,
    gdb_bridge: Option<&str>,
    elf_only: bool,
    json: bool,
    spinner: &indicatif::ProgressBar,
) -> io::Result<()> {
    spinner.set_message("Querying .program_metadata...");

    let elf_info = match explicit_elf {
        Some(path) => Some(autodetect_project_info(path).map_err(|e| {
            spinner.finish_and_clear();
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?),
        None => None,
    };

    let sibling_tables = explicit_elf
        .map(discover_sibling_elf_tables)
        .unwrap_or_default();

    let result = if elf_only {
        let info = elf_info.ok_or_else(|| {
            spinner.finish_and_clear();
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "--elf is required when running query_metadata --elf-only",
            )
        })?;
        let bt = info.board_type.clone();
        Ok(QueriedProgramMetadata {
            info,
            effective_board_type: bt,
            provisioned_board_type: None,
            source: "elf",
        })
    } else if let Some(dump_file) = dump_path {
        let bytes = std::fs::read(dump_file)?;
        query_from_dump_bytes(&bytes, elf_info.as_ref(), &sibling_tables)
            .await
            .map_err(|msg| TargetQueryFailure {
                status: InterfaceStatusReport {
                    interface: TargetInterfaceKind::SwdUsb,
                    availability: InterfaceAvailability::InvalidMetadata,
                    devices: Vec::new(),
                    error: Some(msg.clone()),
                },
                message: msg,
            })
    } else {
        let info = elf_info.as_ref().ok_or_else(|| {
            spinner.finish_and_clear();
            io::Error::new(
                io::ErrorKind::NotFound,
                "Direct target query_metadata requires --elf to resolve .program_metadata address and size.",
            )
        })?;
        query_from_target(info, &sibling_tables, gdb_bridge).await
    };

    spinner.finish_and_clear();
    match result {
        Ok(queried) => {
            if json {
                println!("{}", queried.to_json());
            } else {
                println!("{}", queried.to_text());
            }
            Ok(())
        }
        Err(failure) => {
            if json {
                println!("{}", failure.status.to_json());
            }
            Err(io::Error::other(failure.message))
        }
    }
}
