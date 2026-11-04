# 🟢 `[WORM-SEGMENTED-SIPUNCULID-504]` Motor RPM reads as 0 RPM at 100% speed

- **UUID**: `05658f02-3c1d-456c-ba10-7349ae64966d`
- **ID**: `WORM-SEGMENTED-SIPUNCULID-504`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_detector`
- **Created**: `2026-10-22 01:59:36 UTC`
- **Resolved**: `2026-10-22 02:44:35 UTC`

#### Reproduction Steps

1. 1. Connect target device via probe-rs / RTT
2. 2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. 3. Execute command 'motor speed 100'
4. 4. Execute command 'motor current'
5. 5. Execute command 'motor rpm'
6. 6. Execute command 'motor stop'
7. 7. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 13.059552 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 22355 mC, State: Normal
[] [target] [INFO] 14.059929 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 20951 mC, State: Normal
[] [target] [INFO] 16.060567 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 22355 mC, State: Normal
[] [daemon] [INFO] Updated configuration: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, profile=release, speed=4000
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=True
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.21s
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs)...
[] [daemon] [INFO] Building host utilities: host_cli and host_fs
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.21s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/release/host_cli, fs=/Users/<username>/gh/firmware/target/release/host_fs
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.06s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.12s
[] [flash] [INFO]      [1;32mFinished[0m in 8.96s
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
[] [flash] [INFO] 0.001483 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.021793 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001512 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021810 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] motor speed 1009.909637 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 9.909928 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 9.914207 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 9.914234 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] shell> motor current13.292189 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 13.292411 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor rp14.493571 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Rpm), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 14.493739 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor stop17.156514 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 17.156690 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> 17.166758 INFO [36m[controller::motor_controller][0m Stopping motor...
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-SEGMENTED-SIPUNCULID-504.log](attachments/WORM-SEGMENTED-SIPUNCULID-504/target_system_logs_WORM-SEGMENTED-SIPUNCULID-504.log) | Target system and defmt logs from host daemon session |

#### Resolution Notes

Fixed HallSensor RPM window collapse with 500ms timeout persistence, updated MotorCommand::ReadRpm to sample speed sensor, and scaled MotorController tick interval to 500us for 3333 RPM nominal operation.
