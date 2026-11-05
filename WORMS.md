# Worm Report Tracker: Firmware Engineering Worm Tracker

> Automated bug tracking, triage, and issue registry generated via Firmware Bug Report Engine.

## Tracker Overview

| Metric | Details |
| :--- | :--- |
| **Report Date** | `2026-11-05 15:29:18 UTC` |
| **Total Issues** | `3` |
| **Open Issues** | `1` |
| **Resolved / Closed** | `2 (66%)` |

## Issues by Severity

| Severity | Count | Meaning |
| :--- | :---: | :--- |
| **`[CRITICAL]`** | 0 | System crashes, build failures, blockages, or electrical shorts. |
| **`[HIGH]`** | 1 | Major functional defects, broken routing, DRC violations, or unphysical behavior. |
| **`[MEDIUM]`** | 1 | Silkscreen collisions, layout sub-optimality, or visual clipping. |
| **`[LOW]`** | 1 | Minor aesthetic imperfections or documentation notes. |

## Issues by Category

| Category | Count | Description |
| :--- | :---: | :--- |
| **`FIRMWARE`** | 2 | Core firmware logic, state machines, async tasks. |
| **`CONTROLLER`** | 0 | Domain controllers, PID loops, event dispatch. |
| **`DRIVER`** | 0 | Hardware peripheral drivers, embedded-hal. |
| **`PLATFORM`** | 0 | Chip support, PAC, HAL, clock/power management. |
| **`BOARD`** | 0 | Board support packages (BSP), pinmux, boards. |
| **`MODEL`** | 0 | Domain data models, configurations, state definitions. |
| **`SHELL`** | 0 | CLI interface, shell commands, debug console. |
| **`INFRASTRUCTURE`** | 0 | Build tooling, compilers, test runners, headless tools. |
| **`UI`** | 1 | Web dashboards, CLI viewers, review interfaces. |
| **`GENERAL`** | 0 | Unclassified or cross-cutting firmware issues. |

## Issue Checklist

- [ ] **`[LOW]`** [#WORM-OPEN-003](#worm-open-003): Open defect (`OPEN`)
- [x] **`[MEDIUM]`** [#WORM-LEGACY-001](#worm-legacy-001): Legacy resolved worm (`CLOSED`)
- [x] **`[HIGH]`** [#WORM-RECENT-002](#worm-recent-002): Recently resolved worm awaiting review (`CLOSED`)

## Detailed Issue Log

### <a id="worm-open-003"></a> 🔴 `[WORM-OPEN-003]` Open defect

- **UUID**: `d5b29ec1-4082-46a5-bdb8-bdc66841f9ff`
- **Status**: `OPEN`
- **Severity**: `LOW`
- **Category**: `FIRMWARE`
- **Created**: `2026-11-05 15:29:17 UTC`

---

### <a id="worm-legacy-001"></a> 🟢 `[WORM-LEGACY-001]` Legacy resolved worm

- **UUID**: `795eb7bb-4680-4399-b36a-43d30560d73a`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-09-01 10:00:00 UTC`
- **Resolved**: `2026-09-01 12:00:00 UTC`
- **Accepted**: `2026-09-01 12:00:00 UTC`

---

### <a id="worm-recent-002"></a> 🟢 `[WORM-RECENT-002]` Recently resolved worm awaiting review

- **UUID**: `c66d74de-b949-46a6-acb2-b9c88cffd932`
- **Status**: `CLOSED`
- **Severity**: `HIGH`
- **Category**: `UI`
- **Created**: `2026-11-05 14:15:00 UTC`
- **Resolved**: `2026-11-05 14:20:00 UTC`
- **Accepted**: `2026-11-05 14:20:00 UTC`

---
