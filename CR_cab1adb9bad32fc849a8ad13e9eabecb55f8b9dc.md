# Code Review Report: Code Review: firmware

> Automated code review log and findings generated via Quake Code Review Engine.

## Review Overview

| Metric | Details |
| :--- | :--- |
| **Review Date** | `2026-11-19 01:34:23 UTC` |
| **Revisions** | `cab1adb9bad32fc849a8ad13e9eabecb55f8b9dc` |
| **Overall Verdict** | **`IN_REVIEW`** |
| **Review Progress** | `0/11 files reviewed (0%)` |
| **Total Comments** | `1 findings` |

## Findings by Severity

| Severity | Count | Meaning |
| :--- | :---: | :--- |
| **[MUST FIX]** | 1 | Must be resolved before merge; bugs, defects, or safety regressions. |
| **[PROPOSAL]** | 0 | Architecture ideas, design proposals, or optional enhancements. |
| **[NIT]** | 0 | Minor formatting, naming, or cosmetic cleanups. |

## File-by-File Review Findings

### [`tools/host_debug/src/rsp.rs`](file:///Users/<username>/gh/firmware/tools/host_debug/src/rsp.rs) — ⏳ `PENDING`

#### **[MUST FIX]** [tools/host_debug/src/rsp.rs:L430](file:///Users/<username>/gh/firmware/tools/host_debug/src/rsp.rs#L430)
<!-- comment-uuid: fb959be1-889c-4c81-8fe3-1cae0a223cd2 -->
<!-- comment-commit: cab1adb9bad32fc849a8ad13e9eabecb55f8b9dc -->
<!-- comment-status: RESOLVED -->

```
CORTEX_M_MEMORY_MAP_XML,
```

> **Reviewer (Reviewer)**: this does not belong in the RSP domain; it belongs in the ARM domain. Can we generate this XML dynamically based off a template and update our docs?

> **Resolution**: Moved Cortex-M memory-map descriptors into tools/host_debug/src/arm.rs, rendered GDB <memory-map> XML dynamically via tools/host_debug/templates/memory_map.xml.jinja, and updated mcu_decoupling.md.

## Action Items Checklist

- [x] **[MUST FIX]** [`tools/host_debug/src/rsp.rs:L430`](file:///Users/<username>/gh/firmware/tools/host_debug/src/rsp.rs#L430): this does not belong in the RSP domain; it belongs in the ARM domain. Can we generate this XML dynamically based off a template and update our docs? <!-- uuid:fb959be1-889c-4c81-8fe3-1cae0a223cd2 -->
