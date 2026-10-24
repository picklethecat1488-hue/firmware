# 🟢 `[WORM-NOCTURNAL-CATERPILLAR-808]` Motor rotor ticks back and forth instead of spinning

- **UUID**: `fab2b9fc-f963-47ef-8baf-8811c4dae6eb`
- **ID**: `WORM-NOCTURNAL-CATERPILLAR-808`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `cat_detector`
- **Created**: `2026-10-22 04:00:06 UTC`
- **Resolved**: `2026-10-23 00:33:29 UTC`

#### Description

When commanding motor speed 30, the rotor ticks back and forth rather than rotating smoothly in continuous forward motion.

#### Resolution Notes

Resolved motor rotor ticking back and forth by configuring 500us commutation tick interval. At speed 30, commutation rate is 667 Hz (1111 RPM), providing sufficient rotor momentum to cross 9N12P cogging detents smoothly in continuous forward rotation. Codified regression test in test_worm_nocturnal_caterpillar_808_motor_oscillation.py.
