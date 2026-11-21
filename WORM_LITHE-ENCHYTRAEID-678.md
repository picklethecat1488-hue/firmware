# 🔴 `[WORM-LITHE-ENCHYTRAEID-678]` Single step repeatedly hangs

- **UUID**: `4261aba6-717f-4502-be63-7e8e0112c637`
- **ID**: `WORM-LITHE-ENCHYTRAEID-678`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_detector`
- **Created**: `2026-11-21 19:20:51 UTC`

#### Description

Observation: single stepping is slow, and takes multiple seconds. We seem to be getting stuck in a WFE instruction (scheduler.rs:222). Attempting to disconnect from the target leaves vscode stuck in debug mode. Do we need to wake the core in order to finish single stepping?

#### Reproduction Steps

1. 1. set a breakpoint in cat_detector_shell.rs:60
2. 2. begin single stepping

#### Behavior Comparison

- **Expected**: We should be able to single step 10 times within 30 seconds
- **Actual**: Single stepping eventually times out and hangs at a WFE instruction around the 4th single step

#### Execution / Console Logs

```text
⚡ SYSTEM SHOCK 2 // XERXES HOST DAEMON
thumbv6m-none-eabi
cat_detector_shell
ONLINE
GDB :3333/:3334 (SYNC)
⚡ SYNC
⚡ RESET
🎯 TARGET CONFIG
🐞 TARGET BUG
MODE:
CLI
FS
TEST
TRACE
SCALE:

HIDE LOG
CLEAR
📜 SYSTEM LOGS (DEFMT / RTT)
📋 COPY
AUTO-SCROLL: ON
◀ HIDE
[daemon] Updated configuration: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, profile=debug, trace_mode=NONE, speed=4000, debug_server=True (GDB :3333/:3334)
[daemon] [Sync] Compiling target firmware...
[daemon] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=False, trace_mode=NONE
[build]     Finished `dev` profile [optimized + debuginfo] target(s) in 0.31s
[daemon] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/debug/cat_detector_shell
[daemon] [Sync] Compiling host utilities (host_cli, host_fs, host_debug)...
[daemon] Building host utilities via cargo build-host-tools
[build]     Finished `dev` profile [optimized + debuginfo] target(s) in 0.37s
[daemon] Host tools ready: cli=/Users/<username>/gh/firmware/target/debug/host_cli, fs=/Users/<username>/gh/firmware/target/debug/host_fs, debug=/Users/<username>/gh/firmware/target/debug/host_debug
[daemon] [Sync] Flashing target device via probe-rs...
[daemon] Flashing target via probe-rs: download --chip rp2040 --speed 4000 /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/debug/cat_detector_shell
[probe-rs]      Finished in 10.71s
[daemon] Flash download succeeded via probe-rs
[daemon] [Sync] Starting probe-rs GDB server (:3333/:3334) & RTT bridge...
[debug] Target GDB server ready on Core 0 :3333 / Core 1 :3334 (RTT bridge :50000)
[daemon] RTT streaming process connected
[target] 0.002443 INFO [platform::core_monitor] MPU stack guard configured at 0x2003c000
[target] 0.023483 INFO [platform::core_monitor] MPU stack guard configured at 0x2000cf00
[daemon] [Sync] Target synchronized & GDB server active on :3333/:3334.
[debug] GDB client gdb_connected on Core 1
[debug] GDB client gdb_connected on Core 0
[target] 0.002423 INFO [platform::core_monitor] MPU stack guard configured at 0x2003c000
[target] 0.002423 INFO [platform::core_monitor] MPU stack guard configured at 0x2003c000
[target] 0.023461 INFO [platform::core_monitor] MPU stack guard configured at 0x2000cf00
[target] 21.673808 INFO [controller::shell_controller] received command "Gpio { subcommand: Some(Status), pin: None, value: None }"
[target] 21.675095 INFO [controller::shell_controller] command execution succeeded
[target] 22.822847 INFO [controller::shell_controller] received command "Core { subcommand: Some(Status), arg1: None }"
[target] 22.823525 INFO [controller::shell_controller] command execution succeeded
[daemon] RTT stream disconnected
[debug] GDB client gdb_disconnected on Core 1
[debug] GDB client gdb_disconnected on Core 0
[host_fs] {"checksum":"0xAA0F","chip":"rp2040","board_type":"rp2040_flash_2mb","program_metadata_board_type":"rp2040_flash_2mb","provisioned_board_type":null,"bin_name":"cat_detector_shell","release_build":false,"trace_mode":"none","source":"program_metadata","flash_start":268435456,"flash_write_size":1,"flash_erase_size":4096,"stack_scan_limit":2048,"program_metadata_addr":270200832,"program_metadata_size":11,"partitions":[{"kind":0,"name":"fs","address":270270464,"size":65536},{"kind":1,"name":"telemetry","address":270336000,"size":196608},{"kind":2,"name":"metadata","address":270204928,"size":65536}]}
[debug] Target GDB server ready on Core 0 :3333 / Core 1 :3334 (RTT bridge :50000)
[daemon] RTT streaming process connected
[daemon] RTT stream disconnected
[host_fs] Successfully copied board/board.toml to dev:_host_fs_test.toml
[debug] Target GDB server ready on Core 0 :3333 / Core 1 :3334 (RTT bridge :50000)
[daemon] RTT streaming process connected
[target] 0.002231 INFO [platform::core_monitor] MPU stack guard configured at 0x2003c000
[target] 0.023253 INFO [platform::core_monitor] MPU stack guard configured at 0x2000cf00
[daemon] RTT stream disconnected
[host_fs] Filename                 | Data Type / Format
[host_fs] --------------------------------------------------
[host_fs] _host_fs_test.toml       | Raw Binary / Unknown Type
[debug] Target GDB server ready on Core 0 :3333 / Core 1 :3334 (RTT bridge :50000)
[daemon] RTT streaming process connected
[daemon] RTT stream disconnected
[host_fs] [boards.cat_detector]
[host_fs] chip = "rp2040"
[host_fs] board_type = "rp2040_flash_2mb"
[host_fs] program_metadata_addr = 0x101AF000
[host_fs] flash_base = 0x10000000
[host_fs] flash_size = 0x00200000         # 2 MB
[host_fs] sram_size = 0x00042000          # 264 KB
[host_fs] core0_stack_size = 24576        # 24 KB
[host_fs] core1_stack_size = 16384        # 16 KB
[host_fs] stack_top = 0x20042000
[host_fs] core0_stack_bottom = 0x2003C000
[host_fs] core_monitor_timeout_ms = 10000
[host_fs] core_monitor_warn_pct = 80
[host_fs] [boards.cat_detector.motor]
[host_fs] startup_blanking_ticks = 50
[host_fs] default_max_current_ma = 800
[host_fs] startup_commutation_step_ticks = 100
[host_fs] commutation_speed_divisor = 200
[host_fs] min_commutation_step_ticks = 2
[host_fs] max_commutation_step_ticks = 200
[host_fs] [boards.cat_detector.pins]
[host_fs] PUMP_UH_PIN = 16
[host_fs] PUMP_UL_PIN = 17
[host_fs] PUMP_VH_PIN = 18
[host_fs] PUMP_VL_PIN = 19
[host_fs] PUMP_WH_PIN = 20
[host_fs] PUMP_WL_PIN = 21
[host_fs] PUMP_DIAG_PIN = 22
[host_fs] SPEED_SENSE_PIN = 26
[host_fs] I2C_SDA_PIN = 12
[host_fs] I2C_SCL_PIN = 13
[host_fs] UART_TX_PIN = 0
[host_fs] UART_RX_PIN = 1
[host_fs] LED_PIN = 11
[host_fs] TOF_NORTH_XSHUT_PIN = 4
[host_fs] TOF_EAST_XSHUT_PIN = 5
[host_fs] TOF_WEST_XSHUT_PIN = 6
[host_fs] TOF_NORTH_INT_PIN = 7
[host_fs] TOF_EAST_INT_PIN = 9
[host_fs] TOF_WEST_INT_PIN = 10
[host_fs] FUEL_GAUGE_INT_PIN = 14
[host_fs] [boards.cat_detector.buses.i2c]
[host_fs] sda = "I2C_SDA_PIN"
[host_fs] scl = "I2C_SCL_PIN"
[host_fs] frequency = 400000
[host_fs] [boards.cat_detector.buses.uart]
[host_fs] tx = "UART_TX_PIN"
[host_fs] rx = "UART_RX_PIN"
[host_fs] [boards.cat_detector.buses.pio]
[host_fs] pin = "LED_PIN"
[host_fs] [boards.cat_detector.hardware_resources]
[host_fs] TOF_NORTH_I2C_ADDR = { value = 0x30, type = "u8" }
[host_fs] TOF_EAST_I2C_ADDR = { value = 0x31, type = "u8" }
[host_fs] TOF_WEST_I2C_ADDR = { value = 0x32, type = "u8" }
[host_fs] DEFAULT_WAKE_THRESHOLD_MM = { value = 300, type = "u16" }
[host_fs] DEFAULT_PRESS_THRESHOLD_MM = { value = 20, type = "u16" }
[host_fs] DEFAULT_NEAR_THRESHOLD_MM = { value = 100, type = "u16" }
[host_fs] MAX_CRASH_LOGS = { value = 10, type = "u32" }
[host_fs] [boards.cat_detector.partitions.storage]
[host_fs] start = 0x001B0000
[host_fs] end = 0x00200000
[host_fs] [boards.cat_detector.partitions.metadata]
[host_fs] start = 0x001B0000
[host_fs] end = 0x001C0000
[host_fs] [boards.cat_detector.partitions.fs]
[host_fs] start = 0x001C0000
[host_fs] end = 0x001D0000
[host_fs] [boards.cat_detector.partitions.telemetry]
[host_fs] start = 0x001D0000
[host_fs] end = 0x00200000
[host_fs] [boards.carrier_board]
[host_fs] chip = "MCXN947"
[host_fs] board_type = "carrier_board"
[host_fs] program_metadata_addr = 0x001DF000
[host_fs] flash_base = 0x00000000
[host_fs] flash_size = 0x00200000         # 2 MB internal dual-bank NOR flash
[host_fs] flash_write_size = 128          # 128 B phrase program
[host_fs] flash_erase_size = 8192         # 8 KB sector erase
[host_fs] sram_size = 0x00080000          # 512 KB ECC SRAM
[host_fs] core0_stack_size = 24576        # 24 KB
[host_fs] core1_stack_size = 16384        # 16 KB
[host_fs] stack_top = 0x20080000
[host_fs] core0_stack_bottom = 0x2007A000
[host_fs] core_monitor_timeout_ms = 10000
[host_fs] core_monitor_warn_pct = 80
[host_fs] [boards.carrier_board.pins]
[host_fs] I2C0_SDA_PIN = 0
[host_fs] I2C0_SCL_PIN = 1
[host_fs] I2C1_SDA_PIN = 2
[host_fs] I2C1_SCL_PIN = 3
[host_fs] UART0_TX_PIN = 4
[host_fs] UART0_RX_PIN = 5
[host_fs] FUEL_GAUGE_INT_PIN = 6
[host_fs] CAP_TOUCH_INT_PIN = 7
[host_fs] PIEZO_PWM_PIN = 8
[host_fs] [boards.carrier_board.buses.i2c]
[host_fs] sda = "I2C0_SDA_PIN"
[host_fs] scl = "I2C0_SCL_PIN"
[host_fs] frequency = 400000
[host_fs] [boards.carrier_board.buses.uart]
[host_fs] tx = "UART0_TX_PIN"
[host_fs] rx = "UART0_RX_PIN"
[host_fs] [boards.carrier_board.hardware_resources]
[host_fs] FUEL_GAUGE_I2C_ADDR = { value = 0x36, type = "u8" }
[host_fs] LED_DRIVER_I2C_ADDR = { value = 0x14, type = "u8" }
[host_fs] TOUCH_CTRL_I2C_ADDR = { value = 0x44, type = "u8" }
[host_fs] MAX_CRASH_LOGS = { value = 10, type = "u32" }
[host_fs] [boards.carrier_board.partitions.storage]
[host_fs] start = 0x001E0000
[host_fs] end = 0x00200000
[host_fs] [boards.carrier_board.partitions.metadata]
[host_fs] start = 0x001E0000
[host_fs] end = 0x001E8000
[host_fs] [boards.carrier_board.partitions.fs]
[host_fs] start = 0x001E8000
[host_fs] end = 0x001F0000
[host_fs] [boards.carrier_board.partitions.telemetry]
[host_fs] start = 0x001F0000
[host_fs] end = 0x00200000
[debug] Target GDB server ready on Core 0 :3333 / Core 1 :3334 (RTT bridge :50000)
[daemon] RTT streaming process connected
[daemon] RTT stream disconnected
[host_fs] Successfully copied dev:_host_fs_test.toml to target/_host_fs_roundtrip.toml
[debug] Target GDB server ready on Core 0 :3333 / Core 1 :3334 (RTT bridge :50000)
[daemon] RTT streaming process connected
[daemon] RTT stream disconnected
[host_fs] Successfully removed file: _host_fs_test.toml
[debug] Target GDB server ready on Core 0 :3333 / Core 1 :3334 (RTT bridge :50000)
[daemon] RTT streaming process connected
[target] 0.002235 INFO [platform::core_monitor] MPU stack guard configured at 0x2003c000
[target] 0.023306 INFO [platform::core_monitor] MPU stack guard configured at 0x2000cf00
[target] 0.759411 INFO [controller::shell_controller] received command "System { subcommand: Some(Activity) }"
[target] 0.763834 INFO [controller::shell_controller] command execution succeeded
[daemon] Updated configuration: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, profile=debug, trace_mode=NONE, speed=4000, debug_server=True (GDB :60755/:60756)
[daemon] RTT stream disconnected
[debug] Target GDB server ready on Core 0 :60755 / Core 1 :60756 (RTT bridge :60757)
[debug] GDB client gdb_connected on Core 0
[debug] GDB client gdb_connected on Core 1
[daemon] RTT streaming process connected
[debug] GDB client gdb_disconnected on Core 1
[debug] GDB client gdb_disconnected on Core 0
[daemon] Updated configuration: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, profile=debug, trace_mode=NONE, speed=4000, debug_server=True (GDB :3333/:3334)
[daemon] RTT stream disconnected
[debug] host_debug error: Failed to open debug probe: ProbeSpecific(BoxedProbeError(Usb(Custom { kind: TimedOut, error: "bulk write timed out" })))
[host_cli] Target CLI is not available (RTT console is not connected). Please synchronize target.
INTERACTIVE CLI CONSOLE (host_cli)
📋 COPY
CHANNEL: BOTH
shell> 
shell> 
shell> echo 'Firmware verification output'
shell> Unknown command: 'echo'
shell> gpio status
GPIO Pin Status (0..29):
Pin   Level
-----------
GP0   Alt
GP1   Alt
GP2   Low  (0)
GP3   Low  (0)
GP4   High (1)
GP5   High (1)
GP6   High (1)
GP7   High (1)
GP8   Low  (0)
GP9   High (1)
GP10  High (1)
GP11  Alt
GP12  Alt
GP13  Alt
GP14  High (1)
GP15  Low  (0)
GP16  Low  (0)
GP17  Low  (0)
GP18  Low  (0)
GP19  Low  (0)
GP20  Low  (0)
GP21  Low  (0)
GP22  High (1)
GP23  Low  (0)
GP24  Low  (0)
GP25  Low  (0)
GP26  High (1)
GP27  Low  (0)
GP28  Low  (0)
GP29  Low  (0)
shell> core status
Core Monitor Status:
Core  Status       Last Progress  Stuck  Panicked  Timeout
----------------------------------------------------------
Core0 Running      22023          false  false     10000ms
Core1 Running      22024          false  false     10000ms
shell> 
shell> 
shell> system activity
System activity recorded.
shell> core status
shell> echo 'Firmware verification output'
Target CLI is not available (RTT console is not connected). Please synchronize target.
shell>
SEND [↵]
```
