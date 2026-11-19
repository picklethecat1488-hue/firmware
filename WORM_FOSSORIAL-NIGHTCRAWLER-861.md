# 🟢 `[WORM-FOSSORIAL-NIGHTCRAWLER-861]` Overlapping text in the test mode

- **UUID**: `be04b6b2-67fc-4798-ac26-a2bf20cf3299`
- **ID**: `WORM-FOSSORIAL-NIGHTCRAWLER-861`
- **Status**: `RESOLVED`
- **Severity**: `LOW`
- **Category**: `INFRASTRUCTURE`
- **Component**: `target`
- **Created**: `2026-11-19 03:41:27 UTC`
- **Resolved**: `2026-11-19 03:54:38 UTC`

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_shell
2. Execute target command via python tools/host.py cli <command>
3. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/debug/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs)...
[] [daemon] [INFO] Building host utilities: host_cli and host_fs
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [optimized + debuginfo]]8;;\ target(s) in 0.36s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/debug/host_cli, fs=/Users/<username>/gh/firmware/target/debug/host_fs
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [optimized + debuginfo]]8;;\ target(s) in 0.07s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/debug/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.22s
[] [flash] [INFO]      [1;32mFinished[0m in 12.24s
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
[] [flash] [INFO] 0.002191 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.023222 [32mINFO[0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002187 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023241 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [system] [INFO] Switching to Test mode (on-device @device_test runner active)...
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002175 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023236 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002198 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023242 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002204 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023236 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002194 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.029274 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [manual_test] [INFO] [MANUAL TEST: test_motor_calibration_routine] Procedure: Ensure fountain reservoir is filled above minimum waterline before starting calibration. | Expected: Is the reservoir filled and clear of obstructions?
```

#### Review Tasks & Functional Checklist

- [ ] Launch host.py in Test Mode and verify Mode/Scenarios and Status badges render as non-overlapping pills across full and split widths | `python tools/host.py --no-detach` <!-- id:bb45b342 -->

#### Resolution Notes

Converted .badge to inline-block with explicit line-height and nowrap, added table cell vertical-align/line-height spacing, and wrapped Mode/Scenarios and Status cells in .test-badge-group flex containers with discrete scenario pills.
