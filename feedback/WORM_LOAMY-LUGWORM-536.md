# 🟢 `[WORM-LOAMY-LUGWORM-536]` Add Target Bug pointer to host tool

- **UUID**: `3f007724-d826-429a-bbc0-d0ee46a40167`
- **ID**: `WORM-LOAMY-LUGWORM-536`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host`
- **Created**: `2026-10-17 19:31:59 UTC`
- **Resolved**: `2026-10-17 21:42:52 UTC`

#### Description

Please add a Target Bug button to each host tool window (similar to how Target Bug works in the SSD repo) to allow easy creation of bug reports from the host tool. Automatically attach relevant system logs and populate the reproduction steps. Also add options to attach target files- crash logs, telemetry, etc. from target automatically (if selected)

#### Resolution Notes

Added Target Bug button to host tool header with dedicated defect modal, pre-filled reproduction steps, and automatic collection of system logs, target crash logs, telemetry exports, and filesystem metadata into worm attachments.
