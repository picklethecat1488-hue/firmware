# 🟢 `[WORM-SETOSE-TUBEWORM-177]` Motor vibrates, does not spin

- **UUID**: `19a0571e-86b8-42e9-97bb-4b8ce6030007`
- **ID**: `WORM-SETOSE-TUBEWORM-177`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-10-21 23:57:15 UTC`
- **Resolved**: `2026-10-22 00:18:42 UTC`

#### Description

See below traceback and logs. The motor vibrates, but hall sensor reports no movement, and motor does not physically turn when disconnected from the motor hub.

#### Execution / Console Logs

```text
$ PROBE_RS_SPEED=4000 cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release 
    Finished `release` profile [optimized + debuginfo] target(s) in 0.08s
     Running `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell`
    Finished `release` profile [optimized + debuginfo] target(s) in 0.12s
      Erasing ✔ 100% [####################] 168.00 KiB @  59.29 KiB/s (took 3s)
  Programming ✔ 100% [####################] 168.00 KiB @  44.15 KiB/s (took 4s)     Finished in 7.40s
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

0.001495 INFO [platform::core_monitor] MPU stack guard configured at 0x2003c000
0.021665 INFO [platform::core_monitor] MPU stack guard configured at 0x2000c500
motor speed 10
3.073703 INFO [controller::shell_controller] received command "Motor { subcommand: Some(Speed), arg1: Some(\"10\"), arg2: None, arg3: None, partition: None }"
3.074002 INFO [controller::shell_controller] command execution succeeded

3.078277 WARN [controller::motor_controller] Warning, calibration is not present! Running with fallback limits.
3.078300 INFO [controller::motor_controller] Starting motor...
Motor current: 7 mA

shell> motor current
5.672100 INFO [controller::shell_controller] received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
5.672321 INFO [controller::shell_controller] command execution succeeded

Motor current: 81 mA

shell> motor current
7.607271 INFO [controller::shell_controller] received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
7.607457 INFO [controller::shell_controller] command execution succeeded

Motor current: 87 mA

shell> motor rpm
9.111088 INFO [controller::shell_controller] received command "Motor { subcommand: Some(Rpm), arg1: None, arg2: None, arg3: None, partition: None }"
9.111256 INFO [controller::shell_controller] command execution succeeded

Motor speed: 0 RPM

shell> motor rpm
10.381057 INFO [controller::shell_controller] received command "Motor { subcommand: Some(Rpm), arg1: None, arg2: None, arg3: None, partition: None }"
10.381240 INFO [controller::shell_controller] command execution succeeded

Motor speed: 0 RPM

shell> motor speed 0
14.441079 INFO [controller::shell_controller] received command "Motor { subcommand: Some(Speed), arg1: Some(\"0\"), arg2: None, arg3: None, partition: None }"
14.441307 INFO [controller::shell_controller] command execution succeeded

Motor current: 0 mA

shell> 14.450792 INFO [controller::motor_controller] Stopping motor...
^C
(firmware-env)
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `document` | [digital.csv](attachments/WORM-SETOSE-TUBEWORM-177/digital.csv) | Uploaded document digital.csv |

#### Resolution Notes

Scaled open-loop commutation step rate with commanded speed and implemented soft-start acceleration ramp. Updated HallSensor to periodically compute live RPM.
