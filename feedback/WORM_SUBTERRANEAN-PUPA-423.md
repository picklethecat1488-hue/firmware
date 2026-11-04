# 🟢 `[WORM-SUBTERRANEAN-PUPA-423]` Repro steps doesn’t update

- **UUID**: `7a755039-13cf-4f6a-aa35-0d435c922690`
- **ID**: `WORM-SUBTERRANEAN-PUPA-423`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `cat_detector`
- **Created**: `2026-10-23 00:49:55 UTC`
- **Resolved**: `2026-10-23 01:08:24 UTC`

#### Description

The reproduction steps and description seems to always be cached from the last filed bug. The repro steps in particular does not update if a new sequence of commands is run.

#### Reproduction Steps

1. 1. Connect target device via probe-rs / RTT
2. 2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. 3. Execute command 'motor speed 60'
4. 4. Execute command 'motor speed 50'
5. 5. Execute command 'motor speed 60'
6. 6. Observe defect/failure on target

#### Execution / Console Logs

```text
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
[] [daemon] [INFO] RTT stream disconnected
[] [host_fs] [INFO] [1m[31merror:[0m the following required arguments were not provided:
[] [host_fs] [INFO]   [32m<OUT_CSV>[0m
[] [host_fs] [INFO] [1m[4mUsage:[0m [1mhost_fs export-telemetry[0m <OUT_CSV>
[] [host_fs] [INFO] For more information, try '[1m--help[0m'.
[] [daemon] [INFO] RTT streaming process connected
[] [daemon] [INFO] Target bug created: WORM-GLOWING-SLUG-613 (Motor fails to spin, but moves) with 3 attachments
[] [target] [INFO] 0.001470 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021807 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-SUBTERRANEAN-PUPA-423.log](attachments/WORM-SUBTERRANEAN-PUPA-423/target_system_logs_WORM-SUBTERRANEAN-PUPA-423.log) | Target system and defmt logs from host daemon session |
| `reference` | [target_telemetry_WORM-SUBTERRANEAN-PUPA-423.json](attachments/WORM-SUBTERRANEAN-PUPA-423/target_telemetry_WORM-SUBTERRANEAN-PUPA-423.json) | Target telemetry export retrieved via host_fs |

#### Resolution Notes

Ensured Target Bug modal dynamically regenerates reproduction steps from latest CLI command history without caching old sequences, clears description and actual/expected inputs between reports and upon successful submission, and strips leading numbering in target_bug API to prevent duplicated markdown list numbers.
