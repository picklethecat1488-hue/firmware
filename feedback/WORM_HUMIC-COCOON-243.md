# 🟢 `[WORM-HUMIC-COCOON-243]` Sorting by ID no longer works as intended

- **UUID**: `4ffd9e1f-863d-43ec-9fa2-8e92aaae68a2`
- **ID**: `WORM-HUMIC-COCOON-243`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-22 00:00:15 UTC`
- **Resolved**: `2026-10-22 00:24:46 UTC`

#### Description

Sorting all bugs by the ID no longer works as intended-it no longer lists the newest to oldest bugs or vice versa. Please fix this in the dashboard of both hardware and firmware repos.

#### Resolution Notes

Sorted bugs chronologically by created_at with fallback to numeric ID sequence across SQLite backing store and dashboard UI in both firmware and hardware repositories.
