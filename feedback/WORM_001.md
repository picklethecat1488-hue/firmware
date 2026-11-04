# 🟢 `[WORM-001]` Remove hardware-specific categories from the dashboard

- **UUID**: `0afd22f6-361f-424d-a361-21ec90d4c818`
- **ID**: `WORM-001`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-03 19:26:02 UTC`
- **Resolved**: `2026-10-03 19:43:19 UTC`

#### Description

Remove hardware-specific categories from the dashboard. We will use the hardware dashboard to track hardware issues.

#### Resolution Notes

Removed hardware categories (PCB, CAD, SIMULATION) and replaced with firmware categories in WormCategory, models, exporters, and UI templates
