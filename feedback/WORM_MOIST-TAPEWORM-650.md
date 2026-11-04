# 🟢 `[WORM-MOIST-TAPEWORM-650]` Single clicking on any text control should highlight the CLI text control

- **UUID**: `8c693602-b5cb-4601-97ee-c3094a5a68c7`
- **ID**: `WORM-MOIST-TAPEWORM-650`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-10-19 01:47:23 UTC`
- **Resolved**: `2026-10-21 23:25:59 UTC`

#### Reproduction Steps

1. 1. Connect target device via probe-rs / RTT
2. 2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. 3. Execute command 'motor speed 1'
4. 4. Execute command 'motor speed 1'
5. 5. Execute command 'help'
6. 6. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 7.247221 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 0.001592 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.019267 INFO [36m[peripheral::max17048][0m Reset detected (RI set). Initializing ModelGauge and triggering Quick-Start.
[] [target] [INFO] 0.022279 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] motor speed 17.527010 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"1\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 7.527302 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 7.531566 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 7.531589 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] shell> 7.544049 ERROR [36m[peripheral::tmc6300][0m TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 7.544091 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 0.001578 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021747 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 0.001596 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.019342 INFO [36m[peripheral::max17048][0m Reset detected (RI set). Initializing ModelGauge and triggering Quick-Start.
[] [target] [INFO] 0.022369 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] motor speed 18.736613 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"1\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 8.736904 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 8.741173 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 8.741195 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 8.743728 ERROR [36m[peripheral::tmc6300][0m TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 8.743816 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 0.001605 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021795 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 0.001577 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021786 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 0.001587 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021826 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 0.001570 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.019258 INFO [36m[peripheral::max17048][0m Reset detected (RI set). Initializing ModelGauge and triggering Quick-Start.
[] [target] [INFO] 0.022282 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 0.001574 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.019254 INFO [36m[peripheral::max17048][0m Reset detected (RI set). Initializing ModelGauge and triggering Quick-Start.
[] [target] [INFO] 0.022271 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [system] [INFO] Resetting target microcontroller...
[] [daemon] [INFO] RTT stream disconnected
[] [probe-rs] [INFO] [33m WARN[0m [2mprobe_rs::probe::jlink[0m[2m:[0m J-Link: Target voltage (VTref) is 0.00 V. Is your target device powered?
[] [probe-rs] [INFO] Error: Connecting to the chip was unsuccessful.
[] [probe-rs] [INFO] Caused by:
[] [probe-rs] [INFO]     0: An ARM specific error occurred.
[] [probe-rs] [INFO]     1: An error occurred in the communication with an access port or debug port.
[] [probe-rs] [INFO]     2: Target device did not respond to request.
[] [daemon] [WARN] probe-rs reset returned 1: [33m WARN[0m [2mprobe_rs::probe::jlink[0m[2m:[0m J-Link: Target voltage (VTref) is 0.00 V. Is your target device powered?
Error: Connecting to the chip was unsuccessful.
Caused by:
    0: An ARM specific error occurred.
    1: An error occurred in the communication with an access port or debug port.
    2: Target device did not respond to request.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001481 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021683 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-MOIST-TAPEWORM-650.log](attachments/WORM-MOIST-TAPEWORM-650/target_system_logs_WORM-MOIST-TAPEWORM-650.log) | Target system and defmt logs from host daemon session |

#### Resolution Notes

Configured single-click highlight and text selection across CLI text controls, prompt label, input bar, and interactive terminal command links to focus and select the CLI text control without unintended immediate execution.
