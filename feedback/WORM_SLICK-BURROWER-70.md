# 🟢 `[WORM-SLICK-BURROWER-70]` We should log when DIAG pin gets asserted.

- **UUID**: `9f820909-e867-4252-83b9-92fde8e4a3c7`
- **ID**: `WORM-SLICK-BURROWER-70`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_fountain`
- **Created**: `2026-10-16 01:23:33 UTC`
- **Resolved**: `2026-10-16 01:36:22 UTC`

#### Description

We should log when the DIAG pin gets asserted in the TMC6300 driver tick call.

#### Resolution Notes

Instrumented TMC6300 driver tick calls (standard GPIO and PIO drivers) with structured error logging when DIAG pin is asserted high.
