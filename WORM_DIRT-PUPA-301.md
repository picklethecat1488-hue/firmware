# 🔴 `[WORM-DIRT-PUPA-301]` Unterminated configure_alerts span

- **UUID**: `e2f8d954-dbdb-43a5-8080-c2d52af5d8bf`
- **ID**: `WORM-DIRT-PUPA-301`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-21 23:52:20 UTC`

#### Description

There is an unterminated configure_alerts span in the output which is taking place of idle processor spans on core0.

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_app --release
2. Execute command: python tools/host.py cli fs ls
3. Execute command: python tools/host.py cli fs cat _host_fs_test.toml
4. Execute command: python tools/host.py cli fs cp dev:_host_fs_test.toml target/_host_fs_roundtrip.toml
5. Execute command: python tools/host.py cli fs rm _host_fs_test.toml
6. Execute command: python tools/host.py cli system activity
7. Execute command: python tools/host.py cli core status
8. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 90.975238 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 90.975300 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 91.409327 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (77 mm)
[] [target] [DEBUG] 91.409373 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 92.179397 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 92.179445 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 92.609845 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (77 mm)
[] [target] [DEBUG] 92.609897 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 93.178346 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 93.178398 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 93.409383 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (75 mm)
[] [target] [DEBUG] 93.409444 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 94.780313 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 94.780369 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 95.012273 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (76 mm)
[] [target] [DEBUG] 95.012316 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 95.381726 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 95.381769 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 95.613723 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (75 mm)
[] [target] [DEBUG] 95.613770 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 96.183061 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 96.183117 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 96.413981 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (75 mm)
[] [target] [DEBUG] 96.414023 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 97.787946 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 97.787995 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 98.020004 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (74 mm)
[] [target] [DEBUG] 98.020051 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 99.185717 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 99.185773 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 99.418271 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (73 mm)
[] [target] [DEBUG] 99.418328 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 99.987402 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 99.987443 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 100.219091 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (75 mm)
[] [target] [DEBUG] 100.219145 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 100.391151 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 100.391201 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 100.620573 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (77 mm)
[] [target] [DEBUG] 100.620612 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 100.988039 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 100.988100 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 101.222895 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (79 mm)
[] [target] [DEBUG] 101.222943 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 102.390314 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 102.390358 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 102.624321 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (72 mm)
[] [target] [DEBUG] 102.624365 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 103.622739 INFO [36m[controller::sensor_controller][0m GestureDetector North sensor: IN RANGE (276 mm)
[] [target] [INFO] 104.791994 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
```

#### Review Tasks & Functional Checklist

- [ ] Launch cat_detector_app in PERF trace mode and export Perfetto trace JSON | `python tools/host.py --package cat_detector --bin cat_detector_app --trace-mode PERF --no-browser && sleep 4 && python tools/host.py trace export --format perfetto --json` <!-- id:41890dbb -->
- [ ] Inspect TRACE session status and verify configure_alerts closes cleanly before Core 0 idle windows | `python tools/host.py trace status` <!-- id:40cb0426 -->
