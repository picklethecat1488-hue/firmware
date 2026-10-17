# 🔴 `[WORM-COMPOSTED-PALOLO-725]` Not able to flash a new app with host daemon

- **UUID**: `abe0e38f-f71e-41f9-99c4-a45784148233`
- **ID**: `WORM-COMPOSTED-PALOLO-725`
- **Status**: `OPEN`
- **Severity**: `HIGH`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host_daemon`
- **Created**: `2026-10-17 01:45:50 UTC`

#### Description

Not able to flash a new app with host daemon due to the associated execution error

#### Execution / Console Logs

```text
$ PROBE_RS_SPEED=4000 python tools/host_daemon.py  --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_app --release 
Host daemon is already running (PID 95651 on port 8899).
Workstation URL: http://127.0.0.1:8899
(firmware-env)
```
