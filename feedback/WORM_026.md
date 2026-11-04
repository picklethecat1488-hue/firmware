# 🟢 `[WORM-026]` It doesn’t look like the dashboard is closing automatically :-(

- **UUID**: `f863343e-01e3-4a38-a010-6dbb55320a05`
- **ID**: `WORM-026`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `Das`
- **Created**: `2026-10-09 02:49:22 UTC`
- **Resolved**: `2026-10-09 03:13:46 UTC`

#### Description

See traceback below

#### Execution / Console Logs

```text
<username> at THE-HEARTH-NODE in ~/gh/firmware (main●●) 
$ python tools/dashboard.py 
Dashboard workstation is already running at http://127.0.0.1:8877 (PID 99042).
(firmware-env) 
<username> at THE-HEARTH-NODE in ~/gh/firmware (main●●) 
$ python tools/dashboard.py
Dashboard workstation is already running at http://127.0.0.1:8877 (PID 99042).
(firmware-env) 
<username> at THE-HEARTH-NODE in ~/gh/firmware (main●●) 
$ python tools/dashboard.py
Dashboard workstation is already running at http://127.0.0.1:8877 (PID 99042).
(firmware-env)
```

#### Resolution Notes

Hook window closed event to terminate pywebview process cleanly on window exit, ensure server shutdown unlinks lock file and exits, and automatically detect and recover orphaned background server instances.
