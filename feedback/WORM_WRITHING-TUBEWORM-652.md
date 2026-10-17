# 🟢 `[WORM-WRITHING-TUBEWORM-652]` Limit max TMC6300 current to 2A

- **UUID**: `fb11dc60-30a2-4880-989f-6de052beea64`
- **ID**: `WORM-WRITHING-TUBEWORM-652`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-10-15 23:43:21 UTC`
- **Resolved**: `2026-10-16 00:19:19 UTC`

#### Description

With a BetaFPV 1102 motor, at a full duty cycle, the current level will spike to 25A on the first commutation phase after 10ms. The method of toggling GPIOs we use appears too slow. We should switch to a PIO or PWM-based duty cycling program.

#### Reproduction Steps

1. 1. motor speed 100

#### Behavior Comparison

- **Expected**: DIAG light pops on
- **Actual**: motor speed should ramp up to a full duty cycle without tripping DIAG

#### Resolution Notes

Implemented RP2040 PIO-based 25 kHz PWM duty cycling state machine on PIO1 for GP16..GP21 with zero shoot-through, and clamped max current limits to 2000 mA.
