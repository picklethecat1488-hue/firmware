# Cat Fountain Firmware Architecture & Bringup Guide

This document references the firmware architecture and bringup resources for the **Cat Fountain** (also referred to as **Cat Detector**) system deployed on the Raspberry Pi Pico (RP2040).

---

## Architectural Documentation

The detailed architectural definitions and hardware integration specifications have been factored into dedicated architecture guides:

1. **[Cat Fountain (RP2040) Firmware Architecture Guide](../docs/cat_fountain_architecture.md)**:
   - Hardware block diagram and RP2040 silicon capabilities.
   - Embassy asynchronous executor task breakdown and control sequence.
   - Domain controllers: `SystemController` (3-sensor data fusion), `MotorController` (INA219 torque monitoring), `SensorController` (VL53L0X 2-point calibration), and `LedController`.
   - Hardware peripheral mapping and dynamically assigned I2C address space (`0x30`, `0x31`, `0x32`).
   - 2 MB QSPI NOR flash layout, `sequential-storage` filesystem, and MPU stack guard.

2. **[Unified Firmware Architecture Guide](../docs/firmware_architecture.md)**:
   - High-level architectural pillars: microcontroller decoupling, actor-based concurrency, and zero backward-compatibility shims.
   - Layered crate roles across `model`, `peripheral`, `controller`, `platform`, `board`, and `app`.

---

## Hardware Bringup & Verification

For interactive bringup steps, target shell command checklists, and YAML verification profiles:
- **[Cat Detector Bringup Checklist](cat_detector.md)**
- **[`app/cat_detector_bringup.yaml`](cat_detector_bringup.yaml)**
- Bringup script execution:
  ```bash
  conda run -n firmware-env python tools/helpers/bringup.py --config app/cat_detector_bringup.yaml
  ```
