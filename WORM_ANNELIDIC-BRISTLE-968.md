# 🟢 `[WORM-ANNELIDIC-BRISTLE-968]` Add test name filter to  host.py test cmd line

- **UUID**: `4c9af3c3-00b4-41a1-bdbd-b73b67e47f0b`
- **ID**: `WORM-ANNELIDIC-BRISTLE-968`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-19 18:33:25 UTC`
- **Resolved**: `2026-11-19 19:40:15 UTC`

#### Description

Add a test name filter to the host.py test cmd line to mirror the -k pytest flag

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_shell
2. Execute command: python tools/host.py cli fs ls
3. Execute command: python tools/host.py cli fs cat _host_fs_test.toml
4. Execute command: python tools/host.py cli fs cp dev:_host_fs_test.toml target/_host_fs_roundtrip.toml
5. Execute command: python tools/host.py cli fs rm _host_fs_test.toml
6. Execute command: python tools/host.py cli system activity
7. Execute command: python tools/host.py cli battery status
8. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] Updated configuration: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, profile=debug, speed=4000
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=False
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [optimized + debuginfo]]8;;\ target(s) in 0.25s
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/debug/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs)...
[] [daemon] [INFO] Building host utilities: host_cli and host_fs
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [optimized + debuginfo]]8;;\ target(s) in 0.25s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/debug/host_cli, fs=/Users/<username>/gh/firmware/target/debug/host_fs
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [optimized + debuginfo]]8;;\ target(s) in 0.08s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/debug/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.20s
[] [flash] [INFO]      [1;32mFinished[0m in 12.17s
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
[] [flash] [INFO] 0.002202 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.023213 [32mINFO[0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002202 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023261 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [target] [INFO] 0.414291 INFO [36m[controller::shell_controller][0m received command "Battery { subcommand: Some(Status) }"
[] [target] [INFO] 0.414558 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002211 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.029271 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [target] [INFO] 0.307444 INFO [36m[controller::shell_controller][0m received command "Battery { subcommand: Some(TestAlert) }"
[] [target] [INFO] 0.307707 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002154 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023188 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected

--- Test Execution Log ---
$ /Users/<username>/miniforge3/envs/firmware-env/bin/python -m pytest -c /Users/<username>/gh/firmware/pyproject.toml --rootdir /Users/<username>/gh/firmware -o addopts= -v tools/device_tests/suites/test_cat_detector_integration.py::test_gpio_matrix_and_dual_core_monitor tools/device_tests/suites/test_cat_detector_integration.py::test_host_fs_transfer_and_metadata_checksum tools/device_tests/suites/test_cat_detector_integration.py::test_system_activity_async_log_watcher tools/device_tests/suites/test_cat_detector_integration.py::test_standalone_app_binary_boot_lifecycle tools/device_tests/suites/test_cat_detector_smoke.py::test_battery_fuel_gauge_status tools/device_tests/suites/test_cat_detector_smoke.py::test_battery_alert_pin_assertion tools/device_tests/suites/test_cat_detector_smoke.py::test_tof_proximity_array_and_i2c_bus tools/device_tests/suites/test_cat_detector_smoke.py::test_thermal_and_mcu_die_temperature tools/device_tests/suites/test_cat_detector_smoke.py::test_flash_filesystem_listing tools/device_tests/suites/test_cat_detector_smoke.py::test_ws2812_status_led_actuator tools/device_tests/suites/test_cat_detector_smoke.py::test_bldc_pump_and_current_telemetry --board auto --host-addr 127.0.0.1 --host-port 8899 --junitxml /var/folders/s8/x_s2ynw13ld7cjss4rrkj4xw0000gn/T/xerxes_ui_device_tests_r4ox7de8.xml --manual
============================= test session starts ==============================
platform darwin -- Python 3.10.20, pytest-8.4.2, pluggy-1.6.0 -- /Users/<username>/miniforge3/envs/firmware-env/bin/python
cachedir: .pytest_cache
rootdir: /Users/<username>/gh/firmware
configfile: pyproject.toml
plugins: xdist-3.8.0, asyncio-1.5.0
asyncio: mode=auto, debug=False, asyncio_default_fixture_loop_scope=session, asyncio_default_test_loop_scope=function
collecting ... collected 11 items
tools/device_tests/suites/test_cat_detector_integration.py::test_gpio_matrix_and_dual_core_monitor PASSED [  9%]
tools/device_tests/suites/test_cat_detector_integration.py::test_host_fs_transfer_and_metadata_checksum PASSED [ 18%]
tools/device_tests/suites/test_cat_detector_integration.py::test_system_activity_async_log_watcher PASSED [ 27%]
tools/device_tests/suites/test_cat_detector_integration.py::test_standalone_app_binary_boot_lifecycle PASSED [ 36%]
tools/device_tests/suites/test_cat_detector_smoke.py::test_battery_fuel_gauge_status PASSED [ 45%]
tools/device_tests/suites/test_cat_detector_smoke.py::test_battery_alert_pin_assertion PASSED [ 54%]
tools/device_tests/suites/test_cat_detector_smoke.py::test_tof_proximity_array_and_i2c_bus FAILED [ 63%]
```

#### Attached Commits

- `dc835ae9`

#### Review Tasks & Functional Checklist

- [ ] Run host.py test --list-tests -k 'tof and not calibration' and verify only matching @device_test entries are listed | `python tools/host.py test --list-tests -k 'tof and not calibration'` <!-- id:1d29c161 -->
- [ ] Run a single filtered on-device test via host.py test -k battery_fuel_gauge and verify only that test executes on target | `python tools/host.py test -k battery_fuel_gauge` <!-- id:06394688 -->

#### Resolution Notes

Added -k / --keyword / --filter to python tools/host.py test (and tools/verify.py) mirroring pytest -k boolean keyword expressions across both --list-tests discovery and on-device pytest execution.
