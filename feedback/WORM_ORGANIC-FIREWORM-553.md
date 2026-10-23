# 🟢 `[WORM-ORGANIC-FIREWORM-553]` Motor oscillates back and forth

- **UUID**: `be4d32c9-30f4-4e1b-8780-630daf5bebc5`
- **ID**: `WORM-ORGANIC-FIREWORM-553`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-10-22 04:19:58 UTC`
- **Resolved**: `2026-10-23 00:33:35 UTC`

#### Description

motor speed 10-oscillating
motor speed 30-oscillating
motor speed 50-oscillating
motor speed 100-rotates slowly, and eventually trips drag

#### Reproduction Steps

1. 1. Connect target device via probe-rs / RTT
2. 2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. 3. Execute command 'motor speed 20'
4. 4. Execute command 'motor current'
5. 5. Execute command 'help'
6. 6. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] shell> motor curren166.538159 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 166.538357 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> gpio read 26167.564282 INFO [36m[controller::shell_controller][0m received command "Gpio { subcommand: Some(Read), pin: Some(26), value: None }"
[] [target] [INFO] 167.564476 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 168.594361 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 168.594565 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor stop169.555882 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 169.556071 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> 169.567348 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] shell> motor speed 3518.812294 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"30\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 518.812556 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> 518.816747 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 518.816775 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] motor curren525.160374 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 525.160584 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor rpm529.325034 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Rpm), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 529.325241 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor stop533.378964 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 533.379157 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> 533.389201 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] motor speed 100632.750985 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 632.751221 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 632.755420 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 632.755444 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] shell> motor rpm635.702208 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Rpm), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 635.702420 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor stop640.239891 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 640.240088 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> 640.247849 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 1012.279178 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"10\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 1012.279419 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 1012.282834 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 1012.282862 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 1073.904214 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"30\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 1073.904456 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 1073.915259 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 1080.131206 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 1080.131425 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 1135.045349 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 1135.045583 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> 1135.055263 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 1172.965759 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 1172.966001 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> 1172.975274 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 1189.471612 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 1189.471815 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> 1214.715253 ERROR [36m[peripheral::tmc6300][0m TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 1214.715313 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 1226.960627 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 1226.960824 INFO [36m[controller::shell_controller][0m command execution succeeded
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `document` | [motor_speed_10.MOV](attachments/WORM-ORGANIC-FIREWORM-553/motor_speed_10.MOV) | Attached document (motor_speed_10.MOV) |
| `document` | [motor_speed_30.MOV](attachments/WORM-ORGANIC-FIREWORM-553/motor_speed_30.MOV) | Attached document (motor_speed_30.MOV) |
| `document` | [motor_speed_100.MOV](attachments/WORM-ORGANIC-FIREWORM-553/motor_speed_100.MOV) | Attached document (motor_speed_100.MOV) |
| `document` | [motor_speed_50.MOV](attachments/WORM-ORGANIC-FIREWORM-553/motor_speed_50.MOV) | Attached document (motor_speed_50.MOV) |
| `log` | [target_system_logs_WORM-ORGANIC-FIREWORM-553.log](attachments/WORM-ORGANIC-FIREWORM-553/target_system_logs_WORM-ORGANIC-FIREWORM-553.log) | Target system and defmt logs from host daemon session |

#### Resolution Notes

Eliminated low-frequency detent oscillations across speeds 10, 30, and 50 and eliminated thermal DIAG trip at speed 100 by running at 500us ticks with canonical PWM frequency modulation. Verified on target hardware across all speed commands. Codified regression test in test_worm_organic_fireworm_553_motor_speeds_and_diag.py.
