# 🟢 `[WORM-SEGMENTED-MEALWORM-41]` Startup blanking

- **UUID**: `1a765883-3cb7-43b4-b646-f4c6b9965a06`
- **ID**: `WORM-SEGMENTED-MEALWORM-41`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-10-23 01:53:50 UTC`
- **Resolved**: `2026-10-23 16:29:28 UTC`

#### Description

Can we please configure the motor driver and motor controller in board.toml to perform an ideal startup blanking interval in <= 500ms in a way which maximizes torque during that interval? That way we know for sure the code is starting the motor correctly.

#### Reproduction Steps

1. Connect target device via probe-rs / RTT
2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. Execute command 'motor current'
4. Execute command 'motor rpm'
5. Execute command 'motor speed 30'
6. Execute command 'motor current'
7. Execute command 'motor rpm'
8. Execute command 'motor stop'
9. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [flash] [INFO]       |,4-  ) )-,_. ,\ (  `'-'
[] [flash] [INFO]      '---''(_/--'  `-'\_)  
[] [flash] [INFO] Type 'help' to print usage.
[] [flash] [INFO] shell> 
[] [flash] [INFO] Draining buffered defmt logs:
[] [flash] [INFO] 0.001493 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.021843 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001471 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021801 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 12.507959 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"10\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 12.508270 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 12.512091 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 12.512125 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 18.132487 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"30\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 18.132814 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 18.137203 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 21.519085 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 21.519392 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 21.522695 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 34.763342 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"60\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 34.763580 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 34.768187 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 37.184421 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 37.184702 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 37.189193 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 46.977417 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 46.977625 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] Target bug created: WORM-BURROWING-CLITELLUM-618 (Motor is stationary :-() with 1 attachments
[] [target] [INFO] 63.904602 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 63.904846 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 63.907251 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 99.772730 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 99.772944 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 182.314173 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Rpm), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 182.314360 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 189.279047 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"30\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 189.279258 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 189.283400 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 189.283427 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 190.426286 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 190.426561 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 190.478606 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Rpm), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 190.478813 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 196.467755 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 196.468008 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 196.472372 INFO [36m[controller::motor_controller][0m Stopping motor...
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-SEGMENTED-MEALWORM-41.log](attachments/WORM-SEGMENTED-MEALWORM-41/target_system_logs_WORM-SEGMENTED-MEALWORM-41.log) | Target system and defmt logs from host daemon session |

#### Resolution Notes

Configured startup_blanking_ticks = 50 (500 ms at 100 Hz) in board.toml, model types, and code_gen. Wired Board::MOTOR_CONFIG into cat_detector.rs. Implemented startup alignment full torque boost (100% duty cycle on Step 0 alignment) in MotorController to maximize breakaway torque before transitioning to commanded speed. Codified regression test in controller_tests.rs.
