# 🟢 `[WORM-GLOWING-SLUG-613]` Motor fails to spin, but moves

- **UUID**: `f4cd4549-4887-4b9c-9a44-c019e055dc65`
- **ID**: `WORM-GLOWING-SLUG-613`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `cat_detector`
- **Created**: `2026-10-23 00:49:05 UTC`
- **Resolved**: `2026-10-23 01:26:40 UTC`

#### Description

The motor fails to spin at all, but moves when I command diff motor speeds. See the attached video.

#### Reproduction Steps

1. 1. Connect target device via probe-rs / RTT
2. 2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. 3. Execute command 'motor speed 60'
4. 4. Execute command 'motor speed 50'
5. 5. Execute command 'motor speed 60'
6. 6. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 20.128691 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 20.133321 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 30.411250 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 30.411556 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 30.414815 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 42.927767 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"20\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 42.928073 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 42.932319 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 47.947246 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"30\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 47.947540 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 47.951809 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 51.938630 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 51.938919 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 51.943312 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 58.167636 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"60\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 58.167920 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 58.172315 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 63.731461 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 63.731702 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 63.734315 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 68.146172 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"60\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 68.146457 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 68.149312 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 78.064220 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 78.064516 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 78.066813 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 108.604581 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"60\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 108.604881 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 108.608313 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 110.963120 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 110.963420 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 110.967808 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 112.172625 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"60\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 112.172915 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 112.177310 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 113.926449 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 113.926698 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 113.931316 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 115.801176 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"60\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 115.801469 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 115.804313 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 117.373586 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 117.373874 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 117.378311 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 118.643579 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"60\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 118.643870 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 118.648315 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 125.952968 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 125.953175 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 125.957379 INFO [36m[controller::motor_controller][0m Stopping motor...
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-GLOWING-SLUG-613.log](attachments/WORM-GLOWING-SLUG-613/target_system_logs_WORM-GLOWING-SLUG-613.log) | Target system and defmt logs from host daemon session |
| `reference` | [target_telemetry_WORM-GLOWING-SLUG-613.json](attachments/WORM-GLOWING-SLUG-613/target_telemetry_WORM-GLOWING-SLUG-613.json) | Target telemetry export retrieved via host_fs |
| `document` | [IMG_0201.MOV](attachments/WORM-GLOWING-SLUG-613/IMG_0201.MOV) | Attached document (IMG_0201.MOV) |

#### Resolution Notes

1. Fixed TMC6300 PIO assembler to shift exactly 6 bits to OUT pins followed by 2 null bits, preventing RP2040 out_count modulo wrapping from overwriting GP16 (UH) and GP17 (UL) with zeros and deadlocking Phase U. 2. Scaled open-loop startup commutation interval to 60 ticks (30ms) for smooth open-loop acceleration without rotor pole-slipping. 3. Preserved commutation step interval and state across running speed changes in motor_controller. 4. Added regression tests in controller_tests and integration.rs. 5. Redacted personal paths in system log attachments.
