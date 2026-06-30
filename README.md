# Firmware Repository

This repository contains the Rust-based firmware for our hardware projects, using a modern, decoupled architecture designed for high testability on the host system.

Currently, the supported hardware target is the **Raspberry Pi Pico (RP2040)**, running an asynchronous RTOS-like framework using **Embassy**.

---

## Workspace Architecture

The workspace is organized into target-agnostic crates for logic/simulation and target-specific crates for deployment:

*   **[model/](file:///Users/daparker/gh/firmware/model)**: Core platform-independent system models, state machines, protocols, and calculations. Written using `#![no_std]` and has zero dependencies, making it extremely fast to compile and test on the host. Exposes modular components like the state machine.
*   **[peripherals/](file:///Users/daparker/gh/firmware/peripherals)**: Abstractions (traits) for peripheral wrappers (e.g., `Pump`, `WaterSensor`) and their corresponding generic implementations based on `embedded-hal`. Also contains mock implementations for testing.
*   **[controller/](file:///Users/daparker/gh/firmware/controller)**: Project-agnostic control loop coordinators. Coordinates the state machines and driver traits in a decoupled fashion.
*   **[projects/](file:///Users/daparker/gh/firmware/projects)**: Concrete firmware applications targeted at specific microcontrollers

---

## Projects

*   **[Cat Detector](file:///Users/daparker/gh/firmware/projects/cat_detector.md)**: A low-power water fountain and cat proximity detector system running on the Raspberry Pi Pico (RP2040).

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
    cargo install probe-rs --features cli
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

```bash
# Build the entire package (both library and binaries)
cargo build --package cat_detector --target thumbv6m-none-eabi

# Run/Flash the main Cat Detector application
cargo run --package cat_detector --bin cat_detector

# Run/Flash the interactive bringup shell
cargo run --package cat_detector --bin shell
```

---

## Debugging and Log Output

We use **RTT (Real-Time Transfer)**-based debugging for maximum speed and low overhead compared to slow SWD register polling or semihosting.

### 1. Attaching a Debugger
To attach `probe-rs` and inspect the target without flashing:
```bash
probe-rs attach --chip RP2040
```

### 2. Reading Debug Printfs (defmt over RTT)
All logging uses `defmt` over RTT, which formats strings on the host to minimize binary footprint and transfer times.
Logs are automatically streamed to your terminal when running the project via:
```bash
cargo run --package hello_world
```
Or when using the `probe-rs` tool to watch logs from an already running device:
```bash
probe-rs run --chip RP2040
```
Alternatively, for UART-based logging, configure the microcontroller's UART TX/RX pins using the hardware HAL (`embassy-rp::uart`).
