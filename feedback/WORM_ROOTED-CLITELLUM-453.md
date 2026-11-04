# 🟢 `[WORM-ROOTED-CLITELLUM-453]` Parse the target function namespace intelligently

- **UUID**: `f0c204b3-311b-4b82-bf7f-9589d85ce805`
- **ID**: `WORM-ROOTED-CLITELLUM-453`
- **Status**: `RESOLVED`
- **Severity**: `LOW`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host_cli`
- **Created**: `2026-10-17 23:35:15 UTC`
- **Resolved**: `2026-10-18 00:21:26 UTC`

#### Description

When outputting logs via host_cli; we currently output the full function name of the log PC. Can we reformat this as in the below, and clean up some of the unneeded module prefixes in the existing logging. See example below

#### Behavior Comparison

- **Expected**: Ideally, we could get crate and module name only:
- **Actual**: We instead get:

#### Execution / Console Logs

```text
.991738 ERROR [peripheral::tmc6300::{impl#7}::tick::{async_fn#0}] TMC6300 PIO DIAG pin asserted: driver fault detected
6.001735 ERROR [peripheral::tmc6300::{impl#7}::tick::{async_fn#0}] TMC6300 PIO DIAG pin asserted: driver fault detected
6.011735 ERROR [peripheral::tmc6300::{impl#7}::tick::{async_fn#0}] TMC6300 PIO DIAG pin asserted: driver fault detected
6.021736 ERROR [peripheral::tmc6300::{impl#7}::tick::{async_fn#0}] TMC6300 PIO DIAG pin asserted: driver fault detected
6.031738 ERROR [peripheral::tmc6300::{impl#7}::tick::{async_fn#0}] TMC6300 PIO DIAG pin asserted: driver fault detected
6.041736 ERROR [peripheral::tmc6300::{impl#7}::tick::{async_fn#0}] TMC6300 PIO DIAG pin asserted: driver fault detected
6.042199 INFO [controller::motor_controller::{impl#5}::set_running_state::{async_fn#0}] Motor Controller: Stopping motor...
6.042660 WARN [controller::motor_controller::{impl#5}::update::{async_fn#0}] Motor Controller: Low load / dry detected (current: 7 mA). Stopped motor.
gpio set 19 1
gpio set 19 1
43.168874 INFO [controller::shell_controller::log_command] received command "Gpio { subcommand: Some(Set), pin: Some(19), value: Some(1) }"
43.169210 INFO [controller::shell_controller::log_success] command execution succeeded
GP19 set to High (1)
```

#### Resolution Notes

Implemented intelligent target namespace formatting in host_cli (format_target_namespace) to strip compiler impl/closure tokens and function names down to crate and module paths only, updated RTT log decoration and tracing spans, cleaned up redundant Controller prefixes in firmware logging, and added comprehensive Rust and Python regression tests.
