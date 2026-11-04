# 🟢 `[WORM-SOILBOUND-VERMIS-341]` The CLI console is broken and not updating

- **UUID**: `d6d2ea37-4767-4a13-a69c-9e83855837b6`
- **ID**: `WORM-SOILBOUND-VERMIS-341`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `cat_detector`
- **Created**: `2026-10-24 01:26:49 UTC`
- **Resolved**: `2026-10-24 01:30:39 UTC`

#### Reproduction Steps

1. Connect target device via probe-rs / RTT
2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. Execute command 'motor stop'
4. Execute command 'motor stop'
5. Execute command 'motor speed 10'
6. Execute command 'motor current'
7. Execute command 'motor stop'
8. Execute command 'motor stop'
9. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 437.593200 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"38\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 437.593479 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 437.597775 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 437.712100 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 437.712313 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 447.571711 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"39\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 447.571966 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 447.574455 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 447.751070 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 447.751289 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 457.610700 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"40\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 457.610979 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 457.615405 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 457.850572 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 457.850763 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 467.589241 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"41\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 467.589509 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 467.593754 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 467.768620 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 467.768834 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 468.972057 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 468.972272 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 468.976041 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 477.567736 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"42\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 477.567981 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 477.571651 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 477.571692 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 477.747172 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 477.747420 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 487.606798 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"43\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 487.607067 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 487.611396 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 487.786101 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 487.786334 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 501.870961 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 501.871204 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 501.875459 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 503.019922 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 503.020118 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 506.414716 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"10\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 506.414943 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 506.417289 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 506.417336 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 520.806057 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 520.806313 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 525.154176 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 525.154406 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 525.158696 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 538.942621 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 538.942796 INFO [36m[controller::shell_controller][0m command execution succeeded
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-SOILBOUND-VERMIS-341.log](attachments/WORM-SOILBOUND-VERMIS-341/target_system_logs_WORM-SOILBOUND-VERMIS-341.log) | Target system and defmt logs from host daemon session |
| `reference` | [target_telemetry_WORM-SOILBOUND-VERMIS-341.json](attachments/WORM-SOILBOUND-VERMIS-341/target_telemetry_WORM-SOILBOUND-VERMIS-341.json) | Target telemetry export retrieved via host_fs |

#### Resolution Notes

Resolved CLI and system log console freeze when circular buffer reaches maxlen (5,000 entries). DaemonRunner now assigns monotonic sequence IDs to log, cli_outputs, and fs_outputs entries. host_daemon.html.j2 tracks sequence IDs (lastCliId, lastLogId, lastFsId) to incrementally append new items and handle wrap-arounds even when total array length remains capped at 5000.
