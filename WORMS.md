# Worm Report Tracker: Firmware Engineering Worm Tracker

> Automated bug tracking, triage, and issue registry generated via Firmware Bug Report Engine.

## Tracker Overview

| Metric | Details |
| :--- | :--- |
| **Report Date** | `2026-11-04 14:44:27 UTC` |
| **Total Issues** | `1` |
| **Open Issues** | `0` |
| **Resolved / Closed** | `1 (100%)` |

## Issues by Severity

| Severity | Count | Meaning |
| :--- | :---: | :--- |
| **`[CRITICAL]`** | 0 | System crashes, build failures, blockages, or electrical shorts. |
| **`[HIGH]`** | 1 | Major functional defects, broken routing, DRC violations, or unphysical behavior. |
| **`[MEDIUM]`** | 0 | Silkscreen collisions, layout sub-optimality, or visual clipping. |
| **`[LOW]`** | 0 | Minor aesthetic imperfections or documentation notes. |

## Issues by Category

| Category | Count | Description |
| :--- | :---: | :--- |
| **`FIRMWARE`** | 0 | Core firmware logic, state machines, async tasks. |
| **`CONTROLLER`** | 1 | Domain controllers, PID loops, event dispatch. |
| **`DRIVER`** | 0 | Hardware peripheral drivers, embedded-hal. |
| **`PLATFORM`** | 0 | Chip support, PAC, HAL, clock/power management. |
| **`BOARD`** | 0 | Board support packages (BSP), pinmux, boards. |
| **`MODEL`** | 0 | Domain data models, configurations, state definitions. |
| **`SHELL`** | 0 | CLI interface, shell commands, debug console. |
| **`INFRASTRUCTURE`** | 0 | Build tooling, compilers, test runners, headless tools. |
| **`UI`** | 0 | Web dashboards, CLI viewers, review interfaces. |
| **`GENERAL`** | 0 | Unclassified or cross-cutting firmware issues. |

## Issue Checklist

- [x] **`[HIGH]`** [#WORM-CLI-001](#worm-cli-001): CLI Review Worm (`RESOLVED`)

## Detailed Issue Log

### <a id="worm-cli-001"></a> 🟢 `[WORM-CLI-001]` CLI Review Worm

- **UUID**: `2c0e3289-ad5d-46bd-b99a-b4c6b24494e8`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `CONTROLLER`
- **Created**: `2026-11-04 14:44:27 UTC`

#### Attached Commits

- `a1b2c3d`

#### Review Tasks & Functional Checklist

- [ ] Firmware Step | `echo test`

---
