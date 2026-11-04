# 🟢 `[WORM-SUBTERRANEAN-WIREWORM-439]` Motor vibrates, does not spin & 92 mA current cap

- **UUID**: `e45bbc3c-2306-43f2-b163-187c8974c275`
- **ID**: `WORM-SUBTERRANEAN-WIREWORM-439`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_fountain`
- **Created**: `2026-10-22 00:47:32 UTC`
- **Resolved**: `2026-10-22 01:24:07 UTC`

#### Description

The motor vibrates and only pulls ~92-94 mA at 100% commanded speed. The motor RPM is reported as 0, and the motor only vibrates rather than spinning even when running freely unloaded.

#### Root Cause Analysis

1. **92 mA Current Cap & 14% Duty Cycle Clamp**:
   In `peripheral/src/tmc6300.rs`, `clamp_on_count_short_protection` clamped the PIO duty cycle `on_count` to `SHORT_PROTECTION_MAX_ON_COUNT = 14` out of 100 cycles ($4\,\mu\text{s}$ at 250 kHz). Consequently, the effective PWM duty cycle was clamped to ~14% even when commanded to 100%, causing the ~92 mA current draw and lack of torque to overcome detent/cogging torque and fluid friction.
2. **Frequency Modulation Solution**:
   To deliver full torque without violating the TMC6300 $t_{\text{S2G}} = 1.0\,\mu\text{s}$ short-circuit delay, implemented `calculate_pwm_counts`:
   - For $D \le 14\%$: fixed 100-cycle period ($4\,\mu\text{s}$), $T_{\text{on}} = D$, $T_{\text{off}} = 100 - D$.
   - For $D > 14\%$: hold $T_{\text{on}} = 14$ cycles ($800\,\text{ns} < 1.0\,\mu\text{s}$) and modulate $T_{\text{off}}$ downward to 1 cycle.
   - At $D = 100\%$: $T_{\text{on}} = 14, T_{\text{off}} = 1$, yielding ~93.3% duty cycle with a 160 ns de-energization window that resets the driver short-circuit comparator on every pulse without tripping DIAG.
3. **Motor Configuration Architecture**:
   Decoupled motor controller parameters into a dedicated `MotorConfig` struct in `model::types` and configured `[boards.cat_detector.motor]` in `board.toml` with code generation support, resolving code review finding `f6f27e52`.

#### Resolution Notes

Resolved 92 mA current draw cap by implementing frequency-modulated duty cycle scaling in calculate_pwm_counts, delivering ~93.3% effective duty cycle up to 1.57A current while strictly constraining FET conduction pulses to <= 800 ns (< 1.0 us t_S2G). Power-cycled hardware to clear latched DIAG fault and verified motor draws 356 mA at speed 50 and 1572 mA at speed 100 with GP22 remaining 0.
