# Microcontroller Decoupling & Board Support Packages (BSPs)

## Architectural Purpose
To ensure firmware portability across multiple hardware platforms and target microcontrollers without polluting core application logic, drivers, or domain controllers with vendor-specific peripherals or pin allocations.

---

## Core Invariants

### 1. Separation of Concerns & Clean App Entrypoints
* **No Conditional Pin Setup in Applications**: Do NOT perform conditional driver setup or extract GPIO pins inside main application files (`main.rs`, `shell.rs`).
* **Encapsulated Board Initialization**: Encapsulate all initialization, pin configuration, clock tree setup, and dynamic address assignment inside the `Board::init` constructor in Board Support Package (BSP) target/host implementations (e.g., `bsp_target.rs` / `bsp_host.rs`).
* **Target Independence**: Application tasks, domain state machines, and high-level controllers must receive typed abstraction handles or channel endpoints from the BSP, never raw microcontroller registers or peripherals.

### 2. Driver & Struct Naming Hygiene
* **No MCU Model Prefixes**: Do NOT prefix files or structs with MCU model numbers (e.g., do NOT write `rp2040_sensor.rs` or `struct NxpI2cDriver`). Keep driver wrappers and peripheral interfaces target-independent.
* **Portable Abstractions**: Drivers must implement or accept canonical traits (such as Rust `embedded-hal` / `embedded-hal-async`) rather than concrete vendor peripheral structs.
* **Target-Specific Driver Gating & Explicit Compile Errors**: When an optional hardware driver relies on microcontroller-specific peripherals (such as RP2040 PIO state machines), the driver MUST explicitly guard against unsupported target microcontrollers with descriptive compile errors and build script diagnostics, rather than emitting unrunnable or silently failing code on other platforms.

### 3. Vendor-Specific Code Boundaries
* **Placement Restrictions**: Vendor-specific code (register blocks, PAC access, boot stages, manufacturer errata workarounds) should ONLY be placed inside the `board` or `app` crate.
* **Platform Module Scoping**: Vendor-specific code may go into the `platform` crate ONLY when placed inside a dedicated platform support module for that vendor (e.g., `platform/src/rp2040/lib.rs` or `platform/src/mcx_n947/`).

### 4. Timing & Delay Invariants
* **No Fixed CPU Cycle Delays**: Do NOT use fixed CPU cycle delays (e.g., `cortex_m::asm::delay`) in firmware code, as cycle counts vary unpredictably with clock frequency changes, cache hits, Flash wait states, and pipeline depths.
* **Frequency-Independent Time Delays**: Use frequency-independent time-based delays (e.g., `embassy_time::Timer` or `embassy_time::Delay`) that yield execution back to the async executor or hardware timers.

### 5. Zero Default Chip or Binary Fallbacks
* **Explicit Target & Binary Metadata**: Do NOT fall back to a default chip (e.g., `rp2040`), ARM target triple, board type, or binary name anywhere in device code, `tools/code_gen`, `tools/host.py`, or `tools/device_tests`.
* **Fail-Fast Discovery**: Every firmware binary in `app/Cargo.toml` must declare `[package.metadata.binaries.<bin>]` (`board`, `chip`, `target`), and on-device target discovery (`.program_metadata` / `host_fs query_metadata`) must fail explicitly if chip, board type, or target binary values cannot be inferred.

---

## Supporting a New Processor in `host.py` and Host Debug Tooling

Because `tools/host.py`, `tools/host_debug`, `tools/host_fs`, and `tools/host_cli` resolve chip identity, linker layouts, and GDB register XML dynamically rather than hardcoding defaults, adding a new microcontroller or processor family follows a deterministic 6-step checklist:

### 1. Platform Linker Layout & BSP Target Support (`platform/`, `board/`, `.cargo/config.toml`)
1. Add the processor's `memory.x` and `program_metadata.x` linker scripts under `platform/src/<mcu>/layouts/` with a dedicated 4 KB `PROGRAM_METADATA` `MEMORY` region and `.program_metadata` output section.
2. Register the Rust target triple in `.cargo/config.toml` (`[target.<triple>]`) passing `-C link-arg=-Tprogram_metadata.x` alongside `link.x` and `defmt.x`, and add an alias in `[alias]` so `tools/build/build_firmware.sh` and `tools/validation/validate_all.py` check the new target.
3. Declare the board entry in `board/board.toml` (`chip`, `board_type`, `flash_base`, `program_metadata_addr`, flash erase/write geometry, and partition ranges) so `tools/code_gen` generates `emit_<board>_project_metadata!`.

### 2. Binary Metadata Declaration (`app/Cargo.toml`)
Declare explicit per-binary metadata for every executable targeting the new processor—`host.py` discovers binaries exclusively from these tables:
```toml
[package.metadata.binaries.<bin_name>]
board = "<board_key_in_board_toml>"
chip = "<probe_rs_chip_name>"
target = "<rust_target_triple>"
```

### 3. Target Triple Inference in `host.py` (`tools/host_daemon/provider/daemon/config.py`)
1. When `target` is declared in `[package.metadata.binaries.<bin>]`, `infer_arm_target()` in `tools/host_daemon/provider/daemon/config.py` resolves it directly without heuristics.
2. If supporting `--chip <new_chip>` CLI overrides without an explicit `--target`, add the chip-to-Rust-triple mapping arm in `infer_arm_target()` (`tools/host_daemon/provider/daemon/config.py`).

### 4. GDB RSP Register Profile, Memory Map & Debug Backend (`tools/host_debug/`)
`tools/host_debug` separates protocol framing (`src/rsp.rs` and `src/server.rs`) from processor-specific register, memory-map, and breakpoint logic (`src/arch.rs` and `src/arm.rs`):
* **New ARM Cortex-M MCU (`ARMv6-M` / `ARMv7-M` / `ARMv8-M`)**: Supported automatically by `ArmProbeDebugBackend` (`tools/host_debug/src/arm.rs`) as soon as `probe-rs` recognizes the `chip` string. `ProcessorArch` selects the FPB comparator address range (`Armv6m` `< 0x2000_0000` + SRAM `BKPT #0` vs `Armv7m`/`Armv8m` full address space), queries FPU presence via `core.fpu_support()`, renders `target.xml` dynamically from `tools/host_debug/templates/target.xml.jinja` (marking core general registers with `group="general"` so `RspCoreHandler` parses expedited `T05` stop-reply registers directly from `target.xml`), and renders `qXfer:memory-map:read::` XML dynamically from `build_arm_cortex_m_memory_regions()` via `tools/host_debug/templates/memory_map.xml.jinja`. If a dual-core MCU wipes secondary-core debug comparators during boot (similar to RP2040 `PSM.frce_off`), register its post-reset hook inside `ArmProbeDebugBackend::from_session`.
* **New Non-ARM Processor Architecture (e.g., RISC-V or AVR)**:
  1. Add the new variant to `ProcessorArch` in `tools/host_debug/src/arch.rs` (`gdb_architecture_name()`, `normalize_pc()`, and `is_non_code_address()`).
  2. Create a dedicated backend module (e.g., `tools/host_debug/src/riscv.rs`) defining single-sourced `RegisterFeatureGroup` / `RegisterDescriptor` (`reg_group: "general"` for expedited `T05` core registers) and `MemoryRegionDescriptor` tables rendered through `TargetRegisterLayout::new(...)` (`templates/target.xml.jinja` and `templates/memory_map.xml.jinja`) and implementing `DebugBackend` (`core_register_layout`, `insert_breakpoint`, `remove_breakpoint`, `read_memory_masked`, `step_core`, `resume_core`).
  3. Dispatch to the new backend in `tools/host_debug/src/main.rs` (passing the generic `probe_rs::Session` attached via `tools/host_debug/src/session.rs`) based on the target architecture and add unit + GDB RSP integration tests in `tools/host_debug/tests/host_debug_tests.rs`.

### 5. RTT Control Block, Boot Status & Shell Idle Signaling (`platform/src/system/rtt.rs`, `tools/host_cli/`)
Ensure every target firmware binary publishes `TargetBootState` (`Booting` -> `Ready` / `Faulted`) via `platform::rtt::set_rtt_boot_state(...)` (and `platform::run_rtt_shell_loop!` for interactive shells), updating `_SEGGER_RTT.down_channels[0].flags` so `cargo host-cli boot-status --json`, `host_cli --json-events`, and `python tools/host.py boot-status` detect boot and shell readiness uniformly without binary-name heuristics or console string polling.

### 6. On-Device Pytest Discovery (`tools/device_tests/`)
Add `@device_test(project="...", boards=("<new_board_glob>",), target_binary="<bin_name>")` suites under `tools/device_tests/suites/`. `python tools/host.py test` and `./tools/verify.py` will automatically probe `.program_metadata` via `host_fs query_metadata`, match `board_type`, flash the binary, and run the matching suites.

