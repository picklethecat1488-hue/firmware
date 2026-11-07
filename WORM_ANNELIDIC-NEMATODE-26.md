# 🟢 `[WORM-ANNELIDIC-NEMATODE-26]` motor speed 100 tripped DIAG

- **UUID**: `7340fe94-6993-45f2-b119-41a91487de7a`
- **ID**: `WORM-ANNELIDIC-NEMATODE-26`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `GENERAL`
- **Component**: `target`
- **Created**: `2026-10-24 02:09:15 UTC`
- **Resolved**: `2026-10-24 02:21:26 UTC`
- **Accepted**: `2026-10-24 02:21:26 UTC`

#### Reproduction Steps

1. Connect target device via probe-rs / RTT
2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. Execute command 'motor speed 100'
4. Execute command 'motor stop'
5. Execute command 'motor speed 50'
6. Execute command 'motor stop'
7. Execute command 'motor speed 50'
8. Execute command 'motor stop'
9. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 0.022415 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=True
[] [build] [INFO] [1m[92m   Compiling[0m controller v0.1.0 (/Users/<username>/gh/firmware/controller)
[] [build] [INFO] [1m[92m   Compiling[0m peripheral v0.1.0 (/Users/<username>/gh/firmware/peripheral)
[] [build] [INFO] [1m[92m   Compiling[0m board v0.1.0 (/Users/<username>/gh/firmware/board)
[] [build] [INFO] [1m[92m   Compiling[0m cat_detector v0.1.0 (/Users/<username>/gh/firmware/app)
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 8.79s
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs)...
[] [daemon] [INFO] Building host utilities: host_cli and host_fs
[] [build] [INFO] [1m[92m   Compiling[0m controller v0.1.0 (/Users/<username>/gh/firmware/controller)
[] [build] [INFO] [1m[92m   Compiling[0m peripheral v0.1.0 (/Users/<username>/gh/firmware/peripheral)
[] [build] [INFO] [1m[92m   Compiling[0m board v0.1.0 (/Users/<username>/gh/firmware/board)
[] [build] [INFO] [1m[92m   Compiling[0m host_fs v0.1.0 (/Users/<username>/gh/firmware/tools/host_fs)
[] [target] [INFO] 43.395366 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 43.395672 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 43.399953 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 43.400002 INFO [36m[controller::motor_controller][0m Starting motor...
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 21.13s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/release/host_cli, fs=/Users/<username>/gh/firmware/target/release/host_fs
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.19s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.22s
[] [host_cli] [ERROR] Target CLI is not available (RTT console is not connected). Please synchronize target.
[] [flash] [INFO]      [1;32mFinished[0m in 9.00s
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
[] [flash] [INFO] shell> 
[] [flash] [INFO] Draining buffered defmt logs:
[] [flash] [INFO] 0.001511 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.021789 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001502 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021777 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
```

#### Resolution Notes

Enforce SHORT_PROTECTION_MIN_OFF_COUNT = 4 (280 ns off time) in calculate_pwm_counts to ensure flyback diode and FET turn-off recovery time, preventing short circuit current detection and TMC6300 DIAG fault pin latch-up.
