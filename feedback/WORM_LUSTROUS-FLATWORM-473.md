# 🟢 `[WORM-LUSTROUS-FLATWORM-473]` Tons of untracked files under recordings/ and dashboard is slow to respond

- **UUID**: `85f9dc39-75f5-4b0d-bbd4-1e09c39ad127`
- **ID**: `WORM-LUSTROUS-FLATWORM-473`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-23 00:06:57 UTC`
- **Resolved**: `2026-10-23 00:15:00 UTC`

#### Resolution Notes

Removed hundreds of untracked temporary frame extraction directories and audio files from recordings/previews/ and updated .gitignore with patterns for temporary recording artifacts so workspace git scans and dashboard remain fast.
