# 🟢 `[WORM-DIRT-RIBBONWORM-196]` Motor speed 100- no GPIO activity

- **UUID**: `5195ac5b-7e6d-4b01-b4f1-7261dd111e21`
- **ID**: `WORM-DIRT-RIBBONWORM-196`
- **Status**: `RESOLVED`
- **Severity**: `CRITICAL`
- **Category**: `FIRMWARE`
- **Component**: `cat_fountain`
- **Created**: `2026-10-17 00:29:58 UTC`
- **Resolved**: `2026-10-17 00:56:20 UTC`

#### Description

When I set motor speed to 100, there is no GPIO activity for the trace duration. See tracebacks and log below

#### Execution / Console Logs

```text
RTT connected. Active channels:
  - Up Channel: "defmt" (logs)
  - Up Channel: "cli" (raw output)
  - Down Channel: "cli" (raw input)


       |\      _,,,---,,_
 Zzz   /,`.-'`'    -.  ;-;;,_
      |,4-  ) )-,_. ,\ (  `'-'
     '---''(_/--'  `-'\_)  

Type 'help' to print usage.

shell> 
RTT Session Started (Press Ctrl+C to exit):
Interactive RTT console active. Type commands and press Enter.

0.001488 INFO [platform::core_monitor::configure_mpu_stack_guard] MPU stack guard configured at 0x2003c000
0.021711 INFO [platform::core_monitor::configure_mpu_stack_guard] MPU stack guard configured at 0x2000c500
motor speed 100
31.559932 INFO [controller::shell_controller::log_command] received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
31.560240 INFO [controller::shell_controller::log_success] command execution succeeded
31.564541 WARN [controller::motor_controller::{impl#5}::handle_command::{async_fn#0}] Motor Controller: Warning, calibration is not present! Running with fallback limits.
31.564570 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Starting motor...

Motor current: 7 mA

shell> 35.037432 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Stopping motor...
35.037908 WARN [controller::motor_controller::{impl#5}::update::{async_fn#0}] Motor Controller: Low load / dry detected (current: 6 mA). Stopped motor.
motor speed 10
38.270643 INFO [controller::shell_controller::log_command] received command "Motor { subcommand: Some(Speed), arg1: Some(\"10\"), arg2: None, arg3: None, partition: None }"
38.270913 INFO [controller::shell_controller::log_success] command execution succeeded
38.275143 WARN [controller::motor_controller::{impl#5}::handle_command::{async_fn#0}] Motor Controller: Warning, calibration is not present! Running with fallback limits.
38.275168 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Starting motor...

Motor current: 6 mA

shell> 42.037927 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Stopping motor...
42.038394 WARN [controller::motor_controller::{impl#5}::update::{async_fn#0}] Motor Controller: Low load / dry detected (current: 7 mA). Stopped motor.
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [screenshot_paste_1792197013615_985yn.png](attachments/WORM-DIRT-RIBBONWORM-196/screenshot_paste_1792197013615_985yn.png) | Pasted screenshot screenshot_paste_1792197013615_985yn.png |
