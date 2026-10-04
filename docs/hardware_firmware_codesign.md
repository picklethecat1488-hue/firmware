# Hardware-Firmware Co-Design & Downselection Architecture

## Architectural Purpose
To establish the baseline hardware platform specifications, MCU subsystem capabilities, and component qualification requirements governing bare-metal firmware drivers and board support packages.

---

## Core Invariants

### 1. Target Firmware Environment
* **Bare-Metal Rust Specification**: The target runtime environment is bare-metal Rust (`no_std`, Embassy asynchronous executor, `embedded-hal` and `embedded-hal-async` driver abstractions, `defmt` structured logging, stack-based zero-allocation concurrency, and static memory analysis) as detailed in [CONTRIBUTING.md](file:///Users/daparker/gh/firmware/CONTRIBUTING.md).

### 2. Reference Hardware Architecture (Revision 2.0)
Component selection and pinmux topologies are governed by the Test Board Revision 2.0 Downselection Process documented in [docs/downselection_report.md](file:///Users/daparker/gh/firmware/docs/downselection_report.md):
* **Primary MCU Subsystem**: NXP MCX N947 / N946 dual ARM Cortex-M33 cores @ 150 MHz with integrated eIQ Neutron Neural Processing Unit (NPU, 42 GMACs), PowerQuad DSP coprocessor, 2MB dual-bank Flash, and 512KB SRAM.
* **Secondary Telemetry & Calibration Storage**: Winbond W25N01GV 1Gb Serial SLC NAND Flash over high-speed Dual-Channel FlexSPI (100 MHz SDR OD mode).
* **Power & Battery Management**: Texas Instruments BQ24074 dynamic Power-Path Li-Ion charger (`/PGOOD` wake interrupt, `/CHG` state monitoring) and Analog Devices MAX17048 precision fuel gauge (I2C0).
* **Diagnostic & Console Bridge**: FTDI FT232RNQ USB-UART bridge IC connected to USB-C (UART0 @ 1-3 Mbaud, CBUS reset/boot controls) and a standard 10-pin ARM SWD Cortex debug header.
* **HMI & Touch Sensors**: Texas Instruments LP5009 I2C constant-current RGB LED driver (I2C0 0x14) and Cypress CY8CMBR3116 capacitive touch controller (I2C1 + CAP_INT).

### 3. Programmable Component Qualification Mandate
Any active electronic component, sensor IC, PMIC, LED driver, or microcontroller integrated into hardware designs that requires software control MUST meet the following co-design criteria:
* **Open-Source Driver Availability**: A permissive open-source driver (preferably a Rust crate implementing `embedded-hal` traits or a portable C library) must be publicly available.
* **Public Datasheets**: Complete, non-confidential public datasheets covering electrical specifications, pin assignments, timing diagrams, and sample application circuits must be accessible.
* **Comprehensive Register Maps**: Full register maps documenting all register offsets, bit fields, reset defaults, and initialization/command sequences must be provided.
* **Zero Binary Blobs**: Components requiring proprietary closed-source binary firmware blobs, undocumented black-box registers, or NDA-encumbered software stacks are strictly prohibited.
