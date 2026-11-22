# 🔴 `[WORM-SEGMENTED-COCOON-284]` “Open in perfetto” did not work

- **UUID**: `b33f3c4c-516e-41de-a3ef-8622377ff9f6`
- **ID**: `WORM-SEGMENTED-COCOON-284`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-21 23:54:29 UTC`

#### Description

I didn’t see a “perfetto.ui” tab open in my browser when I clicked “Open in perfetto"

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_app --release
2. Execute command: python tools/host.py cli fs cat _host_fs_test.toml
3. Execute command: python tools/host.py cli fs cp dev:_host_fs_test.toml target/_host_fs_roundtrip.toml
4. Execute command: python tools/host.py cli fs rm _host_fs_test.toml
5. Execute command: python tools/host.py cli system activity
6. Execute command: python tools/host.py cli core status
7. Execute command: python tools/host.py cli echo 'Firmware verification output'
8. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 66.073117 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 66.073248 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 66.303057 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (75 mm)
[] [target] [DEBUG] 66.303114 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 67.092228 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 23292 mC, State: Normal
[] [target] [INFO] 68.073346 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 68.073437 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 68.092890 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 20951 mC, State: Normal
[] [target] [INFO] 68.303224 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (76 mm)
[] [target] [DEBUG] 68.303286 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 68.872491 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 68.872548 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 69.093346 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 22355 mC, State: Normal
[] [target] [INFO] 69.103993 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (73 mm)
[] [target] [DEBUG] 69.104058 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 69.273096 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 69.273169 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 69.503366 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (76 mm)
[] [target] [DEBUG] 69.503421 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 69.673098 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 69.673191 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 70.103952 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (75 mm)
[] [target] [DEBUG] 70.104025 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 70.273162 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 70.273253 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 70.503474 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (75 mm)
[] [target] [DEBUG] 70.503535 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 71.073439 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 71.073513 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 71.303502 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (75 mm)
[] [target] [DEBUG] 71.303561 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 72.073533 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 72.073666 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 72.094914 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 20951 mC, State: Normal
[] [target] [INFO] 72.303672 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (76 mm)
[] [target] [DEBUG] 72.303733 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 72.872932 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 72.873001 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 73.104079 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (78 mm)
[] [target] [DEBUG] 73.104125 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 74.095867 INFO [36m[controller::telemetry_controller][0m Thermal Controller: Temp is 22824 mC, State: Normal
[] [target] [INFO] 74.273114 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 74.273164 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 74.703866 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (74 mm)
[] [target] [DEBUG] 74.703925 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 75.273173 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: OUT OF RANGE (65535 mm)
[] [target] [DEBUG] 75.273228 DEBUG [36m[controller::sensor_controller][0m Object went out of range (proximity inactive)
[] [target] [INFO] 75.503917 INFO [36m[controller::sensor_controller][0m GestureDetector West sensor: NEAR (75 mm)
[] [target] [DEBUG] 75.503980 DEBUG [36m[controller::sensor_controller][0m Object in range (proximity active)
[] [target] [INFO] 75.706994 INFO [36m[controller::sensor_controller][0m GestureDetector North sensor: IN RANGE (196 mm)
```
