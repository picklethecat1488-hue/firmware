# 🟢 `[WORM-INCHING-MILLIPEDE-969]` White artifact to the right of the test stack

- **UUID**: `74318aac-d97f-48e5-bdd9-9a09197ce735`
- **ID**: `WORM-INCHING-MILLIPEDE-969`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-19 18:29:11 UTC`
- **Resolved**: `2026-11-19 19:40:30 UTC`
- **Accepted**: `2026-11-19 19:40:32 UTC`

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_shell
2. Execute command: python tools/host.py cli fs query_metadata --json
3. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [optimized + debuginfo]]8;;\ target(s) in 0.07s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/debug/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.22s
[] [flash] [INFO]      [1;32mFinished[0m in 12.18s
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
[] [flash] [INFO] 0.023289 [32mINFO[0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002208 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023236 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [system] [INFO] Switching to Test mode (on-device @device_test runner active)...
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002214 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023236 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002206 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.029241 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002204 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.029227 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected
[] [host_fs] [INFO] {"checksum":"0x70D6","chip":"rp2040","board_type":"rp2040_flash_2mb","program_metadata_board_type":"rp2040_flash_2mb","provisioned_board_type":null,"source":"program_metadata","flash_start":268435456,"flash_write_size":1,"flash_erase_size":4096,"stack_scan_limit":2048,"program_metadata_addr":270200832,"program_metadata_size":76,"partitions":[{"kind":0,"name":"fs","address":270270464,"size":65536},{"kind":1,"name":"telemetry","address":270336000,"size":196608},{"kind":2,"name":"metadata","address":270204928,"size":65536}]}
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002206 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023282 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002166 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023205 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected

--- Test Execution Log ---
$ /Users/<username>/miniforge3/envs/firmware-env/bin/python -m pytest -c /Users/<username>/gh/firmware/pyproject.toml --rootdir /Users/<username>/gh/firmware -o addopts= -v tools/device_tests/suites/test_cat_detector_integration.py::test_host_fs_transfer_and_metadata_checksum --board auto --host-addr 127.0.0.1 --host-port 8899 --junitxml /var/folders/s8/x_s2ynw13ld7cjss4rrkj4xw0000gn/T/xerxes_ui_device_tests_t967e429.xml --manual
============================= test session starts ==============================
platform darwin -- Python 3.10.20, pytest-8.4.2, pluggy-1.6.0 -- /Users/<username>/miniforge3/envs/firmware-env/bin/python
cachedir: .pytest_cache
rootdir: /Users/<username>/gh/firmware
configfile: pyproject.toml
plugins: xdist-3.8.0, asyncio-1.5.0
asyncio: mode=auto, debug=False, asyncio_default_fixture_loop_scope=session, asyncio_default_test_loop_scope=function
collecting ... collected 1 item
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [image.png](attachments/WORM-INCHING-MILLIPEDE-969/image.png) | Pasted screenshot |

#### Attached Commits

- `97826040`

#### Review Tasks & Functional Checklist

- [ ] Open host.py in TEST mode and verify the right edge of the stacked Test Suite list renders with dark Xerxes scrollbars and zero white scrollbar-gutter bar | `python tools/host.py --mode test` <!-- id:6c74f2e5 -->

#### Resolution Notes

Removed scrollbar-gutter: stable and overflow-y: scroll from .test-stack-list (using overflow-y: auto), added color-scheme: dark on :root across common.css and host_daemon.html.j2, and styled scrollbars with dark Xerxes #04090c/#0c3c47 track and thumb rules.
