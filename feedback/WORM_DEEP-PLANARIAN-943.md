# 🟢 `[WORM-DEEP-PLANARIAN-943]` GPIO pin states do not persist

- **UUID**: `ae470e71-22f8-42bb-a6c1-edacde58492c`
- **ID**: `WORM-DEEP-PLANARIAN-943`
- **Status**: `RESOLVED`
- **Severity**: `CRITICAL`
- **Category**: `FIRMWARE`
- **Component**: `cat_fountain`
- **Created**: `2026-10-17 00:24:41 UTC`
- **Resolved**: `2026-10-17 00:50:39 UTC`

#### Description

When I set GPIO pin states to high, the states only assert on my logic analyzer for around 44 uS. See attached screenshots below, and execution logs.

#### Reproduction Steps

1. Set all motor pin states to 1, see traceback below

#### Behavior Comparison

- **Expected**: Pin states should stay asserted for the trace duration
- **Actual**: Pin states only stay asserted for a very brief amount of time

#### Execution / Console Logs

```text
shell> gpio set 16 1
19.883935 INFO [controller::shell_controller::log_command] received command "Gpio { subcommand: Some(Set), pin: Some(16), value: Some(1) }"
19.884109 INFO [controller::shell_controller::log_success] command execution succeeded
GP16 set to High (1)

shell> gpio set 17 1
26.112928 INFO [controller::shell_controller::log_command] received command "Gpio { subcommand: Some(Set), pin: Some(17), value: Some(1) }"
26.113088 INFO [controller::shell_controller::log_success] command execution succeeded
GP17 set to High (1)

shell> gpio set 18 1
29.499576 INFO [controller::shell_controller::log_command] received command "Gpio { subcommand: Some(Set), pin: Some(18), value: Some(1) }"
29.499724 INFO [controller::shell_controller::log_success] command execution succeeded
GP18 set to High (1)

shell> gpio set 19 1
33.370025 INFO [controller::shell_controller::log_command] received command "Gpio { subcommand: Some(Set), pin: Some(19), value: Some(1) }"
33.370175 INFO [controller::shell_controller::log_success] command execution succeeded
GP19 set to High (1)

shell> gpio set 20 1
36.091446 INFO [controller::shell_controller::log_command] received command "Gpio { subcommand: Some(Set), pin: Some(20), value: Some(1) }"
36.091592 INFO [controller::shell_controller::log_success] command execution succeeded
GP20 set to High (1)

shell> gpio set 21 1
42.078562 INFO [controller::shell_controller::log_command] received command "Gpio { subcommand: Some(Set), pin: Some(21), value: Some(1) }"
42.078705 INFO [controller::shell_controller::log_success] command execution succeeded
GP21 set to High (1)
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [screenshot_paste_1792196789832_c62x7.png](attachments/WORM-DEEP-PLANARIAN-943/screenshot_paste_1792196789832_c62x7.png) | Pasted screenshot screenshot_paste_1792196789832_c62x7.png |
| `screenshot` | [screenshot_paste_1792196806704_olcff.png](attachments/WORM-DEEP-PLANARIAN-943/screenshot_paste_1792196806704_olcff.png) | Pasted screenshot screenshot_paste_1792196806704_olcff.png |
