# 🟢 `[WORM-SLIMY-SNAIL-582]` host_daemon.py-pywebkit window didn’t pop up

- **UUID**: `f039a786-2d34-430a-a840-034538f47df3`
- **ID**: `WORM-SLIMY-SNAIL-582`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host_daemon`
- **Created**: `2026-10-17 01:31:37 UTC`
- **Resolved**: `2026-10-17 01:44:25 UTC`

#### Description

See logs below. The app window didn’t open for me. I had to open the page from the Workstation URL instead. Also, can we remove this banner:

======================================================================
  ⚡ SYSTEM SHOCK 2 // XERXES HOST DAEMON WORKSTATION
======================================================================
  Target:    thumbv6m-none-eabi
  Package:   cat_detector (cat_detector_shell)
  Profile:   release
  Speed:     4000 kHz
  Port:      8899

#### Execution / Console Logs

```text
$ PROBE_RS_SPEED=4000 python tools/host_daemon.py  --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release
======================================================================
  ⚡ SYSTEM SHOCK 2 // XERXES HOST DAEMON WORKSTATION
======================================================================
  Target:    thumbv6m-none-eabi
  Package:   cat_detector (cat_detector_shell)
  Profile:   release
  Speed:     4000 kHz
  Port:      8899
  Workstation URL: http://127.0.0.1:8899 (PID 95651)
➜ Detached from terminal.
(firmware-env) 
<username> at THE-HEARTH-NODE in ~/gh/firmware (main) 
$ PROBE_RS_SPEED=4000 python tools/host_daemon.py  --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release
Host daemon is already running (PID 95651 on port 8899).
Workstation URL: http://127.0.0.1:8899
(firmware-env)
```

#### Resolution Notes

Added repo root to sys.path in host_daemon launcher and cli to fix native webview/eel launcher imports without PYTHONPATH, and removed ASCII box banner from host_daemon startup.
