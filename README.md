# Firmware Repository

This repository contains the Rust-based firmware for our hardware projects, built on a modern, decoupled architecture designed for high testability on the host system and portable execution across microcontrollers.

Embedded runtimes execute in bare-metal `#![no_std]` environments powered by the **Embassy** asynchronous framework.

---

## Supported Hardware Targets

*   **[Cat Fountain / Cat Detector](docs/cat_fountain_architecture.md)**: An autonomous pet proximity detection, dual-sensor gesture recognition, and water pump system running on the **Raspberry Pi Pico (RP2040)**.
    *   Architecture Specification: [`docs/cat_fountain_architecture.md`](docs/cat_fountain_architecture.md)
    *   Bringup & Verification Guide: [`app/cat_detector.md`](app/cat_detector.md) (also available via [`app/cat_fountain.md`](app/cat_fountain.md))
*   **[Carrier Board 2.0](docs/carrier_board_architecture.md)**: A modular sensor fusion, ML gesture classification, and bidirectional BLE workstation powered by the dual-core **NXP MCX N947** (Cortex-M33 with PowerQuad DSP and eIQ Neutron NPU).
    *   Architecture Specification: [`docs/carrier_board_architecture.md`](docs/carrier_board_architecture.md)
    *   Bringup & Verification Guide: [`app/carrier_board.md`](app/carrier_board.md)

---

## Architectural Documentation

Comprehensive architectural guidelines and hardware-specific specifications are maintained under [`docs/`](docs):

1.  **[Unified Firmware Architecture Guide](docs/firmware_architecture.md)**: Top-level repository architectural principles, layered crate demarcation, and cross-platform concurrency patterns.
2.  **[Cat Fountain Architecture Guide](docs/cat_fountain_architecture.md)**: RP2040 dual-core task distribution, ToF data fusion, motor current protection, and MPU stack guards.
3.  **[Carrier Board 2.0 Architecture Guide](docs/carrier_board_architecture.md)**: NXP MCX N947 dual-core Asymmetric Multiprocessing (AMP), inter-core IPC, FlexSPI SLC NAND filesystem, BLE GATT service endpoint, and power states.

### Modular Subsystem Architecture Guides
*   [Microcontroller Decoupling & BSPs](docs/mcu_decoupling.md)
*   [Peripheral Sharing & Concurrency Patterns](docs/peripheral_sharing.md)
*   [Domain Controller Design, Task Runners & Codegen](docs/controller_design.md)
*   [Embedded Logging, Tracing & Host Tools](docs/logging_tracing.md)
*   [Hardware-Firmware Co-Design Architecture](docs/hardware_firmware_codesign.md)

---

## Getting Started

### Prerequisites

To build, test, and flash firmware targets, install the required toolchains and utilities:

1.  **Rust Toolchain & Targets**:
    ```bash
    rustup target add thumbv6m-none-eabi   # Cortex-M0+ (RP2040)
    rustup target add thumbv8m.main-none-eabihf  # Cortex-M33 (NXP MCX N947)
    ```
2.  **probe-rs** (for flashing, debugging, and RTT streaming):
    ```bash
    cargo install probe-rs-tools
    ```

### Running Tests (Host-Based Validation)

Our decoupled architecture allows validating domain logic, state machines, and control loops directly on the host machine without hardware:

```bash
# Run tests using cargo-nextest (recommended)
cargo nextest run

# Or run standard cargo tests
cargo test
```

### Building and Flashing

To build and flash firmware to an attached target device:

```bash
# RP2040 Cat Detector Production App
cargo run --target thumbv6m-none-eabi --package app --bin cat_detector_app

# RP2040 Diagnostic Shell
cargo run --target thumbv6m-none-eabi --package app --bin cat_detector_shell

# NXP MCX N947 Carrier Board Diagnostic Shell
cargo build --target thumbv8m.main-none-eabihf --package app --bin carrier_board_shell
```

For interactive diagnostic shell commands, host tools, and flash decoding, see [CONTRIBUTING.md](CONTRIBUTING.md).

### Python Utilities & Bringup Environment

Hardware bringup scripts (`tools/helpers/bringup.py`) and host utilities rely on a dedicated Conda environment:

```bash
# Create and activate the conda environment
conda env create -f environment.yml
conda activate firmware-env

# Run Python unit tests
pytest tools/tests
```

---

## Workspace Architecture

The workspace is organized into target-agnostic crates for logic/simulation, target-independent platform libraries, and target-specific project deployments:

*   **[`model/`](model)**: Core platform-independent system models, state machines, protocols, and calculations (pure `#![no_std]`, zero external dependencies).
*   **[`peripheral/`](peripheral)**: Drivers and wrappers implementing `embedded-hal` trait interfaces (e.g. `VL53L0X`, `INA219`, `MAX17048`, `LP5009`, `IQS7222A`), alongside mocks for host testing.
*   **[`controller/`](controller)**: Hardware-agnostic domain controllers and state machine orchestrators consuming peripheral traits.
*   **[`platform/`](platform)**: Target-independent firmware platform support, structured `defmt` logging, stack scanning, and crash log capturing.
*   **[`board/`](board)**: Board Support Packages (BSPs) managing pinouts, clocks, bus routing, and `Board::init()`.
*   **[`app/`](app)**: Target application binaries, diagnostic shells, and bringup verification definitions.

### Layered Architecture Diagram

```mermaid
graph TD
    subgraph Controller Layer [controller]
        SC["SystemController"]
        MC["MotorController"]
        BC["BatteryController"]
        TC["ThermalController"]
        SNC["SensorController"]
        LC["LedController"]
        FC["FilesystemController"]
        TMC["TelemetryController"]
        ShC["ShellController"]
    end

    subgraph Domain Models [model]
        TM["Telemetry Types & States"]
        TR["Peripheral Traits"]
    end

    subgraph Peripherals [peripheral]
        HAL["embedded-hal Drivers & Mocks"]
    end

    subgraph Board Support [board]
        BSP["Board::init (RP2040 / MCX N947)"]
    end

    subgraph Application Binaries [app]
        AppBin["cat_detector_app / carrier_board"]
        ShellBin["cat_detector_shell / carrier_board_shell"]
    end

    AppBin -->|Spawns tasks| Controller Layer
    ShellBin -->|Executes| ShC
    Controller Layer -->|Queries & Updates| Domain Models
    Controller Layer -->|Drives| Peripherals
    Peripherals -->|Implements| TR
    ShellBin -->|Resolves devices on| BSP
```
