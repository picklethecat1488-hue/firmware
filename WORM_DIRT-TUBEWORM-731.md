# 🟢 `[WORM-DIRT-TUBEWORM-731]` Only single perf span recorded

- **UUID**: `4e00966b-b2af-40ef-85eb-ee2da450c4a5`
- **ID**: `WORM-DIRT-TUBEWORM-731`
- **Status**: `CLOSED`
- **Severity**: `HIGH`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-21 19:26:35 UTC`
- **Resolved**: `2026-11-21 23:56:21 UTC`
- **Accepted**: `2026-11-21 23:56:23 UTC`

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_app
2. Execute target command via python tools/host.py cli <command>
3. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 0.076350 INFO [36m[controller::system_controller][0m cleared boot trap: Battery
[] [target] [INFO] 0.076398 INFO [36m[controller::system_controller][0m all boot traps cleared. exiting PowerDown state. Waking up to Active mode.
[] [target] [INFO] 0.076701 INFO [36m[controller::system_controller][0m State transition: PowerDown -> Active
[] [target] [WARN] 0.077342 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 0.077452 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 0.078509 INFO [36m[controller::led_controller][0m Playing animation SolidYellow
[] [target] [ERROR] 0.080924 ERROR [36m[peripheral::tmc6300][0m ctx=00000000000000000000000000000002:0000000000000029 parent=0000000000000027 TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 0.081378 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [daemon] [INFO] Target bug created: WORM-RADIANT-VINEGARWORM-918 (Old tracks do not get cleareds when switching trace modes) with 1 attachments
[] [system] [INFO] Resetting target microcontroller...
[] [daemon] [INFO] RTT stream disconnected
[] [probe-rs] [INFO] [33m WARN[0m [2mprobe_rs::architecture::arm::communication_interface[0m[2m:[0m Failed to switch to DP Multidrop(11002927): An error occurred in the communication with an access port or debug port.
[] [system] [INFO] Target reset successfully via probe-rs
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002914 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.027649 INFO [36m[cat_detector_app::__bootstrap_task_task][0m Booting Cat Detector App...
[] [target] [INFO] 0.030778 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000dd00
[] [target] [INFO] 0.040316 INFO [36m[controller::led_controller][0m Playing animation Off
[] [target] [INFO] 0.041382 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 20482 mC, State: Normal
[] [target] [INFO] 0.042107 INFO [36m[controller::system_controller][0m cleared boot trap: Thermal
[] [target] [INFO] 0.042159 INFO [36m[controller::system_controller][0m boot blocked by traps: Battery
[] [target] [INFO] 0.095281 INFO [36m[controller::battery_controller][0m Battery Controller: Voltage is 4025 mV, SoC: 79%, Charging: false, State: Ok
[] [target] [INFO] 0.106196 INFO [36m[controller::system_controller][0m cleared boot trap: Battery
[] [target] [INFO] 0.106247 INFO [36m[controller::system_controller][0m all boot traps cleared. exiting PowerDown state. Waking up to Active mode.
[] [target] [INFO] 0.106570 INFO [36m[controller::system_controller][0m State transition: PowerDown -> Active
[] [target] [WARN] 0.107211 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 0.107330 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 0.108387 INFO [36m[controller::led_controller][0m Playing animation SolidYellow
[] [target] [ERROR] 0.110772 ERROR [36m[peripheral::tmc6300][0m ctx=00000000000000000000000000000002:0000000000000029 parent=0000000000000027 TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 0.111237 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [daemon] [WARN] Target boot/idle signal timed out after RTT attach; issuing recovery reset...
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002905 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.027596 INFO [36m[cat_detector_app::__bootstrap_task_task][0m Booting Cat Detector App...
[] [target] [INFO] 0.030718 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000dd00
[] [target] [INFO] 0.038356 INFO [36m[controller::led_controller][0m Playing animation Off
[] [target] [INFO] 0.039423 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 20951 mC, State: Normal
[] [target] [INFO] 0.040155 INFO [36m[controller::system_controller][0m cleared boot trap: Thermal
[] [target] [INFO] 0.040213 INFO [36m[controller::system_controller][0m boot blocked by traps: Battery
[] [target] [INFO] 0.055617 INFO [36m[controller::battery_controller][0m Battery Controller: Voltage is 4025 mV, SoC: 79%, Charging: false, State: Ok
[] [target] [INFO] 0.067009 INFO [36m[controller::system_controller][0m cleared boot trap: Battery
[] [target] [INFO] 0.067059 INFO [36m[controller::system_controller][0m all boot traps cleared. exiting PowerDown state. Waking up to Active mode.
[] [target] [INFO] 0.067391 INFO [36m[controller::system_controller][0m State transition: PowerDown -> Active
[] [target] [WARN] 0.068023 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 0.068136 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 0.069207 INFO [36m[controller::led_controller][0m Playing animation SolidYellow
[] [target] [ERROR] 0.071664 ERROR [36m[peripheral::tmc6300][0m ctx=00000000000000000000000000000002:0000000000000029 parent=0000000000000027 TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 0.072105 INFO [36m[controller::motor_controller][0m Stopping motor...
```

#### Attached Commits

- `07d2ad02`

#### Review Tasks & Functional Checklist

- [ ] Sync cat_detector_app in PERF mode and verify continuous execution without spinlock deadlock | `python tools/host.py --bin cat_detector_app --trace-mode PERF && sleep 4 && python tools/host.py trace status` <!-- id:372dd545 -->
- [ ] Export captured cat_detector_app PERF trace to CSV and confirm bounded Core 0 and Core 1 spans are recorded | `python tools/host.py trace export -f csv -o recordings/traces/worm_731_verify.csv && head -n 25 recordings/traces/worm_731_verify.csv` <!-- id:6b601897 -->

#### Resolution Notes

Added SeqCst hardware memory barriers (dmb sy) and non-panicking CRASH_LOG_BUFFER borrow in RttProtocol to prevent dual-core RP2040 IOPORT spinlock vs AHB-Lite SRAM RefCell panics under PERF tracing, added cooperative async task suspension across CPU Idle entries in LiveTraceStreamProcessor, wiped stale pre-reset _SEGGER_RTT SRAM headers before MCU reset, and avoided spurious 5s shell_idle resets on non-shell binaries.
