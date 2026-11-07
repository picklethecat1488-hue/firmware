# Embedded Logging, Tracing & Host Diagnostic Tools

## Architectural Purpose
To provide deterministic, zero-allocation structured telemetry on embedded targets and robust host tooling for real-time firmware debugging, filesystem inspection, and RTT tracing.

---

## Core Invariants

### 1. Embedded Logging Standards
* **defmt Instrumentation**: Instrument all async tasks, controller loops, state machine transitions, and application entry points using `defmt` logging macros (`defmt::info!`, `defmt::debug!`, `defmt::warn!`, `defmt::error!`) for startup, heartbeat ticks, and command processing.
* **No Format String Allocations**: Never use standard library formatting (`format!`, `println!`) in target firmware. All logging formats must be resolved statically via `defmt` symbol tables to minimize Flash consumption and runtime execution overhead.

### 2. Consolidated Tracing Facade
* **Platform Facade**: Always use the consolidated tracing facade module `use crate::tracing;` (which re-exports `platform::tracing`).
* **No External Tracing Crate**: Do NOT import the standard crates.io `tracing` crate directly in target code, as it requires dynamic heap allocation (`alloc`) and runtime string buffers incompatible with bare-metal `no_std` constraints.
* **Instrument Skip Rules**: When annotating methods with `#[tracing::instrument]`, do NOT list `self` in the `skip(...)` attribute.

### 3. Host Diagnostic & Debugging Tooling
* **Filesystem Inspection (`tools/host_fs`)**: Host-based Flash filesystem debugging, sequential storage inspection, directory tree walks, and raw partition read/write operations must be performed using `tools/host_fs`:
  ```bash
  # List files in sequential flash storage:
  cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/debug/cat_detector_app ls

  # Export telemetry to CSV:
  cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/release/cat_detector_app export-telemetry telemetry.csv

  # Decode crash logs:
  cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/release/cat_detector_app crash-log
  ```
* **Console Bridge & RTT Streaming (`tools/host_cli`)**: Host-based interactive CLI shell interaction, Real-Time Transfer (RTT) log streaming, defmt decoding, and live event tracing must be performed using `tools/host_cli`:
  ```bash
  # Direct RTT shell console:
  cargo run -p host_cli -- --elf target/thumbv6m-none-eabi/debug/cat_detector_shell

  # Attach to active GDB/OpenOCD session:
  cargo run -p host_cli -- -o localhost:50000 --elf target/thumbv6m-none-eabi/debug/cat_detector_shell
  ```
* **Host Daemon & Live Target Synchronization (`tools/host.py`)**:
  ```bash
  # Launch workstation daemon:
  python tools/host.py --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release

  # Trigger incremental compilation, probe-rs flashing, and RTT reconnection without daemon restart:
  python tools/host.py sync

  # Query sync status:
  curl -s http://127.0.0.1:8899/api/sync/status
  ```
* **Target Flashing & Automated Bringup**:
  ```bash
  # Flash diagnostic shell via probe-rs:
  probe-rs download target/thumbv6m-none-eabi/debug/cat_detector_shell --chip RP2040

  # Flash production application:
  cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_app

  # Execute automated target bringup suite:
  conda run -n firmware-env python tools/host.py bringup --script projects/cat_detector_bringup.yaml --run
  ```
* **VCS Workstation & Issue Matrix (`tools/dashboard.py`)**: Smartlog DAG inspection, commit splitting/combining, code reviews, and worm tracking are conducted via `tools/dashboard.py` (System Shock 2 - Xerxes workstation). Launch via `python tools/dashboard.py` (native pywebview desktop window), `python tools/dashboard.py --eel` (Chromium app mode), or `python tools/dashboard.py --no-browser` (headless).
