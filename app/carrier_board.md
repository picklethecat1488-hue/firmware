# Carrier Board 2.0 Firmware Architecture & Bringup Guide

This document specifies the firmware design, dual-core task execution model, memory partitions, and hardware bringup procedures for the **Carrier Board 2.0** architecture powered by the **NXP MCX N947** dual-core Arm Cortex-M33 microcontroller.

The source of truth for bringup verification steps is [`app/carrier_board_bringup.yaml`](file:///Users/daparker/gh/firmware/app/carrier_board_bringup.yaml).

---

## 1. System Overview & Silicon Architecture

The Carrier Board 2.0 is a modular hardware evaluation, sensor fusion, and telemetry workstation designed for ultra-low-power operation, long-range wireless communication, and real-time proximity sensing. The firmware executes in a bare-metal `no_std` Rust environment built on top of the **Embassy** asynchronous framework.

```
+-----------------------------------------------------------------------------------+
|                            CARRIER BOARD 2.0 FIRMWARE                             |
+-----------------------------------------------------------------------------------+
|  Core 0 (150 MHz Cortex-M33): System Orchestration, Storage, Network & Services   |
|  - Embassy Async Executor (Cooperative Multitasking)                              |
|  - FlexSPI NAND Flash Storage Controller (Winbond W25N01GV 1Gb NAND Filesystem)   |
|  - Bidirectional Wireless BLE Service Stack (u-blox NINA-B312 UART @ 1 Mb/s)       |
|    * Telemetry Egress Streaming                                                   |
|    * BLE GATT Service Endpoint for Mobile/Host Client RPC & Configuration         |
|  - Host FTDI Console & High-Speed UART Service Channel (FC1 UART0 @ 115.2k - 1Mb/s)|
|  - Power Path & Battery State of Charge Manager (BQ24074 & MAX17048)              |
|  - Status & Ambient LED Animations (TI LP5009 Logarithmic RGB Driver)             |
|  - Background OTA Firmware Staging & Secure Boot Verification Pipeline            |
+-----------------------------------------------------------------------------------+
|  Core 1 (150 MHz Cortex-M33): Dedicated Real-Time Peripheral Coprocessor          |
|  - Deterministic Real-Time Task Runner                                            |
|  - High-Speed ProxFusion Capacitive Touch & Proximity Engine (Azoteq IQS7222A)    |
|  - PDM Class-D Audio Stream & Frequency Synthesis (MAX98357A & Piezo Sounder)     |
|  - Camera Gesture Pipeline & Image Preprocessing Interface                        |
|  - PowerQuad Math Accelerator / DSP Filter Pipelines (CMSIS-DSP)                  |
|  - eIQ Neutron NPU Neural Inference Engine (Accelerated ML Graph Execution)       |
+-----------------------------------------------------------------------------------+
```

### Key Silicon Components (Bill of Materials)

| Designator | Component / Part | Description | Interface / Bus | Key Firmware Role |
| :--- | :--- | :--- | :--- | :--- |
| **`U1`** | **NXP MCXN947VDF** | Dual-core Arm Cortex-M33 MCU + eIQ NPU | Host Controller | 150 MHz dual-core, 2MB dual-bank Flash, 512KB SRAM with ECC. |
| **`U8`** | **Winbond W25N01GV** | 1Gb (128MB) SLC Serial NAND Flash | FlexSPI Port A (Core 0)| Persistent CBOR telemetry queue, crash dump logs, calibration data, OTA staging. |
| **`U3`** | **TI BQ24074** | 1.5A Dynamic Power Path Li-Ion Charger | GPIO (`/CHG`, `/PGOOD`) | Autonomous charge management, input current limit, brownout avoidance. |
| **`U7`** | **ADI MAX17048** | 1-Cell Li+ ModelGauge Fuel Gauge | Core `I2C0` (`0x36`) | Precision voltage, state of charge (SoC), alert interrupt (`G4`). |
| **`U6`** | **TI LP5009** | 9-Channel Logarithmic RGB LED Driver | Core `I2C0` (`0x14`) | Low-power breathing, charge animations, offloading MCU core. |
| **`U9`** | **FTDI FT232RNQ** | High-Speed USB 2.0 to UART Serial Bridge | `FC1` UART0 (115.2k - 1 Mb/s)| Diagnostic bringup shell, 1Mb/s telemetry, UART service endpoint. |
| **`U2`** | **Azoteq IQS7222A**| ProxFusion Cap-Touch & Proximity IC | Touch `I2C1` (`0x44`)| 4-electrode sensing (3 mutual, 1 self-cap), `CAP_INT` (`C4`) on Core 1. |
| **`U4`** | **ADI MAX98357A** | 3.2W Class-D Mono Audio Amplifier | `PDM0` Audio (`B14`/`A14`)| Chime audio feedback, filterless Class-D drive on Core 1. |
| **`U11`**| **u-blox NINA-B312**| Bluetooth Low Energy 5.0 Module | UART1 (1 Mb/s) | Bidirectional BLE: Telemetry streaming & GATT service endpoint. |
| **`Q2`–`Q4`**| **TI TPS22918**| 5.5V, 2A Load Switches | GPIO (`L4`, `L5`, `M4`)| Power-gating Audio (`Q2`), Sensors (`Q3`), and Debug Bridge (`Q4`). |

---

## 2. Memory Map & Storage Layout

### Internal Microcontroller Memory (NXP MCX N947)

The NXP MCX N947 integrates **2,048 KB (2 MB) of dual-bank on-chip flash** and **512 KB of SRAM with ECC**.

```
0x0000_0000 +---------------------------------------+
            |  Primary Bootloader & Vector Table    |
            |  (64 KB Protected Boot Sector)        |
0x0001_0000 +---------------------------------------+
            |  Active Firmware Application Slot A   |
            |  (1,920 KB Single-Slot Partition)     |
            |  Supports Dual Cores, DSP, eIQ & Exp. |
0x001F_0000 +---------------------------------------+
            |  Non-Volatile System Keystore & Config|
            |  (64 KB Protected Secure Store)       |
0x0020_0000 +---------------------------------------+
```

### Detailed Flash Cost Breakdown (`.text` and `.data`)

To prove conclusively that the application fits comfortably within the internal flash budget while supporting both Cortex-M33 cores, DSP kernels, eIQ Neutron NPU inference, built-in peripherals, bidirectional BLE communication, and future expansion scenarios (audio playback, camera gesture detection, location services), the following flash sizing analysis is established:

| Subsystem / Functional Module | Estimated `.text` (Code) | Estimated `.data` / `.rodata` | Total Flash Footprint | Description / Scope |
| :--- | :---: | :---: | :---: | :--- |
| **Core 0 Application & Embassy Executor** | 210 KB | 40 KB | **250 KB** | Cooperative async task runner, system state machine, timer scheduler, channel routing. |
| **Core 1 Coprocessor Runtime** | 150 KB | 30 KB | **180 KB** | Coprocessor bootstrap, real-time sensor loop, Azoteq IQS7222A touch sampling engine. |
| **DSP & PowerQuad Math Kernels** | 75 KB | 15 KB | **90 KB** | CMSIS-DSP FFT, biquad IIR/FIR filter cascades, frequency synthesis algorithms. |
| **eIQ Neutron NPU Runtime & Driver** | 100 KB | 20 KB | **120 KB** | NPU command stream builder, operator graph dispatcher, weight decompression loader. |
| **eIQ Quantized Model Weights (Internal)** | 0 KB | 280 KB | **280 KB** | 8-bit quantized gesture classification & keyword spotting weights in `.rodata`. |
| **Built-in Peripherals & Bus Drivers** | 75 KB | 15 KB | **90 KB** | FlexSPI NAND (Core 0), LPI2C0/1, I3C, LPUART0/1, PDM audio, WUU/SPC drivers. |
| **Bidirectional BLE Stack (NINA-B312)** | 55 KB | 15 KB | **70 KB** | Telemetry egress framing, GATT service endpoint dispatcher, client RPC handlers. |
| **Expansion: Audio Playback Engine** | 38 KB | 7 KB | **45 KB** | WAV / ADPCM streaming decoder, ring buffer feeder to PDM Class-D output. |
| **Expansion: Camera Gesture Pipeline** | 115 KB | 25 KB | **140 KB** | 2D optical flow filter, image patch normalization, edge trigger detection. |
| **Expansion: Location Services Engine** | 45 KB | 10 KB | **55 KB** | NMEA-0183 / UBX sentence parsing, Kalman dead-reckoning filter, geodesic solver. |
| **Diagnostic Shell, RTT & Formatting** | 30 KB | 10 KB | **40 KB** | Command parser, bringup test routines, ASCII banner, RTT formatting. |
| **Total Estimated Footprint** | **893 KB** | **467 KB** | **1,360 KB** | **70.8% of 1,920 KB partition (560 KB headroom remaining).** |

### Firmware Partitioning & Flash Filesystems (`sequential-storage`)

A strict architectural invariant of the Carrier Board 2.0 firmware is that **every firmware flash I/O operation outside of the primary ROM/bootloader stage MUST be governed by the `sequential-storage` crate**. Raw, unbounded, or ad-hoc flash sector writes are prohibited.

#### 1. Internal Flash Partitions (2 MB NXP MCX N947)

| Partition Name | Memory Address Range | Size | Filesystem / Storage Driver | Description & Stored Data |
| :--- | :--- | :---: | :--- | :--- |
| **`bootloader`** | `0x0000_0000` – `0x0001_0000` | 64 KB | Read-Only Bare-Metal Code | Custom Second-Stage Bootloader (SSBL): Initial startup vector table, XIP pre-boot evaluation, and SRAM-relocatable flash programmer kernel (`flash_loader_ram`) for OTA staging. |
| **`slot_a`** | `0x0001_0000` – `0x001E_0000` | 1,856 KB | Read-Only Application Code | Primary active dual-core application firmware image (Core 0 + Core 1). |
| **`fs`** | `0x001E_0000` – `0x001F_0000` | 64 KB | `sequential_storage::map` | Internal non-volatile key-value store: sensor calibration baselines, device UUID, boot counters. |
| **`keystore`** | `0x001F_0000` – `0x0020_0000` | 64 KB | Hardware Protected Keystore | Cryptographic public keys, anti-rollback monotonic counters, security credentials. |

#### 2. External Serial SLC NAND Partitions (128 MB Winbond W25N01GV)

| Partition Name | NAND Address Range | Size | Filesystem / Storage Driver | Description & Stored Data |
| :--- | :--- | :---: | :--- | :--- |
| **`telemetry`** | `0x0000_0000` – `0x0200_0000` | 32 MB | `sequential_storage::queue` | High-throughput FIFO circular telemetry queue storing CBOR-encoded event frames with wear leveling. |
| **`crash_logs`** | `0x0200_0000` – `0x0400_0000` | 32 MB | `sequential_storage::map` | Diagnostic crash dump storage: CPU register snapshots, core task callstacks, and panic assertions. |
| **`ota_staging`** | `0x0400_0000` – `0x0600_0000` | 32 MB | `sequential_storage::queue` | Staging area for incoming firmware update chunks received over BLE or USB UART prior to verification. |
| **`recovery`** | `0x0600_0000` – `0x0700_0000` | 16 MB | Read-Only Recovery Image | Golden factory fallback firmware image restored if Slot A boot verification fails. |
| **`models`** | `0x0700_0000` – `0x0800_0000` | 16 MB | `sequential_storage::map` | External neural network weights (eIQ Neutron), audio chime samples, and gesture templates. |

#### 3. Flash I/O Invariants & Guarantees

- **Key-Value Records**: All configuration objects and calibration baselines utilize `sequential_storage::map::store_item`, `fetch_item`, and `remove_item` with `sequential_storage::cache::NoCache`.
- **Time-Series Telemetry**: High-rate telemetry events are appended via `sequential_storage::queue::push` and drained during BLE / USB upload bursts using `sequential_storage::queue::iter` and `pop`.
- **Power-Loss Atomicity**: Incomplete or torn writes occurring during battery disconnect or brownout are cleanly discarded during the subsequent boot traversal without corrupting existing records.
- **Wear Leveling**: Flash write operations are distributed evenly across SLC NAND erase blocks, preventing premature block degradation.

### Application Software Framework for DSP & NPU (Rust ML Ecosystem)

To drive the eIQ Neutron NPU and PowerQuad DSP accelerator from Rust without relying on proprietary, opaque C runtimes, Carrier Board 2.0 targets modern high-level Rust tensor and machine learning frameworks:

1. **Burn (`burn-rs`)**:
   - Modern, flexible deep learning framework for Rust with a dynamic/static computation graph, automatic differentiation, and modular backends.
   - Supports `no_std` environments and features an ahead-of-time (AOT) code generation engine (`burn-import`) that compiles PyTorch/ONNX models directly into strongly-typed, memory-safe Rust structs with zero runtime dynamic allocations.
2. **Candle (Hugging Face)**:
   - Minimalist, performance-oriented ML framework for Rust. Offers a PyTorch-compatible API, lightweight tensor primitives, and native support for quantized GGUF and ONNX models.
3. **tract (Sonos)**:
   - Pure-Rust neural network inference engine supporting ONNX and TensorFlow models. Extensively proven in resource-constrained embedded devices, tract features static graph optimization, constant folding, and direct emission of Arm CMSIS-DSP kernel calls.
4. **JAX-like Functional AOT / JIT Compilation Model for Embedded Rust**:
   - The production software architecture adopts a **JAX-inspired functional compilation workflow**:
     - *Offline Model Definition & Training*: Networks are defined and trained on host workstations in PyTorch or JAX.
     - *Graph Lowering & Optimization*: The model is lowered to an intermediate representation (ONNX / StableHLO) where functional transformations (fused convolutions, quantization to INT8, activation pruning) are performed.
     - *Kernel Dispatch to MCX N947 Hardware*: The Rust code generation pipeline maps tensor operations directly to hardware acceleration blocks:
       - 2D Convolutions & Dense Matrix Multiplications $\to$ **eIQ Neutron NPU**.
       - FFTs, Biquad IIR/FIR Filters, Trigonometric & Coordinate Transformations $\to$ **PowerQuad DSP Accelerator**.
       - Element-wise operations and tensor reshapes $\to$ Zero-copy slice operations in internal SRAM.

### Design Hardening & Memory Protection (ARMv8-M MPU & Storage Security)

Security and execution integrity are enforced across both hardware and firmware layers:

1. **ARMv8-M Memory Protection Unit (MPU)**:
   - Both Cortex-M33 cores incorporate a hardware MPU supporting up to 16 configurable regions per core.
   - *Privilege Separation*: The Embassy async executor runs in privileged Handler mode, while user tasks and peripheral service handlers execute in unprivileged Thread mode.
   - *Execute Never (`XN`)*: All SRAM regions (including shared SRAMX and tensor arenas) are marked strictly `XN` (Execute Never) to prevent code injection attacks.
   - *Stack Guard Regions*: A 1 KB unmapped guard page is positioned directly beneath each core's execution stack (`0x2006_C000`). Any stack overflow immediately trips a hardware `MemManage` fault rather than corrupting adjacent memory.
2. **Internal Flash Protection**:
   - The MCX N947 Flash Access Control (FAC) registers and Code Security registers configure execute-only regions and lock out unauthorized SWD readback in production lifecycle states (OEM Closed / Production Secure).
3. **External NAND Storage Hardening (Winbond W25N01GV)**:
   - *Block Locking*: Hardware Block Lock registers (`SR-1` BP0..BP3) are asserted to write-protect critical firmware staging and golden recovery blocks against accidental or malicious overwrite.
   - *Hardware Write Protect (`/WP`)*: The physical `/WP` pin is held low during normal operation, only unasserted by the bootloader during authorized OTA update staging.
   - *Cryptographic Envelope Encryption*: Sensitive telemetry data, crash dumps, and customer credentials stored in external NAND are encrypted using the on-chip hardware AES-128/256 engine. Encryption keys are securely stored in the internal hardware Keystore (`0x001F_0000`) and are never exposed off-chip.

### Secure Boot & Firmware Update Architecture

Firmware updates are governed by a cryptographic Root of Trust (RoT):

1. **Hardware Root of Trust**:
   - The MCX N947 ROM bootloader verifies the initial boot sector using OEM public key hashes permanently programmed into on-chip eFuse (CMPA - Customer Manufacturing Programmable Area).
2. **Ed25519 Digital Signatures**:
   - Every candidate firmware update payload is signed with an offline private key. The on-device bootloader validates the Ed25519 signature and SHA-256 hash before committing any image to Flash Slot A.
3. **Anti-Rollback Protection**:
   - A monotonic anti-rollback counter is tracked in eFuse. The bootloader rejects any signed update payload bearing a security version lower than the current hardware counter.
4. **Custom Second-Stage Bootloader (SSBL) & Dual-Mode OTA Execution**:
   - The firmware architecture incorporates a **customizable Second-Stage Bootloader (SSBL)** residing in the dedicated 64 KB partition (`0x0000_0000`–`0x0001_0000`), authenticated by the ROM RoT.
   - To support flexible OTA update workflows, the SSBL provides two distinct operational execution modes:
     - **Pre-Application XIP Execution**: Prior to launching the primary application, the SSBL boots directly from internal flash in Execute-in-Place (XIP) mode. It inspects boot flags in `keystore`/`fs`, checks the integrity of the external Winbond W25N01GV NAND `ota_staging` partition, and verifies whether a valid staged update image or rollback request is pending. If no update is requested, it directly hands off control to the Slot A application vector table (`0x0001_0000`).
     - **SRAM-Resident Flashing Kernel (`flash_loader_ram`)**: When an OTA commit is initiated (either detected by the SSBL at boot or kicked off by the active application upon completing chunk download over BLE/USB), internal flash Slot A cannot be safely erased and rewritten while actively executing code from the same physical flash controller. The SSBL or application relocates a self-contained, position-independent flash programming kernel (`flash_loader_ram`, ~8 KB) into the Reserved SRAM buffer (`0x2007_C000`).
     - *MPU Reconfiguration*: The privileged supervisor temporarily updates the MPU configuration for the `0x2007_C000` region from `Execute-Never (XN)` to `Privileged Execution (RX)` with interrupts disabled (`CPSID I`).
     - *NAND-to-Flash Streaming*: Running entirely out of SRAM, `flash_loader_ram` streams the verified binary blocks from the NAND `ota_staging` partition over FlexSPI DMA, erases Slot A sectors, programs the internal flash, calculates the hardware CRC32 to guarantee image integrity, updates the monotonic anti-rollback counter, and executes an atomic system reset (`NVIC_SystemReset()`).
5. **Single-Slot NAND Staging Workflow**:
   ```mermaid
   flowchart TD
       A["Incoming OTA Payload via BLE/USB"] --> B["Stage in 32MB NAND Partition (0x0400_0000)"]
       B --> C["Validate Ed25519 Signature & SHA-256 Hash"]
       C --> D{"Signature Valid & Version >= Rollback Counter?"}
       D -- No --> E["Reject Update & Erase Staging Partition"]
       D -- Yes --> F["Reboot or Enter OTA Mode"]
       F --> G{"Execution Mode Selection"}
       G -- XIP Boot --> H["Custom SSBL Evaluates Pending OTA in XIP Flash"]
       G -- In-App Trigger --> I["Relocate flash_loader_ram Kernel into SRAM (0x2007_C000)"]
       H --> I
       I --> J["Program Slot A from NAND via FlexSPI DMA while Running in SRAM"]
       J --> K["Verify Slot A Flash CRC32 & Update Monotonic Counter"]
       K --> L["Trigger NVIC_SystemReset() -> Boot Verified Slot A"]
   ```

### Internal SRAM Allocation Budget (Including RTT & CLI Buffers)

The MCX N947 features **512 KB of total internal SRAM** with ECC protection:

| SRAM Domain | Base Address | Size | Primary Function / Memory Consumer |
| :--- | :--- | :---: | :--- |
| **Core 0 System Heapless Arena** | `0x2000_0000` | 128 KB | Embassy executor task arena, networking buffers, BLE packet queues. |
| **Segger RTT & defmt Logging Buffers**| `0x2002_0000` | 24 KB | RTT control block (`_SEGGER_RTT`), Up/Down channel ring buffers, defmt queue. |
| **CLI & Interactive Console Buffers** | `0x2002_6000` | 8 KB | Command history ring buffer, tokenizer scratchpad, VT100 terminal escape line buffers. |
| **Core 1 Sensor Fusion Arena** | `0x2002_8000` | 80 KB | ProxFusion high-rate sample buffers, touch filter states. |
| **Neutron NPU Tensor Arena** | `0x2003_C000` | 128 KB | Intermediate neural activation maps, feature tensor scratchpad. |
| **DSP & Audio Circular Buffers** | `0x2005_C000` | 48 KB | PDM audio double-buffers, 512-point FFT scratchpad. |
| **Inter-Core Shared IPC (SRAMX)**| `0x2006_8000` | 32 KB | Lock-free SPSC circular ring buffers and hardware mailbox registers. |
| **Stacks & Hardware Guard Pages** | `0x2007_0000` | 48 KB | Core 0 stack (24 KB), Core 1 stack (16 KB), MPU guard pages (8 KB). |
| **Reserved / DMA Bounce Buffers** | `0x2007_C000` | 16 KB | Transient eDMA scatter-gather descriptors, USB packet staging, and SRAM-relocated OTA flash loader kernel (`flash_loader_ram`). |
| **Total SRAM Allocation** | | **512 KB** | **100% mapped, zero uncontrolled heap allocation.** |

### Dual-Core Rust Architecture: Embassy Multi-Executor AMP (Option 3)

The architecture establishes **Embassy Multi-Executor Asymmetric Multiprocessing (AMP)**:

- **Core 0**: Runs the primary Embassy asynchronous executor. Handles system timers, power rail scheduling, FlexSPI NAND filesystem access, battery fuel gauge monitoring, and bidirectional BLE communication.
- **Core 1**: Dedicated real-time peripheral coprocessor running a deterministic Embassy executor. Drives Azoteq IQS7222A touch capture, PDM Class-D audio streaming, and camera gesture preprocessing.
- **Inter-Core IPC Mechanism**:
  - Zero-copy lock-free Single-Producer Single-Consumer (SPSC) ring buffers reside in shared SRAMX (`0x2006_8000`).
  - Signaling is mediated by the MCX N947 hardware **Messaging Unit (MU)**. When Core 0 enqueues an audio playback request or ML command, it triggers an MU interrupt to wake Core 1 instantly without polling.

---

## 3. Communication, Telemetry & Logging Strategy

Carrier Board 2.0 maintains a dual-channel strategy tailored for development vs. production deployment:

```
+-----------------------------------------------------------------------------------+
|                     CARRIER BOARD 2.0 COMMUNICATION CHANNELS                      |
+-----------------------------------------------------------------------------------+
|  DEVELOPMENT & BENCH CHANNEL: Segger RTT (over SWD Probe)                         |
|  - Required for Perfetto multi-core timeline tracing (multi-MB/s throughput).     |
|  - Interactive developer bringup CLI console.                                     |
|  - Zero peripheral pin overhead; active during lab bringing and silicon bringup.  |
+-----------------------------------------------------------------------------------+
|  PRODUCTION & FIELD CHANNELS: High-Speed UART & Wireless BLE                      |
|  - FTDI USB-C Bridge (FC1 UART0 @ 115.2k - 1 Mb/s) & NINA-B312 BLE UART (1 Mb/s). |
|  - Telemetry Egress: defmt binary structured logs & CBOR telemetry frames.        |
|  - Structured Service Model: Binary RPC service endpoint for post-bringup testing |
|    (Stage 2+ hardware tests), remote device configuration, and OTA update staging.|
+-----------------------------------------------------------------------------------+
```

### Tradeoff Evaluation: Segger RTT vs. High-Speed UART Service Model

| Feature | Segger RTT (Development) | High-Speed UART / BLE (Production Service Model) |
| :--- | :--- | :--- |
| **Hardware Required** | External SWD debug probe (J-Link, CMSIS-DAP) physically wired. | Standard USB-C cable or wireless Bluetooth connection; zero probes. |
| **Perfetto Tracing** | **Supported** (High-bandwidth zero-copy memory ring buffer). | Not recommended for high-rate timeline tracing due to UART baud ceiling. |
| **CLI / Shell** | **Interactive CLI Shell** for low-level developer commands. | Structured **Service RPC Protocol** (framed CBOR commands/responses). |
| **Field Telemetry** | Not possible in standalone/enclosed devices. | **Fully Supported** wirelessly via BLE and over external USB-C port. |
| **Enclosure Access** | None (requires open enclosure). | Accessible via external connectors and RF window. |

---

## 4. Subsystem Power Consumption & Operating State Matrix

### <a id="operating-power-state-matrix"></a>Operating Power State Matrix

The table below delineates active hardware blocks, operational frequencies, and aggregate power consumption across the defined operating states:

| Operating State | Core 0 State | Core 1 State | NPU / DSP State | Peripherals Active | Target Current | Target Power | Primary Use Case |
| :--- | :--- | :--- | :--- | :--- | :---: | :---: | :--- |
| **State 0: Deep Standby** | Deep Sleep (WFI) | Deep Sleep (WFI) | Power-gated (OFF) | Fuel gauge, IQS7222A proximity scan | **$\le 185\,\mu\text{A}$** | $\le 0.61\text{ mW}$ | Long-term battery shelf life; wake on touch/RTC. |
| **State 1: Low-Power Sensing** | 12 MHz Low-Freq | WFI Idle Sleep | Power-gated (OFF) | IQS7222A active touch, LP5009 idle | **3.8 mA** | 12.5 mW | User proximity detected; awaiting interaction. |
| **State 2: Normal Active** | 150 MHz Active | 150 MHz Active | Standby | FlexSPI read, BLE connected, RGB breathe | **28.5 mA** | 94.1 mW | Normal interactive sensing, UI display, telemetry. |
| **State 3: Peak Burst** | 150 MHz Active | 150 MHz Active | NPU & DSP Active | Flash write, Class-D audio, BLE TX | **139.35 mA** | 459.8 mW | Gesture classification burst, audio chime, telemetry flush. |

### <a id="per-subsystem-power-measurements"></a>Per-Subsystem Power Measurements

Measurements below quantify the individual current and power contributions corresponding to the states defined in the [Operating Power State Matrix](#operating-power-state-matrix):

| Subsystem / Functional Block | Operating State | Voltage Rail | Current ($I_{active}$) | Power ($P_{active}$) | Active in State | Power Gating / Management Strategy |
| :--- | :--- | :---: | :---: | :---: | :---: | :--- |
| **Core 0 (Cortex-M33)** | 150 MHz Active | `SYS_3V3` | 18.0 mA | 59.4 mW | State 2, 3 | Frequency scaling down to 12 MHz; WFI idle sleep. |
| **Core 1 (Cortex-M33)** | 150 MHz Active | `SYS_3V3` | 16.5 mA | 54.5 mW | State 2, 3 | Clock gated when no touch or audio events pending. |
| **eIQ Neutron NPU** | Inference Burst | `SYS_3V3` | 22.0 mA | 72.6 mW | State 3 | Power-gated autonomously; active only during classification. |
| **PowerQuad / DSP** | Filter Burst | `SYS_3V3` | 8.5 mA | 28.1 mW | State 3 | Dynamically clocked on demand for math operations. |
| **FlexSPI NAND Flash (`U8`)** | Active Read/Write | `SYS_3V3` | 15.0 mA | 49.5 mW | State 2, 3 | Standby current $\le 10\,\mu\text{A}$ between telemetry flushes. |
| **Class-D Audio Amp (`U4`)** | Playing Tone | `SW_3V3_AUDIO` | 18.0 mA | 59.4 mW | State 3 | Hard power-gated via load switch `Q2` ($I_Q \le 0.01\,\mu\text{A}$). |
| **BLE Module (`U11`)** | TX @ +4 dBm | `SYS_3V3` | 9.5 mA | 31.4 mW | State 2, 3 | BLE sleep mode ($I_Q \le 1.5\,\mu\text{A}$) with wake on UART. |
| **RGB LED Driver (`U6`)** | 3 LEDs @ 5 mA | `SYS_3V3` | 15.0 mA | 49.5 mW | State 2, 3 | Low-power shutdown pin disabled when LEDs are dark. |
| **Touch Controller (`U2`)**| Sensing Active | `SW_3V3_SENSORS`| 1.8 mA | 5.9 mW | State 1, 2, 3 | Low-power proximity scan ($15\,\mu\text{A}$) or gated via `Q3`. |
| **FTDI USB Bridge (`U9`)** | USB Connected | `SW_3V3_DEBUG` | 15.0 mA | 49.5 mW | State 2, 3 | Hard power-gated via `Q4` to eliminate parasitic draw. |
| **Fuel Gauge (`U7`)** | Active ADC | `SYS_3V3` | 0.05 mA | 0.17 mW | State 0, 1, 2, 3 | Sleep mode ($I_Q \le 0.5\,\mu\text{A}$) when VBUS unpowered. |

---

## 5. Custom Embassy HAL Implementation & Validation Plan

Because the NXP MCX N947 is a next-generation microcontroller, a custom Embassy Hardware Abstraction Layer (`embassy-mcx`) is developed within the repository to drive the hardware.

### Architecture of `embassy-mcx`

The custom HAL implements canonical `embedded-hal` (blocking) and `embedded-hal-async` (asynchronous) traits across all core peripherals:

1. **Peripheral Access Crate (PAC)**: Generated directly from NXP MCX N947 SVD files using `svd2rust` with strongly-typed register structs and safe bitfield accessors.
2. **Clock & Power Management (`clocks`)**: Configures the 48 MHz Fast Internal Reference Oscillator (FRO-48M), 144/150 MHz Phase-Locked Loop (PLL0), and peripheral clock dividers across the Clock Generation (CLKCON) module.
3. **GPIO & Pin Multiplexing (`gpio`)**: Implements `embedded_hal::digital::InputPin` and `OutputPin` over PORT0..PORT4 and GPIO0..GPIO4 with edge-triggered asynchronous pin interrupts.
4. **Flexcomm Serial Drivers (`flexcomm`)**: Unified Flexcomm architecture supporting runtime configuration as LPUART (with eDMA ring buffers up to 1 Mb/s), LPI2C (Master/Slave with clock stretching), or LPSPI.
5. **High-Speed I3C Controller Driver (`i3c`)**: High-speed I3C bus driver supporting Single Data Rate (SDR up to 12.5 Mbps), Dynamic Address Assignment (DAA), In-Band Interrupts (IBI), and Hot-Join enumeration.
6. **FlexSPI Memory Controller (`flexspi`)**: Configurable Quad-SPI engine allocated to Core 0 supporting MMIO read access and Command Lookup Table (LUT) execution for Winbond W25N01GV NAND flash.
7. **Timers, Watchdog & RTC**:
   - **High-Resolution Timer (`timer`)**: Microsecond-precision timing using CTIMER / MRT peripherals.
   - **Windowed Watchdog (`wwdt`)**: Watchdog timer driver with reset status query (`RSTCTL` / `CMC`) to inspect reset cause on boot (watchdog timeout vs brownout vs software reset).
   - **Real-Time Clock (`rtc`)**: 32.768 kHz battery-backed RTC driver providing calendar timestamps and low-power BLE wake scheduling.
8. **Inter-Core Messaging Unit (`mu`)**: Asynchronous multi-channel hardware mailbox driver enabling zero-copy SPSC buffer passing and cross-core interrupt signaling.

### On-Device Rust Test Frameworks & Post-Bringup UART Service Model

For firmware validation across hardware lifecycle phases:

1. **Bringup Phase (SWD / RTT Available)**:
   - Uses `defmt-test` and interactive RTT CLI shell to execute low-level register and hardware smoke tests directly from host test runners (`probe-rs test`).
2. **Post-Bringup & Production Phase (Only USB UART or BLE UART Available)**:
   - Once units are packaged in enclosures, SWD/RTT is no longer accessible.
   - The **UART/BLE Diagnostic Service Model** implements a framed binary RPC protocol supporting on-device hardware verification (Stage 2 and later tests: I2C/I3C scans, NAND flash verification, fuel gauge reading, touch calibration, and loopback checks). Host test scripts dispatch test commands over USB UART or BLE and assert structured test verdict responses.

### Validation Gate for `embassy-mcx`

The 5-stage validation gate verifies both external board components and internal on-chip peripheral cores:

```mermaid
flowchart LR
    S1["Stage 1: Register Smoke Tests<br/>(GPIO / CLKOUT)"] --> S2["Stage 2: Synchronous HAL<br/>(On-Chip I2C, I3C, UART, BLE)"]
    S2 --> S3["Stage 3: Async DMA HAL<br/>(eDMA / Interrupts / Timers)"]
    S3 --> S4["Stage 4: Dual-Core IPC<br/>(MU Handshake & Ping-Pong)"]
    S4 --> S5["Stage 5: Bringup YAML Gate<br/>(carrier_board_bringup.yaml)"]
```

1. **Stage 1: Silicon Register Smoke Tests**:
   - Verify FRO-48M and PLL clock outputs on physical test point `CLKOUT` using an oscilloscope.
   - Toggle GPIO lines `L4`, `L5`, `M4` (load switch enables) and confirm active-high voltage transitions.
2. **Stage 2: Synchronous `embedded-hal` Validation (On-Chip Peripheral Cores)**:
   - Validate on-chip peripheral controllers:
     - **UART**: Blocking loopback and echo test over FTDI `FC1` UART0 at 115.2 kbps and 1 Mb/s.
     - **I2C**: Bus scan confirming ACK responses from TI LP5009 (`0x14`), ADI MAX17048 (`0x36`), and Azoteq IQS7222A (`0x44`).
     - **I3C**: Bus enumeration, DAA assignment, and SDR communication.
     - **BLE**: AT command query to u-blox NINA-B312 over 1 Mb/s UART.
     - **FlexSPI**: JEDEC ID command to Winbond W25N01GV (confirm `0xEF` and `0xAA21`).
3. **Stage 3: Asynchronous `embedded-hal-async` & DMA Validation**:
   - Verify eDMA-backed asynchronous UART reception and transmission under heavy logging load.
   - Run asynchronous I2C/I3C telemetry read tasks with Embassy timer delays.
   - Verify WWDT watchdog reset cause logging and RTC timestamp increments.
4. **Stage 4: Dual-Core Bringup & Inter-Core IPC Benchmark**:
   - Core 0 initiates Core 1 startup; Core 1 transmits a greeting token back to Core 0 via the Messaging Unit (MU).
   - Measure round-trip ping-pong latency across the shared SRAMX ring buffer; verify zero data corruption.
5. **Stage 5: Full Bringup YAML Integration**:
   - Execute all automated checks codified in [`app/carrier_board_bringup.yaml`](file:///Users/daparker/gh/firmware/app/carrier_board_bringup.yaml). Verification passes only when 100% of checks succeed.

---

## 6. Hardware Bringup & Verification Protocol

The authoritative source of truth for bringup verification is [`app/carrier_board_bringup.yaml`](file:///Users/daparker/gh/firmware/app/carrier_board_bringup.yaml). All hardware verification procedures must follow the step definitions established in that configuration.
