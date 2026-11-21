# 🟢 `[WORM-SEGMENTED-CHRYSALIS-575]` Core 1: garbled stack

- **UUID**: `90ac8a8d-d26e-4dcf-850f-0e483b8f54d2`
- **ID**: `WORM-SEGMENTED-CHRYSALIS-575`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-20 02:21:16 UTC`
- **Resolved**: `2026-11-20 20:16:08 UTC`
- **Accepted**: `2026-11-20 20:16:10 UTC`

#### Description

Observed garbled stack on Core1. It should not be possible for a CLI to branch into a sequential storage queue write on core1. This indicates a debug symbol mismatch:


Cortex-Debug: VSCode debugger extension version 1.12.1 git(652d042). Usage info: https://github.com/Marus/cortex-debug#usage
Reading symbols from arm-none-eabi-objdump --syms -C -h -w /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/debug/cat_detector_shell
Reading symbols from arm-none-eabi-nm --defined-only -S -l -C -p /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/debug/cat_detector_shell
Launching GDB: arm-none-eabi-gdb -q --interpreter=mi2
    IMPORTANT: Set "showDevDebugOutput": "raw" in "launch.json" to see verbose GDB transactions here. Very helpful to debug issues or report problems
Finished reading symbols from objdump: Time: 49 ms
Output radix now set to decimal 10, hex a, octal 12.
Input radix now set to decimal 10, hex a, octal 12.
Finished reading symbols from nm: Time: 310 ms
0x1002034e in arrayvec::arrayvec::ArrayVec<u16, 32>::extend_from_iter<u16, 32, core::iter::adapters::map::Map<core::slice::iter::Iter<pio::Instruction>, pio::{impl#5}::assemble::{closure_env#0}<32>>, true> (self=<optimized out>, iterable=...) at /Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.8/src/arrayvec.rs:1137
1137	                debug_assert_ne!(ptr, end_ptr);
Program stopped, probably due to a reset and/or halt issued by debugger
Undefined command: "from".  Try "help".
bt
#0  0x1002034e in arrayvec::arrayvec::ArrayVec<u16, 32>::extend_from_iter<u16, 32, core::iter::adapters::map::Map<core::slice::iter::Iter<pio::Instruction>, pio::{impl#5}::assemble::{closure_env#0}<32>>, true> (self=<optimized out>, iterable=...) at /Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.8/src/arrayvec.rs:1137
#1  0x10020fda in platform::battery_manager::BatteryManager::update_battery_status (self=0x10020347 <arrayvec::arrayvec::ArrayVec<u16, 32>::extend_from_iter<u16, 32, core::iter::adapters::map::Map<core::slice::iter::Iter<pio::Instruction>, pio::{impl#5}::assemble::{closure_env#0}<32>>, true>+106>, state_of_charge=<optimized out>, charger_state=36, system_status=(unknown: 0xe8), is_boot_trapped=true) at platform/src/power/battery.rs:122
#2  0x10011f0c in sequential_storage::queue::push::{async_fn#0}<platform::flash::BlockingAsyncFlash<&mut embassy_rp::flash::Flash<embassy_rp::peripherals::FLASH, embassy_rp::flash::Blocking, 2097152>>, sequential_storage::cache::NoCache> () at /Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sequential-storage-3.0.1/src/queue.rs:77
#3  platform::flash::write_telemetry_record_direct::{async_fn#0}<platform::flash::BlockingAsyncFlash<&mut embassy_rp::flash::Flash<embassy_rp::peripherals::FLASH, embassy_rp::flash::Blocking, 2097152>>> () at platform/src/io/flash.rs:252
#4  embassy_futures::block_on::block_on<platform::flash::write_telemetry_record_direct::{async_fn_env#0}<platform::flash::BlockingAsyncFlash<&mut embassy_rp::flash::Flash<embassy_rp::peripherals::FLASH, embassy_rp::flash::Blocking, 2097152>>>> (fut=...) at /Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/embassy-futures-0.1.2/src/block_on.rs:29
#5  0x10011f04 in sequential_storage::queue::push::{async_fn#0}<platform::flash::BlockingAsyncFlash<&mut embassy_rp::flash::Flash<embassy_rp::peripherals::FLASH, embassy_rp::flash::Blocking, 2097152>>, sequential_storage::cache::NoCache> () at /Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sequential-storage-3.0.1/src/queue.rs:79
#6  platform::flash::write_telemetry_record_direct::{async_fn#0}<platform::flash::BlockingAsyncFlash<&mut embassy_rp::flash::Flash<embassy_rp::peripherals::FLASH, embassy_rp::flash::Blocking, 2097152>>> () at platform/src/io/flash.rs:252
#7  embassy_futures::block_on::block_on<platform::flash::write_telemetry_record_direct::{async_fn_env#0}<platform::flash::BlockingAsyncFlash<&mut embassy_rp::flash::Flash<embassy_rp::peripherals::FLASH, embassy_rp::flash::Blocking, 2097152>>>> (fut=...) at /Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/embassy-futures-0.1.2/src/block_on.rs:29
#8  0x1001fd94 in core::num::{impl#6}::eq_ignore_ascii_case (self=<optimized out>, other=<optimized out>) at /Users/<username>/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/mod.rs:713
#9  core::slice::ascii::{impl#0}::eq_ignore_ascii_case (self=..., other=...) at /Users/<username>/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/slice/ascii.rs:75
#10 core::str::{impl#0}::eq_ignore_ascii_case (self=..., other=...) at /Users/<username>/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/str/mod.rs:2825
#11 controller::sensor_controller::{impl#34}::from_arg (arg=...) at platform/src/services/cli.rs:25
#12 0x00000000 in ?? ()
{"output":"","token":83,"outOfBandRecord":[],"resultRecords":{"resultClass":"done","results":[]}}

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_shell --release
2. Execute command: python tools/host.py cli help
3. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] Updated configuration: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, profile=release, speed=4000, debug_server=True (GDB :3333/:3334)
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=True
[] [build] [INFO] [1m[92m   Compiling[0m cat_detector v0.1.0 (/Users/<username>/gh/firmware/app)
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 2.59s
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs, host_debug)...
[] [daemon] [INFO] Building host utilities via cargo build-host-tools
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.24s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/release/host_cli, fs=/Users/<username>/gh/firmware/target/release/host_fs, debug=/Users/<username>/gh/firmware/target/release/host_debug
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via probe-rs: download --chip rp2040 --speed 4000 /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [probe-rs] [INFO]      [1;32mFinished[0m in 7.64s
[] [daemon] [INFO] Flash download succeeded via probe-rs
[] [daemon] [INFO] [Sync] Starting probe-rs GDB server (:3333/:3334) & RTT bridge...
[] [debug] [INFO] Target GDB server ready on Core 0 :3333 / Core 1 :3334 (RTT bridge :50000)
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001567 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021617 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] [Sync] Target synchronized & GDB server active on :3333/:3334.
[] [debug] [INFO] GDB client gdb_connected on Core 0
[] [target] [INFO] 0.001600 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021640 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 0.001599 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021630 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [debug] [INFO] GDB client gdb_disconnected on Core 0
[] [debug] [INFO] GDB client gdb_connected on Core 0
[] [debug] [INFO] GDB client gdb_disconnected on Core 0
[] [daemon] [INFO] Target bug created: WORM-GLOWING-BURROWER-8 (Weird errors observed from GDB terminal) with 2 attachments
[] [debug] [INFO] GDB client gdb_connected on Core 1
```

#### Attached Commits

- `8ba3f17ddf411d40fc653109a4fafab63ba9252c`

#### Review Tasks & Functional Checklist

- [ ] Verify host.py debug sync-symbols and target-info dynamically resolve active debug/release ELF and target device | `python tools/host.py debug sync-symbols --json` <!-- id:5e6e0c5e -->
- [ ] Verify .program_metadata encodes bin_name and build_profile with distinct 16-bit CRCs for debug vs release builds | `./target/release/host_fs --elf target/thumbv6m-none-eabi/release/cat_detector_shell query_metadata --elf-only` <!-- id:ed3a0fff -->

#### Resolution Notes

Embedded bin_name and build_profile in .program_metadata with distinct 16-bit CRCs, added DWARF CFI .cfi_undefined lr root-frame termination on Core1_Reset_Handler, synchronized target/active_target.elf via host.py debug sync-symbols / target-info, and updated .vscode/launch.json and tasks.json to dynamically track both debug and release executables across all targets.
