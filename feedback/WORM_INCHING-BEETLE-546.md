# 🟢 `[WORM-INCHING-BEETLE-546]` Creating a bug with a large attachment may crash the dashboard

- **UUID**: `198ffd38-695e-4360-b783-99f5f1a5fb48`
- **ID**: `WORM-INCHING-BEETLE-546`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-21 23:58:47 UTC`
- **Resolved**: `2026-10-22 00:31:44 UTC`

#### Description

Creatign a bug with a very large attatchment (>=2GB) may crash the dashboard. Please fix this one in the hardware and firmware repos.

#### Resolution Notes

Guarded against requests and attachments >= 2GB (GitHub LFS limit) with HTTP 413 and client-side size checks, and streamed file downloads in 64 KiB chunks across firmware and hardware dashboards.
