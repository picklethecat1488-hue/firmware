# 🟢 `[WORM-DAMP-MEALWORM-592]` Motor oscillates at speed 30

- **UUID**: `1f9d77de-2617-4812-868f-38f5d858609d`
- **ID**: `WORM-DAMP-MEALWORM-592`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-10-23 00:03:42 UTC`
- **Resolved**: `2026-10-23 00:33:25 UTC`

#### Description

Some additional context here:

  ### Remediation Plan

  1. **Torque Floor & Duty Scaling in calculate_pwm_counts**:
      • Modulate OFF-count within a window that sustains sufficient inductive current to overcome physical cogging detents (> 150 mA)
      while preserving individual FET ON-pulse clamping (t_{ON} ≤ 800 ns) so the TMC6300 t_{S2G} comparator never trips.
  2. **Commutation Step Scaling & Soft-Start Ramp in MotorController**:
      • Align calculate_commutation_step_ticks and the soft-start acceleration ramp so the motor starts reliably and scales smoothly into
      continuous rotation without pole slipping.
  3. Hardware Verification:
      • Re-flash target via python tools/host.py sync and confirm smooth rotation at speeds 10, 30, 50, and 100 via interactive CLI.

  ### 3. Core 1 Concurrency Risks at 500 μs Ticks & Mitigations

  Core 1 executes the following tasks under Embassy's cooperative executor:
  • controllers.core1.motor (MotorController)
  • controllers.core1.sensor_north, sensor_east, sensor_west (VL53L0X ToF distance sensors)
  • core1_command_task (command routing from Core 0)
  • core_monitor (MPU stack guard & watchdog)

   Potential Risk                  | Root Cause Mechanism                             | Verified Mitigation in Codebase
  ---------------------------------|--------------------------------------------------|---------------------------------------------------
   I2C Bus Contention & Starvation | INA219 and the 3 VL53L0X distance sensors share  | Fast tick is 100% I2C-free: The 500 μs
                                   | SHARED_I2C via mutex. An I2C transaction takes   | commutation tick tick_motor() only pushes a 32-
                                   | 100--400 μs. If MotorController polled I2C every | bit word to the PIO TX FIFO and samples the GPIO
                                   | 500 μs, the distance sensors would starve and    | pin. Execution takes < 50 CPU cycles (≈ 350 ns,
                                   | the motor tick deadline would overrun.           | or 0.07% CPU load). All I2C current reads and
                                   |                                                  | safety limit checks are strictly quarantined to
                                   |                                                  | the 1.0-second slow telemetry tick
                                   |                                                  | (MOTOR_INACTIVE_TICK_INTERVAL).
   Cooperative Executor Starvation | Tasks only yield at .await points. If the motor  | Bounded non-blocking channel drain + explicit
                                   | task looped or blocked, other Core 1 tasks would | .await yield: The active loop drains pending
                                   | hang.                                            | commands non-blockingly via while let Ok(cmd) =
                                   |                                                  | command_rx.try_receive() (bounded to channel
                                   |                                                  | capacity 4), executes tick_motor(), and
                                   |                                                  | immediately yields to the executor via
                                   |                                                  | ticker.next().await.
   Idle CPU Burn                   | Running a 500 μs ticker when the motor is        | Dynamic Ticker Lifecycle: When stopped, the 500
                                   | stopped would waste CPU cycles and battery       | μs ticker is dropped. MotorController waits on
                                   | power.                                           | command_rx.receive() with a 1.0-second timeout,
                                   |                                                  | consuming 0 CPU cycles when idle.

#### Reproduction Steps

1. 1. Connect target device via probe-rs / RTT
2. 2. Synchronize firmware: package cat_detector (cat_detector_shell)
3. 3. Execute command 'motor speed 50'
4. 4. Execute command 'motor speed 100'
5. 5. Execute command 'motor speed 30'
6. 6. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] Building host utilities: host_cli and host_fs
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.23s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/release/host_cli, fs=/Users/<username>/gh/firmware/target/release/host_fs
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.07s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/release/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.10s
[] [flash] [INFO]      [1;32mFinished[0m in 9.02s
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
[] [flash] [INFO] 0.001506 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.021895 [32mINFO[0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001495 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021896 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [target] [INFO] motor speed 105.976777 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"10\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 5.977098 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 5.981394 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 5.981435 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] shell> motor current10.812830 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 10.813060 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] shell> motor speed 3020.793343 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"30\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 20.793586 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 20.803978 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 26.778474 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 26.778680 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 37.182346 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 37.182574 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 37.193973 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] shell> motor speed 1047.828116 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"100\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 47.828335 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 47.833991 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] shell> motor speed 3053.571346 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"30\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 53.571567 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 53.583980 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-DAMP-MEALWORM-592.log](attachments/WORM-DAMP-MEALWORM-592/target_system_logs_WORM-DAMP-MEALWORM-592.log) | Target system and defmt logs from host daemon session |

#### Resolution Notes

Set TMC6300 tick interval to 500us (2000 Hz commutation rate) and restored canonical calculate_pwm_counts with FET on-pulse duration clamping <= 800ns (<1.0us t_S2G threshold). Commutation step advances reliably without pole slipping or 5-degree oscillation at speed 30. Verified on hardware and codified regression test in test_worm_damp_mealworm_592_motor_commutation.py.
