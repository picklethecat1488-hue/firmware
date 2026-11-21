# 🟢 `[WORM-BRISTLY-ROUNDWORM-431]` GDB warning when running flushregs

- **UUID**: `fe7d1a81-6a36-4e65-8526-3284d9b1f281`
- **ID**: `WORM-BRISTLY-ROUNDWORM-431`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-20 20:19:09 UTC`
- **Resolved**: `2026-11-21 19:20:20 UTC`
- **Accepted**: `2026-11-21 19:20:23 UTC`

#### Description

GDB is emitting a warning when running the flushregs command:

$  arm-none-eabi-gdb -q -batch -ex 'target extended-remote 127.0.0.1:56947' -ex 'monitor reset halt' -ex 'flushregs' -ex 'detach' -ex 'disconnect'
warning: remote target does not support file transfer, attempting to access files from local filesystem.
Reading symbols from /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell...
embassy_executor::arch::thread::Executor::run<cat_detector_shell::__cortex_m_rt_main::{closure_env#0}> (self=0x20041fe8, init=...) at /Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/embassy-executor-0.7.0/src/arch/cortex_m.rs:105
105                         self.inner.poll();
Target reset and halted at Reset_Handler
Warning: 'flushregs', an alias for the command 'maintenance flush register-cache', is deprecated.
Use 'maintenance flush register-cache'.

[Inferior 1 (Remote target) detached]
(firmware-env)

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_shell --release
2. Execute target command via python tools/host.py cli <command>
3. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] Updated configuration: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, profile=release, speed=1000, debug_server=True (GDB :56947/:56948)
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=True
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.27s
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs, host_debug)...
[] [daemon] [INFO] Building host utilities via cargo build-host-tools
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.32s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/release/host_cli, fs=/Users/<username>/gh/firmware/target/release/host_fs, debug=/Users/<username>/gh/firmware/target/release/host_debug
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via probe-rs: download --chip rp2040 --speed 1000 /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [probe-rs] [INFO]      [1;32mFinished[0m in 9.27s
[] [daemon] [INFO] Flash download succeeded via probe-rs
[] [daemon] [INFO] [Sync] Starting probe-rs GDB server (:56947/:56948) & RTT bridge...
[] [debug] [INFO] Target GDB server ready on Core 0 :56947 / Core 1 :56948 (RTT bridge :50000)
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001571 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021744 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] [Sync] Target synchronized & GDB server active on :56947/:56948.
[] [debug] [INFO] GDB client gdb_connected on Core 0
[] [debug] [INFO] GDB client gdb_disconnected on Core 0
[] [target] [INFO] 0.001580 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021762 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
```

#### Attached Commits

- `9e42cf07`
- `615cdf74`

#### Review Tasks & Functional Checklist

- [ ] Verify batch GDB attach, reset halt, and maintenance flush register-cache emit zero file-transfer or deprecation warnings | `python tools/host.py debug start && arm-none-eabi-gdb -q -batch -ex 'set sysroot /' -ex 'target extended-remote 127.0.0.1:3333' -ex 'monitor reset halt' -ex 'maintenance flush register-cache' -ex 'detach' -ex 'disconnect'` <!-- id:f88585f5 -->
- [ ] Inspect .vscode/launch.json Cortex-Debug preAttachCommands and overrideRestartCommands | `python tools/host.py debug status` <!-- id:21f6344a -->

#### Resolution Notes

Added 'set sysroot /' to Cortex-Debug preAttachCommands in .vscode/launch.json so arm-none-eabi-gdb resolves qXfer:exec-file:read symbols directly from the local filesystem without remote file-transfer warnings, replaced deprecated 'flushregs' references with 'maintenance flush register-cache', and added live arm-none-eabi-gdb regression test test_worm_bristly_roundworm_431_gdb_zero_warnings_on_exec_file_and_reg_cache_flush.
