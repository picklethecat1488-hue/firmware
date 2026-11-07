# 🟢 `[WORM-FLEXIBLE-WOODLOUSE-713]` When motor speed is reset, the duty cycle resets

- **UUID**: `e80e1e2c-1bc4-4a77-a893-77cc6d0c3d39`
- **ID**: `WORM-FLEXIBLE-WOODLOUSE-713`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_detector`
- **Created**: `2026-10-24 00:47:14 UTC`
- **Resolved**: `2026-10-24 00:52:25 UTC`
- **Accepted**: `2026-10-24 00:52:25 UTC`

#### Description

When motor speed is reset, the current duty cycle seems to spike and reset to bare minimum. This causes the motor not to ramp up smoothly during the stress test.

#### Reproduction Steps

1. Connect target device via probe-rs / RTT
2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. Execute command 'motor current'
4. Execute command 'motor speed 2'
5. Execute command 'motor current'
6. Execute command 'motor current'
7. Execute command 'motor current'
8. Execute command 'motor current'
9. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 512.399895 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 512.404154 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 512.581040 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 512.581237 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 513.427741 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"8\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 513.427998 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 513.430490 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 513.548653 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 513.548876 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 514.395350 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"9\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 514.395616 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 514.398483 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 514.510200 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 514.510420 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 514.514711 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 566.162677 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"1\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 566.162903 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 566.167086 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 566.167136 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 568.400444 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 568.400693 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 569.609870 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 569.610054 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 570.577436 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 570.577659 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 571.666010 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 571.666234 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 573.238399 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 573.238629 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 575.234069 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 575.234285 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 577.350788 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 577.350977 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 578.439340 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 578.439546 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 585.212633 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 585.212852 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 587.752602 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 587.752805 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 595.312202 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"2\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 595.312466 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 595.316732 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 597.126403 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 597.126640 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 598.093995 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 598.094189 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 598.880126 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 598.880337 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 599.908251 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 599.908469 INFO [36m[controller::shell_controller][0m command execution succeeded
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-FLEXIBLE-WOODLOUSE-713.log](attachments/WORM-FLEXIBLE-WOODLOUSE-713/target_system_logs_WORM-FLEXIBLE-WOODLOUSE-713.log) | Target system and defmt logs from host daemon session |
| `reference` | [target_telemetry_WORM-FLEXIBLE-WOODLOUSE-713.json](attachments/WORM-FLEXIBLE-WOODLOUSE-713/target_telemetry_WORM-FLEXIBLE-WOODLOUSE-713.json) | Target telemetry export retrieved via host_fs |

#### Resolution Notes

Preserve active regulated duty cycle across running speed changes in MotorController tick_motor, preventing duty cycle collapse and current spikes during speed transitions. Validated on hardware with 1% to 50% ramp over 25s completing with zero DIAG faults and peak current capped at 765 mA.
