# 🟢 `[WORM-021]` Detach dashboard from terminal after opening

- **UUID**: `61e94a99-68fb-4ea0-9c7b-f4f4d09bd59f`
- **ID**: `WORM-021`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-08 21:06:01 UTC`
- **Resolved**: `2026-10-08 21:26:46 UTC`

#### Description

Add a check to dashboard.py to ensure that only one dashboard can run at a time, and detach the dashboard from the current terminal after loading completes.

#### Resolution Notes

Added single-instance check preventing duplicate dashboard processes using lock file and port check, and implemented terminal detachment after loading completes with --no-detach/--foreground and --stop CLI controls.
