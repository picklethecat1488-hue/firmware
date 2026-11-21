# 🟢 `[WORM-MOIST-BRISTLE-687]` Tracebacks in debug terminal

- **UUID**: `f41aed47-84ea-4145-8070-f64b63da97d8`
- **ID**: `WORM-MOIST-BRISTLE-687`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `target`
- **Created**: `2026-11-20 20:12:07 UTC`
- **Resolved**: `2026-11-20 23:03:48 UTC`

#### Description

Trace backs in debug terminal when attaching to dual-core. Also, we need to update the defualt debugg ports (They no longer match the default ports in the target selection window):

 *  Executing task: python tools/host.py debug sync-symbols 

Traceback (most recent call last):
  File "/Users/<username>/gh/firmware/tools/host.py", line 31, in <module>
    from host_daemon.cli import main
  File "/Users/<username>/gh/firmware/tools/host_daemon/__init__.py", line 4, in <module>
    from host_daemon.provider.daemon.runner import DaemonRunner
  File "/Users/<username>/gh/firmware/tools/host_daemon/provider/daemon/__init__.py", line 3, in <module>
    from host_daemon.provider.daemon.runner import DaemonRunner
  File "/Users/<username>/gh/firmware/tools/host_daemon/provider/daemon/runner.py", line 24, in <module>
    from host_daemon.provider.daemon.test_manager import DeviceTestRunnerManager
  File "/Users/<username>/gh/firmware/tools/host_daemon/provider/daemon/test_manager.py", line 16, in <module>
    from device_tests.model import (
    ...<4 lines>...
    )
  File "/Users/<username>/gh/firmware/tools/device_tests/__init__.py", line 3, in <module>
    from device_tests.client import AsyncHostSessionClient, LogStreamWatcher, SyncOutcome
  File "/Users/<username>/gh/firmware/tools/device_tests/client.py", line 15, in <module>
    from device_tests.model import (
    ...<5 lines>...
    )
  File "/Users/<username>/gh/firmware/tools/device_tests/model.py", line 14, in <module>
    import pytest
ModuleNotFoundError: No module named 'pytest'

 *  The terminal process "/bin/zsh '-l', '-c', 'python tools/host.py debug sync-symbols'" failed to launch (exit code: 1). 
 *  Terminal will be reused by tasks, press any key to close it. 

 *  Executing task: python tools/host.py debug sync-symbols 

Traceback (most recent call last):
  File "/Users/<username>/gh/firmware/tools/host.py", line 31, in <module>
    from host_daemon.cli import main
  File "/Users/<username>/gh/firmware/tools/host_daemon/__init__.py", line 4, in <module>
    from host_daemon.provider.daemon.runner import DaemonRunner
  File "/Users/<username>/gh/firmware/tools/host_daemon/provider/daemon/__init__.py", line 3, in <module>
    from host_daemon.provider.daemon.runner import DaemonRunner
  File "/Users/<username>/gh/firmware/tools/host_daemon/provider/daemon/runner.py", line 24, in <module>
    from host_daemon.provider.daemon.test_manager import DeviceTestRunnerManager
  File "/Users/<username>/gh/firmware/tools/host_daemon/provider/daemon/test_manager.py", line 16, in <module>
    from device_tests.model import (
    ...<4 lines>...
    )
  File "/Users/<username>/gh/firmware/tools/device_tests/__init__.py", line 3, in <module>
    from device_tests.client import AsyncHostSessionClient, LogStreamWatcher, SyncOutcome
  File "/Users/<username>/gh/firmware/tools/device_tests/client.py", line 15, in <module>
    from device_tests.model import (
    ...<5 lines>...
    )
  File "/Users/<username>/gh/firmware/tools/device_tests/model.py", line 14, in <module>
    import pytest
ModuleNotFoundError: No module named 'pytest'

 *  The terminal process "/bin/zsh '-l', '-c', 'python tools/host.py debug sync-symbols'" failed to launch (exit code: 1). 
 *  Terminal will be reused by tasks, press any key to close it.

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
```

#### Attached Commits

- `0e036c49`
- `05233081`

#### Review Tasks & Functional Checklist

- [ ] Verify VS Code login-shell preLaunchTask syncs active target symbols cleanly without pytest installed | `/bin/zsh -l -c 'python3 tools/host.py debug sync-symbols'` <!-- id:fe716276 -->
- [ ] Verify GDB server status reports default 3333/3334 ports matching Target Config and VS Code gdb-proxy attaches without manual port prompts | `python tools/host.py debug status` <!-- id:c7345f87 -->

#### Resolution Notes

Eliminated eager pytest/daemon imports so 'python3 tools/host.py debug sync-symbols' runs cleanly on pure Python 3 stdlib in any VS Code login shell, isolated ephemeral test debug ports in DaemonRunner.enable_live_debug_server/disable_live_debug_server so Target Selection defaults (3333/3334) are never overwritten, and added 'host.py debug gdb-proxy --core 0/1' in .vscode/launch.json so VS Code attaches automatically to the active ports configured in Target Selection.
