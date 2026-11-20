# 🟢 `[WORM-LOAMY-CLITELLUM-820]` investigate test failures

- **UUID**: `84515a48-b63e-482f-95f5-f34dd28638b5`
- **ID**: `WORM-LOAMY-CLITELLUM-820`
- **Status**: `CLOSED`
- **Severity**: `HIGH`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-19 18:35:51 UTC`
- **Resolved**: `2026-11-20 02:23:22 UTC`
- **Accepted**: `2026-11-20 02:23:24 UTC`
- **Rejected**: `2026-11-19 19:40:49 UTC`

#### Description

investigate stress test failures on device

#### Reproduction Steps

1. python tools/host.py test --stress-runs=10

#### Behavior Comparison

- **Expected**: python tools/host.py test --stress-runs=10 passes with no test failures
- **Actual**: python tools/host.py test --stress-runs=10 fails

#### Execution / Console Logs

```text
$ python tools/host.py test --stress-runs=10
🔌 Discovered target board 'rp2040_flash_2mb' (rp2040) — running on-device pytest suites (scenario: smoke or integration)...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔁 Stress Run [1/10] — board 'rp2040_flash_2mb' (rp2040)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
====================================================== test session starts =======================================================
platform darwin -- Python 3.10.20, pytest-8.4.2, pluggy-1.6.0 -- /Users/<username>/miniforge3/envs/firmware-env/bin/python
cachedir: .pytest_cache
rootdir: /Users/<username>/gh/firmware
configfile: pyproject.toml
plugins: xdist-3.8.0, asyncio-1.5.0
asyncio: mode=auto, debug=False, asyncio_default_fixture_loop_scope=session, asyncio_default_test_loop_scope=function
collected 30 items / 8 deselected / 22 selected                                                                                  

tools/device_tests/suites/test_carrier_board_integration.py::test_pmic_charger_and_fuel_gauge SKIPPED (Target board 'r...) [  4%]
tools/device_tests/suites/test_carrier_board_integration.py::test_iqs7222a_touch_controller_baseline SKIPPED (Target b...) [  9%]
tools/device_tests/suites/test_carrier_board_integration.py::test_w25n01gv_qspi_nand_flash_rw SKIPPED (Target board 'r...) [ 13%]
tools/device_tests/suites/test_carrier_board_integration.py::test_expansion_card_fault_isolation SKIPPED (Target board...) [ 18%]
tools/device_tests/suites/test_carrier_board_integration.py::test_dual_core_sramx_ipc_ping SKIPPED (Target board 'rp20...) [ 22%]
tools/device_tests/suites/test_carrier_board_integration.py::test_armv8m_mpu_and_stack_guard_status SKIPPED (Target bo...) [ 27%]
tools/device_tests/suites/test_carrier_board_smoke.py::test_mcxn947_dual_core_boot_info SKIPPED (Target board 'rp2040_...) [ 31%]
tools/device_tests/suites/test_carrier_board_smoke.py::test_i3c_and_i2c_bus_enumeration SKIPPED (Target board 'rp2040_...) [ 36%]
tools/device_tests/suites/test_carrier_board_smoke.py::test_lp5009_rgb_led_driver_sweep SKIPPED (Target board 'rp2040_...) [ 40%]
tools/device_tests/suites/test_carrier_board_smoke.py::test_audio_amplifier_and_piezo_chime SKIPPED (Target board 'rp2...) [ 45%]
tools/device_tests/suites/test_carrier_board_smoke.py::test_nina_b312_ble_transceiver_link SKIPPED (Target board 'rp20...) [ 50%]
tools/device_tests/suites/test_cat_detector_integration.py::test_gpio_matrix_and_dual_core_monitor PASSED                  [ 54%]
tools/device_tests/suites/test_cat_detector_integration.py::test_host_fs_transfer_and_metadata_checksum PASSED             [ 59%]
tools/device_tests/suites/test_cat_detector_integration.py::test_system_activity_async_log_watcher FAILED                  [ 63%]

============================================================ FAILURES ============================================================
_____________________________________________ test_system_activity_async_log_watcher _____________________________________________

host = <device_tests.client.AsyncHostSessionClient object at 0x106e37250>

    @device_test(
        project="cat_detector",
        boards=("rp2040*",),
        scenarios=(ScenarioTier.INTEGRATION, ScenarioTier.BRINGUP),
        target_binary="cat_detector_shell",
    )
    async def test_system_activity_async_log_watcher(host: AsyncHostSessionClient) -> None:
        """Await target `defmt` RTT logs emitted concurrently while dispatching `system activity`."""
        async with host.watch_logs(
            r"(command execution succeeded|Activity|System|INFO)",
            timeout=6.0,
        ) as watcher:
            out = await host.cli("system activity")
>           assert re.search(r"(Executed:\s*system activity|System controller|OK)", out), out
E           AssertionError: Dispatched: system activity
E           assert None
E            +  where None = <function search at 0x104a89bd0>('(Executed:\\s*system activity|System controller|OK)', 'Dispatched:system activity')
E            +    where <function search at 0x104a89bd0> = re.search

tools/device_tests/suites/test_cat_detector_integration.py:80: AssertionError
==================================================== short test summary info =====================================================
FAILED tools/device_tests/suites/test_cat_detector_integration.py::test_system_activity_async_log_watcher - AssertionError: Dispatched: system activity
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! stopping after 1 failures !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
=============================== 1 failed, 2 passed, 11 skipped, 8 deselected in 118.05s (0:01:58) ================================

❌ Stress Run [1/10] FAILED after 118.3s (exit code 1; 0/10 runs passed in 118.3s).
(firmware-env)
```

#### Attached Commits

- `f6832b10`
- `6e131b77`

#### Review Tasks & Functional Checklist

- [ ] Run test_tof_proximity_array_and_i2c_bus on target via host.py test -k tof_proximity and verify both i2c scan and sensor status succeed cleanly | `python tools/host.py test -k tof_proximity` <!-- id:5adc0393 -->
- [ ] Open host.py in TEST mode, run a smoke test, and inspect the TEST EXECUTION LOG tab to verify CLI commands and target responses appear with [device_ts] timestamps | `python tools/host.py --mode test` <!-- id:bba23cf2 -->
- [ ] Run 10-iteration hardware stress suite via host.py test --stress-runs=10 and verify test_system_activity_async_log_watcher and all smoke/integration tests pass cleanly | `python tools/host.py test --stress-runs=10` <!-- id:ea263ca8 -->

#### Resolution Notes

Added structured NDJSON event streaming (--json-events) to host_cli with deterministic session_started, cli_prompt, and command_completed events, explicit 'System activity recorded.' output on system activity, and post-reset boot/prompt readiness gating in DaemonRunner.start_rtt_streaming().

#### Rejection Feedback

Serialized execute_cli(wait_for_output=True) with _fs_lock and cached verified .program_metadata per connected ELF so concurrent metadata probes and consecutive same-binary tests never tear down RTT mid-command, and integrated real-time [device_ts]-prefixed CLI/FS commands and outputs into Test Execution Logs.

-I can still repro this
