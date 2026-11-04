# 🟢 `[WORM-COILING-CASTING-220]` Add RESET button to host daemon

- **UUID**: `aa2ef55c-a6b3-4c69-89ff-1fa2c9985890`
- **ID**: `WORM-COILING-CASTING-220`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host_daemon`
- **Created**: `2026-10-17 01:30:56 UTC`
- **Resolved**: `2026-10-17 01:47:23 UTC`

#### Description

Please add a way to system reset the target using host daemon

#### Resolution Notes

Added target microcontroller reset support via probe-rs, exposed /api/reset POST endpoint, added Reset button in workstation UI header, and added 'reset' CLI subcommand.
