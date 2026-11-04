# 🟢 `[WORM-ROUND-POLYCHAETE-84]` The motor seemed to stop spinning around speed 50 here

- **UUID**: `1bba7e85-1263-4f27-b7dd-b6bd8738872b`
- **ID**: `WORM-ROUND-POLYCHAETE-84`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_detector`
- **Created**: `2026-10-24 00:56:38 UTC`
- **Resolved**: `2026-10-24 01:39:26 UTC`

#### Reproduction Steps

1. Connect target device via probe-rs / RTT
2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. Execute command 'motor current'
4. Execute command 'motor speed 59'
5. Execute command 'motor current'
6. Execute command 'motor speed 60'
7. Execute command 'motor current'
8. Execute command 'motor stop'
9. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 102.735808 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 102.736032 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 103.584575 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"52\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 103.584852 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 103.589045 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 103.703443 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 103.703670 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 104.552200 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"53\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 104.552453 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 104.556700 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 104.731546 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 104.731766 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 105.580268 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"54\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 105.580542 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 105.582676 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 105.699136 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 105.699359 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 106.547885 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"55\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 106.548138 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 106.552366 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 106.727268 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 106.727458 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 107.575971 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"56\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 107.576241 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 107.580674 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 107.755314 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 107.755535 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 108.543585 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"57\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 108.543848 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 108.548055 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 108.722975 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 108.723189 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 109.571688 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"58\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 109.571957 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 109.576343 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 109.751015 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 109.751218 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 110.599783 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"59\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 110.600059 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 110.602686 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 110.718672 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 110.718889 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 111.567409 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"60\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 111.567668 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 111.572009 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 111.746720 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 111.746927 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 112.103545 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 112.103745 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 112.108151 INFO [36m[controller::motor_controller][0m Stopping motor...
```

#### Resolution Notes

Resolved motor stall at speed 50. Eliminated single-tick (5,000 RPM) discontinuity by setting commutation_speed_divisor = 200, which maps speed 50 to 4 ticks (1,250 RPM) and speed 51 to 3 ticks (1,666 RPM). Verified on live target hardware across entire 1% to 100% speed sweep.
