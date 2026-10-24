# 🟢 `[WORM-UNDULATING-SERPULA-688]` Temporarily remove hall sensor support

- **UUID**: `37fc64de-00b0-4ded-9755-a6177ee6b387`
- **ID**: `WORM-UNDULATING-SERPULA-688`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_detector`
- **Created**: `2026-10-23 23:15:50 UTC`
- **Resolved**: `2026-10-23 23:28:39 UTC`

#### Description

Can we disable support for the hall sensor until issues with the motor are resolved

#### Reproduction Steps

1. Connect target device via probe-rs / RTT
2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. Execute command 'motor speed 30'
4. Execute command 'motor stop'
5. Execute command 'motor speed 100'
6. Execute command 'motor stop'
7. Execute command 'motor speed 100'
8. Execute command 'motor stop'
9. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 6.890605 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 6.890644 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 8.023631 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 8.024090 ERROR [36m[controller::motor_controller][0m Zero RPM stall detected via Hall sensor. Stopped motor.
[] [target] [INFO] 10.806755 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 10.806989 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=True
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.09s
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
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.13s
[] [flash] [INFO]      [1;32mFinished[0m in 9.03s
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
[] [flash] [INFO] 0.001498 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.021968 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001499 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021950 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 4.466970 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 4.467296 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 4.471584 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 4.471620 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 5.024655 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 5.025127 ERROR [36m[controller::motor_controller][0m Zero RPM stall detected via Hall sensor. Stopped motor.
[] [target] [INFO] 9.476323 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 9.476556 INFO [36m[controller::shell_controller][0m command execution succeeded
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-UNDULATING-SERPULA-688.log](attachments/WORM-UNDULATING-SERPULA-688/target_system_logs_WORM-UNDULATING-SERPULA-688.log) | Target system and defmt logs from host daemon session |
| `reference` | [target_telemetry_WORM-UNDULATING-SERPULA-688.json](attachments/WORM-UNDULATING-SERPULA-688/target_telemetry_WORM-UNDULATING-SERPULA-688.json) | Target telemetry export retrieved via host_fs |

#### Resolution Notes

Disabled support for Hall effect speed sensor in cat_detector (replacing target SpeedSensorDevice with MockSpeedSensor::new(0) in Board::init) and removed zero-RPM stall detection in MotorController to allow freewheeling motor operation without false stall shutdowns.
