# 🟢 `[WORM-SILKEN-TUBEWORM-979]` Motor stalled out at 35% speed

- **UUID**: `94be444c-f49e-4090-8774-8013b502b958`
- **ID**: `WORM-SILKEN-TUBEWORM-979`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `GENERAL`
- **Component**: `cat_detector`
- **Created**: `2026-10-24 01:03:16 UTC`
- **Resolved**: `2026-10-24 01:39:23 UTC`
- **Accepted**: `2026-10-24 01:39:23 UTC`

#### Description

When I run this command, the motor stalls at 35% speed:

$  python tools/motor_stress.py --start 1 --end 50 --duration 50

I could get the motor to recover by pushing on it.

#### Reproduction Steps

1. Connect target device via probe-rs / RTT
2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. Execute command 'motor speed 37'
4. Execute command 'motor current'
5. Execute command 'motor speed 38'
6. Execute command 'motor current'
7. Execute command 'motor speed 39'
8. Execute command 'motor current'
9. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 300.659311 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 300.663727 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 300.838394 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 300.838607 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 310.698305 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 310.702681 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 310.816939 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 310.817143 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 320.676570 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"20\"), arg2: None, arg3: None, partition: None }"
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
```

#### Resolution Notes

Resolved motor stalling at 35% speed. Configured commutation_speed_divisor = 200 and min_commutation_step_ticks = 2 in board.toml, scaling speed 35 to 5 ticks (1,000 RPM) and capping maximum speed at 2,500 RPM (2 ticks). Verified smooth continuous rotation with full 1% to 100% ramp stress <username> on hardware.
