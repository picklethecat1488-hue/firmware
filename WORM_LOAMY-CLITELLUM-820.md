# 🟢 `[WORM-LOAMY-CLITELLUM-820]` investigate test failures

- **UUID**: `84515a48-b63e-482f-95f5-f34dd28638b5`
- **ID**: `WORM-LOAMY-CLITELLUM-820`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-19 18:35:51 UTC`
- **Resolved**: `2026-11-19 19:14:54 UTC`

#### Description

test_tof_proximity_array_and_i2c_bus appears to be failing on HEAD

we should also integrate CLI commands and output into the test execution logs with device timestamps

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_shell
2. Execute command: python tools/host.py cli led off
3. Execute command: python tools/host.py cli motor current
4. Execute command: python tools/host.py cli motor speed 50
5. Execute command: python tools/host.py cli motor current
6. Execute command: python tools/host.py cli motor rpm
7. Execute command: python tools/host.py cli motor stop
8. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002154 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023191 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [target] [INFO] 0.353751 INFO [36m[controller::shell_controller][0m received command "Thermal { subcommand: Some(Status) }"
[] [target] [INFO] 0.354012 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 0.710391 INFO [36m[controller::shell_controller][0m received command "Thermal { subcommand: Some(Mcu) }"
[] [target] [INFO] 0.710547 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002198 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.029274 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [target] [INFO] 0.281156 INFO [36m[controller::shell_controller][0m received command "Fs { subcommand: Some(Ls), target: None, partition: None }"
[] [target] [INFO] 0.281488 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002197 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023259 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [target] [INFO] 0.343713 INFO [36m[controller::shell_controller][0m received command "Led { subcommand: Some(Green), arg1: None }"
[] [target] [INFO] 0.344001 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 0.347782 INFO [36m[controller::led_controller][0m Playing animation SolidGreen
[] [target] [INFO] 0.885796 INFO [36m[controller::shell_controller][0m received command "Led { subcommand: Some(Blue), arg1: None }"
[] [target] [INFO] 0.885980 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 0.889850 INFO [36m[controller::led_controller][0m Playing animation SolidBlue
[] [target] [INFO] 1.266858 INFO [36m[controller::shell_controller][0m received command "Led { subcommand: Some(Brightness), arg1: Some(\"50\") }"
[] [target] [INFO] 1.267107 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 1.609459 INFO [36m[controller::shell_controller][0m received command "Led { subcommand: Some(Off), arg1: None }"
[] [target] [INFO] 1.609620 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 1.612848 INFO [36m[controller::led_controller][0m Playing animation Off
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002194 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023278 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [target] [INFO] 0.351832 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 0.352223 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 0.716538 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Speed), arg1: Some(\"50\"), arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 0.716893 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 0.720794 WARN [36m[controller::motor_controller][0m Warning, calibration is not present! Running with fallback limits.
[] [target] [INFO] 0.720849 INFO [36m[controller::motor_controller][0m Starting motor...
[] [target] [INFO] 0.722480 ERROR [36m[peripheral::tmc6300][0m TMC6300 PIO DIAG pin asserted: driver fault detected
[] [target] [INFO] 0.722590 INFO [36m[controller::motor_controller][0m Stopping motor...
[] [target] [INFO] 1.077453 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Current), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 1.077768 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 1.432123 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Rpm), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 1.432406 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 1.797002 INFO [36m[controller::shell_controller][0m received command "Motor { subcommand: Some(Stop), arg1: None, arg2: None, arg3: None, partition: None }"
[] [target] [INFO] 1.797276 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002225 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023267 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00

--- Test Execution Log ---
tools/device_tests/suites/test_cat_detector_smoke.py::test_tof_proximity_array_and_i2c_bus FAILED [ 63%]
tools/device_tests/suites/test_cat_detector_smoke.py::test_thermal_and_mcu_die_temperature PASSED [ 72%]
tools/device_tests/suites/test_cat_detector_smoke.py::test_flash_filesystem_listing PASSED [ 81%]
tools/device_tests/suites/test_cat_detector_smoke.py::test_ws2812_status_led_actuator PASSED [ 90%]
tools/device_tests/suites/test_cat_detector_smoke.py::test_bldc_pump_and_current_telemetry PASSED [100%]
=================================== FAILURES ===================================
_____________________ test_tof_proximity_array_and_i2c_bus _____________________
host = <device_tests.client.AsyncHostSessionClient object at 0x1057a2680>
    @device_test(
        project="cat_detector",
        boards=("rp2040*",),
        scenarios=(ScenarioTier.SMOKE, ScenarioTier.BRINGUP),
        target_binary="cat_detector_shell",
    )
    async def test_tof_proximity_array_and_i2c_bus(host: AsyncHostSessionClient) -> None:
        """Scan I2C0 and verify North, East, and West VL53L0X ToF channels report valid distance or INVALID."""
        scan = await host.cli("i2c scan", timeout=10.0)
        assert re.search(r"(Scanning I2C bus|[0-9a-f]{2}:|0x[0-9a-fA-F]{2})", scan), scan
    
        sensor = await host.cli("sensor status", timeout=12.0)
>       assert re.search(r"(\d+\s*mm|INVALID)", sensor), sensor
E       AssertionError: Dispatched: sensor status
E       assert None
E        +  where None = <function search at 0x10325e440>('(\\d+\\s*mm|INVALID)', 'Dispatched: sensor status')
E        +    where <function search at 0x10325e440> = re.search
tools/device_tests/suites/test_cat_detector_smoke.py:49: AssertionError
- generated xml file: /var/folders/s8/x_s2ynw13ld7cjss4rrkj4xw0000gn/T/xerxes_ui_device_tests_r4ox7de8.xml -
=========================== short test summary info ============================
FAILED tools/device_tests/suites/test_cat_detector_smoke.py::test_tof_proximity_array_and_i2c_bus
=================== 1 failed, 10 passed in 251.32s (0:04:11) ===================
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-LOAMY-CLITELLUM-820.log](attachments/WORM-LOAMY-CLITELLUM-820/target_system_logs_WORM-LOAMY-CLITELLUM-820.log) | Target system and defmt logs from host daemon session |
| `log` | [target_test_execution_logs_WORM-LOAMY-CLITELLUM-820.log](attachments/WORM-LOAMY-CLITELLUM-820/target_test_execution_logs_WORM-LOAMY-CLITELLUM-820.log) | Test execution logs from host daemon Test Mode |

#### Attached Commits

- `f6832b10`

#### Review Tasks & Functional Checklist

- [ ] Run test_tof_proximity_array_and_i2c_bus on target via host.py test -k tof_proximity and verify both i2c scan and sensor status succeed cleanly | `python tools/host.py test -k tof_proximity` <!-- id:5adc0393 -->
- [ ] Open host.py in TEST mode, run a smoke test, and inspect the TEST EXECUTION LOG tab to verify CLI commands and target responses appear with [device_ts] timestamps | `python tools/host.py --mode test` <!-- id:bba23cf2 -->

#### Resolution Notes

Serialized execute_cli(wait_for_output=True) with _fs_lock and cached verified .program_metadata per connected ELF so concurrent metadata probes and consecutive same-binary tests never tear down RTT mid-command, and integrated real-time [device_ts]-prefixed CLI/FS commands and outputs into Test Execution Logs.
