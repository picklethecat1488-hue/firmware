# 🟢 `[WORM-009]` Still seeing issues with multiple Diff Viewer windows being open after exiting from the Code Review and Bug Report windows

- **UUID**: `454cbbfa-94ce-4f56-8fa8-99fca7fc20de`
- **ID**: `WORM-009`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `INFRASTRUCTURE`
- **Created**: `2026-10-04 00:20:04 UTC`
- **Resolved**: `2026-10-04 00:31:49 UTC`

#### Resolution Notes

Enforced window.name reuse on Diff Viewer, Code Review, and Worm Tracker; updated returnToDashboard and setVerdict to focus opener or existing named window and close instead of spawning duplicate windows
