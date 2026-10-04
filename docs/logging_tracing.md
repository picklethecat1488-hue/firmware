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
* **Filesystem Inspection (`tools/host_fs`)**: Host-based Flash filesystem debugging, LittleFS block allocation inspection, directory tree walks, and raw partition read/write operations must be performed using `tools/host_fs`.
* **Console Bridge & RTT Streaming (`tools/host_cli`)**: Host-based interactive CLI shell interaction, Real-Time Transfer (RTT) log streaming, defmt decoding, and live event tracing must be performed using `tools/host_cli`.
* **VCS Workstation & Issue Matrix (`tools/dashboard.py`)**: Smartlog DAG inspection, commit splitting/combining, code reviews, and worm tracking are conducted via `tools/dashboard.py` (System Shock 2 - Xerxes workstation).
