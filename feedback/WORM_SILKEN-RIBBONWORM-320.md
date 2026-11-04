# 🟢 `[WORM-SILKEN-RIBBONWORM-320]` DIAG trip coincides with drop in input voltage

- **UUID**: `d5d7326e-9066-4d4f-8b70-92c496789c10`
- **ID**: `WORM-SILKEN-RIBBONWORM-320`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-10-18 02:04:57 UTC`
- **Resolved**: `2026-10-18 02:14:15 UTC`

#### Description

This is a battery powered device and the trip in DIAG coincides with a drop in input voltage.

#### Reproduction Steps

1. 1. Connect target device via probe-rs / RTT
2. 2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. 3. Execute command 'help'
4. 4. Execute command 'motor speed 100'
5. 5. Execute command 'motor speed 100'
6. 6. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] shell> motor speed 1070.990169 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 70.990390 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 70.994574 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 70.994591 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 70.996974 ERROR [36m[peripheral::tmc6300][0m TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 70.997041 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 0.000617 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.018368 INFO [36m[peripheral::max17048][0m Reset detected (RI set). Initializing ModelGauge and triggering Quick-Start.
[] [target] [INFO] 0.027418 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [system] [INFO] Resetting target microcontroller...
[] [daemon] [INFO] RTT stream disconnected
[] [probe-rs] [INFO] [33m WARN[0m [2mprobe_rs::architecture::arm::communication_interface[0m[2m:[0m Failed to switch to DP Multidrop(11002927): An error occurred in the communication with an access port or debug port.
[] [probe-rs] [INFO] [33m WARN[0m [2mprobe_rs::architecture::arm::core::armv6m[0m[2m:[0m Expected core to be halted, but core is running
[] [system] [INFO] Target reset successfully via probe-rs
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001481 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021707 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] shell> motor speed 107.732316 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 7.732603 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 7.736873 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 7.736903 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] shell> 7.759379 ERROR [36m[peripheral::tmc6300][0m TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 7.759414 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 0.001409 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.019189 INFO [36m[peripheral::max17048][0m Reset detected (RI set). Initializing ModelGauge and triggering Quick-Start.
[] [target] [INFO] 0.028233 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [system] [INFO] Resetting target microcontroller...
[] [daemon] [INFO] RTT stream disconnected
[] [probe-rs] [INFO] [33m WARN[0m [2mprobe_rs::architecture::arm::communication_interface[0m[2m:[0m Failed to switch to DP Multidrop(11002927): An error occurred in the communication with an access port or debug port.
[] [system] [INFO] Target reset successfully via probe-rs
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001476 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021736 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [system] [INFO] Resetting target microcontroller...
[] [daemon] [INFO] RTT stream disconnected
[] [probe-rs] [INFO] [33m WARN[0m [2mprobe_rs::architecture::arm::communication_interface[0m[2m:[0m Failed to switch to DP Multidrop(11002927): An error occurred in the communication with an access port or debug port.
[] [probe-rs] [INFO] [33m WARN[0m [2mprobe_rs::architecture::arm::core::armv6m[0m[2m:[0m Expected core to be halted, but core is running
[] [probe-rs] [INFO] [33m WARN[0m [2mprobe_rs::session[0m[2m:[0m Could not clear all hardware breakpoints: An ARM specific error occurred.
[] [probe-rs] [INFO] Caused by:
[] [probe-rs] [INFO]     A timeout occurred during an operation.
[] [system] [INFO] Target reset successfully via probe-rs
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001507 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021766 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 12.147252 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 12.147532 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 12.151299 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 12.151327 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] shell> 12.173789 ERROR [36m[peripheral::tmc6300][0m TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 12.173827 INFO [36m[controller::motor_controller][0m Stopping motor...
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-SILKEN-RIBBONWORM-320.log](attachments/WORM-SILKEN-RIBBONWORM-320/target_system_logs_WORM-SILKEN-RIBBONWORM-320.log) | Target system and defmt logs from host daemon session |
| `document` | [digital.csv](attachments/WORM-SILKEN-RIBBONWORM-320/digital.csv) | Attached document (digital.csv) |

#### Resolution Notes

Disabled motor speed ramping in MotorController so target speed applies directly on command/tick. Implemented adjustable freewheeling window (default 500 us) with LowSide and Float decay modes in TMC6300 driver to recirculate and drain coil energy between commutation steps and prevent voltage drop DIAG trips. Added dedicated regression unit tests in peripheral/tests/integration.rs.
