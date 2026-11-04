# 🟢 `[WORM-GROUNDED-PLANARIAN-948]` GP26 is logic high- measuing .29V on multimeter

- **UUID**: `b35796e6-ef70-447d-8c21-e66a767d203f`
- **ID**: `WORM-GROUNDED-PLANARIAN-948`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-10-15 21:00:30 UTC`
- **Resolved**: `2026-10-15 21:28:14 UTC`

#### Description

GP26  High (1)

#### Reproduction Steps

1. 1. flash shell firmware
2. 2. run gpio status command

#### Resolution Notes

Configured early Pull::Up on SPEED_SENSE_PIN (GP26) in Board::init alongside non-destructive SIO register reads in handle_gpio_cli, preventing open-drain Hall sensor line from floating or dropping to 0.29V.
