# 🟢 `[WORM-EARTHEN-NYMPH-383]` SensorController: Periodic read failed.

- **UUID**: `a177e2b9-941f-4bd8-85ad-bc66c2789e13`
- **ID**: `WORM-EARTHEN-NYMPH-383`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-10-15 20:53:09 UTC`
- **Resolved**: `2026-10-15 21:20:04 UTC`

#### Description

I see a warning message from SEnsorController when I run `GpioStatus`

#### Reproduction Steps

1. 1. flash shell app
2. 2. run `gpio status`

#### Execution / Console Logs

```text
gpio status
g372.454013 INFO [controller::shell_controller::log_command] received command "Gpio { subcommand: Some(Status), pin: None }"
372.454165 INFO [controller::shell_controller::log_success] command execution succeeded
GPIO Pin Status (0..29):
Pin   Level
-----------
GP0   Low  (0)
GP1   Low  (0)
GP2   Low  (0)
GP3   Low  (0)
GP4   Hih (1)
GP5   High (1)
GP6   High (1)
GP7   Low  (0)
GP8   Low  (0)
GP9   High (1)
GP10  High (1)
GP11  Low  (0)
GP12  Low  (0)
GP13  High (1)
GP14  High (1)
GP15  High (1)
GP16  High (1)
GP17  Low  (0)
GP18  Low  (0)
GP19  Low  (0)
GP20  High (1)
GP21  Low  (0)
GP22  Low  (0)
GP23  Low  (0)
GP24  Low  (0)
GP25  Low  (0)
GP26  High (1)
GP27  Low  (0)
GP28  High (1)
GP29  High (1)

shell> 372.458954 WARN [controller::sensor_controller::{impl#11}::run::{async_fn#0}] SensorController: Periodic read failed.
372.509120 WARN [controller::sensor_controller::{impl#11}::run::{async_fn#0}] SensorController: Periodic read failed.
```

#### Resolution Notes

Replaced destructive pin stealing in handle_gpio_cli with direct non-invasive rp_pac::SIO.gpio_in read, preserving FUNCSEL and active I2C peripheral operation.
