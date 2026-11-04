# 🟢 `[WORM-SLIMY-ROOTER-821]` Can we hide system logs by default?

- **UUID**: `828fa6c6-c69a-4d68-a463-f416584d8e3f`
- **ID**: `WORM-SLIMY-ROOTER-821`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `Host`
- **Created**: `2026-10-23 00:14:52 UTC`
- **Resolved**: `2026-10-23 00:23:10 UTC`

#### Description

In the host UI, can we hide system logs by default and add an option to show them on startup?

#### Reproduction Steps

1. 1. Connect target device via probe-rs / RTT
2. 2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. 3. Execute command 'motor speed 50'
4. 4. Execute command 'motor speed 100'
5. 5. Execute command 'motor speed 30'
6. 6. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001495 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021896 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] motor speed 105.976777 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"10\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 5.977098 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 5.981394 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 5.981435 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] shell> motor current10.812830 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 10.813060 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor speed 3020.793343 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"30\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 20.793586 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 20.803978 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 26.778474 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 26.778680 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 37.182346 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 37.182574 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 37.193973 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] shell> motor speed 1047.828116 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 47.828335 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 47.833991 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] shell> motor speed 3053.571346 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"30\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 53.571567 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 53.583980 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [daemon] [INFO] Target bug created: WORM-DAMP-MEALWORM-592 (Motor oscillates at speed 30) with 1 attachments
[] [target] [INFO] shell> motor speed 3363.026490 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"30\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 363.026749 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor current364.173550 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 364.173754 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 365.314441 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Rpm), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 365.314634 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor speed 100375.184193 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 375.184413 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 375.193989 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 376.329200 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 376.329380 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor rp377.470129 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Rpm), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 377.470331 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor sto386.301630 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 386.301825 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> 386.313980 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [daemon] [INFO] Target bug created: WORM-VERMIFORM-REDWORM-38 (I see some extra output in the logs here) with 1 attachments
[] [target] [INFO] shell> motor current447.993046 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 447.993258 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor rp449.073522 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Rpm), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 449.073695 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> gpio read 26456.094805 INFO [36m[controller::shell_controller][0m received command "Gpio { subcommand: Some(Read), pin: Some(26), value: None }"
[] [target] [INFO] 456.094974 INFO [36m[controller::shell_controller][0m command execution succeeded
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-SLIMY-ROOTER-821.log](attachments/WORM-SLIMY-ROOTER-821/target_system_logs_WORM-SLIMY-ROOTER-821.log) | Target system and defmt logs from host daemon session |
