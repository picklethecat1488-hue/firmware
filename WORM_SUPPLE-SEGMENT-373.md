# 🟢 `[WORM-SUPPLE-SEGMENT-373]` DIAG tripped after 260s

- **UUID**: `d8a043b6-7547-461e-b6ab-401576b60aff`
- **ID**: `WORM-SUPPLE-SEGMENT-373`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_detector`
- **Created**: `2026-10-24 01:03:39 UTC`
- **Resolved**: `2026-10-24 01:30:50 UTC`
- **Accepted**: `2026-10-24 01:30:50 UTC`

#### Description

DIAG tripped after 260s, see below:

$  python tools/motor_stress.py --start 1 --end 50 --duration 500
Starting motor ramp stress <username>: 1% -> 50% over 500.0s
Connecting to host daemon at http://127.0.0.1:8899...
[  1/50] ( 10.0s) Speed:   1% | Current:   58 mA | DIAG: OK
[  2/50] ( 20.0s) Speed:   2% | Current:  341 mA | DIAG: OK
[  3/50] ( 30.0s) Speed:   3% | Current:  742 mA | DIAG: OK
[  4/50] ( 40.0s) Speed:   4% | Current:  721 mA | DIAG: OK
[  5/50] ( 50.0s) Speed:   5% | Current:  740 mA | DIAG: OK
[  6/50] ( 60.0s) Speed:   6% | Current:  729 mA | DIAG: OK
[  7/50] ( 70.0s) Speed:   7% | Current:  722 mA | DIAG: OK
[  8/50] ( 80.0s) Speed:   8% | Current:  738 mA | DIAG: OK
[  9/50] ( 90.0s) Speed:   9% | Current:  737 mA | DIAG: OK
[ 10/50] (100.0s) Speed:  10% | Current:  735 mA | DIAG: OK
[ 11/50] (110.0s) Speed:  11% | Current:  762 mA | DIAG: OK
[ 12/50] (120.0s) Speed:  12% | Current:  724 mA | DIAG: OK
[ 13/50] (130.0s) Speed:  13% | Current:  737 mA | DIAG: OK
[ 14/50] (140.0s) Speed:  14% | Current:  761 mA | DIAG: OK
[ 15/50] (150.0s) Speed:  15% | Current:  762 mA | DIAG: OK
[ 16/50] (160.0s) Speed:  16% | Current:  760 mA | DIAG: OK
[ 17/50] (170.0s) Speed:  17% | Current:  745 mA | DIAG: OK
[ 18/50] (180.0s) Speed:  18% | Current:  743 mA | DIAG: OK
[ 19/50] (190.0s) Speed:  19% | Current:  722 mA | DIAG: OK
[ 20/50] (200.0s) Speed:  20% | Current:  747 mA | DIAG: OK
[ 21/50] (210.0s) Speed:  21% | Current:  742 mA | DIAG: OK
[ 22/50] (220.0s) Speed:  22% | Current:  756 mA | DIAG: OK
[ 23/50] (230.0s) Speed:  23% | Current:  781 mA | DIAG: OK
[ 24/50] (240.0s) Speed:  24% | Current:  737 mA | DIAG: OK
[ 25/50] (250.0s) Speed:  25% | Current:  763 mA | DIAG: OK
[ 26/50] (260.0s) Speed:  26% | Current:    0 mA | DIAG: TRIPPED!
Sent motor stop command.

======================================================================
  MOTOR RAMP STRESS <username> REPORT: FAIL
======================================================================
  Speed Range       : 1% -> 50%
  Duration Target   : 500.0 s
  Completed Steps   : 26 / 50
  DIAG Fault Active : YES (TRIPPED)
  Peak Current      : 781 mA
  Min Current       : 0 mA
  Average Current   : 673.3 mA
  Failure Detail    : DIAG fault tripped at speed 26% (0 mA): 380.736072 ERROR [peripheral::tmc6300] TMC6300 PIO DIAG pin asserted: driver fault detected
======================================================================
(firmware-env)

#### Reproduction Steps

1. Connect target device via probe-rs / RTT
2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. Execute command 'motor speed 25'
4. Execute command 'motor current'
5. Execute command 'motor speed 26'
6. Execute command 'motor current'
7. Execute command 'motor stop'
8. Execute command 'fs export-telemetry'
9. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 320.676854 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 320.681053 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 320.855938 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 320.856112 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 330.715571 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"21\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 330.715856 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 330.720306 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 330.834433 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 330.834651 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 340.694120 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"22\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 340.694378 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 340.698652 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 340.812977 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 340.813151 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 350.672592 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"23\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 350.672861 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 350.677321 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 350.851968 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 350.852193 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 360.711629 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"24\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 360.711888 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 360.716281 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 360.830459 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 360.830662 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 370.690154 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"25\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 370.690432 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 370.694636 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
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
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-SUPPLE-SEGMENT-373.log](attachments/WORM-SUPPLE-SEGMENT-373/target_system_logs_WORM-SUPPLE-SEGMENT-373.log) | Target system and defmt logs from host daemon session |
| `reference` | [target_telemetry_WORM-SUPPLE-SEGMENT-373.json](attachments/WORM-SUPPLE-SEGMENT-373/target_telemetry_WORM-SUPPLE-SEGMENT-373.json) | Target telemetry export retrieved via host_fs |

#### Resolution Notes

Resolved thermal DIAG tripping during extended ramp stress tests. Tightened closed-loop current regulation to regulate within 450..=550 mA (nominal 500 mA) with CLOSED_LOOP_REGULATION_CEILING_MA = 550. When current exceeds 550 mA under load, duty cycle is actively throttled down by 5% per cycle, preventing sustained ~760 mA coil current and keeping driver temperature well below the 150°C thermal shutdown threshold.
