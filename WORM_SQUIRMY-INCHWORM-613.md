# 🟢 `[WORM-SQUIRMY-INCHWORM-613]` Transient INA219 current spike during 50% BLDC pump spin-up

- **UUID**: `363a63f3-02de-40d9-b0b4-dec4a62ba732`
- **ID**: `WORM-SQUIRMY-INCHWORM-613`
- **Status**: `CLOSED`
- **Severity**: `MEDIUM`
- **Category**: `CONTROLLER`
- **Component**: `target`
- **Created**: `2026-11-16 02:25:57 UTC`
- **Resolved**: `2026-11-19 16:02:03 UTC`

#### Description

Observed brief 184 mA startup transient on TMC6300 phase engagement prior to steady-state Hall lock.

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_shell --release
2. Execute command: python tools/host.py cli fs cat metadata
3. Execute command: python tools/host.py cli fs export-telemetry
4. Execute command: python tools/host.py cli fs crash-log
5. Execute command: python tools/host.py cli fs format
6. Execute command: python tools/host.py cli battery status
7. Execute command: python tools/host.py cli thermal status
8. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [target] [INFO] 5.900000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [target] [DEBUG] 7.250000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=101mm East=104mm West=98mm (state=Active)
[] [target] [INFO] 8.600000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [target] [DEBUG] 9.950000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=103mm East=102mm West=100mm (state=Active)
[] [target] [INFO] 11.300000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [target] [DEBUG] 12.650000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=102mm East=104mm West=99mm (state=Active)
[] [target] [INFO] 14.000000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [target] [DEBUG] 15.350000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=101mm East=102mm West=98mm (state=Active)
[] [target] [INFO] 16.700000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [target] [DEBUG] 18.050000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=103mm East=104mm West=100mm (state=Active)
[] [target] [INFO] 19.400000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [bringup] [INFO] Executing Step 1: Format Filesystem Partition [shell_command]
[] [bringup] [INFO] Rebooting target device for step 1...
[] [system] [INFO] Resetting target microcontroller via CMSIS-DAP SWD...
[] [system] [INFO] Target RP2040 reset vector complete; re-attaching RTT channels
[] [target] [INFO] 0.000120 INFO  [platform::rtt] Draining buffered defmt logs (SEGGER_RTT @ 0x2003F800)
[] [target] [INFO] 0.001480 INFO  [board::rp2040] Board::init complete — XOSC 12MHz -> PLL_SYS 125MHz, I2C0/I2C1 @ 400kHz
[] [target] [INFO] 0.004910 INFO  [platform::core_monitor] Core0 MPU stack guard armed (timeout=2000ms, warn=75%)
[] [target] [INFO] 0.008320 INFO  [app::cat_detector] Core1 executor bootstrapped (MotorController + SensorController)
[] [target] [INFO] 0.012640 INFO  [controller::filesystem] Mounted sequential-storage partition @ 0x101C0000 (256 KiB)
[] [bringup] [INFO] Pattern matched: Formatting sequential-storage partition @ 0x101C0000... Formatting successful!
[] [target] [DEBUG] 1.850000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=103mm East=104mm West=100mm (state=Active)
[] [target] [INFO] 3.200000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [bringup] [INFO] Step 1 marked PASSED (Sequential-storage formatted cleanly)
[] [target] [DEBUG] 4.550000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=102mm East=102mm West=99mm (state=Active)
[] [bringup] [INFO] Executing Step 2: Verify Fuel Gauge Communication [shell_command]
[] [target] [INFO] INFO  [controller::shell] Querying MAX17048 I2C0 @ 0x36
[] [bringup] [INFO] Pattern matched: Direct battery reading: 4128 mV, 94% state of charge
[] [target] [INFO] 5.900000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [target] [DEBUG] 7.250000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=101mm East=104mm West=98mm (state=Active)
[] [bringup] [INFO] Step 2 marked PASSED (MAX17048 4128mV / 94% SOC verified)
[] [target] [INFO] 8.600000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [bringup] [INFO] Executing Step 3: Verify Thermal Monitoring [shell_command]
[] [bringup] [INFO] Pattern matched: Direct thermal reading (ThermalController): 28.4 C
[] [target] [DEBUG] 9.950000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=103mm East=102mm West=100mm (state=Active)
[] [target] [INFO] 11.300000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [target] [DEBUG] 12.650000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=102mm East=104mm West=99mm (state=Active)
[] [bringup] [INFO] Step 3 marked PASSED (Board thermistor nominal (28.4 C))
[] [target] [INFO] 14.000000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [bringup] [INFO] Generated bringup verification report at: /Users/<username>/gh/firmware/bringup_report.md
[] [target] [DEBUG] 15.350000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=101mm East=102mm West=98mm (state=Active)
[] [target] [INFO] 16.700000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [target] [DEBUG] 18.050000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=103mm East=104mm West=100mm (state=Active)
[] [target] [INFO] 19.400000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [target] [DEBUG] 20.750000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=102mm East=102mm West=99mm (state=Active)
[] [target] [INFO] 22.100000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [target] [DEBUG] 23.450000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=101mm East=104mm West=98mm (state=Active)
[] [target] [INFO] 24.800000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
[] [target] [DEBUG] 26.150000 DEBUG [controller::sensor] VL53L0X ToF fusion: North=103mm East=102mm West=100mm (state=Active)
[] [target] [INFO] 27.500000 INFO  [controller::battery] MAX17048 VCELL=4128mV SOC=94% | Core0 stack=18% Core1 stack=22%
```

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `reference` | [target_metadata_WORM-SQUIRMY-INCHWORM-613.txt](attachments/WORM-SQUIRMY-INCHWORM-613/target_metadata_WORM-SQUIRMY-INCHWORM-613.txt) | Filesystem metadata retrieved via host_fs |
