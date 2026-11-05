# 🟢 `[WORM-COMPOSTED-DIGGER-386]` Motor spins half cycle, then starts ticking

- **UUID**: `6dde94f7-181b-42f0-b5da-d9e1ed4c5df5`
- **ID**: `WORM-COMPOSTED-DIGGER-386`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_detector`
- **Created**: `2026-10-23 21:37:59 UTC`
- **Resolved**: `2026-10-23 21:59:58 UTC`

#### Description

The motor spins for a half cycle, during which the motor duty cycle is 100%. After which, it starts ticking. The motor is freewheeling, so it should easily be able to spin.

#### Reproduction Steps

1. Connect target device via probe-rs / RTT
2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. Execute command 'motor speed 30'
4. Execute command 'motor stop'
5. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=True
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.28s
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs)...
[] [daemon] [INFO] Building host utilities: host_cli and host_fs
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.30s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/release/host_cli, fs=/Users/<username>/gh/firmware/target/release/host_fs
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.06s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.10s
[] [flash] [INFO]      [1;32mFinished[0m in 9.04s
[] [flash] [INFO] RTT connected. Active channels:
[] [flash] [INFO]   - Up Channel: "defmt" (logs)
[] [flash] [INFO]   - Up Channel: "cli" (raw output)
[] [flash] [INFO]   - Down Channel: "cli" (raw input)
[] [flash] [INFO] shell> 
[] [flash] [INFO] [2K
[] [flash] [INFO]        |\      _,,,---,,_
[] [flash] [INFO]  Zzz   /,`.-'`'    -.  ;-;;,_
[] [flash] [INFO]       |,4-  ) )-,_. ,\ (  `'-'
[] [flash] [INFO]      '---''(_/--'  `-'\_)  
[] [flash] [INFO] Type 'help' to print usage.
[] [flash] [INFO] shell> 
[] [flash] [INFO] Draining buffered defmt logs:
[] [flash] [INFO] 0.001465 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.021943 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001511 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021962 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] 15.171041 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 15.171341 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 15.174620 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 15.174649 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 18.608212 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 18.608440 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 18.617265 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 20.430521 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"30\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 20.430773 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 20.434952 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 20.434976 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 22.781054 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 22.781263 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 22.787425 INFO [36m[controller::motor_controller][0m Stopping motor...
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `reference` | [target_telemetry_WORM-COMPOSTED-DIGGER-386.json](attachments/WORM-COMPOSTED-DIGGER-386/target_telemetry_WORM-COMPOSTED-DIGGER-386.json) | Target telemetry export retrieved via host_fs |
| `reference` | [IMG_0202.MOV](attachments/WORM-COMPOSTED-DIGGER-386/IMG_0202.MOV) | Uploaded reference IMG_0202.MOV |
| `log` | [target_system_logs_WORM-COMPOSTED-DIGGER-386.log](attachments/WORM-COMPOSTED-DIGGER-386/target_system_logs_WORM-COMPOSTED-DIGGER-386.log) | Target system and defmt logs from host daemon session |

#### Resolution Notes

Configured max_commutation_step_ticks = 2 in board.toml to guarantee open-loop commutation step rate remains >= 50 Hz, avoiding the 22-35 Hz cogging detent resonance band that caused rotor ticking at speed 30. Implemented 100% duty cycle breakaway torque boost during startup alignment with gradual slowdown (2% per tick) down to commanded speed. Added dedicated regression unit tests in controller_tests.rs.
