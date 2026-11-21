# 🟢 `[WORM-RADIANT-VINEGARWORM-918]` Old tracks do not get cleareds when switching trace modes

- **UUID**: `90a2f829-fb88-49b6-b840-d5ba9f558fbd`
- **ID**: `WORM-RADIANT-VINEGARWORM-918`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-21 19:25:26 UTC`
- **Resolved**: `2026-11-21 19:53:13 UTC`

#### Description

When switching trace modes from sensor to perf, I noticed the sensor tracks did not get cleared leading to log corruption.

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_app
2. Execute target command via python tools/host.py cli <command>
3. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs, host_debug)...
[] [daemon] [INFO] Building host utilities via cargo build-host-tools
[] [target] [INFO] 56.877853 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [INFO] 57.092756 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 22355 mC, State: Normal
[] [target] [INFO] 57.109376 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (79 mm)
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [optimized + debuginfo]]8;;\ target(s) in 0.28s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/debug/host_cli, fs=/Users/<username>/gh/firmware/target/debug/host_fs, debug=/Users/<username>/gh/firmware/target/debug/host_debug
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via probe-rs: download --chip rp2040 --speed 4000 /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/debug/cat_detector_app
[] [probe-rs] [INFO]      [1;32mFinished[0m in 10.24s
[] [daemon] [INFO] Flash download succeeded via probe-rs
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002894 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.027632 INFO [36m[cat_detector_app::__bootstrap_task_task][0m Booting Cat Detector App...
[] [target] [INFO] 0.030750 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000dd00
[] [target] [INFO] 0.039835 INFO [36m[controller::led_controller][0m Playing animation Off
[] [target] [INFO] 0.040887 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 18609 mC, State: Normal
[] [target] [INFO] 0.041629 INFO [36m[controller::system_controller][0m cleared boot trap: Thermal
[] [target] [INFO] 0.041681 INFO [36m[controller::system_controller][0m boot blocked by traps: Battery
[] [target] [INFO] 0.061108 INFO [36m[controller::battery_controller][0m Battery Controller: Voltage is 4025 mV, SoC: 79%, Charging: false, State: Ok
[] [target] [INFO] 0.075734 INFO [36m[controller::system_controller][0m cleared boot trap: Battery
[] [target] [INFO] 0.075785 INFO [36m[controller::system_controller][0m all boot traps cleared. exiting PowerDown state. Waking up to Active mode.
[] [target] [INFO] 0.076095 INFO [36m[controller::system_controller][0m State transition: PowerDown -> Active
[] [target] [WARN] 0.076747 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 0.076864 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 0.077926 INFO [36m[controller::led_controller][0m Playing animation SolidYellow
[] [target] [ERROR] 0.080357 ERROR [36m[peripheral::tmc6300][0m ctx=00000000000000000000000000000002:0000000000000029 parent=0000000000000027 TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 0.080812 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [daemon] [WARN] Target boot/idle signal timed out after RTT attach; issuing recovery reset...
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002895 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.027572 INFO [36m[cat_detector_app::__bootstrap_task_task][0m Booting Cat Detector App...
[] [target] [INFO] 0.030704 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000dd00
[] [target] [INFO] 0.039922 INFO [36m[controller::led_controller][0m Playing animation Off
[] [target] [INFO] 0.041000 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 20014 mC, State: Normal
[] [target] [INFO] 0.041732 INFO [36m[controller::system_controller][0m cleared boot trap: Thermal
[] [target] [INFO] 0.041786 INFO [36m[controller::system_controller][0m boot blocked by traps: Battery
[] [target] [INFO] 0.061241 INFO [36m[controller::battery_controller][0m Battery Controller: Voltage is 4025 mV, SoC: 79%, Charging: false, State: Ok
[] [target] [INFO] 0.076350 INFO [36m[controller::system_controller][0m cleared boot trap: Battery
[] [target] [INFO] 0.076398 INFO [36m[controller::system_controller][0m all boot traps cleared. exiting PowerDown state. Waking up to Active mode.
[] [target] [INFO] 0.076701 INFO [36m[controller::system_controller][0m State transition: PowerDown -> Active
[] [target] [WARN] 0.077342 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 0.077452 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 0.078509 INFO [36m[controller::led_controller][0m Playing animation SolidYellow
[] [target] [ERROR] 0.080924 ERROR [36m[peripheral::tmc6300][0m ctx=00000000000000000000000000000002:0000000000000029 parent=0000000000000027 TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 0.081378 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
```

#### Attached Commits

- `b4bc497c`

#### Review Tasks & Functional Checklist

- [ ] Sync target in SENSOR mode, emit sensor readings, and confirm Proximity/Thermal tracks appear | `python tools/host.py sync --bin cat_detector_shell --trace-mode SENSOR && python tools/host.py cli sensor status && python tools/host.py trace status` <!-- id:fe1bd451 -->
- [ ] Switch target to PERF mode via sync and verify stale SENSOR tracks are cleared and only PERF/CPU tracks remain without log corruption | `python tools/host.py sync --bin cat_detector_shell --trace-mode PERF && python tools/host.py cli fs ls && python tools/host.py trace status` <!-- id:4a2963ca -->

#### Resolution Notes

Stopped active RTT/debug streaming before build_target() overwrites the ELF on disk (preventing defmt string-table log corruption and stale pre-flash events), added session_epoch tracking on TraceSessionStore.clear() + mode category filtering in DaemonRunner, and reset TraceViewerController local buffers on trace_mode/session_epoch transitions.
