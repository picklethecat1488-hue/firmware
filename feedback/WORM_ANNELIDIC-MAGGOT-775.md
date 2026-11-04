# 🟢 `[WORM-ANNELIDIC-MAGGOT-775]` The RTT and system log view don’t appear when I open the App

- **UUID**: `72adfd2a-91d8-4c82-bcec-3d7c8ebf6f7e`
- **ID**: `WORM-ANNELIDIC-MAGGOT-775`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `target`
- **Created**: `2026-10-17 23:20:51 UTC`
- **Resolved**: `2026-10-17 23:25:24 UTC`

#### Description

The RTT and system log don’t appear when I open the app. When I click the Target Bug button, both panels appear, but the formatting is off

#### Reproduction Steps

1. 1. Connect target device via probe-rs / RTT
2. 2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. 3. Execute target CLI/FS commands
4. 4. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [flash] [INFO]  Zzz   /,`.-'`'    -.  ;-;;,_
[] [flash] [INFO]       |,4-  ) )-,_. ,\ (  `'-'
[] [flash] [INFO]      '---''(_/--'  `-'\_)  
[] [flash] [INFO] Type 'help' to print usage.
[] [flash] [INFO] shell> Draining buffered defmt logs:
[] [flash] [INFO] 0.001522 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.022043 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [flash] [INFO] Error: unexpected EOF during frame decoding (possible target/ELF mismatch or truncated log buffer)
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001498 INFO [36m[platform::core_monitor::configure_mpu_stack_guard][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021990 INFO [36m[platform::core_monitor::configure_mpu_stack_guard][0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=True
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.22s
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs)...
[] [daemon] [INFO] Building host utilities: host_cli and host_fs
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.23s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/release/host_cli, fs=/Users/<username>/gh/firmware/target/release/host_fs
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.07s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.10s
[] [flash] [INFO]      [1;32mFinished[0m in 7.43s
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
[] [flash] [INFO] 0.001517 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.022047 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [flash] [INFO] Error: unexpected EOF during frame decoding (possible target/ELF mismatch or truncated log buffer)
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001465 INFO [36m[platform::core_monitor::configure_mpu_stack_guard][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021985 INFO [36m[platform::core_monitor::configure_mpu_stack_guard][0m MPU stack guard configured at 0x2000c500
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-ANNELIDIC-MAGGOT-775.log](attachments/WORM-ANNELIDIC-MAGGOT-775/target_system_logs_WORM-ANNELIDIC-MAGGOT-775.log) | Target system and defmt logs from host daemon session |
| `log` | [target_crash_log_WORM-ANNELIDIC-MAGGOT-775.log](attachments/WORM-ANNELIDIC-MAGGOT-775/target_crash_log_WORM-ANNELIDIC-MAGGOT-775.log) | Target crash log retrieved via host_fs |
| `reference` | [target_telemetry_WORM-ANNELIDIC-MAGGOT-775.json](attachments/WORM-ANNELIDIC-MAGGOT-775/target_telemetry_WORM-ANNELIDIC-MAGGOT-775.json) | Target telemetry export retrieved via host_fs |
| `screenshot` | [screenshot_paste_1792279314613_97e6l.png](attachments/WORM-ANNELIDIC-MAGGOT-775/screenshot_paste_1792279314613_97e6l.png) | Pasted screenshot screenshot_paste_1792279314613_97e6l.png |

#### Resolution Notes

Closed unclosed <div> in targetBugModal automatic attachments container in tools/host_daemon/templates/host_daemon.html.j2 so workspace-body is no longer nested in targetBugModal.
