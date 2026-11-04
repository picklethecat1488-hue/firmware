# 🟢 `[WORM-ANNELIDIC-BOOKWORM-399]` Cleanup host_cli/host_fs when closing host daemon

- **UUID**: `6293b901-13c6-4b18-8721-204fd7bd0bdb`
- **ID**: `WORM-ANNELIDIC-BOOKWORM-399`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host_daemon`
- **Created**: `2026-10-17 02:59:21 UTC`
- **Resolved**: `2026-10-17 05:45:44 UTC`

#### Description

Please clean up the host_cli and host_fs tools when the main host daemon window is closed. I had to kill host_cli to start a new daemon instance

#### Execution / Console Logs

```text
<username> at THE-HEARTH-NODE in ~/gh/firmware (main) 
$ PROBE_RS_SPEED=4000 python tools/host_daemon.py  --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release
➜ Host daemon running at http://127.0.0.1:8899 (PID 22338).
➜ Detached from terminal.
(firmware-env) 
<username> at THE-HEARTH-NODE in ~/gh/firmware (main) 
$ PROBE_RS_SPEED=4000 python tools/host_daemon.py  --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release
➜ Host daemon running at http://127.0.0.1:8899 (PID 23134).
➜ Detached from terminal.
(firmware-env) 
<username> at THE-HEARTH-NODE in ~/gh/firmware (main) 
$ killall host_cli
(firmware-env) 
<username> at THE-HEARTH-NODE in ~/gh/firmware (main) 
$ PROBE_RS_SPEED=4000 python tools/host_daemon.py  --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release
➜ Host daemon is running (PID 23134 on port 8899).
➜ Reconfiguring and flashing target: cat_detector (cat_detector_shell) [release]...
➜ Target synchronization and flash started.
Workstation URL: ht
```

#### Resolution Notes

Terminated both rtt_process (host_cli) and _active_sync_process in runner.stop(), and hooked runner.stop() into _on_app_close(), atexit, and SIGTERM/SIGINT handlers.
