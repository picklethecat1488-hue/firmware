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

---

## Flash Diagnostics Tool (`fs_tool`)

We provide a host command-line utility, `fs_tool`, to inspect, query, and decode flash memory contents from the microcontroller's sequential-storage partition. It supports two modes of operation:
- **Direct Device Mode**: Connects directly to the attached target via `probe-rs`, using `--project <PROJECT>` to dynamically look up the target chip and flash partition mapping.
- **Offline Dump Mode**: Reads and writes a local binary flash image file via `--dump <PATH>`.

### 1. Build host fs_tool
```bash
cargo build --bin fs_tool --release
```

### 2. Query directory files (`ls`)
- **Direct connection**:
  ```bash
  cargo run --bin fs_tool -- --project cat_detector ls
  ```
- **Offline dump file**:
  ```bash
  cargo run --bin fs_tool -- --dump flash_dump.bin ls
  ```

### 3. Copy files to/from device (`cp`)
- **Copy telemetry from device to host**:
  ```bash
  cargo run --bin fs_tool -- --project cat_detector cp dev:telemetry.rrd local_telemetry.rrd
  ```
- **Copy new calibration config to device**:
  ```bash
  cargo run --bin fs_tool -- --project cat_detector cp local_cal.bin dev:calibration.bin
  ```

### 4. Export telemetry to CSV
- **Direct connection**:
  ```bash
  cargo run --bin fs_tool -- --project cat_detector export-telemetry telemetry.csv
  ```
- **Offline dump file**:
  ```bash
  cargo run --bin fs_tool -- --dump flash_dump.bin export-telemetry telemetry.csv
  ```

### 5. Decode crash dumps to symbolicated backtraces
- **Direct connection**:
  ```bash
  cargo run --bin fs_tool -- --project cat_detector crash-log --elf target/thumbv6m-none-eabi/release/cat_detector
  ```
- **Offline dump file**:
  ```bash
  cargo run --bin fs_tool -- --dump flash_dump.bin crash-log --elf target/thumbv6m-none-eabi/release/cat_detector
  ```

---

### Extracting flash partition manually (Fallback)
If you prefer to extract the raw binary flash memory partition manually from the target Pico using `probe-rs`:
```bash
probe-rs read-mem --chip RP2040 0x101C0000 262144 flash_dump.bin
```

---

## Design & Integration Patterns

For peripheral sharing and task integration, we adhere to the following architectural standards:
*   **The Actor / Message-Passing Pattern**: Our standard for core system integration. Shared peripherals run inside their own isolated tasks, and other components communicate via async channels (e.g. `embassy_sync::channel::Channel`).
*   **Interior Mutability & Shared References (`Rc` + `RefCell` or `Mutex`)**: Used strictly for the bringup shell.
*   **We never use The Reference Passing Pattern (Recommended, Zero-Cost)**.
