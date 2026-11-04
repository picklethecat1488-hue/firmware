# 🟢 `[WORM-GLIDING-BRISTLE-252]` Update TMC6300 to 6-PWM control and drop SENSE pin support

- **UUID**: `7e4bd5e7-82e8-44b4-bce7-6b05b7cfbc56`
- **ID**: `WORM-GLIDING-BRISTLE-252`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `FIRMWARE`
- **Component**: `tmc6300`
- **Created**: `2026-10-15 20:53:40 UTC`
- **Resolved**: `2026-10-15 21:11:59 UTC`

#### Description

TMC6300 3-phase BLDC driver requires 6 independent PWM/GPIO inputs (UH, UL, VH, VL, WH, WL) for proper 3-phase commutation. Remove bridged inputs, adopt 6-pin control (Option A), implement 6-step trapezoidal commutation, and drop support for the SENSE pin in favor of the dedicated INA219 current monitor.

#### Resolution Notes

Updated TMC6300 driver to 6-PWM control across GP16-GP21 (UH, UL, VH, VL, WH, WL) plus DIAG on GP22. Dropped EN and SENSE pins from driver and BSP. Implemented 6-step trapezoidal commutation with software break-before-make and PhaseState (Float, High, Low). Added dedicated unit tests and regression assertions guarding against shoot-through and ensuring independent low-side conduction.
