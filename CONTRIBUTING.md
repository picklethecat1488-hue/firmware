# Contributing to Firmware

Welcome! This document outlines the architectural standards, code patterns, and onboarding guidelines for contributing to this firmware repository.

---

## Architectural Guidelines

To maintain testability, reliability, and modularity, follow these core principles:

### 1. File Structure Standards
*   **Separation of Source Files**: Avoid grouping multiple interfaces and implementations into a single `lib.rs`. Keep each module in its own Rust listing (e.g., `model/src/state_machine.rs`). Expose them cleanly in `lib.rs` (e.g., `pub mod state_machine;`).
*   **Test Isolation**: Keep tests completely isolated from implementation code. Do not mix unit tests inside the main module source files. All unit tests must be separated from module code and stored in a `tests/` subfolder at the crate root level (e.g., `model/tests/state_machine_tests.rs` or `peripherals/tests/integration.rs`).

### 2. Microcontroller Decoupling (Canonical Peripherals)
To make peripheral and controller implementations target-independent, we use **Generics and Traits** via the **`embedded-hal`** (v1.0.0) ecosystem:
*   Define peripheral traits (e.g., `Pump` or `WaterSensor`) in `peripherals/src/`.
*   Implement generic, platform-agnostic wrappers over `embedded-hal` traits (e.g., `GpioPump<P: OutputPin>`).
*   In the target-specific package (e.g., `projects/cat_detector/`), instantiate the microcontroller's HAL-specific pin (which implements the relevant `embedded-hal` trait) and pass it to your generic peripheral wrapper.
*   This pattern ensures that we can easily write `mock` implementations of peripherals for host-based testing.

### 3. Separation of Concerns
*   **Domain Logic (`model/`)**: Pure states and traits interfaces. Target-agnostic.
*   **Peripheral Interfaces (`peripherals/`)**: `embedded-hal` generic implementations of peripheral traits.
*   **Control Loop Coordinators (`controller/`)**: Project-agnostic orchestrators. Connects peripherals and models together.
*   **Projects (`projects/`)**: Binds microcontroller HAL pins/peripherals to the generic wrappers and implements system behavior.

### 4. Platform-Specific Naming Conventions
*   **Peripherals & Controllers**: Do **not** prefix files or structs with microcontroller model numbers (e.g., do not name them `rp2040_pump.rs`). Since they only interact with generic `embedded-hal` traits, they are completely platform-independent and compile on any target (including the host).
*   **Target Projects**: If a project requires custom initialization files or custom drivers that cannot be represented by generic `embedded-hal` traits, they should live in the specific target's project directory (e.g., `projects/cat_detector/`) and can use model numbers or board suffixes (e.g., `rp2040_board.rs`).
*   **Conditional Compilation**: If a generic crate must contain MCU-specific code, use feature flags (e.g., `#[cfg(feature = "rp2040")]`) to toggle the compilation of platform-specific modules in a structured way.

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

### 4. Profiling Blocking Peripheral Calls via `#[instrument]`
To identify latencies, stuck buses, or slow I/O polling, any potentially blocking operations (such as peripheral reads/writes, ADC polling, or I2C/SPI bus transactions) must be instrumented using `tracing`'s `#[instrument]` macro:
*   **Purpose**: Automatically registers a trace span upon function entry/exit, collecting argument parameters and allowing tracing subscribers (such as `defmt-tracing`) to calculate execution elapsed time.
*   **Example**:
    ```rust
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn read_voltage_mv(&mut self) -> Result<u32, Error> {
        // Potentially blocking I/O/ADC read
    }
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

### Creating a New Project
1.  Create a folder under `projects/` (e.g., `projects/my_project`).
2.  Add a `Cargo.toml` and configure target settings in `.cargo/config.toml` and `memory.x`.
3.  Implement target-specific pins, configure peripherals, and invoke your controllers.
4.  Link the new project in the root [Cargo.toml](file:///Users/daparker/gh/firmware/Cargo.toml) workspace members list.

### Running Tests
Validate all logic (including host-compatible board mocks) on the host. We use `cargo-nextest` for faster, parallel, and clean test execution:
```bash
# Run all tests using cargo-nextest (faster & cleaner)
cargo nextest run

# Alternatively, run standard cargo tests (e.g. for doctests)
cargo test
```

### Debugging Workflows

For interactive debugging of tests on the host and bare-metal binaries on the target device, configure VS Code as follows:

#### 1. Host Test Debugging
For the most reliable breakpoint debugging experience using the VS Code **"Debug"** CodeLens buttons:
1. Ensure the **CodeLLDB** extension is installed in VS Code.
2. Ensure you do **not** override the test run command in `.vscode/settings.json`. By letting VS Code default to standard `cargo test` when debugging, CodeLLDB attaches directly to the test process, allowing your breakpoints to be hit out of the box.
3. If you specifically want to debug a test under nextest using the terminal, install the `codelldb-launch` helper and run:
   ```bash
   cargo install --locked --git https://github.com/vadimcn/codelldb codelldb-launch
   cargo nextest run --debugger codelldb-launch <test_name>
   ```

#### 2. Target Device Debugging (probe-rs & Cortex-Debug)
To flash and debug firmware binaries directly on the target RP2040 microcontroller, use the VS Code **Run and Debug** view (`Ctrl+Shift+D` / `Cmd+Shift+D`) and select one of the following configurations:
*   **Cortex-Debug (Recommended - Highly Stable)**: Uses GDB connected to a J-Link server or OpenOCD in the background. It is highly robust at resolving source files on your local disk.
    - **Debug App (Cortex-Debug J-Link)**: Debugs the main `cat_detector_app` using a J-Link probe.
    - **Debug Shell (Cortex-Debug J-Link)**: Debugs the diagnostic `cat_detector_shell` bringup utility using a J-Link probe.
    - **Debug App (Cortex-Debug Pico Probe)**: Debugs the main `cat_detector_app` using a Raspberry Pi Debug Probe (CMSIS-DAP over OpenOCD).
    - **Debug Shell (Cortex-Debug Pico Probe)**: Debugs the diagnostic `cat_detector_shell` bringup utility using a Raspberry Pi Debug Probe (CMSIS-DAP over OpenOCD).
*   **probe-rs-debug (Experimental)**: Uses the `Debugger for probe-rs` extension.
    - **Debug Firmware (probe-rs)**: Flashes and debugs the main `cat_detector_app`.
    - **Debug Shell (probe-rs)**: Flashes and debugs the diagnostic `cat_detector_shell` bringup utility.
Code execution will automatically halt at the entry point (`main`), allowing you to step through hardware initialization.

#### 3. Log Output Streaming and Interactive Console (host_cli)
We provide a host command-line utility, `host_cli`, to stream and decode plaintext `defmt` logs and run an interactive diagnostic command console via RTT.
First, build the tool:
```bash
cargo build -p host_cli --release
```

*   **Default Run (Auto-detecting chip/channels and running logs + console)**:
    ```bash
    cargo run -p host_cli -- --elf target/thumbv6m-none-eabi/debug/cat_detector_shell
    ```
*   **Via RTT (Specifying chip directly)**:
    ```bash
    cargo run -p host_cli -- --chip rp2040 --elf target/thumbv6m-none-eabi/debug/cat_detector_app
    ```
*   **Via RTT using `pico-debug` (Bypassing multidrop scan)**:
    ```bash
    cargo run -p host_cli -- --chip Cortex-M0+ --elf target/thumbv6m-none-eabi/debug/cat_detector_app
    ```

#### 4. Flashing the Interactive Bringup Shell
To build and flash the interactive bringup shell onto the target:
```bash
# Flash the shell binary directly to the microcontroller
probe-rs download target/thumbv6m-none-eabi/debug/cat_detector_shell --chip RP2040
```
Once flashed, connect to the target using `host_cli` (as documented in Section 3) to interact with the diagnostic console.

#### 5. Host Flash Filesystem Tool (`host_fs`)
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
    - *Copy telemetry from device to host*:
      ```bash
      cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/release/cat_detector_app cp dev:telemetry.rrd local_telemetry.rrd
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

#### 6. Raw Flash Extraction Fallback
If you prefer to extract the raw binary flash memory partition manually from the target Pico using `probe-rs`:
```bash
probe-rs read-mem --chip RP2040 0x101C0000 262144 flash_dump.bin
```

---

### Build Checks
Check target compilation via:
```bash
cargo build --package cat_detector --target thumbv6m-none-eabi
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
