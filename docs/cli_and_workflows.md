# CLI Tools, Flashing, Host Daemon & Verification Workflows

This guide covers toolchain prerequisites, host-based unit testing, embedded target flashing, the **Xerxes Host Workstation & Daemon** (`tools/host.py`), **Forge** VCS/review/worm tracking (`tools/forge.py`), and pre-commit verification (`./tools/verify.py`).

---

## 1. Prerequisites & Environment Setup

### Rust Toolchain & Debug Probe Utilities

Install the target Cortex-M toolchains and `probe-rs` for SWD/JTAG flashing, RTT `defmt` streaming, and debugging (see [Microcontroller Decoupling & BSPs](mcu_decoupling.md) for the step-by-step checklist on adding support for a new processor in `host.py` and `host_debug`):

```bash
rustup target add thumbv6m-none-eabi          # Cortex-M0+ (RP2040)
rustup target add thumbv8m.main-none-eabihf   # Cortex-M33 (NXP MCX N947)

cargo install probe-rs-tools
```

### Python Workstation & Bringup Conda Environment

Host daemon utilities, bringup runners, Forge VCS tools, and JavaScript/Python test suites run in the `firmware-env` Conda environment:

```bash
conda env create -f environment.yml
conda activate firmware-env
```

---

## 2. Interactive Xerxes Host Workstation & Daemon (`tools/host.py`)

`tools/host.py` provides a System Shock 2 – Xerxes styled desktop/web workstation and background daemon for target synchronization (`⚡ Sync`), real-time RTT `defmt` log streaming, bidirectional embedded shell commands (`CLI` mode), sequential-storage flash filesystem management (`FS` mode), interactive `@device_test` scanning/filtering/execution (`TEST` mode, superseding the legacy bringup UI), live Perfetto/Rerun timeline visualization & `.csv`/`.perfetto.json` export (`TRACE` mode with `NONE`, `SENSOR`, and `PERF` Target Config modes), and one-click **Target Bug** (`WORM-*`) diagnostic packaging:

```bash
# Launch interactive workstation (defaults to port 8899 in native desktop window)
python tools/host.py

# Launch with PERF (--features trace-perf -> Perfetto) or SENSOR (--features trace-sensors -> Rerun) tracing
python tools/host.py --trace-mode PERF
python tools/host.py --trace-mode SENSOR

# Inspect, switch, export (.csv / .perfetto.json), or stream live trace sessions to Rerun
python tools/host.py trace status
python tools/host.py trace mode PERF --sync
python tools/host.py trace export --format csv --output recordings/traces/session.csv
python tools/host.py trace export --format perfetto --output recordings/traces/session.perfetto.json
python tools/host.py trace open-rerun

# Launch with probe-rs dual-core GDB server (:3333/:3334) and :50000 RTT bridge enabled
python tools/host.py --debug-server --gdb-core0-port 3333 --gdb-core1-port 3334

# Run daemon in background / headless mode
python tools/host.py --no-browser --detach

# Control or inspect the live target GDB debug server (host_debug)
python tools/host.py debug status
python tools/host.py debug start --gdb-core0-port 3333 --gdb-core1-port 3334 --halt-on-reset
python tools/host.py debug stop

# File a structured target worm report bundling RTT/test logs, host_fs dumps, and attachments
python tools/host.py bug "ToF sensor NACK under motor load" --severity HIGH --category DRIVER \
  --attach-crash-log --attach-telemetry --attach ./scope_trace.png

# Forward shell commands directly to the running target over RTT
python tools/host.py cli motor speed 100
python tools/host.py cli battery status
python tools/host.py cli i2c scan
python tools/host.py cli i2c peek8 0x36 0x04
python tools/host.py cli i2c peek16 0x36 0x0c --bus i2c0
python tools/host.py cli i2c peek16 0x29 0x010f --reg16
python tools/host.py cli i2c poke8 0x36 0x0c 0x97
python tools/host.py cli i2c poke16 0x36 0x0c 0x971c --bus i2c0

# Inspect or manipulate the target sequential-storage flash filesystem & USB probe status
python tools/host.py fs ls
python tools/host.py fs cat metadata
python tools/host.py fs crash-log
cargo host-debug probe-status --json    # Non-resetting USB probe enumeration & in-use lock check

# Auto-discover board_type via host_fs and run async pytest device suites
python tools/host.py fs board-type
python tools/host.py test --list
python tools/host.py test --scenario smoke
python tools/host.py test --scenario integration
python tools/host.py test -k "tof_proximity or battery"
python tools/host.py test --stress-runs 10
python tools/host.py test --trace-mode PERF --stress-runs 10
python tools/host.py test --trace-mode ALL --stress-runs 10   # Iterates NONE -> SENSOR -> PERF across 10 cycles
python tools/host.py test -k tof_proximity --debug    # Wait for VS Code debugpy attach on 127.0.0.1:5678
python tools/host.py test tools/device_tests/suites/test_cat_detector_bringup.py --manual
python tools/host.py test --script tools/device_tests/suites/test_carrier_board_bringup.py --manual

# Trigger target compilation, flashing, hardware reset, or daemon teardown
python tools/host.py sync
python tools/host.py reset
python tools/host.py status
python tools/host.py stop
```

See [CONTRIBUTING.md](../CONTRIBUTING.md) and [Embedded Logging, Tracing & Host Tools](logging_tracing.md) for `host_cli`, `host_fs`, Perfetto span export, and GDB/Cortex-Debug attachment details.

---

## 3. Unified VCS Workstation, Code Review & Worm Matrix (`tools/forge.py`)

Launch **Forge** for Smartlog DAG navigation, side-by-side/unified commit diffs (`/`), inline Code Review (`/review`), and Worm defect tracking (`/bugs`):

```bash
# Launch interactive Forge workstation (default port 8877)
python tools/forge.py

# Headless CLI queries
python tools/forge.py list-commits
python tools/forge.py list-reviews --open
python tools/forge.py list-worms --open
```

See [VCS Workstation, Code Review & Xerxes HUD Templates](vcs_code_review.md) for dual-layer SQLite + `feedback/` synchronization details.

---

## 4. Building, Flashing & Host-Based Testing

### Host-Based Rust Validation

Domain models, state machines, controllers, and mocked BSPs compile and run natively on the host without physical hardware:

```bash
# Run parallel host tests using cargo-nextest (recommended)
cargo nextest run

# Or run standard cargo test
cargo test
```

### Building & Flashing Target Binaries

```bash
# RP2040 Cat Detector production application
cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_app

# RP2040 interactive diagnostic shell
cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell

# NXP MCX N947 Carrier Board diagnostic shell
cargo build --target thumbv8m.main-none-eabihf --package cat_detector --bin carrier_board_shell

# Build complete MCU firmware artifacts into target/out/
./tools/build/build_firmware.sh
```

---

## 5. Pre-Commit Verification & Cloud Execution

Always run `./tools/verify.py` before committing changes to execute parallel Rust formatting, host & target MCU Clippy checks, custom AST architectural validators (tracing hierarchy, `#[controller_context]`, multicore safety, `Debug` derives, constant addresses), Oxlint zero-globals JavaScript linting, parallel `node --test` ES module suites, Python `pytest` suites, and target/host debug binary builds:

```bash
# Complete local pre-commit verification
./tools/verify.py

# Individual Python & JS test runners
node --test --test-concurrency=4 tools/tests/js/*.test.mjs
pytest tools/tests

# Offload verification, cargo, or pytest to the remote anvil cloud server
tools/anvil verify
tools/anvil cargo nextest run
tools/anvil pytest
```

### Adding Host Test Resource Artifacts (`tools/tests/resources/`)

Host Python test suites (`tools/tests/`) keep dummy Cargo workspaces, sample `@device_test` modules, and mock `host_cli` / `host_debug` / `host_fs` executables as standalone files under `tools/tests/resources/` rather than embedding inline script strings:

1. **Directory & Manifest Fixtures (`tools/tests/resources/fake_workspace/`, `tools/tests/resources/suites/`)**: Add static `Cargo.toml` manifests or sample `@device_test` Python modules under `tools/tests/resources/` and materialize an isolated per-test copy in `tmp_path` via `_copy_resource_tree("<subdir>", tmp_path / "...")`.
2. **Executable Mock Tool Scripts (`tools/tests/resources/mock_*.py`, `mock_*.sh`)**: Place mock RTT/CLI/debug tool scripts in `tools/tests/resources/` and wrap them into `tmp_path` via `_make_resource_launcher(tmp_path / "mock_bin", "mock_<name>.py", env_overrides={"XERXES_MOCK_...": str(tmp_path / "out.json")})` so `pytest -n auto` workers remain completely isolated.
3. **Multi-Transport Target Interface Probing (`tools/host_daemon/model/transport.py`)**: `host.py` determines target availability without brittle stderr substring checks across `SWD_USB` (`cargo host-debug probe-status --json`), `USB_UART` (non-blocking POSIX `flock`), and `BLE` (GATT adapter lease callbacks). Test resource directories (`**/tests/resources/`) are excluded by default from CLI searches (`.ignore` for `ag`/`rg`), `pytest` discovery (`norecursedirs`), and IDE search/indexing (`.vscode/settings.json`).

See [Remote Cloud Runner (`anvil` / Vast.ai)](remote_cloud_runner.md) for remote provisioning and reverse-proxied hardware CLI execution (`tools/anvil cli ...`).

---

## 6. Interactive Symbolic Test & Target Debugging (VS Code)

The workspace `.vscode/` configuration ([.vscode/launch.json](../.vscode/launch.json), [.vscode/settings.json](../.vscode/settings.json), [.vscode/tasks.json](../.vscode/tasks.json), and [.vscode/extensions.json](../.vscode/extensions.json)) provides out-of-the-box source-level symbolic breakpoint debugging across **Python host & `@device_test` hardware-in-the-loop suites**, **Rust host unit/integration suites**, **JavaScript `node --test` suites**, and **bare-metal Cortex-M target firmware**.

### Recommended VS Code Extensions

Accept the workspace extension recommendations when opening the repository (or run `Extensions: Show Recommended Extensions`):
- `rust-lang.rust-analyzer` + `vadimcn.vscode-lldb` — Rust language server, inline `Run Test | Debug` CodeLens, and native DWARF symbolic debugging.
- `ms-python.python` + `ms-python.debugpy` — Pytest Test Explorer and async Python symbolic debugging.
- `marus25.cortex-debug` + `probe-rs.probe-rs-debug` — SWD/JTAG GDB & DAP target debugging with live `defmt` / RTT consoles.

### 1. Python Host & On-Device `@device_test` Symbolic Debugging (`debugpy`)

`.vscode/settings.json` configures the VS Code **Testing** sidebar (`pytestArgs: ["-o", "addopts=-v --maxfail=1", ...]`) to override `pyproject.toml`'s `-n auto` `pytest-xdist` multi-worker flag when running inside the IDE so single-process breakpoints and single-probe hardware tests never collide:

- **Testing Sidebar / Gutter Icons**: Open any suite under `tools/device_tests/suites/*.py` or `tools/tests/*.py`, set a breakpoint on any line (including `async def test_*` coroutines), and click **Debug Test** in the editor gutter.
- **Run & Debug (`Cmd+Shift+D` / `Ctrl+Shift+D`) Launch Profiles**:
  - **`Python: Debug Current Test File (Pytest)`** — Runs the active Python test file in-process with `justMyCode: false` so you can step seamlessly from `@device_test` functions into `AsyncHostSessionClient` and `DaemonRunner`.
  - **`Python: Debug Device Test (-k Filter)`** — Prompts for a `-k` keyword expression (e.g., `tof_proximity` or `battery and not alert`) and launches `tools/host.py test -k <expr>` with `subProcess: true`.
  - **`Python: Debug Manual Bringup Suite (--manual)`** — Prompts for `cat_detector` or `carrier_board` and debugs interactive benchtop bringup tests.
  - **`Python: Attach to host.py test --debug (Port 5678)`** — Attaches to a terminal session started with:
    ```bash
    python tools/host.py test -k tof_proximity --debug
    ```
  - **Live Hardware Inspection While Paused**: When paused at a breakpoint inside an `async def test_*(host)` function, inspect `host.last_output_log` in **Variables** or evaluate live target commands directly in the **Debug Console** (e.g., `await host.cli("gpio status")`).

### 2. Rust Host Unit & Integration Test Debugging (`CodeLLDB`) & Cargo Tool Aliases

`.cargo/config.toml` defines convenient aliases for all Rust host utilities (`cargo host-cli`, `cargo host-fs`, `cargo host-debug`, `cargo code-gen`, `cargo build-host-tools`), and `[profile.test]` in `Cargo.toml` sets `opt-level = 0` and `debug = 2` (while preserving `opt-level = "s"` on `[profile.dev]` for MCU flash budgets) so local variables are never optimized out during host test stepping:

- **Inline CodeLens**: Open any `<crate>/tests/*.rs` integration test file and click **`Debug`** directly above `#[test]`.
- **Run & Debug Launch Profiles**:
  - **`Rust: Debug Tests in Current File (CodeLLDB)`** — Compiles and launches the active `<crate>/tests/<name>.rs` binary under `CodeLLDB` with `--nocapture`.
  - **`Rust: Debug Single Test by Name (CodeLLDB)`** — Prompts for a test function filter in the active `<crate>/tests/*.rs` file.
  - **`Rust: Debug Host Tool (host_cli / host_fs / host_debug)`** — Builds and symbolically debugs `host_cli`, `host_fs`, `host_debug`, or `code_gen`.
- **Terminal `cargo nextest` + `CodeLLDB`**:
  ```bash
  cargo nextest run --debugger codelldb-launch <test_name>
  ```

### 3. JavaScript ES Module Test Debugging (`node --test`)

Open any suite in `tools/tests/js/*.test.mjs` (or module under `tools/js/`), set a breakpoint, and launch **`JS: Debug Current Test File (node --test)`** to step through zero-globals workstation controllers with full call-stack and closure inspection.

### 4. Bare-Metal Target Firmware Symbolic Debugging (Primary: `tools/host.py` · Secondary: `probe-rs`)

Target firmware debugging uses a unified `probe-rs` architecture across all supported SWD/JTAG probes:

1. **Primary Interface — `tools/host.py` (`tools/host_debug` Dual-Core GDB + Concurrent RTT Console)**:
   - Open **🎯 Target Config** in the `host.py` workstation and check **Enable target GDB debug server on Sync (`host_debug`)** (optionally check **Halt Core 0 at `Reset_Handler` after Sync / Reset** and set explicit **Core 0 / Core 1 GDB Ports** for multi-target benches), or launch/toggle via CLI (`python tools/host.py --debug-server` or `python tools/host.py debug start`).
   - On `⚡ Sync`, `host.py` flashes the target and launches `host_debug` in a **single reset** while keeping `defmt` logs and interactive `shell>` commands live over the non-halting `:50000` RTT memory bridge (`cargo host-cli --gdb-bridge 127.0.0.1:50000 --json-events`). See [Host-Target RTT Handshaking Protocols](logging_tracing.md#4-host-target-rtt-handshaking-protocols) for full details on pre-reset `_SEGGER_RTT` wiping, `TargetBootState`, `ShellIdle` epochs, and `:50000` GDB step-priority yielding.
   - In VS Code **Run & Debug**, attach without stopping `host.py` — both **`debug`** and **`release`** builds across all boards/binaries (`cat_detector_shell`, `cat_detector_app`, `carrier_board_shell`) are resolved automatically from `host.py` via `target/active_target.elf` (`python tools/host.py debug sync-symbols` / `python tools/host.py debug target-info --json`):
     - **`Host Daemon: Attach Core 0 (GDB)`**
     - **`Host Daemon: Attach Core 1 (GDB)`**
     - **`Host Daemon: Attach Dual-Core (GDB)`** (attaches Core 0 and Core 1 simultaneously)
   - Clicking **Restart** in VS Code resets Core 0 to `Reset_Handler`, flushes GDB's register cache, and automatically continues directly to your active breakpoints (`runToEntryPoint: ""`, `postRestartSessionCommands: ["-exec-continue"]`). To stress-test or benchmark repeated restart + break-at-`main` + 10x single-step cycles from the CLI, run `python tools/scripts/repro_gdb_break_and_step.py --iterations 5 --steps 10` (raw MI2 packet logs are written to `target/logs/repro_gdb_break_and_step.log`).
   - Supports setting breakpoints from **`Reset_Handler`** (Core 0) and **`Core1_Reset_Handler`** (Core 1) to any Rust source LOC in Flash or SRAM across both Segger J-Link and CMSIS-DAP probes, and stress-testing on-device suites with or without symbolic debugging via `python tools/host.py test --debug-server` / `--no-debug-server`.
2. **Secondary Interface — Standalone `probe-rs` (`probe-rs-debug`)**:
   - When debugging standalone without a running `host.py` daemon, stop `host.py` (`python tools/host.py stop` or VS Code task **`host: stop host.py daemon (release USB probe)`**) and launch **`probe-rs (Secondary): Debug Shell (RP2040)`**, **`probe-rs (Secondary): Debug Firmware (RP2040)`**, or **`probe-rs (Secondary): Debug Carrier Board Shell (MCXN947)`** (prompting for `debug` or `release` profile).
