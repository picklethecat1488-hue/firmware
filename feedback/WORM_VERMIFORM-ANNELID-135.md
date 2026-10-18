# 🟢 `[WORM-VERMIFORM-ANNELID-135]` Providing scope trace with motor detached

- **UUID**: `493795b8-c629-422d-9147-b8b24aa575a7`
- **ID**: `WORM-VERMIFORM-ANNELID-135`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-10-18 00:39:31 UTC`
- **Resolved**: `2026-10-18 01:10:11 UTC`

#### Description

FYR, pls take a look at this trace with motor detached.

#### Reproduction Steps

1. motor speed 100

#### Execution / Console Logs

```text
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs)...
[] [daemon] [INFO] Building host utilities: host_cli and host_fs
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.26s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/release/host_cli, fs=/Users/<username>/gh/firmware/target/release/host_fs
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.08s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.18s
[] [flash] [INFO]      [1;32mFinished[0m in 7.48s
[] [flash] [INFO] RTT connected. Active channels:
[] [flash] [INFO]   - Up Channel: "defmt" (logs)
[] [flash] [INFO]   - Up Channel: "cli" (raw output)
[] [flash] [INFO]   - Down Channel: "cli" (raw input)
[] [flash] [INFO] shell> 
[] [flash] [INFO] [2K
[] [flash] [INFO]        |\      _,,,---,,_
[] [flash] [INFO]  Zzz   /,`.-'`'    -.  ;-;;,_
[] [flash] [INFO]       |,4-  ) )-,_. ,\ (  `'-'
[] [flash] [INFO]      '---''(_/--'  `-'\_)  
[] [flash] [INFO] Type 'help' to print usage.
[] [flash] [INFO] shell> Draining buffered defmt logs:
[] [flash] [INFO] 0.001490 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.021743 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [flash] [INFO] Error: unexpected EOF during frame decoding (possible target/ELF mismatch or truncated log buffer)
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001468 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021734 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] Target bug created: WORM-SLICK-FIREWORM-677 (Error: unexpected EOF during frame decoding (possible target/ELF mismatch or truncated log buffer) RTT connected. Active channels:) with 1 attachments
[] [target] [INFO] motor speed 1038.030879 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 38.031166 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 38.035467 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 38.035511 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] shell> 38.057941 ERROR [36m[peripheral::tmc6300][0m TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 38.057971 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [daemon] [INFO] Target bug created: WORM-GLIDING-LARVA-522 (Motor fails, DIAG light pops on) with 2 attachments
[] [target] [INFO] 0.001591 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.019365 INFO [36m[peripheral::max17048][0m Reset detected (RI set). Initializing ModelGauge and triggering Quick-Start.
[] [target] [INFO] 0.022414 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] shell> motor speed 10017.997383 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 17.997669 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 18.001941 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 18.001966 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] shell> 21.034827 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 21.035286 WARN [36m[controller::motor_controller][0m Low load / dry detected (current: 8 mA). Stopped motor.
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-VERMIFORM-ANNELID-135.log](attachments/WORM-VERMIFORM-ANNELID-135/target_system_logs_WORM-VERMIFORM-ANNELID-135.log) | Target system and defmt logs from host daemon session |

#### Resolution Notes

Resolved in conjunction with WORM-GLIDING-LARVA-522. Confirmed that trace with motor detached ran for 3 seconds without hardware driver fault due to absence of motor stator coil current / rotor stall, stopping solely on expected low-load dry-run detection (8 mA). Restored canonical pin assignments and forward commutation progression.
