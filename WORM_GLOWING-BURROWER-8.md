# 🔴 `[WORM-GLOWING-BURROWER-8]` Weird errors observed from GDB terminal

- **UUID**: `d256f589-959d-470f-b27f-45783e44556a`
- **ID**: `WORM-GLOWING-BURROWER-8`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-20 02:18:34 UTC`

#### Description

See this anytime I restart the target, and the target doesn’t stop at the breakpoint I set in main:

Target reset and halted at Reset_Handler
Undefined command: "from".  Try "help”.

Also saw a error message trying to disconnect from target.

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
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [image.png](attachments/WORM-GLOWING-BURROWER-8/image.png) | Pasted screenshot |

#### Review Tasks & Functional Checklist

- [ ] Start target debug server via host.py and verify GDB Core 0/Core 1 RSP listeners are active | `python tools/host.py debug start && python tools/host.py debug status` <!-- id:5076079d -->
