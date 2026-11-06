# Worm Report Tracker: Firmware Engineering Worm Tracker

> Automated bug tracking, triage, and issue registry generated via Firmware Bug Report Engine.

## Tracker Overview

| Metric | Details |
| :--- | :--- |
| **Report Date** | `2026-11-06 00:07:50 UTC` |
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

- **UUID**: `731149bb-d1e2-4930-9d05-a7cb29374722`
- **Status**: `OPEN`
- **Severity**: `LOW`
- **Category**: `FIRMWARE`

---

### <a id="worm-legacy-001"></a> 🟢 `[WORM-LEGACY-001]` Legacy resolved worm

- **UUID**: `0ee55c03-a046-4124-8ed9-e5a8c8477e7f`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-09-01 10:00:00 UTC`
- **Resolved**: `2026-09-01 12:00:00 UTC`
- **Accepted**: `2026-09-01 12:00:00 UTC`

---

### <a id="worm-recent-002"></a> 🟢 `[WORM-RECENT-002]` Recently resolved worm awaiting review

- **UUID**: `e42050f4-1d28-4a24-8d54-e48fd4819dda`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `UI`
- **Created**: `2026-11-05 14:15:00 UTC`
- **Resolved**: `2026-11-05 14:20:00 UTC`

---
