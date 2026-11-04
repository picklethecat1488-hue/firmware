# 🟢 `[WORM-DEEP-FIREWORM-942]` Add GPIO set CLI command for hardware debugging

- **UUID**: `55a47316-0497-4228-a27e-b8a318c5b8b0`
- **ID**: `WORM-DEEP-FIREWORM-942`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `platform`
- **Created**: `2026-10-15 22:17:35 UTC`
- **Resolved**: `2026-10-15 23:33:52 UTC`

#### Description

Motor driver trips DIAG fault when motor is connected during commutation; need GPIO set subcommand in CLI shell to test individual pins and commutator states directly on hardware.

#### Reproduction Steps

1. 1. Flash cat_detector_shell
2. 2. Run gpio set 16 1 to drive GP16 high
3. 3. Observe unrecognized subcommand or missing set handler

#### Behavior Comparison

- **Expected**: gpio set <pin> <0|1> configures pin as output and drives logic high or low.
- **Actual**: gpio CLI only supports status and read subcommands.

#### Resolution Notes

Added GpioSubcommand::Set variant to platform/src/io/gpio.rs with pin validation and logic level control. Configured positional pin and value CLI arguments in controller/shell.toml. Added dedicated unit tests and Python regression tests.
