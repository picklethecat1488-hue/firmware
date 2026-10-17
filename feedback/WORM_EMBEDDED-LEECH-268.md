# 🔴 `[WORM-EMBEDDED-LEECH-268]` Motor does not spin

- **UUID**: `f7f59f93-5829-427e-b5ec-301b2de7486b`
- **ID**: `WORM-EMBEDDED-LEECH-268`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_fountain`
- **Created**: `2026-10-16 01:24:25 UTC`

#### Description

Motor does not spin, current readings are constant at 7 mA. See tracebacks below

#### Execution / Console Logs

```text
6.821608 INFO [controller::shell_controller::log_success] command execution succeeded
6.824300 WARN [controller::motor_controller::{impl#5}::handle_command::{async_fn#0}] Motor Controller: Warning, calibration is not present! Running with fallback limits.
6.824331 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Starting motor...

Motor current: 7 mA

shell> motor10.027180 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Stopping motor...
10.027651 WARN [controller::motor_controller::{impl#5}::update::{async_fn#0}] Motor Controller: Low load / dry detected (current: 7 mA). Stopped motor.
motor speed 10
Unknown command: 'motormotor'

shell> motor speed 10
21.639727 INFO [controller::shell_controller::log_command] received command "Motor { subcommand: Some(Speed), arg1: Some(\"10\"), arg2: None, arg3: None, partition: None }"
21.639977 INFO [controller::shell_controller::log_success] command execution succeeded
21.644206 WARN [controller::motor_controller::{impl#5}::handle_command::{async_fn#0}] Motor Controller: Warning, calibration is not present! Running with fallback limits.
21.644233 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Starting motor...

Motor current: 7 mA

shell> 25.047018 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Stopping motor...
25.047488 WARN [controller::motor_controller::{impl#5}::update::{async_fn#0}] Motor Controller: Low load / dry detected (current: 7 mA). Stopped motor.
motor speed 100
28.233725 INFO [controller::shell_controller::log_command] received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
28.233986 INFO [controller::shell_controller::log_success] command execution succeeded
28.238202 WARN [controller::motor_controller::{impl#5}::handle_command::{async_fn#0}] Motor Controller: Warning, calibration is not present! Running with fallback limits.
28.238227 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Starting motor...

Motor current: 7 mA

shell> 32.061002 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Stopping motor...
32.061470 WARN [controller::motor_controller::{impl#5}::update::{async_fn#0}] Motor Controller: Low load / dry detected (current: 7 mA). Stopped motor.
motor stop
45.277803 INFO [controller::shell_controller::log_command] received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
45.278027 INFO [controller::shell_controller::log_success] command execution succeeded

shell> motor speed 100
47.888373 INFO [controller::shell_controller::log_command] received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
47.888620 INFO [controller::shell_controller::log_success] command execution succeeded

Motor current: 7 mA

shell> 47.892833 WARN [controller::motor_controller::{impl#5}::handle_command::{async_fn#0}] Motor Controller: Warning, calibration is not present! Running with fallback limits.
47.892858 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Starting motor...
51.075608 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Stopping motor...
51.076074 WARN [controller::motor_controller::{impl#5}::update::{async_fn#0}] Motor Controller: Low load / dry detected (current: 7 mA). Stopped motor.
motor stop
53.563057 INFO [controller::shell_controller::log_command] received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
53.563289 INFO [controller::shell_controller::log_success] command execution succeeded

shell>
```
