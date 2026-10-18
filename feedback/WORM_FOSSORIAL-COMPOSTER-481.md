# 🟢 `[WORM-FOSSORIAL-COMPOSTER-481]` RTT logs showing up in CLI view

- **UUID**: `71aed942-1a62-45c4-ae7b-f07bfae5cf55`
- **ID**: `WORM-FOSSORIAL-COMPOSTER-481`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-10-18 01:04:48 UTC`
- **Resolved**: `2026-10-18 01:14:52 UTC`

#### Description

There is some extra output showing up in the RTT CLI view:

shell> Draining buffered defmt logs:
0.001497 INFO MPU stack guard configured at 0x2003c000
0.021760 INFO MPU stack guard configured at 0x2000c500

#### Reproduction Steps

1. 1. Connect target device via probe-rs / RTT
2. 2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. 3. Execute command 'core status'
4. 4. Execute command 'led blink-four'
5. 5. Execute command 'led blink-four'
6. 6. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=True
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.32s
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs)...
[] [daemon] [INFO] Building host utilities: host_cli and host_fs
[] [build] [INFO] [1m[92m   Compiling[0m host_cli v0.1.0 (/Users/<username>/gh/firmware/tools/host_cli)
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 22.61s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/release/host_cli, fs=/Users/<username>/gh/firmware/target/release/host_fs
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.21s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m   Compiling[0m host_cli v0.1.0 (/Users/<username>/gh/firmware/tools/host_cli)
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 22.33s
[] [flash] [INFO]      [1;32mFinished[0m in 7.45s
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
[] [flash] [INFO] shell> Draining buffered defmt logs:
[] [flash] [INFO] 0.001497 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.021760 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001454 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021750 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] shell> core statu57.314486 INFO [36m[controller::shell_controller][0m received command "Core { subcommand: Some(Status), arg1: None }"
[] [target] [INFO] shell> 57.315143 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] led blink-four89.554177 INFO [36m[controller::shell_controller][0m received command "Led { subcommand: Some(BlinkFour), arg1: None }"
[] [target] [INFO] 89.554281 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 89.558392 INFO [36m[controller::led_controller][0m Playing animation BlinksRedFourTimes
[] [target] [INFO] shell> led blink-fou104.975500 INFO [36m[controller::shell_controller][0m received command "Led { subcommand: Some(BlinkFour), arg1: None }"
[] [target] [INFO] 104.975601 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 104.979663 INFO [36m[controller::led_controller][0m Playing animation BlinksRedFourTimes
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-FOSSORIAL-COMPOSTER-481.log](attachments/WORM-FOSSORIAL-COMPOSTER-481/target_system_logs_WORM-FOSSORIAL-COMPOSTER-481.log) | Target system and defmt logs from host daemon session |

#### Resolution Notes

Set log_to_cli=False in flash_target and run_probe_rs, and filtered out 'Draining buffered defmt logs' from CLI stream in DaemonRunner._stream_worker so defmt logs and flash output do not leak into the CLI console view.
