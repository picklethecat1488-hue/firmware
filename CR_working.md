# Code Review Report: Code Review: firmware

> Automated code review log and findings generated via Quake Code Review Engine.

## Review Overview

| Metric | Details |
| :--- | :--- |
| **Review Date** | `2026-11-22 02:54:26 UTC` |
| **Revisions** | `working` |
| **Overall Verdict** | **`CHANGES_REQUESTED`** |
| **Review Progress** | `0/5 files reviewed (0%)` |
| **Total Comments** | `2 findings` |

## Findings by Severity

| Severity | Count | Meaning |
| :--- | :---: | :--- |
| **[MUST FIX]** | 2 | Must be resolved before merge; bugs, defects, or safety regressions. |
| **[PROPOSAL]** | 0 | Architecture ideas, design proposals, or optional enhancements. |
| **[NIT]** | 0 | Minor formatting, naming, or cosmetic cleanups. |

## File-by-File Review Findings

### [`controller/shell.toml`](file:///Users/<username>/gh/firmware/controller/shell.toml) — ⏳ `PENDING`

#### **[MUST FIX]** [controller/shell.toml:L144](file:///Users/<username>/gh/firmware/controller/shell.toml#L144)
<!-- comment-uuid: 00fcfe1e-7edf-4833-97e3-745daf2cc749 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```
{ name = "bus", type = "string", help = "Target bus name" }
```

> **Reviewer (Reviewer)**: we should pass this by flag

> **Resolution**: Declared explicit -b/--bus and --reg16 flags alongside positional <addr> <reg> [<val>] in controller/shell.toml.

#### **[MUST FIX]** [controller/shell.toml:L142](file:///Users/<username>/gh/firmware/controller/shell.toml#L142)
<!-- comment-uuid: 62346140-4196-4f9d-a0ac-a87162ea6fec -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```
{ name = "arg2", type = "string", max_len = 16, help = "Register offset (hex 0x.. or decimal)" },
```

> **Reviewer (Reviewer)**: can we forward fix this to support 16 bit addresses as well?

> **Resolution**: Added --reg16 flag and RegisterAddress::{Reg8, Reg16} supporting 2-byte big-endian register addresses alongside 8-bit offsets.

## Action Items Checklist

- [x] **[MUST FIX]** [`controller/shell.toml:L144`](file:///Users/<username>/gh/firmware/controller/shell.toml#L144): we should pass this by flag <!-- uuid:00fcfe1e-7edf-4833-97e3-745daf2cc749 -->
- [x] **[MUST FIX]** [`controller/shell.toml:L142`](file:///Users/<username>/gh/firmware/controller/shell.toml#L142): can we forward fix this to support 16 bit addresses as well? <!-- uuid:62346140-4196-4f9d-a0ac-a87162ea6fec -->
