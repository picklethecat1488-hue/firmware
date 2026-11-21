# Contributing to Firmware

Welcome! This document outlines the architectural standards, code patterns, and onboarding guidelines for contributing to this firmware repository.

---

## Architectural Guidelines

To maintain testability, reliability, and modularity, follow these core principles:

### 1. File Structure Standards
*   **Separation of Source Files**: Avoid grouping multiple interfaces and implementations into a single `lib.rs`. Keep each module in its own Rust listing (e.g., `model/src/state_machine.rs`). Expose them cleanly in `lib.rs` (e.g., `pub mod state_machine;`).
*   **Test Isolation**: Keep tests completely isolated from implementation code. Do not mix unit tests inside the main module source files. All unit tests must be separated from module code and stored in a `tests/` subfolder at the crate root level (e.g., `model/tests/state_machine_tests.rs` or `peripherals/tests/integration.rs`), `tools/tests/` for host Python tools, and `tools/tests/js/` (`*.test.mjs`) for JavaScript ES modules.
*   **Test Resource Artifacts (`tools/tests/resources/`)**: Never embed multi-line dummy workspace files (`Cargo.toml`), sample `@device_test` suites, or mock subprocess scripts inline inside Python test listings. Store reusable test fixtures as standalone checked-in artifacts under `tools/tests/resources/`:
    *   `tools/tests/resources/fake_workspace/` — Minimal multi-binary Cargo workspace manifests (`Cargo.toml`, `app/Cargo.toml`) copied into `tmp_path` via `_copy_resource_tree("fake_workspace", dest_dir)`.
    *   `tools/tests/resources/suites/` — Standalone `@device_test` suite fixtures and non-decorated helper modules copied into temporary suite directories during discovery/execution tests.
    *   `tools/tests/resources/mock_*.py` & `mock_*.sh` — Mock `host_cli`, `host_debug`, and `host_fs` scripts invoked via `_make_resource_launcher(dest_exe, "<script>", env_overrides={"XERXES_MOCK_*": ...})`, passing per-test temporary paths (such as `tmp_path` argv logs) through `XERXES_MOCK_*` environment variables.
    *   When adding a new test resource, place the file in the appropriate `tools/tests/resources/` subdirectory with a concise module/header comment explaining the simulated behavior, keep all dynamic state in `tmp_path` or `XERXES_MOCK_*` environment variables so parallel `pytest -n auto` workers never collide, and invoke it via `_copy_resource_tree` or `_make_resource_launcher`. Resource directories (`**/tests/resources/`) are excluded by default from CLI searches (`.ignore`), `pytest` discovery (`norecursedirs`), and IDE search/indexing (`.vscode/settings.json`).
*   **JavaScript Zero-Globals Architecture (`tools/js/`)**: All browser ES modules and static assets for `tools/forge.py` and `tools/host.py` live under the single canonical `tools/js/` directory, with zero inline JS or `on*` attributes in `.html.j2` templates and all browser globals isolated behind `tools/js/core/runtime_context.js` (see [docs/javascript_style_architecture.md](docs/javascript_style_architecture.md)).

### 2. Microcontroller Decoupling & Board Support Packages (BSPs)
To keep peripheral logic target-independent and clean, we decouple hardware implementation from high-level code using traits and Board Support Packages:
*   **Peripheral Abstraction**: Define driver interfaces (e.g., `Motor` or `ProximitySensor`) in the `peripherals` crate, implemented generically over `embedded-hal` primitives.
*   **BSP Encapsulation (`bsp_target.rs` / `bsp_host.rs`)**: Avoid conditional driver setup and GPIO pin extraction inside application entry points (`main.rs`, `shell.rs`). Instead, encapsulate all initialization—including dynamic sensor I2C address assignment, boot timing delays, and concrete driver construction—inside the `Board::init` constructor of the project's BSP.
*   **Unified Board API**: The `Board` struct must expose identical driver and pin fields on both host and target (using mock types on the host). This allows binaries to consume physical hardware or simulated nodes transparently.
*   **Mocks**: Mock devices (`MockLed`, `MockMotor`, `DummyProximitySensor`, etc.) are placed under the `peripherals::mock` module, enabled via the `mock` feature flag, for host-based test execution.

### 3. Separation of Concerns
*   **Domain Logic (`model/`)**: Pure states, serialization, and traits interfaces. Target-agnostic and free of framework/I2C dependencies.
*   **Platform Abstractions (`platform/`)**: The core system abstraction library exposing hardware interfaces, telemetry, scheduling, and power management utilities. Restructured into distinct architectural subdomains:
    *   `system/`: Low-level system scheduling, registers, panic handlers, Segger RTT, and concurrency helpers.
    *   `telemetry/`: Defmt logging configuration, tracing macro configurations, and CBOR serialization.
    *   `power/`: Power management loops, wake locks, battery levels, and thermal controls.
    *   `io/`: Dynamic flash wrappers, I2C shared resource multiplexing, and hardware tickers.
    *   `services/`: Serial CLI command parsers, file directories, and gesture detection utilities.
    *   `host/`: Common host-side developer tools utilities (e.g. ELF parsing, symbolication, GDB connections).
*   **Peripheral Interfaces (`peripherals/`)**: Platform-independent implementations of hardware drivers over generic `embedded-hal` constraints.
*   **Control Loop Coordinators (`controller/`)**: Project-agnostic domain orchestrators. Exposes CLI handlers that accept a reference to `ShellDeviceResolver` to lookup required peripherals on demand.
*   **Board & Application Crates (`board/` & `app/`)**: Declaratively configure microcontroller-specific Board Support Packages (`board/board.toml`, `board/src/<board>.rs`) and application controller topologies + binaries (`app/app.toml`, `app/src/<app>.rs`, `app/src/<name>_shell.rs`, `app/src/<name>_app.rs`).

### 4. Platform-Specific Naming Conventions & CLI Routing
*   **Peripherals & Controllers**: Do **not** prefix files or structs with MCU model numbers (e.g. do not name them `rp2040_sensor.rs`). Since they interact with generic drivers, they compile on any target.
*   **Target Projects**: MCU-specific setup code and drivers (e.g. `Rp2040TempSensor`) live in the project directory (e.g. `projects/board/src/bsp_target.rs`).
*   **CLI Router Forwarding**: Avoid writing wrapper forwarding methods in `ShellController` for domain-level command execution. Instead, define match actions inside the CLI macro blocks to call domain handlers (e.g. `crate::motor_controller::handle_motor_cli`) directly.

---

## Multi-Control Loop Execution

We support concurrent control loops in a project-agnostic way using Embassy's asynchronous executor.

### Stack-Based Async Concurrency
To avoid the synchronization pitfalls, memory overhead, and queue-deadlocks common in traditional RTOSs, we utilize **purely stack-based communication** between control loops:

1.  **Sequential Polling (Single Task)**:
    For tightly coupled loops, update them sequentially in a single loop using mutable references passed on the stack. No queues, no allocations, and zero thread-safety overhead:
    ```rust
    loop {
        // Direct, stack-based communication
        loop_a.tick(&mut shared_state);
        loop_b.tick(&mut shared_state);
        Timer::after(Duration::from_millis(10)).await;
    }
    ```
2.  **Zero-Capacity Signals (Cross-Task & ISR-Friendly)**:
    If loops must run as separate async tasks, communicate using `embassy_sync::signal::Signal` or stack-allocated mutexes/cells (e.g. `NoopRawMutex`). 
    *   **Interrupt Safety**: `Signal` is fully safe to use within **Interrupt Service Routines (ISRs) and callbacks**. You can trigger `signal.signal(value)` inside an ISR or callback, which will safely and instantly wake up the async task waiting on `signal.wait().await` using a critical section.
3.  **Awaiting Multiple Events Concurrently (The Select Pattern)**:
    To avoid busy-waiting and allow a task to block on multiple async events concurrently (e.g., a timer, a command channel, and a hardware pin interrupt), use the `embassy_futures::select` family of functions:
    *   **Timeout Selector**: Use `embassy_time::with_timeout(duration, future)` to wrap any single async operation with a maximum wait time.
    *   **Multi-Event Selector**: Use `select`, `select3`, or `select4` to concurrently await up to 4 async operations. This is completely `no_std`, zero-heap-allocation, and resolves into a single compiler-generated state machine:
        ```rust
        use embassy_futures::select::{select3, Either3};

        loop {
            let timer = Timer::after(Duration::from_millis(1500));
            let command = command_rx.receive();
            let pin_irq = button.wait_for_any_edge();

            match select3(timer, command, pin_irq).await {
                Either3::First(_) => {
                    // Periodic timer tick (e.g. read telemetry)
                }
                Either3::Second(cmd) => {
                    // Command received from shell or channel
                }
                Either3::Third(_) => {
                    // Hardware button pressed or ISR callback event
                }
            }
        }
        ```

---

## Compile-Time Memory Usage Analysis

To guarantee safety and prevent stack overflow or memory exhaustion, we measure stack and data usage at compile time.

### 1. Static Data & RAM Usage (BSS, Data, Text)
To inspect the RAM/Flash overhead of a build:
*   Install `cargo-binutils`:
    ```bash
    cargo install cargo-binutils
    rustup component add llvm-tools-preview
    ```
*   Run the size utility to calculate the bytes allocated to `.text` (flash), `.data` (RAM), and `.bss` (uninitialized RAM) sections:
    ```bash
    cargo install cargo-bloat
    cargo bloat --bin app --target thumbv6m-none-eabi
    ```

---

## Implementing Debug & Diagnostic Functionality

For debug utilities or diagnostic code (such as I2C bus scanners), use one of the following three idiomatic Cargo/Rust patterns:

### 1. Conditional Cargo Features (Feature Flag gating)
If the diagnostic logic is integrated directly into the main firmware but should be omitted from release builds, use a Cargo feature:
*   Define the feature in your project's `Cargo.toml`:
    ```toml
    [features]
    default = []
    i2c-scan = []
    ```
*   Gate the code using `#[cfg(feature = "i2c-scan")]`:
    ```rust
    #[cfg(feature = "i2c-scan")]
    scan_i2c_bus();
    ```
*   Build with the feature enabled:
    ```bash
    cargo build --package cat_detector --target thumbv6m-none-eabi --features i2c-scan
    ```

---

## Sharing Peripherals Between Controllers

In Rust, strict ownership rules prevent a peripheral driver from being owned by multiple controllers simultaneously. To share a peripheral (such as a battery sensor) between multiple controllers (such as thermal and power controllers), use one of the following two patterns depending on the context (we never use the Reference Passing pattern):

### 1. The Actor / Message-Passing Pattern (Standard for System Integration)
For complex systems running separate asynchronous tasks, run the shared peripheral inside its own isolated task (the "Controller"). Other controllers communicate by sending request/response messages over channels (e.g. `embassy_sync::channel::Channel`):
*   The `ThermalController` sends a `BatteryQuery::GetTemperature` message over a channel.
*   The `PowerController` sends a `BatteryQuery::GetVoltage` message.
*   The battery task polls the channel, performs the I2C/ADC reads, and sends results back.
This completely isolates hardware registers from concurrent access issues.

### 2. Interior Mutability & Shared References (`Rc` + `RefCell` or `Mutex`) (Standard for the Shell)
If controllers are stored in separate tasks/structs and must hold a reference to the peripheral driver over their entire lifetime (typically used in the bringup shell):
*   **Single-Threaded Async Executor (Embassy default)**: Wrap the peripheral in a reference-counted `Rc` pointer with a `RefCell` for interior mutability:
    ```rust
    let battery = Rc::new(RefCell::new(Battery::new(pin)));
    
    let mut thermal = ThermalController::new(Rc::clone(&battery));
    let mut power = PowerController::new(Rc::clone(&battery));
    ```
*   **Multi-core / Core-shared execution**: Wrap the peripheral in an `Arc` pointer with an Embassy-sync `Mutex` (non-blocking, async-aware) or `critical_section::Mutex`:
    ```rust
    let battery = Arc::new(embassy_sync::mutex::Mutex::new(Battery::new(pin)));
    ```

---

## Interactive Bringup CLI / Diagnostic Reads

To support hardware bringup and diagnostics without requiring active asynchronous runtime loop schedulers:
1. **Blocking Reader Traits**:
   Subsystem controllers (e.g. `BatteryController`, `ThermalController`, `SensorController`, `MotorController`) must implement target-independent blocking reader traits defined in `controller/src/lib.rs` (e.g. `BlockingBatteryReader`, `BlockingThermalReader`, `BlockingProximityReader`, `BlockingMotorReader`).
2. **Mutex Try-Locking**:
   These blocking implementations must use non-blocking mutex try-locking (`try_lock()`) on shared peripheral drivers to inspect status/sensors immediately without yielding or blocking the current CPU thread.
3. **Dummy/Unit Type Implementations**:
   Provide implementations of the blocking reader traits for the unit type `()` to enable compiling the bringup shell and test suites under different hardware or mock targets without instantiating full subsystem controllers.
4. **Shell Output Formatting**:
   CLI commands must be designed to execute and report a standard `Result<(), &'static str>` code:
   - On success, the command prints the required output and writes `Command succeeded` to the console.
   - On failure, it outputs the error reason and writes `Command failed: <reason>`.
   Repetitive debug logs (like `Sent command to controller`) should be omitted to keep bringup console outputs clean.

---

## Controller Design & Constraints

To maintain decoupled domain architectures and clean Embassy task separation:
1. **Controller Struct Attribute**:
   Every domain controller context struct defined inside the `controller` crate must be decorated with `#[crate::tracing::controller_context]` (e.g. `#[crate::tracing::controller_context] pub struct MotorController { ... }`).
2. **Direct Instantiation Restrictions**:
   Controllers must never directly instantiate other controllers (e.g. calling `FilesystemController::new` from `MotorController`). This strict isolation prevents duplicate peripheral/task ownership.
3. **Platform-Level Diagnostic Reads**:
   If diagnostics or calibration commands (which run inside CLI shell contexts) need to access files or direct storage without a running executor task, they must use stateless platform-level direct operations (such as `platform::flash::read_file_direct` and `platform::flash::write_file_direct`) instead of instantiating controller tasks directly.
4. **Controller & CLI Code Generation (`controllers.toml` / `shell.toml` / Rinja Templates)**:
   All controllers, their channel definitions, task runner macros (`run_*_task!`), `spawn_single_controller!` matching arms, and interactive CLI resolvers/commands are automatically generated at build time using the **Rinja** template engine. 
   - To add a new controller or modify module parameters, edit the configuration in [controller/controllers.toml](controller/controllers.toml).
   - To add or modify interactive CLI subcommands, arguments, or resolver fields, edit [shell.toml](shell.toml).
   - The template files are defined in `controller/templates/` (e.g. [generated_controllers.rs.jinja](controller/templates/generated_controllers.rs.jinja) and [sample_cli.rs.jinja](controller/templates/sample_cli.rs.jinja)).
   - The `has_telemetry` flag defaults to `true`. If a controller does not use telemetry reporting, explicitly set `has_telemetry = false` in the TOML file.
   - **Host Code Generation Viewer Tool (`code_gen`)**:
     We provide a host utility to print and inspect the rendered Rust code, list controllers/CLIs, and output sample implementations to the filesystem (defaulting to `target/out`).
     ```bash
     # Inspect generated macros/channels for a specific controller (e.g. Led)
     cargo run -p code_gen -- Led

     # List all currently defined controllers
     cargo run -p code_gen -- list-controllers

     # List all currently defined CLI commands/groups
     cargo run -p code_gen -- list-clis
     
     # Generate a compiling sample CLI wrapper code under target/out/sample_cli.rs
     cargo run -p code_gen -- cli-sample

     # Generate specific CLI subcommand handler skeletons (e.g. Motor, Battery)
     cargo run -p code_gen -- cli-sample Motor Battery

     # Generate all boilerplate runloop implementations under target/out/sample_runloops.rs
     cargo run -p code_gen -- runloop-sample

     # Generate a specific controller's runloop boilerplate (e.g. Motor)
     cargo run -p code_gen -- runloop-sample Motor

     # Specify a custom output directory using --out-dir
     cargo run -p code_gen -- cli-sample Motor --out-dir target/out/my_custom_dir
     ```

---

## Logging & Instrumentation Standards

To ensure debugging, status tracking, and performance monitoring are unified across all binaries and target applications, we enforce the following logging and instrumentation standards:

### 1. Mandatory Task & Loop Instrumentation
All asynchronously executed tasks, controller loops, and main entry points (`main`) must be instrumented by default:
*   **Startup/Initialization**: Log when a subsystem or peripheral is initializing (e.g., `defmt::info!("Initializing hardware...")`).
*   **Loop Ticks/Telemetry**: Periodically log sensor reads, state updates, or telemetry changes (e.g. `defmt::info!("Battery Controller: Voltage is {} mV, State: {:?}", voltage, defmt::Debug2Format(&self.state))`).
*   **Command Receipts**: Log when a task receives an external command or interrupt event (e.g. `defmt::debug!("Received command Stop")`).

### 2. Standard Logging Macros
*   Use `defmt::info!` for general application lifecycle updates and telemetry.
*   Use `defmt::debug!` for verbose diagnostic checks (e.g., individual register states) to avoid spamming production logs.
*   Use `defmt::warn!` and `defmt::error!` to log hardware warnings, low voltage conditions, and thermal thresholds.

### 3. Debug Adapter formatting
When printing custom enums or structs that do not implement `defmt::Format` directly:
*   Derive `Debug` on the types and wrap them in `defmt::Debug2Format(&value)` to prevent compiler errors.

### 4. Profiling Blocking Peripheral Calls via Tracing
To identify latencies, stuck buses, or slow I/O polling, any potentially blocking operations (such as peripheral reads/writes, ADC polling, or I2C/SPI bus transactions) must be instrumented using `#[tracing::instrument]`:
*   **Imports**: Always use the consolidated tracing facade module `use crate::tracing;` (which re-exports `platform::tracing`). Do NOT import the standard `tracing` crate directly, as it requires `alloc` and is incompatible with our target `no_std` environments.
*   **Skip Self/Keyword Parameters**: When using `#[tracing::instrument]`, do NOT list `self` inside the `skip(...)` attribute. `self` is a keyword and causes macro parsing failures. The macro automatically ignores `self` during formatting anyway. Only list other parameters you want to exclude (e.g. channels or executors).
*   **Example**:
    ```rust
    use crate::tracing;

    #[tracing::instrument(level = "debug", skip(command_rx))]
    pub async fn run(&mut self, command_rx: Receiver) {
        // ...
    }
    ```
*   **Domain-Scoped Tracing Modes (`NONE`, `SENSOR`, `PERF`, and planned `AUDIO`)**:
    Tracing is compiled out by default (`NONE`). Target selection in `python tools/host.py` (`🎯 Target Config` scrollable dropdown or `--trace-mode`) passes mode-specific Cargo feature flags and embeds `FirmwareTraceMode` into `.program_metadata`:
    *   `NONE`: Standard build with zero runtime trace overhead.
    *   `PERF` (`--features trace-perf`): Enables all non-sensor `#[tracing::instrument]` execution spans, `Core 0/1 CPU Usage (%)`, and non-sensor `TelemetryRecord` trace events (excluding `#[tracing::instrument(domain = "sensor")]` and sensor telemetry reports). Opens externally in **Perfetto UI** (`ui.perfetto.dev`).
    *   `SENSOR` (`--features trace-sensors`): Enables real-time sensor telemetry reports and measured values only (`Proximity` mm + valid/invalid, `Thermal` mC, `Gesture` via `platform::tracing::trace_sensor_report`), with zero function-span overhead. Streams externally into **Rerun Viewer** (`rerun-sdk`).
    *   `AUDIO` (`--features trace-audio`, `[PLANNED]`): Reserved for I2S / codec streaming into **Rerun Viewer**.
    ```bash
    # Launch workstation in PERF or SENSOR trace mode:
    python tools/host.py --trace-mode PERF
    python tools/host.py --trace-mode SENSOR

    # Export buffered trace to .csv or Perfetto JSON (.perfetto.json) or open in Rerun from CLI:
    python tools/host.py trace export --format csv --output recordings/traces/session.csv
    python tools/host.py trace export --format perfetto --output recordings/traces/session.perfetto.json
    python tools/host.py trace open-rerun
    ```
*   **Standalone `host_cli` Span & CSV / Perfetto Export**:
    You can also capture directly via `host_cli --trace <FILE>` (`.perfetto.json` or `.csv`):
    ```bash
    cargo host-cli --elf target/thumbv6m-none-eabi/debug/cat_detector_shell --trace recordings/traces/trace.perfetto.json
    cargo host-cli --elf target/thumbv6m-none-eabi/debug/cat_detector_shell --trace recordings/traces/trace.csv
    ```

---

## Developer Workflows

### Formatting and Linting
To maintain code quality and style standards, run these checks before committing code:
*   **Format Check**:
    ```bash
    # Check formatting style (configured in .rustfmt.toml)
    cargo fmt --all --check
    ```
*   **Linting**:
    ```bash
    # Run clippy on host targets
    cargo clippy --all-targets -- -D warnings
    
    # Run clippy on embedded targets (specify lib and bins to skip std-based test harnesses)
    cargo clippy --package cat_detector --lib --bins --target thumbv6m-none-eabi -- -D warnings
    ```

### Creating New Firmware Binaries & Board Targets
All target binaries (`*_shell.rs`, `*_app.rs`) reside in the unified `app` crate (`package = "cat_detector"`) and share a declarative code-generation pipeline across `.cargo/config.toml`, `board/board.toml`, and `app/app.toml`:

1. **Configure Target Linker & `.program_metadata` Layout (`.cargo/config.toml` & `platform/`)**:
   - Register the target triple in [`.cargo/config.toml`](.cargo/config.toml) with `-Tlink.x`, `-Tdefmt.x`, and `-Tprogram_metadata.x` (e.g. `[target.thumbv6m-none-eabi]` for RP2040 and `[target.thumbv8m.main-none-eabihf]` for NXP MCX N947).
   - Provide `memory.x` (including a dedicated 4 KB `PROGRAM_METADATA` flash region) and `program_metadata.x` under `platform/src/<mcu>/layouts/` and stage them in `build.rs` so `tools/host_fs board-type` can read board identity in O(1) at a fixed flash address.
2. **Define the Board Support Package (`board/board.toml` & `board/src/<board>.rs`)**:
   - Add `[boards.<board>]` in [`board/board.toml`](board/board.toml) specifying `chip`, `board_type`, `program_metadata_addr`, `flash_base`, `flash_size`, `sram_size`, stack bounds, pins, buses, and non-overlapping `storage` / `metadata` / `fs` / `telemetry` partitions.
   - Generate `generated_<board>.rs` in [`board/build.rs`](board/build.rs) via `code_gen::generate_board_definitions(&content, "<board>")`.
   - Create `board/src/<board>.rs` starting with `include!(concat!(env!("OUT_DIR"), "/generated_<board>.rs"));` immediately after module doc comments, and implement `Board::init` for both `target` (`no_std` ARM) and `host` (mock drivers for `cargo nextest`).
3. **Declare Application Topology & Shell Config (`app/app.toml` & `app/src/<board>.rs`)**:
   - Add `[apps.<board>]` in [`app/app.toml`](app/app.toml) declaring channels, controllers, partitions, and `shell_config`.
   - Implement `<Board>ShellConfig` via `controller::impl_shell_config!`, `init_controllers(board, peripherals)`, `get_filesystem_config()`, and `declare_shell_pointers!`.
4. **Create & Register the Binary Entrypoint (`app/Cargo.toml` & `app/src/<name>_shell.rs`)**:
   - Register a unique `[[bin]]` entry in [`app/Cargo.toml`](app/Cargo.toml) (workspace binary names must be globally unique, enforced by `tools/build/build_firmware.sh`).
   - Structure shell binaries cleanly after [`app/src/cat_detector_shell.rs`](app/src/cat_detector_shell.rs) and [`app/src/carrier_board_shell.rs`](app/src/carrier_board_shell.rs):
     - Emit `.program_metadata` via `board::<board>::emit_project_metadata!();` (never hardcode flash addresses inline in binaries).
     - Wire `#[panic_handler]` to `app::handle_panic(info)`.
     - Declare CLI command dispatchers via `controller::declare_active_shell_commands!`.
     - Use `#[embassy_executor::main] async fn main(spawner: Spawner)` to run `app::Board::init().await`, construct `CliBuilder` over `platform::rtt::RttTxWriter`, print the startup banner, call `core_monitor::init_core(...)`, bind `app::declare_shell_pointers!(...)`, and enter `platform::run_rtt_shell_loop!`.
   - Declare `[package.metadata.binaries.<name>_shell]` (`board`, `chip`, `target`) in [`app/Cargo.toml`](app/Cargo.toml) so `tools/host.py` and `@device_test(target_binary="<name>_shell")` discover, compile, and flash it automatically without fallbacks.

### Running Tests
Validate all logic (including host-compatible board mocks) on the host. We use `cargo-nextest` for faster, parallel, and clean test execution:
```bash
# Run all tests using cargo-nextest (faster & cleaner)
cargo nextest run

# Alternatively, run standard cargo tests (e.g. for doctests)
cargo test
```

### Debugging Workflows

For interactive symbolic debugging of Python `@device_test` & host suites, Rust host unit/integration tests, JavaScript `node --test` suites, and bare-metal target binaries, accept the workspace extension recommendations in [`.vscode/extensions.json`](.vscode/extensions.json) and use the configurations in [`.vscode/launch.json`](.vscode/launch.json) and [`.vscode/settings.json`](.vscode/settings.json):

#### 1. Python Host & On-Device `@device_test` Symbolic Debugging (`debugpy`)
1. **VS Code Testing Sidebar & Editor Gutter**: `.vscode/settings.json` configures `python.testing.pytestArgs` (`-o addopts="-v --maxfail=1"`) across `tools/device_tests/suites`, `tools/tests`, and `tools/validation/tests` so clicking **Debug Test** next to any sync or `async def test_*` runs single-process under `debugpy` without `pytest-xdist` (`-n auto`) probe contention.
2. **Run and Debug (`Cmd+Shift+D` / `Ctrl+Shift+D`)**:
   - **`Python: Debug Current Test File (Pytest)`**: Debugs the currently open Python host or `@device_test` file with `justMyCode: false`.
   - **`Python: Debug Device Test (-k Filter)`**: Prompts for a `-k` keyword filter (e.g. `tof_proximity`) and runs `tools/host.py test -k <expr>` with automatic child-process `debugpy` attachment (`subProcess: true`).
   - **`Python: Debug Manual Bringup Suite (--manual)`**: Prompts for `cat_detector` or `carrier_board` and debugs interactive `--manual` bringup suites.
   - **`Python: Attach to host.py test --debug (Port 5678)`**: Attaches to a terminal run started with:
     ```bash
     python tools/host.py test -k tof_proximity --debug
     ```

#### 2. Rust Host Unit & Integration Test Debugging (`CodeLLDB`)
`[profile.test]` in `Cargo.toml` sets `opt-level = 0` and `debug = 2` so local variables are never optimized out when stepping through `<crate>/tests/*.rs`:
1. Ensure the **CodeLLDB** (`vadimcn.vscode-lldb`) and **rust-analyzer** extensions are installed, and keep `rust-analyzer.cargo.target` unset on the host so inline **`Run Test | Debug`** CodeLens buttons appear above every `#[test]`.
2. In **Run and Debug**, use **`Rust: Debug Tests in Current File (CodeLLDB)`**, **`Rust: Debug Single Test by Name (CodeLLDB)`**, or **`Rust: Debug Host Tool (host_cli / host_fs)`**.
3. To debug a test under `cargo-nextest` from the terminal, install `codelldb-launch` and run:
   ```bash
   cargo install --locked --git https://github.com/vadimcn/codelldb codelldb-launch
   cargo nextest run --debugger codelldb-launch <test_name>
   ```

#### 3. JavaScript ES Module Test Debugging (`node --test`)
Open any `tools/tests/js/*.test.mjs` test file or `tools/js/` ES module, set a breakpoint, and launch **`JS: Debug Current Test File (node --test)`**.

#### 4. Target Device Symbolic Debugging (Primary: `tools/host.py` · Secondary: `probe-rs`)
Target symbolic debugging runs natively on `probe-rs` across both Segger J-Link and CMSIS-DAP probes:
*   **Primary Interface — `tools/host.py` (`tools/host_debug` Dual-Core GDB + Live RTT)**:
    - Enable **Target GDB debug server (`host_debug`)** inside the **🎯 Target Config** modal in `host.py` (optionally enabling **Halt Core 0 at `Reset_Handler` after Sync / Reset** and custom **Core 0 / Core 1 GDB Ports** for multi-target benches), or start from the CLI via `python tools/host.py --debug-server` / `python tools/host.py debug start`.
    - In VS Code **Run and Debug**, attach directly while `host.py` continues streaming `defmt` logs and `shell>` commands over the `:50000` RTT bridge — `host.py` automatically synchronizes `target/active_target.elf` (`python tools/host.py debug sync-symbols` / `python tools/host.py debug target-info --json`) for both **`debug`** and **`release`** executables across all targets:
      - **Host Daemon: Attach Core 0 (GDB)**
      - **Host Daemon: Attach Core 1 (GDB)**
      - **Host Daemon: Attach Dual-Core (GDB)**
    - Supports breaking at **`Reset_Handler`** (Core 0) and **`Core1_Reset_Handler`** (Core 1) and stepping to any Rust LOC in Flash or SRAM on either core in both `debug` and `release` profiles.
*   **Secondary Interface — Standalone `probe-rs-debug` (`probe-rs.probe-rs-debug`)**:
    - Stop any background `host.py` daemon first (`python tools/host.py stop` or task **`host: stop host.py daemon (release USB probe)`**) and launch **probe-rs (Secondary): Debug Shell (RP2040)**, **probe-rs (Secondary): Debug Firmware (RP2040)**, or **probe-rs (Secondary): Debug Carrier Board Shell (MCXN947)** (selecting `debug` or `release` profile).

#### 5. Log Output Streaming and Interactive Console (`host_cli` / `cargo host-cli`)
We provide a host command-line utility, `host_cli` (aliased as `cargo host-cli`), to stream and decode plaintext `defmt` logs, emit structured NDJSON events (`--json-events`), and run an interactive diagnostic command console via RTT.
Build all host tools with:
```bash
cargo build-host-tools
```

*   **Default Run (Auto-detecting chip/channels and running logs + console)**:
    ```bash
    cargo host-cli --elf target/thumbv6m-none-eabi/debug/cat_detector_shell
    ```
*   **Via RTT (Specifying chip directly)**:
    ```bash
    cargo host-cli --chip rp2040 --elf target/thumbv6m-none-eabi/debug/cat_detector_app
    ```
*   **Concurrent Connection to an Active `host_debug` GDB Session (`--gdb-bridge`)**:
    When `host_debug` is active (via `python tools/host.py --debug-server` or `cargo host-debug`), attach `host_cli` to its non-halting `:50000` RTT memory bridge:
    ```bash
    cargo host-cli --gdb-bridge 127.0.0.1:50000 --elf target/thumbv6m-none-eabi/debug/cat_detector_shell
    ```
    This lets you stream system logs and interact with the CLI console concurrently while debugging, setting breakpoints, and stepping through code in VS Code.

    > [!NOTE]
    > If the debugger or connection drops, `host_cli` automatically attempts to reconnect in the background. On reconnection, it will attach directly to the running state without resetting the target CPU.

#### 6. Flashing the Interactive Bringup Shell
To build and flash the interactive bringup shell onto the target:
```bash
# Flash the shell binary directly to the microcontroller
probe-rs download target/thumbv6m-none-eabi/debug/cat_detector_shell --chip RP2040
```
Once flashed, connect to the target using `host_cli` (as documented in Section 5) to interact with the diagnostic console.

#### 7. Host Flash Filesystem Tool (`host_fs`)
We provide a host command-line utility, `host_fs`, to inspect, query, and decode flash memory contents from the microcontroller's sequential-storage partition.
First, build the tool:
```bash
cargo build -p host_fs --release
```

*   **Query directory files (`ls`)**:
    - *Direct connection*:
      ```bash
      cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/debug/cat_detector_app ls
      ```
    - *Offline dump file*:
      ```bash
      cargo run -p host_fs -- --dump flash_dump.bin ls
      ```
*   **Copy files to/from device (`cp`)**:
    - *Copy calibration file from device to host*:
      ```bash
      cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/release/cat_detector_app cp dev:vl53l0x_cal.cbor local_cal.cbor
      ```
    - *Copy new calibration config to device*:
      ```bash
      cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/release/cat_detector_app cp local_cal.bin dev:calibration.bin
      ```
*   **Export telemetry to CSV**:
    - *Direct connection*:
      ```bash
      cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/release/cat_detector_app export-telemetry telemetry.csv
      ```
    - *Offline dump file*:
      ```bash
      cargo run -p host_fs -- --dump flash_dump.bin export-telemetry telemetry.csv
      ```
*   **Decode crash dumps to symbolicated backtraces**:
    - *Direct connection*:
      ```bash
      cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/release/cat_detector_app crash-log
      ```
    - *Offline dump file*:
      ```bash
      cargo run -p host_fs -- --dump flash_dump.bin crash-log --elf target/thumbv6m-none-eabi/release/cat_detector_app
      ```

#### 8. Raw Flash Extraction Fallback
If you prefer to extract the raw binary flash memory partition manually from the target Pico using `probe-rs`:
```bash
probe-rs read b8 0x101C0000 262144 -f binary -o flash_dump.bin --chip RP2040
```

---

### Build and Verification Checks
We provide unified scripts and cargo aliases to make checking and verifying local changes fast and convenient:

#### 1. Complete Pre-Commit Verification
To verify code formatting, static tracing assertions, clippy rules, host unit/integration tests, and build checks before submitting code:
```bash
./tools/verify.py
```
This script executes formatting/tracing checks in parallel, runs target compilation checks, and compiles host tools in debug mode (reusing the test compiler cache) to complete all verification checks in under 30 seconds.

#### 2. Remote Cloud Execution (`tools/anvil` / Vast.ai)
For offloading heavy test suites, verification, and cargo builds to the Vast.ai cloud server to conserve local compute:
```bash
tools/anvil provision                # Rent Vast.ai Ubuntu instance and configure ~/.anvil.env
tools/anvil bootstrap                # Install build tools, Rust, Miniforge, and Conda on remote VM
tools/anvil status                   # Check instance health, IP/port, GPU, and lock state
tools/anvil sync                     # Sync repository to cloud container (/workspace/firmware)
tools/anvil verify                   # Run ./tools/verify.py remotely
tools/anvil pytest [args...]         # Run pytest remotely on anvil
tools/anvil cargo [args...]          # Run cargo checks/tests remotely on anvil
tools/anvil cli <command...>         # Reverse proxy host CLI command to local host.py daemon
tools/anvil pull <remote_path>       # Download generated artifacts/recordings
tools/anvil auto-release             # Release instance if idle or past Close of Business (COB)
tools/anvil release                  # Destroy rented instance and clean up ~/.anvil.env
tools/anvil stop                     # Clean up remote processes on anvil
```


#### 3. Cargo Check Aliases
If you want to quickly run compilation checks on demand, you can use these configured cargo aliases:
* **Check Host Crate Targets** (libraries, binaries, tests, and examples):
  ```bash
  cargo check-host
  ```
* **Check Target MCU Firmware**:
  ```bash
  cargo check-mcu
  ```

#### 4. Full MCU Target Build
To build the final target MCU firmware image manually:
```bash
./tools/build/build_firmware.sh
```

### Diagnostics and Telemetry Verification
When introducing or modifying telemetry records, filesystem files, or crash logs, developers must verify the changes locally:
1. **Model Updates**: Ensure telemetry fields are encoded/decoded correctly under CBOR size limits inside `model/src/telemetry_test.rs`.
2. **Offline Decoding**: Rebuild `host_fs` and check that telemetry parses into CSV:
   ```bash
   cargo run -p host_fs -- --dump <flash_dump.bin> export-telemetry <output.csv>
   ```
3. **Backtrace Validation**: Trigger a panic (e.g. via the shell `crash` command), extract the partition, and run symbolication with your debug ELF binary:
   ```bash
   cargo run -p host_fs -- --dump <flash_dump.bin> crash-log --elf target/thumbv6m-none-eabi/debug/cat_detector_app
   ```
   Verify that all frames resolve demangled function names, filenames, and correct source line numbers.
4. **Live Trace Workstation, Perfetto UI & Rerun Telemetry Visualization**:
   - Select **`PERF`** or **`SENSOR`** under **`PERFETTO / DEFMT TRACING MODE`** in `python tools/host.py` (`🎯 Target Config`) and click **⚡ Flash & Connect**, then open the **`TRACE`** header tab.
   - Use the hovering bottom trackbar above the slice/sample inspector to pause/resume (`Space`), clear, filter tracks, or zoom/pan (`W`/`+`, `S`/`-`, `A`/`←`, `D`/`→`, `F` to fit, or mouse scroll wheel).
   - Export directly to `.csv` or `.perfetto.json`, click **🚀 Open in Perfetto** (in `PERF` mode), or click **🚀 Open in Rerun** (in `SENSOR` / `AUDIO` modes, or run `python tools/host.py trace open-rerun`) to stream scalar time-series directly into Rerun via `rerun-sdk`.

---

## Interactive Xerxes Host Workstation, Bringup & Flash Matrix (`tools/host.py`)

The repository includes a unified, terminal-aesthetic developer workstation and background daemon (`tools/host.py` / `tools/host_daemon/`) designed to control physical target microcontrollers, stream real-time RTT `defmt` logging and interactive CLI shells, manage the sequential-storage flash filesystem, and execute hardware bringup verification checklists:

### 1. Launching the Host Workstation

```bash
# Launch interactive workstation (defaults to port 8899 in native desktop webview window)
python tools/host.py

# Launch using Eel (Chromium) or custom network interface
python tools/host.py --browser eel --port 8899
python tools/host.py --host 0.0.0.0

# Run daemon detached in the background
python tools/host.py --no-browser --detach

# Inspect daemon status or stop the background service
python tools/host.py status
python tools/host.py stop
```

### 2. Operational Workstation Modes

1. **CLI Console Mode (`shell>`)**:
   - Direct, bi-directional interaction with the target embedded shell over dual-channel Segger RTT.
   - Dispatch peripheral and controller commands directly (e.g. `motor speed 100`, `battery status`, `thermal status`, `gpio status`).
   - Built-in reverse-i-search (`Ctrl+R`) command history search and one-click clipboard copying.
2. **Filesystem Workstation Mode (`fs>`)**:
   - Direct host interaction with target sequential-storage flash partitions (`host_fs`).
   - Quick action triggers and manual commands: `ls`, `cat metadata`, `crash-log`, `export-telemetry`, and `format`.
   - Intelligent probe arbitration: automatically pauses background RTT streaming during filesystem flash accesses and cleanly restores streaming upon completion.
3. **Bringup & On-Device Suite Mode (`bringup>` / `host.py test`)**:
   - Automated and interactive hardware-in-the-loop pytest suites (`tools/device_tests/suites/test_cat_detector_bringup.py` and `tools/device_tests/suites/test_carrier_board_bringup.py`) built on top of `host.py`.
   - Automatic board identity discovery via `python tools/host.py fs board-type` (reading `.program_metadata` and `metadata` partitions in O(1) without scanning flash) with glob-based `@device_test(boards=...)` filtering and automatic `target_binary` flashing.
   - Multi-scenario support (`smoke`, `integration`, `bringup`) executed in strict top-to-bottom source-line order, plus interactive benchtop prompts (`Pass`, `Fail`, `Skip`) and Markdown/JUnit verification reports.

### 3. Integrated Hardware Controls & Diagnostic Reporting

* **Target Synchronization Matrix (`⚡ Sync`)**:
  - One-click compile, flash, and RTT reconnection matrix.
  - Automatically compiles target embedded binaries and host utilities (`host_cli`, `host_fs`), resolves target chips via ELF metadata, programs target flash via `probe-rs`, and restores streaming with progress telemetry and cancellation support.
* **Hardware Target Reset (`⚡ Reset`)**:
  - Issues hardware reset pulse over the attached SWD/JTAG debug probe and re-establishes RTT capture.
* **Automated Target Bug Reporting (`🐞 Target Bug`)**:
  - Defect reporting modal available directly in the workstation header.
  - Automatically captures and packages active RTT defmt session logs, target crash dumps (`host_fs crash-log`), telemetry records (`host_fs export-telemetry`), and filesystem metadata (`host_fs cat metadata`) into the Worm Defect Tracker (`attachments/<worm_id>/`, SQLite, and `feedback/WORM_<id>.md`).
* **UI Scale Selector & Keyboard Shortcuts**:
  - Global scaling selector (`75%` to `150%`) with `Cmd/Ctrl +=`, `Cmd/Ctrl +-`, and `Cmd/Ctrl +0` keyboard shortcuts and persistent `localStorage` across page reloads.

### 4. Host CLI Subcommands

All workstation operations can be performed directly from the command line:

```bash
# Forward shell commands to running target
python tools/host.py cli motor speed 100
python tools/host.py cli battery status

# Forward flash filesystem subcommands
python tools/host.py fs ls
python tools/host.py fs cat metadata
python tools/host.py fs crash-log

# On-device pytest suites, stress testing, & interactive symbolic debugging
python tools/host.py fs board-type
python tools/host.py test --list
python tools/host.py test --scenario smoke --board "rp2040*"
python tools/host.py test --scenario integration
python tools/host.py test -k "tof_proximity or battery"
python tools/host.py test --stress-runs 10
python tools/host.py test -k tof_proximity --debug
python tools/host.py test tools/device_tests/suites/test_cat_detector_bringup.py --manual
python tools/host.py test --script tools/device_tests/suites/test_carrier_board_bringup.py --manual

# Target GDB debug server, worm defect reporting, & daemon lifecycle controls
python tools/host.py debug status
python tools/host.py debug start --gdb-core0-port 3333 --gdb-core1-port 3334 --halt-on-reset
python tools/host.py debug stop
python tools/host.py bug "Motor stall under load" --severity HIGH --attach-crash-log
python tools/host.py sync
python tools/host.py flash
python tools/host.py reset
python tools/host.py status
python tools/host.py stop
```

---

## Unified VCS Workstation, Code Review & Worm Matrix (`tools/forge.py`)

The repository features an integrated web workstation and CLI suite (`tools/forge.py`) styled after the System Shock 2 - Xerxes terminal HUD aesthetic. It unifies commit diff inspection, code review audits, and defect (worm) tracking into a centralized local workflow.

### 1. Launching the Web Workstation

Launch the local workstation server in your browser:

```bash
# Launch interactive forge workstation (default port 8877)
python tools/forge.py

# Specify custom port, host, or browser mode
python tools/forge.py --port 8877 --browser vscode
python tools/forge.py --no-browser
```

#### Core Workstation View Modes
1. **Diff View (`/diff`)**:
   - Side-by-side and unified git diff visualization.
   - Interactive syntax highlighting and line wrapping toggle (`WRAP: ON` / `WRAP: OFF`).
   - Smartlog DAG commit graph navigation and unstaged working-tree delta inspection.
   - Global UI Scale Selector (`75%` to `150%`, shortcuts: `Cmd/Ctrl +=`, `Cmd/Ctrl +-`, `Cmd/Ctrl +0`).
2. **Code Review Audit (`/review`)**:
   - Granular, inline code commenting anchored to specific file paths, line ranges, and commit revisions.
   - Smartlog commit selection highlight styling with direct revision navigation.
   - Severity categorization: `MUST_FIX` (blockers), `PROPOSAL` (architecture design suggestions), and `NIT` (minor cosmetic cleanups).
   - Review progress tracking, resolution checklist verification, and overall verdict assignment (`IN_REVIEW`, `CHANGES_REQUESTED`, `APPROVED`).
3. **Worm Matrix (`/bugs`)**:
   - Defect tracking and bug lifecycle management with Xerxes terminal HUD styling.
   - Severity filtering (`CRITICAL`, `HIGH`, `MEDIUM`, `LOW`) and component classification (`FIRMWARE`, `CONTROLLER`, `DRIVER`, `PLATFORM`, `BOARD`, `MODEL`, `SHELL`, `INFRASTRUCTURE`, `UI`, `GENERAL`).
   - File attachment support: diagnostic logs, screenshots, and core dumps stored in `feedback/attachments/` or `attachments/<worm_id>/` and tracked via Git LFS.
   - Resolution tracking with formal engineering root cause and resolution notes.

---

### 2. Command-Line Interface (CLI Parity)

All workstation capabilities are accessible headless via CLI subcommands for automation, scripts, and terminal-centric workflows:

#### Code Review Management
```bash
# List all active code review comments
python tools/forge.py list-reviews

# List only open/unresolved review comments
python tools/forge.py list-reviews --open

# Add an inline review comment
python tools/forge.py add-comment --file app/carrier_board.md --line 42 --body "Use 1 Mb/s UART" --severity MUST_FIX

# Mark a review comment as resolved
python tools/forge.py resolve-comment <comment-uuid>

# Set the overall review verdict
python tools/forge.py set-verdict APPROVED
python tools/forge.py set-verdict CHANGES_REQUESTED
```

#### Worm (Defect) Tracking
```bash
# List all active worms
python tools/forge.py list-worms

# List only open worms
python tools/forge.py list-worms --open

# Quickly register a new defect
python tools/forge.py add-worm "Update Contributing.md with dashboard docs" \
  --severity MEDIUM \
  --category FIRMWARE \
  --component "CONTRIBUTING.md" \
  --description "Document dashboard CLI and workstation features"

# Mark a worm as resolved with resolution notes
python tools/forge.py resolve-worm WORM-008 --notes "Added comprehensive dashboard docs to CONTRIBUTING.md"
```

#### VCS & Smartlog Operations
```bash
# View smartlog DAG commit tree
python tools/forge.py list-commits

# Synchronize with upstream remote
python tools/forge.py sync

# Commit staged or untracked changes
python tools/forge.py commit -m "docs: update contributing guide"
```

---

### 3. Dual-Layer Persistence & Synchronization Protocol

The workstation employs a dual-layer storage architecture combining ACID transactional speed with human-readable markdown version control:

1. **SQLite Backing Stores (Single Source of Truth)**:
   - `target/code_review.sqlite`: Review comments, line mappings, file statuses, and review verdicts.
   - `target/worms.sqlite`: Defect records, severity classifications, and resolution logs.
2. **Bidirectional Markdown Synchronization (`feedback/`)**:
   - The server continuously monitors and synchronizes state with markdown reports in `feedback/`:
     - `feedback/CR_<commit>.md`: Revision-specific review findings with action item checkboxes (`- [ ]` / `- [x]`).
     - `feedback/WORM_<id>.md`: Granular defect tickets.
   - External edits to markdown checkboxes or headers are automatically detected via modification timestamps and merged safely into the SQLite database without losing resolution history.
3. **Git LFS Non-Confidential Attachments**:
   - All diagnostic attachments (screenshots, crash logs, traces) must reside in `feedback/attachments/`.
   - Tracked in **Git LFS** and considered public/non-confidential; never attach API tokens, keys, or private credentials.

---

### 4. Contribution Invariants & Pre-Commit Rules

When contributing changes, developers must adhere to the following workflow invariants:
- **Single-Bug Focus**: Investigate, reproduce, and resolve ONE worm at a time. Write a minimal failing test before editing code, apply the targeted fix, verify regression tests pass, and resolve the worm atomically.
- **Pre-Commit Script Execution**: Always execute `./tools/verify.py` from the repository root before committing. All tests, linters, and MCU target builds must pass with `Verification PASSED`.
- **User-Managed Pull Requests**: Autonomous PR creation (`gh pr create`) and merging (`gh pr merge`) are strictly prohibited; peer review and PR operations are executed manually by project maintainers.

---

## Configuration Schemas

The firmware uses two configuration files (`board.toml` and `app.toml`) to declaratively describe the hardware environment and the software controller task layout.

### 1. Board Configuration (`board/board.toml`)

Defines low-level chip parameters, stack sizes, pin configurations, serial buses, and memory partitions.

```toml
[boards.cat_detector]
chip = "rp2040"
flash_base = 0x10000000
flash_size = 0x00200000         # 2 MB
sram_size = 0x00042000          # 264 KB
core0_stack_size = 24576        # 24 KB
core1_stack_size = 16384        # 16 KB
stack_top = 0x20042000
core0_stack_bottom = 0x2003C000
core_monitor_timeout_ms = 10000
core_monitor_warn_pct = 80

[boards.cat_detector.pins]
I2C_SDA_PIN = 12
I2C_SCL_PIN = 13

[boards.cat_detector.buses.i2c]
sda = "I2C_SDA_PIN"
scl = "I2C_SCL_PIN"
frequency = 400000

[boards.cat_detector.hardware_resources]
TOF_NORTH_I2C_ADDR = { value = 0x30, type = "u8" }

[boards.cat_detector.partitions.fs]
start = 0x001C0000
end = 0x001D0000
```

---

### 2. Application Topology (`app/app.toml`)

Defines messaging channels, software controllers mapping to execution cores, and flash layout storage strategies.

```toml
[apps.cat_detector]
shell_config = "CatDetectorShellConfig"
cli_handlers = ["Battery", "Thermal", "Motor", "Sensor", "Fs", "System", "I2c", "Gpio", "Led", "Core"]

[apps.cat_detector.channels]
thermal = { type = "controller::ThermalChannel", capacity = 4 }
thermal_action = { type = "embassy_sync::channel::Channel<MutexRaw, controller::types::ThermalUpdateAction, 4>", val = "embassy_sync::channel::Channel::new()" }

[[apps.cat_detector.controllers]]
name = "Thermal"
instance = "controllers.core0.thermal"
generics = "app::TempSensorDevice"
# Optional overrides for shell pointers generation
# shell_field = "thermals"
# shell_type = "ThermalControllerType"
# shell_name = "default"
# ptr = "&mut controllers.core0.thermal"

[[apps.cat_detector.controllers]]
name = "Motor"
core = 1
instance = "controllers.core1.motor"
generics = "crate::MotorDevice, crate::CurrentSensorDevice"
device_shell_type = "MotorDevice"

[[apps.cat_detector.partitions]]
name = "panic"
board_partition = "fs"
kind = "Map"
```
