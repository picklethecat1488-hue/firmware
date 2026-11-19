# 🔴 `[WORM-ROOTED-FIREWORM-570]` Move this console widget into a “test execution log” on the left panel

- **UUID**: `9cfd6e9b-8a89-4c39-8593-22de71bb6c62`
- **ID**: `WORM-ROOTED-FIREWORM-570`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `target`
- **Created**: `2026-11-19 03:47:29 UTC`

#### Description

We should move this console widget into a “test execution log” tab on the left panel, and include test execution logs in bug reports, for test mode only

#### Execution / Console Logs

```text
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002206 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023264 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [target] [INFO] 0.353827 INFO [36m[controller::shell_controller][0m received command "Battery { subcommand: Some(Status) }"
[] [target] [INFO] 0.354091 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002223 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023251 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [target] [INFO] 0.361911 INFO [36m[controller::shell_controller][0m received command "Battery { subcommand: Some(TestAlert) }"
[] [target] [INFO] 0.362176 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002200 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023248 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [target] [INFO] 0.341698 INFO [36m[controller::shell_controller][0m received command "I2c { subcommand: Some(Scan), bus: None }"
[] [target] [INFO] 0.341984 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 0.774988 INFO [36m[controller::shell_controller][0m received command "Sensor { subcommand: Some(Status), arg1: None, partition: None }"
[] [target] [INFO] 0.775233 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002208 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023228 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [target] [INFO] 0.353788 INFO [36m[controller::shell_controller][0m received command "Thermal { subcommand: Some(Status) }"
[] [target] [INFO] 0.354049 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 0.770924 INFO [36m[controller::shell_controller][0m received command "Thermal { subcommand: Some(Mcu) }"
[] [target] [INFO] 0.771082 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002173 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023235 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [target] [INFO] 0.335596 INFO [36m[controller::shell_controller][0m received command "Fs { subcommand: Some(Ls), target: None, partition: None }"
[] [target] [INFO] 0.335919 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002189 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023228 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [target] [INFO] 0.343675 INFO [36m[controller::shell_controller][0m received command "Led { subcommand: Some(Green), arg1: None }"
[] [target] [INFO] 0.343962 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 0.348117 INFO [36m[controller::led_controller][0m Playing animation SolidGreen
[] [target] [INFO] 0.946241 INFO [36m[controller::shell_controller][0m received command "Led { subcommand: Some(Blue), arg1: None }"
[] [target] [INFO] 0.946428 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 0.950516 INFO [36m[controller::led_controller][0m Playing animation SolidBlue
[] [target] [INFO] 1.327303 INFO [36m[controller::shell_controller][0m received command "Led { subcommand: Some(Brightness), arg1: Some(\"50\") }"
[] [target] [INFO] 1.327551 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 1.730378 INFO [36m[controller::shell_controller][0m received command "Led { subcommand: Some(Off), arg1: None }"
[] [target] [INFO] 1.730534 INFO [36m[controller::shell_controller][0m command execution succeeded
[] [target] [INFO] 1.734601 INFO [36m[controller::led_controller][0m Playing animation Off
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [image.png](attachments/WORM-ROOTED-FIREWORM-570/image.png) | Pasted screenshot |

#### Review Tasks & Functional Checklist

- [ ] Open host.py in TEST mode, click '📋 TEST EXECUTION LOG' in the left panel tab bar, run a smoke test, and verify pytest execution output streams into the left panel with a working COPY button | `python tools/host.py --mode test` <!-- id:bf848b80 -->
