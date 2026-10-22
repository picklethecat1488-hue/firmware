# 🟢 `[WORM-PROBOSCID-ROUNDWORM-8]` Motor does not spin at full duty cycle

- **UUID**: `9d3fb9e3-2e4a-449e-aa56-bf9c01c87256`
- **ID**: `WORM-PROBOSCID-ROUNDWORM-8`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_detector`
- **Created**: `2026-10-22 02:53:29 UTC`
- **Resolved**: `2026-10-22 03:26:59 UTC`

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
[] [flash] [INFO] 0.001468 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.021595 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001511 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021607 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 5.795045 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"10\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 5.795342 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 5.797855 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 5.797896 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 26.112976 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 26.113193 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor speed 100121.547964 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 121.548195 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> 121.552399 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 137.751502 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 137.751690 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 140.950579 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 140.950744 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 140.954946 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] shell> motor speed 100154.809601 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 154.809806 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] Motor current: 154.814002 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 154.814031 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] shell> motor curren181.777986 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 181.778179 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor stop183.767560 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 183.767742 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> 183.771953 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] motor speed 100252.599128 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 252.599360 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 252.603571 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 252.603601 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 286.879001 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 286.879235 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 286.882073 INFO [36m[controller::motor_controller][0m Stopping motor...
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-PROBOSCID-ROUNDWORM-8.log](attachments/WORM-PROBOSCID-ROUNDWORM-8/target_system_logs_WORM-PROBOSCID-ROUNDWORM-8.log) | Target system and defmt logs from host daemon session |

#### Resolution Notes

Restored DEFAULT_TICK_INTERVAL_US to 10,000 us (10 ms) and startup_blanking_ticks to 300 (3s) to maintain open-loop BLDC motor synchronism and torque at startup. Fixed calculate_clock_divider clippy::manual_checked_ops using checked_div. Codified dedicated regression unit tests in controller_tests.rs, integration.rs, and test_worm_proboscid_roundworm_8_motor_spin.py.
