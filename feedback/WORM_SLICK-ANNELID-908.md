# 🟢 `[WORM-SLICK-ANNELID-908]` When calling host.py with no args, it gets stuck at the initial flashing UI

- **UUID**: `a75fd6e7-5113-4316-ad45-66c67f310f98`
- **ID**: `WORM-SLICK-ANNELID-908`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host`
- **Created**: `2026-10-21 23:50:31 UTC`
- **Resolved**: `2026-10-21 23:54:49 UTC`

#### Description

When host.py is invoked without arguments, confirming the target package configuration modal displays the flashing matrix modal (syncModal) but fails to initiate progress polling (startSyncPolling), leaving the UI permanently frozen at 0% with "Synchronizing target firmware...". Furthermore, invoking host.py with no arguments while a daemon is running with different build profile flags erroneously treats default CLI options as a configuration change and starts background sync without bringing forward the UI.

#### Resolution Notes

Fixed submitTargetConfig in host_daemon.html.j2 to trigger startSyncPolling() and initialize UI progress indicators. Synchronously set active state in runner.sync_async() to avoid polling race conditions. Guarded config_changed in cli.py with is_explicit so invoking host.py with no args against a running daemon preserves configuration.
