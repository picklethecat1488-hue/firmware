# 🟢 `[WORM-SUBTERRANEAN-EARTHWORM-231]` Cannot start motor stress at 50% speed

- **UUID**: `70d98201-eed9-49ff-bd57-5370e97df642`
- **ID**: `WORM-SUBTERRANEAN-EARTHWORM-231`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_detector`
- **Created**: `2026-10-24 01:05:39 UTC`
- **Resolved**: `2026-10-24 01:39:03 UTC`
- **Accepted**: `2026-10-24 01:39:03 UTC`

#### Description

The motor stalls at 50% speed, see below:

$  python tools/motor_stress.py --start 50 --end 100 --duration 500
Starting motor ramp stress test: 50% -> 100% over 500.0s
Connecting to host daemon at http://127.0.0.1:8899...
[  1/51] (  9.8s) Speed:  50% | Current:  489 mA | DIAG: OK
[  2/51] ( 19.6s) Speed:  51% | Current:  757 mA | DIAG: OK
^CSent motor stop command.

======================================================================
  MOTOR RAMP STRESS TEST REPORT: FAIL
======================================================================
  Speed Range       : 50% -> 100%
  Duration Target   : 500.0 s
  Completed Steps   : 2 / 51
  DIAG Fault Active : NO (HEALTHY)
  Peak Current      : 757 mA
  Min Current       : 489 mA
  Average Current   : 623.0 mA
  Failure Detail    : Stress test cancelled by user interrupt.
======================================================================
(firmware-env)

#### Reproduction Steps

1. Connect target device via probe-rs / RTT
2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. Execute command 'motor stop'
4. Execute command 'motor speed 50'
5. Execute command 'motor current'
6. Execute command 'motor speed 51'
7. Execute command 'motor current'
8. Execute command 'motor stop'
9. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 370.869509 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 370.869710 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 375.153480 ERROR [36m[peripheral::tmc6300][0m TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 375.153541 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 380.729130 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"26\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 380.729382 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 380.733583 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 380.733627 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 380.736072 ERROR [36m[peripheral::tmc6300][0m TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 380.736129 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 380.847999 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 380.848189 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 380.962790 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 380.962950 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] RTT stream disconnected
[] [host_fs] [INFO] [1m[31merror:[0m the following required arguments were not provided:
[] [host_fs] [INFO]   [32m<OUT_CSV>[0m
[] [host_fs] [INFO] [1m[4mUsage:[0m [1mhost_fs export-telemetry[0m <OUT_CSV>
[] [host_fs] [INFO] For more information, try '[1m--help[0m'.
[] [daemon] [INFO] RTT streaming process connected
[] [daemon] [INFO] Target bug created: WORM-SILKEN-TUBEWORM-979 (Motor stalls at 35% speed) with 2 attachments
[] [target] [INFO] 0.001455 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021476 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] RTT stream disconnected
[] [host_fs] [INFO] [1m[31merror:[0m the following required arguments were not provided:
[] [host_fs] [INFO]   [32m<OUT_CSV>[0m
[] [host_fs] [INFO] [1m[4mUsage:[0m [1mhost_fs export-telemetry[0m <OUT_CSV>
[] [host_fs] [INFO] For more information, try '[1m--help[0m'.
[] [daemon] [INFO] RTT streaming process connected
[] [daemon] [INFO] Target bug created: WORM-SUPPLE-SEGMENT-373 (DIAG tripped after 260s) with 2 attachments
[] [target] [INFO] 0.001480 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021509 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 0.001568 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021574 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 40.706921 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 40.707188 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 40.956825 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 40.957068 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 40.961283 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 40.961332 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 41.075845 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 41.076083 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 50.753946 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"51\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 50.754226 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 50.758447 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 50.933379 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 50.933576 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 57.398254 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 57.398476 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 57.402755 INFO [36m[controller::motor_controller][0m Stopping motor...
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-SUBTERRANEAN-EARTHWORM-231.log](attachments/WORM-SUBTERRANEAN-EARTHWORM-231/target_system_logs_WORM-SUBTERRANEAN-EARTHWORM-231.log) | Target system and defmt logs from host daemon session |
| `reference` | [target_telemetry_WORM-SUBTERRANEAN-EARTHWORM-231.json](attachments/WORM-SUBTERRANEAN-EARTHWORM-231/target_telemetry_WORM-SUBTERRANEAN-EARTHWORM-231.json) | Target telemetry export retrieved via host_fs |

#### Resolution Notes

Resolved standstill pole-slip stall when starting motor at 50% speed. Configured startup_commutation_step_ticks = 100 in board.toml, ensuring that regardless of commanded speed, initial motor breakaway begins at 100 ticks (33.3 ms per step = 50 RPM) so the stationary rotor locks to the stator field, after which the soft-start ramp smoothly accelerates to target speed (4 ticks = 1,250 RPM). Verified on live target hardware with 50% start passing cleanly without stall.
