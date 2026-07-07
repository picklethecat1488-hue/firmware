# Firmware Repository

This repository contains the Rust-based firmware for our hardware projects, using a modern, decoupled architecture designed for high testability on the host system.

Currently, the supported hardware target is the **Raspberry Pi Pico (RP2040)**, running an asynchronous RTOS-like framework using **Embassy**.

---

## Workspace Architecture

The workspace is organized into target-agnostic crates for logic/simulation and target-specific crates for deployment:

*   **[model/](model)**: Core platform-independent system models, state machines, protocols, and calculations. Written using `#![no_std]` and has zero dependencies, making it extremely fast to compile and test on the host. Exposes modular components like the state machine.
*   **[peripherals/](peripherals)**: Abstractions (traits) for peripheral wrappers (e.g., `Pump`, `WaterSensor`) and their corresponding generic implementations based on `embedded-hal`. Also contains mock implementations for testing.
*   **[controller/](controller)**: Project-agnostic control loop coordinators. Coordinates the state machines and driver traits in a decoupled fashion.
*   **[projects/](lib)**: General purpose microcontroller code libraries.

---

## Projects

*   **[Cat Detector](projects/cat_detector.md)**: A low-power water fountain and cat proximity detector system running on the Raspberry Pi Pico (RP2040).

---

## Getting Started

### Prerequisites

To build and run this firmware, you need the following tools installed on your host:

1.  **Rust Toolchain & Targets**:
    ```bash
    rustup target add thumbv6m-none-eabi
    ```
2.  **probe-rs** (for flashing, debugging, and RTT log reading):
    ```bash
    cargo install probe-rs-tools
    ```

### Running Tests (Host-Based Validation)

Our decoupled architecture allows you to validate all business logic and control loops directly on your host machine without flashing a microcontroller. We use `cargo-nextest` for faster, parallelized, and less noisy test execution:

```bash
# Run tests using cargo-nextest (highly recommended)
cargo nextest run

# Or fall back to standard cargo tests
cargo test
```

### Building and Flashing

To build and flash the firmware to an attached target device (RP2040):

```bash
# Build target-compatible crates
cargo build --target thumbv6m-none-eabi

# Flash the main application
cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_app
```

For interactive diagnostic shell execution, host logging tools, flash extraction/decoding commands, and other diagnostic procedures, see [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Design & Integration Patterns

For peripheral sharing and task integration, we adhere to the following architectural standards:
*   **The Actor / Message-Passing Pattern**: Our standard for core system integration. Shared peripherals run inside their own isolated tasks, and other components communicate via async channels (e.g. `embassy_sync::channel::Channel`).
*   **Interior Mutability & Shared References (`Rc` + `RefCell` or `Mutex`)**: Used strictly for the bringup shell.
