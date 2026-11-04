# 🟢 `[WORM-015]` Unable to open code review or worm report pages from VCS UI

- **UUID**: `5d358f8e-a5fa-4bfc-ba86-0231449f60e8`
- **ID**: `WORM-015`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-07 03:39:52 UTC`
- **Resolved**: `2026-10-07 03:41:04 UTC`

#### Description

workstationModal in diff_view.html.j2 was mistakenly nested inside modalBranchPicker due to a missing closing div tag on modalBranchPicker introduced in f4fc95e2. Because modalBranchPicker has display: none, workstationModal was never visible when opened.

#### Resolution Notes

Closed modalBranchPicker div properly so workstationModal is mounted directly under body rather than being hidden inside modalBranchPicker.
