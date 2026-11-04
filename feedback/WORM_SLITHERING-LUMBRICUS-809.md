# 🟢 `[WORM-SLITHERING-LUMBRICUS-809]` TMC6300-Short protection fault on startup

- **UUID**: `96bf370a-4dde-42d3-a00f-e6399fc7d737`
- **ID**: `WORM-SLITHERING-LUMBRICUS-809`
- **Status**: `RESOLVED`
- **Severity**: `CRITICAL`
- **Category**: `FIRMWARE`
- **Component**: `cat_fountain`
- **Created**: `2026-10-21 22:51:40 UTC`
- **Resolved**: `2026-10-21 23:16:18 UTC`

#### Description

The short protection circuit of the TMC6300 motor driver will prevent motor communication and assert the DIAG pin if it detects a short between any FET for more than 1 uS in order to prevent overheating. The winding resistance of the motor is 0.5 Ohm; so we must make sure to connect each winding for less than 1 uS (ideally 0.5us to 0.8us) in order to commutate the motor. We have previously (in another session) verified power is good with a battery connected during motor startup, so this was the next thing we needed to invesgitate and fix with the motor driver.

#### Reproduction Steps

1. 1. $ PROBE_RS_SPEED=4000 cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell —release
2. 2. motor speed 10

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `document` | [digital.csv](attachments/WORM-SLITHERING-LUMBRICUS-809/digital.csv) | Uploaded document digital.csv |
| `screenshot` | [screenshot_paste_1792623504516_fkls0.png](attachments/WORM-SLITHERING-LUMBRICUS-809/screenshot_paste_1792623504516_fkls0.png) | Pasted screenshot screenshot_paste_1792623504516_fkls0.png |

#### Resolution Notes

Configured TMC6300 PIO clock divider to 5 (25 MHz) and clamped active duty cycle on-count to 7..=14 cycles, ensuring commutation winding connection pulses strictly stay between 520 ns and 800 ns (< 1.0 us t_S2G short protection delay) across all motor speeds to eliminate short-circuit trips on startup.
