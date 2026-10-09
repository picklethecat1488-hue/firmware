# 🟢 `[WORM-023]` Add Planned status to worms

- **UUID**: `be9303ee-9ed8-4d56-a20c-0974ab03587c`
- **ID**: `WORM-023`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-08 21:08:10 UTC`
- **Resolved**: `2026-10-08 21:18:20 UTC`

#### Description

Add a "Planned" status to Worms, and update skills and dashboard CLI so that the agent will not work on Planned Worms by default. Keep the default status in the Worm console as Open.

#### Resolution Notes

Added PLANNED status to WormStatus enum, updated CLI to exclude PLANNED from --open and added --planned filter, updated worm report template to include PLANNED tab with default OPEN status, and documented agent planned worm exemption in GEMINI.md and skill.
