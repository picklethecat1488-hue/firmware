# 🟢 `[WORM-SUBTERRANEAN-WIREWORM-439]` Motor vibrates, does not spin & 92 mA current cap

- **UUID**: `e45bbc3c-2306-43f2-b163-187c8974c275`
- **ID**: `WORM-SUBTERRANEAN-WIREWORM-439`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `GENERAL`
- **Component**: `cat_fountain`
- **Created**: `2026-10-22 00:47:32 UTC`
- **Resolved**: `2026-10-22 01:24:07 UTC`
- **Accepted**: `2026-10-22 01:24:07 UTC`

#### Description

The motor vibrates and only pulls ~92-94 mA at 100% commanded speed. The motor RPM is reported as 0, and the motor only vibrates rather than spinning even when running freely unloaded.

#### Resolution Notes

Resolved 92 mA current draw cap by implementing frequency-modulated duty cycle scaling in calculate_pwm_counts, delivering ~93.3% effective duty cycle up to 1.57A current while strictly constraining FET conduction pulses to <= 800 ns (< 1.0 us t_S2G). Power-cycled hardware to clear latched DIAG fault and verified motor draws 356 mA at speed 50 and 1572 mA at speed 100 with GP22 remaining 0.
