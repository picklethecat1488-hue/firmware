# 🔴 `[WORM-SERPENTINE-PUPA-18]` Integrate bringup.py with host

- **UUID**: `ca867715-3521-49ac-b103-a48314db3f37`
- **ID**: `WORM-SERPENTINE-PUPA-18`
- **Status**: `OPEN`
- **Severity**: `LOW`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host, bringup`
- **Created**: `2026-10-17 19:32:52 UTC`

#### Description

Pls integrate the bringup.py functionality (./tools/helpers/bringup.py) into tools/host.py; add a separate Bringup script view which supports the bringup YAML schema, as well as a bringup subcommand which allows bringup scripts to be parsed and loaded. Allow different bringup scripts to be loaded from the bringup script view. The bringup script view should show CLI output, as well as present interactive prompt and detailed bringup status.
