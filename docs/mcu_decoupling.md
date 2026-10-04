# Microcontroller Decoupling & Board Support Packages (BSPs)

## Architectural Purpose
To ensure firmware portability across multiple hardware platforms and target microcontrollers without polluting core application logic, drivers, or domain controllers with vendor-specific peripherals or pin allocations.

---

## Core Invariants

### 1. Separation of Concerns & Clean App Entrypoints
* **No Conditional Pin Setup in Applications**: Do NOT perform conditional driver setup or extract GPIO pins inside main application files (`main.rs`, `shell.rs`).
* **Encapsulated Board Initialization**: Encapsulate all initialization, pin configuration, clock tree setup, and dynamic address assignment inside the `Board::init` constructor in Board Support Package (BSP) target/host implementations (e.g., `bsp_target.rs` / `bsp_host.rs`).
* **Target Independence**: Application tasks, domain state machines, and high-level controllers must receive typed abstraction handles or channel endpoints from the BSP, never raw microcontroller registers or peripherals.

### 2. Driver & Struct Naming Hygiene
* **No MCU Model Prefixes**: Do NOT prefix files or structs with MCU model numbers (e.g., do NOT write `rp2040_sensor.rs` or `struct NxpI2cDriver`). Keep driver wrappers and peripheral interfaces target-independent.
* **Portable Abstractions**: Drivers must implement or accept canonical traits (such as Rust `embedded-hal` / `embedded-hal-async`) rather than concrete vendor peripheral structs.

### 3. Vendor-Specific Code Boundaries
* **Placement Restrictions**: Vendor-specific code (register blocks, PAC access, boot stages, manufacturer errata workarounds) should ONLY be placed inside the `board` or `app` crate.
* **Platform Module Scoping**: Vendor-specific code may go into the `platform` crate ONLY when placed inside a dedicated platform support module for that vendor (e.g., `platform/src/rp2040/lib.rs` or `platform/src/mcx_n947/`).

### 4. Timing & Delay Invariants
* **No Fixed CPU Cycle Delays**: Do NOT use fixed CPU cycle delays (e.g., `cortex_m::asm::delay`) in firmware code, as cycle counts vary unpredictably with clock frequency changes, cache hits, Flash wait states, and pipeline depths.
* **Frequency-Independent Time Delays**: Use frequency-independent time-based delays (e.g., `embassy_time::Timer` or `embassy_time::Delay`) that yield execution back to the async executor or hardware timers.
