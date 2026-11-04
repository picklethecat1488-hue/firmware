# 🟢 `[WORM-CREEPING-WRIGGLER-223]` VL not asserting

- **UUID**: `f73dcd72-2efa-4e89-b6f4-2b39c9b1f9dd`
- **ID**: `WORM-CREEPING-WRIGGLER-223`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `FIRMWARE`
- **Component**: `cat_fountain`
- **Created**: `2026-10-17 23:41:41 UTC`
- **Resolved**: `2026-10-18 00:02:49 UTC`

#### Description

The PIO program is not asserting VL. This is causing an overcurrent condition which asserts the DIAG pin. See attached trace

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `document` | [digital.csv](attachments/WORM-CREEPING-WRIGGLER-223/digital.csv) | Uploaded document digital.csv |

#### Resolution Notes

Corrected TMC6300 pin mapping and 6-step trapezoidal commutation masks to match physical SparkFun TMC6300 J1 header pinout (UL, UH, VL, VH, WL, WH mapped to GP16..GP21). VL (bit 2 / GP18) now properly asserts in steps 0 and 5.
