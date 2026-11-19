# 🟢 `[WORM-SILKEN-LUMBRICUS-589]` Add progress bar to right panel

- **UUID**: `41761a6c-1ba4-41e7-a106-13cac9a81bb2`
- **ID**: `WORM-SILKEN-LUMBRICUS-589`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `target`
- **Created**: `2026-11-19 03:45:39 UTC`
- **Resolved**: `2026-11-19 06:54:20 UTC`

#### Description

We should add a test progress and status indicator to the right panel to be shown when a manual prompt is not active so that execution has a more interactive experience. Test status should indicate when the target is being flashed, along with target mode switches, reboots, and test execution status

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_shell
2. Execute command: python tools/host.py cli motor calibrate
3. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [host_fs] [INFO] [boards.carrier_board]
[] [host_fs] [INFO] chip = "MCXN947"
[] [host_fs] [INFO] board_type = "carrier_board"
[] [host_fs] [INFO] program_metadata_addr = 0x001DF000
[] [host_fs] [INFO] flash_base = 0x00000000
[] [host_fs] [INFO] flash_size = 0x00200000         # 2 MB internal dual-bank NOR flash
[] [host_fs] [INFO] flash_write_size = 128          # 128 B phrase program
[] [host_fs] [INFO] flash_erase_size = 8192         # 8 KB sector erase
[] [host_fs] [INFO] sram_size = 0x00080000          # 512 KB ECC SRAM
[] [host_fs] [INFO] core0_stack_size = 24576        # 24 KB
[] [host_fs] [INFO] core1_stack_size = 16384        # 16 KB
[] [host_fs] [INFO] stack_top = 0x20080000
[] [host_fs] [INFO] core0_stack_bottom = 0x2007A000
[] [host_fs] [INFO] core_monitor_timeout_ms = 10000
[] [host_fs] [INFO] core_monitor_warn_pct = 80
[] [host_fs] [INFO] [boards.carrier_board.pins]
[] [host_fs] [INFO] I2C0_SDA_PIN = 0
[] [host_fs] [INFO] I2C0_SCL_PIN = 1
[] [host_fs] [INFO] I2C1_SDA_PIN = 2
[] [host_fs] [INFO] I2C1_SCL_PIN = 3
[] [host_fs] [INFO] UART0_TX_PIN = 4
[] [host_fs] [INFO] UART0_RX_PIN = 5
[] [host_fs] [INFO] FUEL_GAUGE_INT_PIN = 6
[] [host_fs] [INFO] CAP_TOUCH_INT_PIN = 7
[] [host_fs] [INFO] PIEZO_PWM_PIN = 8
[] [host_fs] [INFO] [boards.carrier_board.buses.i2c]
[] [host_fs] [INFO] sda = "I2C0_SDA_PIN"
[] [host_fs] [INFO] scl = "I2C0_SCL_PIN"
[] [host_fs] [INFO] frequency = 400000
[] [host_fs] [INFO] [boards.carrier_board.buses.uart]
[] [host_fs] [INFO] tx = "UART0_TX_PIN"
[] [host_fs] [INFO] rx = "UART0_RX_PIN"
[] [host_fs] [INFO] [boards.carrier_board.hardware_resources]
[] [host_fs] [INFO] FUEL_GAUGE_I2C_ADDR = { value = 0x36, type = "u8" }
[] [host_fs] [INFO] LED_DRIVER_I2C_ADDR = { value = 0x14, type = "u8" }
[] [host_fs] [INFO] TOUCH_CTRL_I2C_ADDR = { value = 0x44, type = "u8" }
[] [host_fs] [INFO] MAX_CRASH_LOGS = { value = 10, type = "u32" }
[] [host_fs] [INFO] [boards.carrier_board.partitions.storage]
[] [host_fs] [INFO] start = 0x001E0000
[] [host_fs] [INFO] end = 0x00200000
[] [host_fs] [INFO] [boards.carrier_board.partitions.metadata]
[] [host_fs] [INFO] start = 0x001E0000
[] [host_fs] [INFO] end = 0x001E8000
[] [host_fs] [INFO] [boards.carrier_board.partitions.fs]
[] [host_fs] [INFO] start = 0x001E8000
[] [host_fs] [INFO] end = 0x001F0000
[] [host_fs] [INFO] [boards.carrier_board.partitions.telemetry]
[] [host_fs] [INFO] start = 0x001F0000
[] [host_fs] [INFO] end = 0x00200000
[] [daemon] [INFO] RTT streaming process connected
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `log` | [target_system_logs_WORM-SILKEN-LUMBRICUS-589.log](attachments/WORM-SILKEN-LUMBRICUS-589/target_system_logs_WORM-SILKEN-LUMBRICUS-589.log) | Target system and defmt logs from host daemon session |

#### Review Tasks & Functional Checklist

- [ ] Launch host.py in TEST mode, run a multi-binary suite (e.g. cat_detector smoke + integration), and verify the right-panel Test Progress & Target Status card updates percentage, target flashing, mode switches, reboots, and active test queue in real time | `python tools/host.py --mode test` <!-- id:b9340829 -->
- [ ] Run a manual bringup test with 'Prompt Manual' checked and verify the right panel transitions from the Progress Card to the Manual Benchtop Prompt card while awaiting input, then returns to the Progress Card upon submission | `python tools/host.py test --project cat_detector --scenario bringup --manual` <!-- id:3735ec5f -->

#### Resolution Notes

Added the right-panel Test Progress & Target Lifecycle Status card (#testProgressCard) displayed whenever a manual prompt is inactive, surfacing live percentage progress, target flashing, mode switches, SWD reboots, active CLI/FS commands, and per-test queue status.
