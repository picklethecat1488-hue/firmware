# Worm Report Tracker: Firmware Engineering Worm Tracker

> Automated bug tracking, triage, and issue registry generated via Firmware Bug Report Engine.

## Tracker Overview

| Metric | Details |
| :--- | :--- |
| **Report Date** | `2026-11-05 15:25:31 UTC` |
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
- [x] **`[HIGH]`** [#WORM-RECENT-002](#worm-recent-002): Recently resolved worm awaiting review (`RESOLVED`)

## Detailed Issue Log

### <a id="worm-open-003"></a> 🔴 `[WORM-OPEN-003]` Open defect

- **UUID**: `67a74751-8375-4ea3-bae5-b086764d335b`
- **Status**: `OPEN`
- **Severity**: `LOW`
- **Category**: `FIRMWARE`

---

### <a id="worm-legacy-001"></a> 🟢 `[WORM-LEGACY-001]` Legacy resolved worm

- **UUID**: `26b46cde-b427-4342-89a1-156dc2ff8070`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-09-01 10:00:00 UTC`
- **Resolved**: `2026-09-01 12:00:00 UTC`
- **Accepted**: `2026-09-01 12:00:00 UTC`

---

### <a id="worm-recent-002"></a> 🟢 `[WORM-RECENT-002]` Recently resolved worm awaiting review

- **UUID**: `52ed71f2-27ec-4f4b-b8f3-58f38a877a52`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `UI`
- **Created**: `2026-11-05 14:15:00 UTC`
- **Resolved**: `2026-11-05 14:20:00 UTC`

---
