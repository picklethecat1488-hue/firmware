# 🟢 `[WORM-HUMIC-SANDWORM-365]` motor speed 50 tripped DIAG

- **UUID**: `18a14b2a-76fb-4cbc-8536-8fc431b8fa60`
- **ID**: `WORM-HUMIC-SANDWORM-365`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `GENERAL`
- **Component**: `target`
- **Created**: `2026-10-24 02:10:03 UTC`
- **Resolved**: `2026-10-24 02:21:31 UTC`
- **Accepted**: `2026-10-24 02:21:31 UTC`

#### Reproduction Steps

1. Connect target device via probe-rs / RTT
2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. Execute command 'motor speed 50'
4. Execute command 'motor stop'
5. Execute command 'motor speed 50'
6. Execute command 'motor stop'
7. Execute command 'motor speed 50'
8. Execute command 'motor stop'
9. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001502 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021777 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] Target bug created: WORM-ANNELIDIC-NEMATODE-26 (motor speed 100 tripped DIAG) with 1 attachments
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=True
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.08s
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs)...
[] [daemon] [INFO] Building host utilities: host_cli and host_fs
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.12s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/release/host_cli, fs=/Users/<username>/gh/firmware/target/release/host_fs
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.07s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.10s
[] [flash] [INFO]      [1;32mFinished[0m in 8.99s
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
[] [flash] [INFO] 0.001510 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.021763 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001468 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021760 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 5.311384 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 5.311688 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 5.315660 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 5.315711 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 6.023451 WARN [36m[controller::motor_controller][0m Current 1380 mA > hard ceiling 800 mA, throttling duty cycle to 90%
[] [target] [INFO] 7.023758 WARN [36m[controller::motor_controller][0m Current 1365 mA > hard ceiling 800 mA, throttling duty cycle to 80%
[] [target] [INFO] 7.043470 ERROR [36m[peripheral::tmc6300][0m TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 7.043503 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 10.746090 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 10.746303 INFO [36m[controller::shell_controller][0m command execution succeeded
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-HUMIC-SANDWORM-365.log](attachments/WORM-HUMIC-SANDWORM-365/target_system_logs_WORM-HUMIC-SANDWORM-365.log) | Target system and defmt logs from host daemon session |
| `reference` | [target_telemetry_WORM-HUMIC-SANDWORM-365.json](attachments/WORM-HUMIC-SANDWORM-365/target_telemetry_WORM-HUMIC-SANDWORM-365.json) | Target telemetry export retrieved via host_fs |

#### Resolution Notes

Resolved by eliminating TMC6300 FET turn-off timing violations with SHORT_PROTECTION_MIN_OFF_COUNT = 4 and removing blocking CNVR wait during periodic telemetry. Verified on target: motor runs at speed 50 stably without tripping DIAG.
