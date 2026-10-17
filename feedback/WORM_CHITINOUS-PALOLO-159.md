# 🟢 `[WORM-CHITINOUS-PALOLO-159]` Disable CLI while running a command

- **UUID**: `2cbea987-58db-49c3-9adc-d5e41e0ee9ab`
- **ID**: `WORM-CHITINOUS-PALOLO-159`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host_daemon, dashboard`
- **Created**: `2026-10-17 02:56:38 UTC`
- **Resolved**: `2026-10-17 03:07:16 UTC`

#### Description

We should disable the CLI input window while a command is running. I was able to call “fs ls” twice, and generate the below execution log.

#### Reproduction Steps

1. 0. PROBE_RS_SPEED=4000 python tools/host_daemon.py  --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release
2. 1. Fs ls
3. 2. fs ls

#### Execution / Console Logs

```text
[host_fs] Error: Custom { kind: Other, error: "Failed to open probe: Usb(Custom { kind: Other, error: \"error while taking control over USB device: could not open interface for exclusive access (error 0xe00002c5)\" })" }
[host_fs] Error: Custom { kind: Other, error: "Failed to open probe: Usb(Custom { kind: Other, error: \"error while taking control over USB device: could not open interface for exclusive access (error 0xe00002c5)\" })" }
[host_fs] No files found (directory empty).
```

#### Resolution Notes

Disabled CLI and FS command input windows and buttons in host_daemon and code_review workstations while a command is actively executing. Added _fs_lock and sync check to DaemonRunner.execute_fs to serialize hardware probe access and prevent USB exclusive access errors when commands are invoked consecutively.
