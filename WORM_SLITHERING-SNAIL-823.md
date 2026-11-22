# 🔴 `[WORM-SLITHERING-SNAIL-823]` The RSP barfed out again

- **UUID**: `cbf5a1cd-bfa6-4e88-8b3a-e8232fbcc9f5`
- **ID**: `WORM-SLITHERING-SNAIL-823`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-21 23:46:13 UTC`

#### Description

The RSP appeared to fail when I tried restarting and single stepping:

ortex-Debug: VSCode debugger extension version 1.12.1 git(652d042). Usage info: https://github.com/Marus/cortex-debug#usage
Reading symbols from arm-none-eabi-objdump --syms -C -h -w /Users/<username>/gh/firmware/target/active_target.elf
Reading symbols from arm-none-eabi-nm --defined-only -S -l -C -p /Users/<username>/gh/firmware/target/active_target.elf
Launching GDB: arm-none-eabi-gdb -q --interpreter=mi2
    IMPORTANT: Set "showDevDebugOutput": "raw" in "launch.json" to see verbose GDB transactions here. Very helpful to debug issues or report problems
Finished reading symbols from objdump: Time: 23 ms
Finished reading symbols from nm: Time: 41 ms
Output radix now set to decimal 10, hex a, octal 12.
Input radix now set to decimal 10, hex a, octal 12.
platform::system::{impl#0}::run_loop (self=0x20026614 <rp2040::EXECUTOR_CORE1+4>, cpu_id=<optimized out>) at platform/src/system/scheduler.rs:222
222	                cortex_m::asm::wfe();
Program stopped, probably due to a reset and/or halt issued by debugger
Warning:
Cannot insert breakpoint 2.
Cannot access memory at address 0x10008aca
Cannot insert breakpoint 1.
Cannot access memory at address 0x10008e42

GDB Error? continue command is reported as error after initially succeeding. token '25'
Error: A serious error occurred with gdb, unable to continue or interrupt We may not be able to recover from this point. You can try continuing or ending session. Must address root cause though
Command aborted.
Warning:
Cannot insert breakpoint 2.
Cannot access memory at address 0x10008aca
Cannot insert breakpoint 1.
Cannot access memory at address 0x10008e42

GDB Error? continue command is reported as error after initially succeeding. token '34'
Error: A serious error occurred with gdb, unable to continue or interrupt We may not be able to recover from this point. You can try continuing or ending session. Must address root cause though
Command aborted.
I also observed there are almost no useful logs in target/host_daemon.log:

$ cat target/host_daemon.log 
[HostDaemon] Native webview launch failed: No module named 'tools'
[HostDaemon] Native webview launch failed: No module named 'tools'
======================================================================
  ⚡ SYSTEM SHOCK 2 // XERXES HOST DAEMON WORKSTATION
======================================================================
  Target:    thumbv6m-none-eabi
  Package:   cat_detector (cat_detector_shell)
  Profile:   release
  Speed:     4000 kHz
  Port:      8899

  Workstation URL: http://127.0.0.1:8899 (PID 95651)
  Press Ctrl+C to stop daemon.

➜ Host daemon running at http://127.0.0.1:8899 (PID 8106).
  Press Ctrl+C to stop daemon.

➜ Target package selection mode: awaiting configuration modal confirmation in workstation.
➜ Host daemon running at http://127.0.0.1:8899 (PID 14903).
  Press Ctrl+C to stop daemon.

➜ Target package selection mode: awaiting configuration modal confirmation in workstation.
➜ Host daemon running at http://127.0.0.1:8899 (PID 19566).
  Press Ctrl+C to stop daemon.

➜ Host daemon running at http://127.0.0.1:8899 (PID 10585).
  Press Ctrl+C to stop daemon.

➜ Host daemon running at http://127.0.0.1:8899 (PID 56593).
  Press Ctrl+C to stop daemon.

➜ Host daemon running at http://127.0.0.1:8899 (PID 47440).
  Press Ctrl+C to stop daemon.

➜ Target package selection mode: awaiting configuration modal confirmation in workstation.
➜ Host daemon running at http://127.0.0.1:8899 (PID 18554).
  Press Ctrl+C to stop daemon.

➜ Host daemon running at http://127.0.0.1:8899 (PID 21346).
  Press Ctrl+C to stop daemon.

➜ Host daemon running at http://127.0.0.1:8899 (PID 60836).
  Press Ctrl+C to stop daemon.

➜ Host daemon running at http://127.0.0.1:8899 (PID 62895).
  Press Ctrl+C to stop daemon.

➜ Host daemon running at http://127.0.0.1:8899 (PID 65048).
  Press Ctrl+C to stop daemon.

➜ Target package selection mode: awaiting configuration modal confirmation in workstation.
➜ Host daemon running at http://127.0.0.1:55170 (PID 70057).
  Press Ctrl+C to stop daemon.

➜ Host daemon running at http://127.0.0.1:8899 (PID 17784).
  Press Ctrl+C to stop daemon.

➜ Target package selection mode: awaiting configuration modal confirmation in workstation.
➜ Host daemon running at http://127.0.0.1:8899 (PID 34185).
  Press Ctrl+C to stop daemon.

➜ Target package selection mode: awaiting configuration modal confirmation in workstation.
➜ Host daemon running at http://127.0.0.1:8899 (PID 11323).
  Press Ctrl+C to stop daemon.

(firmware-env) 

This should log when any API is invoked, as well as when target state changes. The debug and trace backends should log to this file and host_daemon logs should be attached to host bug reports

#### Reproduction Steps

1. Attach using “Attach Dual-Core” or “Attach Core0”
2. Click restart

#### Execution / Console Logs

```text
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
[] [debug] [INFO] Target GDB server ready on Core 0 :3333 / Core 1 :3334 (RTT bridge :50000)
[] [daemon] [INFO] RTT streaming process connected
[] [daemon] [INFO] RTT stream disconnected
[] [host_fs] [INFO] Successfully copied dev:_host_fs_test.toml to target/_host_fs_roundtrip.toml
[] [debug] [INFO] Target GDB server ready on Core 0 :3333 / Core 1 :3334 (RTT bridge :50000)
[] [daemon] [INFO] RTT streaming process connected
[] [daemon] [INFO] RTT stream disconnected
```

#### Review Tasks & Functional Checklist

- [ ] Verify GDB restart, XIP hardware breakpoint insertion, and continue over gdb-proxy with zero warnings | `python tools/host.py --bin cat_detector_shell --debug-server && arm-none-eabi-gdb --nx --batch -ex 'set sysroot /' -ex 'source .vscode/cortex_debug_init.gdb' -ex 'target extended-remote | python3 -u tools/host.py debug gdb-proxy --core 0' -ex 'break app/src/cat_detector_shell.rs:61' -ex 'monitor reset halt' -ex 'maintenance flush register-cache' -ex 'continue' -ex 'detach' target/active_target.elf` <!-- id:96622b5b -->
