# 🔴 `[WORM-LUBRICATED-FLATWORM-512]` Debug connection broken after single stepping

- **UUID**: `00096b59-c515-43fe-93ce-5dc6c32910fa`
- **ID**: `WORM-LUBRICATED-FLATWORM-512`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-20 20:14:54 UTC`

#### Description

The debug connection broke after single stepping past a breakpoint, about a minute later I got a target reset event from RSP:

ortex-Debug: VSCode debugger extension version 1.12.1 git(652d042). Usage info: https://github.com/Marus/cortex-debug#usage
Reading symbols from arm-none-eabi-objdump --syms -C -h -w /Users/<username>/gh/firmware/target/active_target.elf
Reading symbols from arm-none-eabi-nm --defined-only -S -l -C -p /Users/<username>/gh/firmware/target/active_target.elf
Launching GDB: arm-none-eabi-gdb -q --interpreter=mi2
    IMPORTANT: Set "showDevDebugOutput": "raw" in "launch.json" to see verbose GDB transactions here. Very helpful to debug issues or report problems
Finished reading symbols from objdump: Time: 24 ms
Finished reading symbols from nm: Time: 38 ms
Output radix now set to decimal 10, hex a, octal 12.
Input radix now set to decimal 10, hex a, octal 12.
embassy_executor::arch::thread::Executor::run<cat_detector_shell::__cortex_m_rt_main::{closure_env#0}> (self=0x20041fe8, init=...) at /Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/embassy-executor-0.7.0/src/arch/cortex_m.rs:105
105	                    self.inner.poll();
Program stopped, probably due to a reset and/or halt issued by debugger
Undefined command: "from".  Try "help".

Program
 received signal SIGINT, Interrupt.
embassy_executor::arch::thread::Executor::run<cat_detector_shell::__cortex_m_rt_main::{closure_env#0}> (self=0x20041fe8, init=...) at /Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/embassy-executor-0.7.0/src/arch/cortex_m.rs:105
105	                    self.inner.poll();
Undefined command: "from".  Try "help".

Program
 received signal SIGTRAP, Trace/breakpoint trap.
0x100001c0 in Reset ()
Undefined command: "from".  Try "help".
b main
Breakpoint 2 at 0x100118b0: file app/src/cat_detector_shell.rs, line 55.
{"output":"","token":51,"outOfBandRecord":[],"resultRecords":{"resultClass":"done","results":[]}}

Breakpoint 2, cat_detector_shell::__cortex_m_rt_main_trampoline () at app/src/cat_detector_shell.rs:55
55	#[embassy_executor::main]
Undefined command: "from".  Try "help".

Breakpoint 1.1, cat_detector_shell::____embassy_main_task::{async_fn#0} () at app/src/cat_detector_shell.rs:60
60	    let mut board = app::Board::init(p).await;
Undefined command: "from".  Try "help".
Target reset and halted at Reset_Handler
Register cache flushed.
Undefined command: "from".  Try "help".

Breakpoint 2, cat_detector_shell::__cortex_m_rt_main_trampoline () at app/src/cat_detector_shell.rs:55
55	#[embassy_executor::main]
Undefined command: "from".  Try "help".

Breakpoint 1.1, cat_detector_shell::____embassy_main_task::{async_fn#0} () at app/src/cat_detector_shell.rs:60
60	    let mut board = app::Board::init(p).await;
Undefined command: "from".  Try "help".

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_shell --release
2. Execute target command via python tools/host.py cli <command>
3. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] Updated configuration: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, profile=release, speed=1000, debug_server=True (GDB :56947/:56948)
[] [daemon] [INFO] [Sync] Compiling target firmware...
[] [daemon] [INFO] Building target firmware: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi, release=True
[] [build] [INFO] [1m[92m   Compiling[0m cat_detector v0.1.0 (/Users/<username>/gh/firmware/app)
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 3.31s
[] [daemon] [INFO] Target binary built successfully: /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs, host_debug)...
[] [daemon] [INFO] Building host utilities via cargo build-host-tools
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.31s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/release/host_cli, fs=/Users/<username>/gh/firmware/target/release/host_fs, debug=/Users/<username>/gh/firmware/target/release/host_debug
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via probe-rs: download --chip rp2040 --speed 1000 /Users/<username>/gh/firmware/target/thumbv6m-none-eabi/release/cat_detector_shell
[] [probe-rs] [INFO]      [1;32mFinished[0m in 9.20s
[] [daemon] [INFO] Flash download succeeded via probe-rs
[] [daemon] [INFO] [Sync] Starting probe-rs GDB server (:56947/:56948) & RTT bridge...
[] [debug] [INFO] Target GDB server ready on Core 0 :56947 / Core 1 :56948 (RTT bridge :50000)
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.001596 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.021783 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000c500
[] [daemon] [INFO] [Sync] Target synchronized & GDB server active on :56947/:56948.
[] [daemon] [INFO] Target bug created: WORM-MOIST-BRISTLE-687 (Tracebacks in debug terminal) with 1 attachments
[] [debug] [INFO] GDB client gdb_connected on Core 0
[] [debug] [INFO] GDB client gdb_connected on Core 1
[] [target] [INFO] 0.001571 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.001593 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
```

#### Review Tasks & Functional Checklist

- [ ] Verify GDB attach, breakpoint at main / Board::init, single-step (stepi / next), and continue complete cleanly without ISR vectoring, reset, or Undefined command 'from' | `python tools/host.py debug start && arm-none-eabi-gdb -q -batch -ex 'set sysroot /' -ex 'source .vscode/cortex_debug_init.gdb' -ex 'target extended-remote | python3 -u tools/host.py debug gdb-proxy --core 0' -ex 'monitor reset halt' -ex 'maintenance flush register-cache' -ex 'b main' -ex 'continue' -ex 'stepi' -ex 'stepi' -ex 'detach' -ex 'disconnect'` <!-- id:fde800af -->
