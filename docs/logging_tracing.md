# Embedded Logging, Tracing & Host Diagnostic Tools

## Architectural Purpose
To provide deterministic, zero-allocation structured telemetry on embedded targets and robust host tooling for real-time firmware debugging, filesystem inspection, and RTT tracing.

---

## Core Invariants

### 1. Embedded Logging Standards
* **defmt Instrumentation**: Instrument all async tasks, controller loops, state machine transitions, and application entry points using `defmt` logging macros (`defmt::info!`, `defmt::debug!`, `defmt::warn!`, `defmt::error!`) for startup, heartbeat ticks, and command processing.
* **No Format String Allocations**: Never use standard library formatting (`format!`, `println!`) in target firmware. All logging formats must be resolved statically via `defmt` symbol tables to minimize Flash consumption and runtime execution overhead.

### 2. Consolidated Tracing Facade & Domain-Scoped Trace Modes
* **Platform Facade**: Always use the consolidated tracing facade module `use crate::tracing;` (which re-exports `platform::tracing`).
* **No External Tracing Crate**: Do NOT import the standard crates.io `tracing` crate directly in target code, as it requires dynamic heap allocation (`alloc`) and runtime string buffers incompatible with bare-metal `no_std` constraints.
* **Instrument Skip Rules & Domain Tagging**: When annotating methods with `#[tracing::instrument]`, do NOT list `self` in the `skip(...)` attribute. Annotate sensor peripheral/controller methods with `#[tracing::instrument(domain = "sensor")]` (and future audio paths with `domain = "audio"`) so `PERF` mode (`--features trace-perf`) captures all non-sensor execution spans and CPU usage while excluding sensor spans, whereas `SENSOR` mode (`--features trace-sensors`) emits real-time measured sensor reports only (`platform::tracing::trace_sensor_report`) with zero span overhead.
* **Target Selection, `TRACE` Tab & External Viewers (`Perfetto` vs `Rerun`)**: `tools/host.py` embeds `FirmwareTraceMode` (`None`, `Sensor`, `Perf`, `Audio`) in `.program_metadata`, streams structured `HostCliEvent::Trace` records into the **`TRACE`** workstation tab (with a separate hovering bottom playback/zoom/filter trackbar), exports `.csv` and `.perfetto.json` files to disk, opens **`PERF`** traces in `ui.perfetto.dev` (**🚀 Open in Perfetto**), and streams **`SENSOR`** / **`AUDIO`** traces directly into Rerun via `rerun-sdk` (**🚀 Open in Rerun** / `python tools/host.py trace open-rerun`).

### 3. Host Diagnostic & Debugging Tooling
* **Filesystem Inspection (`tools/host_fs` / `cargo host-fs`)**: Host-based Flash filesystem debugging, sequential storage inspection, directory tree walks, and raw partition read/write operations must be performed using `tools/host_fs`:
  ```bash
  # List files in sequential flash storage:
  cargo host-fs --elf target/thumbv6m-none-eabi/debug/cat_detector_app ls

  # Export telemetry to CSV:
  cargo host-fs --elf target/thumbv6m-none-eabi/release/cat_detector_app export-telemetry telemetry.csv

  # Decode crash logs:
  cargo host-fs --elf target/thumbv6m-none-eabi/release/cat_detector_app crash-log
  ```
* **Console Bridge & RTT Streaming (`tools/host_cli` / `cargo host-cli`)**: Host-based interactive CLI shell interaction, Real-Time Transfer (RTT) log streaming, defmt decoding, structured NDJSON event streaming (`--json-events`), and live event tracing must be performed using `tools/host_cli`:
  ```bash
  # Direct RTT shell console:
  cargo host-cli --elf target/thumbv6m-none-eabi/debug/cat_detector_shell

  # Attach concurrently to active host_debug non-halting RTT memory bridge (:50000):
  cargo host-cli --gdb-bridge 127.0.0.1:50000 --elf target/thumbv6m-none-eabi/debug/cat_detector_shell
  ```
* **Target GDB Server & RTT Memory Bridge (`tools/host_debug` / `cargo host-debug`)**: Dual-core `probe-rs` GDB RSP server (`:3333` Core 0, `:3334` Core 1) and `:50000` non-halting RTT bridge managed automatically by `tools/host.py --debug-server` (or `python tools/host.py debug start`):
  ```bash
  cargo host-debug --elf target/thumbv6m-none-eabi/debug/cat_detector_shell --gdb-core0-port 3333 --gdb-core1-port 3334
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

  # Execute automated target device test & bringup suites (or --debug for VS Code debugpy attach):
  conda run -n firmware-env python tools/host.py test
  conda run -n firmware-env python tools/host.py test --stress-runs 10
  conda run -n firmware-env python tools/host.py test -k tof_proximity --debug
  conda run -n firmware-env python tools/host.py test --script tools/device_tests/suites/test_cat_detector_bringup.py --manual
  ```
  See [CLI Tools, Flashing, Host Daemon & Verification Workflows](cli_and_workflows.md#6-interactive-symbolic-test--target-debugging-vs-code) for full VS Code `debugpy`, `CodeLLDB`, `node --test`, `probe-rs`, and `Cortex-Debug` symbolic debugging configurations.
* **VCS Workstation & Issue Matrix (`tools/forge.py`)**: Smartlog DAG inspection, commit splitting/combining, code reviews, and worm tracking are conducted via `tools/forge.py` (System Shock 2 - Xerxes workstation). Launch via `python tools/forge.py` (native pywebview desktop window), `python tools/forge.py --eel` (Chromium app mode), or `python tools/forge.py --no-browser` (headless).
