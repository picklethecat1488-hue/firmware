# 🟢 `[WORM-VELVETY-HELMINTH-401]` System LED yellow after hours of charging

- **UUID**: `9becc1a3-9073-4ebc-9600-ea56deb439ca`
- **ID**: `WORM-VELVETY-HELMINTH-401`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-21 23:59:20 UTC`
- **Resolved**: `2026-11-22 17:45:12 UTC`

#### Description

The system LED is yellow after hours of charging. The `battery status` command returns 79%. I think there is some problem in the way model gauge is being configured.

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_app --release
2. Execute command: python tools/host.py cli battery status
3. Execute command: python tools/host.py cli echo 'Firmware verification output'
4. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [DEBUG] 25.469220 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 25.900006 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (77 mm)
[] [target] [DEBUG] 25.900066 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 26.064904 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 20951 mC, State: Normal
[] [target] [INFO] 26.070234 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 26.070341 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 26.300495 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (78 mm)
[] [target] [DEBUG] 26.300558 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 26.469783 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 26.469859 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 27.065511 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 22355 mC, State: Normal
[] [target] [INFO] 27.100380 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (72 mm)
[] [target] [DEBUG] 27.100454 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 27.269839 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 27.269940 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 27.700206 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (78 mm)
[] [target] [DEBUG] 27.700284 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 28.669584 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 28.669658 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 29.100696 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (76 mm)
[] [target] [DEBUG] 29.100766 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 29.270084 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 29.270172 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 29.500159 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (78 mm)
[] [target] [DEBUG] 29.500227 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [DEBUG] 31.068379 DEBUG [36m[peripheral::max17048][0m Temperature is 21887 mC, compensating RCOMP to 0x96
[] [target] [INFO] 31.669736 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 31.669824 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 32.100942 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (73 mm)
[] [target] [DEBUG] 32.101008 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 32.469932 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 32.470006 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 32.700656 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (74 mm)
[] [target] [DEBUG] 32.700728 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 33.269924 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 33.269995 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 33.500742 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (78 mm)
[] [target] [DEBUG] 33.500816 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [daemon] [INFO] Updated configuration: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, profile=debug, trace_mode=NONE, speed=4000, debug_server=False (GDB :3333/:3334)
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=False, trace_mode=NONE
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [optimized + debuginfo]]8;;\ target(s) in 0.14s
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/debug/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs, host_debug)...
[] [daemon] [INFO] Building host utilities via cargo build-host-tools
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [optimized + debuginfo]]8;;\ target(s) in 0.16s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/debug/host_cli, fs=/Users/<username>/gh/firmware/target/debug/host_fs, debug=/Users/<username>/gh/firmware/target/debug/host_debug
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via probe-rs: download --chip rp2040 --speed 4000 /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/debug/cat_detector_shell
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-VELVETY-HELMINTH-401.log](attachments/WORM-VELVETY-HELMINTH-401/target_system_logs_WORM-VELVETY-HELMINTH-401.log) | Target system and defmt logs from host daemon session |
| `reference` | [target_telemetry_WORM-VELVETY-HELMINTH-401.json](attachments/WORM-VELVETY-HELMINTH-401/target_telemetry_WORM-VELVETY-HELMINTH-401.json) | Target telemetry export retrieved via host_fs |

#### Attached Commits

- `5b8c85af`

#### Review Tasks & Functional Checklist

- [ ] Inspect live MAX17048 HIBRT (0x0a == 0x0000), MODE (0x06 == 0x0000), and rounded battery status via i2c peek16 + battery status | `python tools/host.py --package cat_detector --bin cat_detector_shell --no-browser && python tools/host.py cli 'i2c peek16 0x36 0x0a' && python tools/host.py cli 'i2c peek16 0x36 0x06' && python tools/host.py cli 'battery status'` <!-- id:1d15fc35 -->
- [ ] Sync cat_detector_app and confirm System LED transitions to SolidGreen at >= 80% SoC | `python tools/host.py --package cat_detector --bin cat_detector_app --release --no-browser && sleep 3 && python tools/host.py logs --stream defmt --lines 25` <!-- id:74a7aee3 -->

#### Resolution Notes

Disabled MAX17048 Hibernate Mode (HIBRT = 0x0000) and Sleep Mode on init() with one-shot Quick-Start when recovering from POR/Hibernate/Sleep (preventing <17%/hr 500 mA 18650 charging from trapping ModelGauge in HibStat=1 with CRATE=0), rounded 8.8 fixed-point SOC (+0x0080) instead of truncating 79.61% (0x4F9B) to 79%, and closed the 21..49% SolidOrange LED band gap in BatteryManager::get_soc_led_state().
