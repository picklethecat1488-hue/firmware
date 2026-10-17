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

#### Behavior Comparison

- **Expected**: During motor operation, the 6-PWM bridge outputs continuous 25 kHz PWM switching across commutating phase pairs (U, V, W) at the proper electrical commutation angle (6-step trapezoidal commutation) to generate a rotating stator magnetic field. When commanded to speed 100%, the driver continuously energizes the phases according to the rotating commutation sequence.
- **Actual**: In the attached logic analyzer capture (`digital.csv`), when commanded to `motor speed 100`, only Phase U High (`UH`) and Phase V Low (`VL`) exhibit activity, pulsing briefly for 2.88 to 40 microseconds on each 10 ms (100 Hz) `motor_controller` tick. Phases UL, VH, WL, and WH remain completely unasserted (logic 0.0V) across the entire capture. The commutation sequence never advances beyond initial step 0 (`UH=1, VL=1`), failing to generate a rotating stator magnetic field, which leaves the motor stationary. Consequently, motor current consumption remains flat at standby (7 mA), and upon expiration of the 3-second startup blanking window, the controller triggers a low load / dry detection fault and halts the motor.

#### Trace Analysis (`digital.csv`)

1. **Pulse Timing & Periodicity**:
   - Pulses occur at exact ~10.0 ms intervals (e.g., $t=15.143874$, $15.153815$, $15.163800$, $15.173784$, etc.), which precisely corresponds to the 100 Hz tick rate of `motor_controller::tick_motor`.
   - Each pulse duration spans only 2.88 µs to ~40 µs before instantly returning all outputs to zero. Continuous 25 kHz PWM modulation is absent between ticks.
2. **Channel Distribution**:
   - Channel 2 (`UH`) and Channel 3 (`VL`) toggle together on every 10 ms pulse (`0,1,1,0,0,0`).
   - Channel 1 (`UL`), Channel 4 (`VH`), Channel 5 (`WL`), and Channel 6 (`WH`) remain constant 0.
   - Initial commutation step 0 uses mask `0b001001` (UH=1, VL=1). The lack of commutation advance indicates either the state machine word recycling drops on non-blocking pull, or `commutation_step` is continually reset or overridden before phase progression can occur.
3. **Current & Fault Telemetry Correlation**:
   - Standby current: 7 mA (measured by INA219).
   - Startup blanking duration: 3.0 seconds (300 ticks). At $t \approx 3.2$ s after start command, `update()` flags `Low load / dry detected (current: 7 mA)` and stops the motor.
4. **Status**:
   - Issue remains **OPEN** per user direction pending definitive hardware/gate-driver bench validation and commutation sequencer root-cause resolution.

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

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `document` | [digital.csv](attachments/WORM-EMBEDDED-LEECH-268/digital.csv) | Uploaded document digital.csv |
