# 🟢 `[WORM-SLITHERING-EARTHWORM-854]` Bootloop-

- **UUID**: `919eccfe-37b7-4464-9733-fb87345157e2`
- **ID**: `WORM-SLITHERING-EARTHWORM-854`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-10-16 01:08:14 UTC`
- **Resolved**: `2026-10-16 01:12:32 UTC`

#### Description

See execution logs

#### Execution / Console Logs

```text
========================================================
🔥 TARGET DEVICE CRASH DETECTED 🔥
========================================================
UUID: bd2d32b6-bf9a-4788-8b75-e643bb6f09cc
Revision Hash: da4881c

--------------------------------------------------------
CPU Core: Core 0
Panicked: true
Registers:
  R0: 0x20024088
  R1: 0x0000000E
  R2: 0x20024088
  R3: 0x0000000E
  SP: 0x20041A68
  LR: 0x10020D5F
  PC: 0x100119C8

Backtrace:
  Raw PCs: [0, 1001195a, 1001193c, 10000204, 10019cee, 10019d24, 1001af9e, 10009f42, 1001aff2, 1001aff2, 1001aff2, 1001aff2, 1001aff2, 1001aff2, 1001afc0, 10008c44, 1001b028, 10008cfc, 10008d5a, 10011338, 100001e6]
  0x1001AF9E - __defmt_default_panic (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/defmt-1.1.1/src/lib.rs:385)
  0x10009F42 - cat_detector_shell::____embassy_main_task::{{closure}} (/Users/<username>/gh/firmware/app/src/cat_detector_shell.rs:0)
  0x1001AFF2 - _critical_section_1_0_release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:305)
  0x1001AFF2 - critical_section::release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:219)
  0x1001AFF2 - critical_section::with (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:249)
  0x1001AFF2 - _critical_section_1_0_release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:305)
  0x1001AFF2 - critical_section::release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:219)
  0x1001AFF2 - critical_section::with (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:249)
  0x1001AFF2 - _critical_section_1_0_release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:305)
  0x1001AFF2 - critical_section::release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:219)
  0x1001AFF2 - critical_section::with (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:249)
  0x1001AFF2 - _critical_section_1_0_release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:305)
  0x1001AFF2 - critical_section::release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:219)
  0x1001AFF2 - critical_section::with (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:249)
  0x1001AFF2 - _critical_section_1_0_release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:305)
  0x1001AFF2 - critical_section::release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:219)
  0x1001AFF2 - critical_section::with (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:249)
  0x1001AFF2 - _critical_section_1_0_release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:305)
  0x1001AFF2 - critical_section::release (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:219)
  0x1001AFF2 - critical_section::with (/Users/<username>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/critical-section-1.2.0/src/lib.rs:249)
  0x10008D5A - cat_detector_shell::__cortex_m_rt_main (/Users/<username>/gh/firmware/app/src/cat_detector_shell.rs:53)
  0x10011338 - main (/Users/<username>/gh/firmware/app/src/cat_detector_shell.rs:53)
  0x100001E6 - (no symbol found)

--------------------------------------------------------
CPU Core: Core 1
Panicked: false
Registers:
  R0: 0x00000000
  R1: 0x00000000
  R2: 0x00000000
  R3: 0x00000000
  SP: 0x00000000
  LR: 0x00000000
  PC: 0x00000000

Backtrace:
  (No backtrace frames captured. Check compiler optimization or stack pointer alignment)

Crash Context System Logs:
  0.000328 INFO MPU stack guard configured at 0x2003c000
  0.019401 ERROR panicked at 'assertion failed: pins.len() <= 5'
  0.019524 ERROR --- PANIC DETECTED ---
  0.019562 ERROR Core 0: SP=20041a68 PC=100119c8 LR=10020d5f
  0.019641 ERROR   Message: explicit panic
========================================================
```

#### Resolution Notes

Removed erroneous cfg.set_set_pins call on the 6 PWM pins in Tmc6300Pio::new. RP2040 PIO SET instructions only support up to 5 pins (assertion failed: pins.len() <= 5), and the driver only uses OUT and MOV PINS, NULL instructions.
