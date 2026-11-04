# 🟢 `[WORM-BRISTLY-BURROWER-873]` Current sensor does not appear to be workign right.

- **UUID**: `72775417-f6f4-4d42-8d88-9c376d796de5`
- **ID**: `WORM-BRISTLY-BURROWER-873`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_fountain`
- **Created**: `2026-10-15 23:41:18 UTC`
- **Resolved**: `2026-10-15 23:57:55 UTC`

#### Description

From my understanding, the TMC6300 part draws max 2A current before tripping DIAG, however the current sensor cannot detect any change in current level

#### Reproduction Steps

1. 1. call `motor speed 1`
2. 2. call `motor speed 2`
3. 3. DIAG light pops on

#### Behavior Comparison

- **Expected**: At a motor speed of 1, DIAG light does not come on, yet current measurement appears to be the quiescent current level, not even close to 2A
- **Actual**: Current measurement should be at or neat the current limit

#### Execution / Console Logs

```text
shell> motor speed 1
88.281766 INFO [controller::shell_controller::log_command] received command "Motor { subcommand: Some(Speed), arg1: Some(\"1\"), arg2: None, arg3: None, partition: None }"
88.281952 INFO [controller::shell_controller::log_success] command execution succeeded
88.286118 WARN [controller::motor_controller::{impl#5}::handle_command::{async_fn#0}] Motor Controller: Warning, calibration is not present! Running with fallback limits.
88.286151 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Starting motor...

Motor current: 75 mA

shell> motor current
99.288506 INFO [controller::shell_controller::log_command] received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
99.288727 INFO [controller::shell_controller::log_success] command execution succeeded

Motor current: 71 mA

shell> motor speed 2
105.275616 INFO [controller::shell_controller::log_command] received command "Motor { subcommand: Some(Speed), arg1: Some(\"2\"), arg2: None, arg3: None, partition: None }"
105.275851 INFO [controller::shell_controller::log_success] command execution succeeded
105.288464 WARN [controller::motor_controller::{impl#5}::handle_command::{async_fn#0}] Motor Controller: Warning, calibration is not present! Running with fallback limits.

Motor current: 75 mA

shell> motor current
111.081261 INFO [controller::shell_controller::log_command] received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
111.081446 INFO [controller::shell_controller::log_success] command execution succeeded

Motor current: 69 mA
```

#### Resolution Notes

Fixed INA219 current sensor scaling from raw 0.1 mA counts to integer mA (dividing by CURRENT_LSB_DIVISOR = 10), added Register::SHUNT_VOLTAGE fallback with automatic calibration recovery if CALIBRATION register is lost/cleared, and ensured CALIBRATION is reprogrammed to DEFAULT_CALIBRATION (4096) on mode changes in set_measurement_mode.
