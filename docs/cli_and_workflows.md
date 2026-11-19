# CLI Tools, Flashing, Host Daemon & Verification Workflows

This guide covers toolchain prerequisites, host-based unit testing, embedded target flashing, the **Xerxes Host Workstation & Daemon** (`tools/host.py`), **Forge** VCS/review/worm tracking (`tools/forge.py`), and pre-commit verification (`./tools/verify.py`).

---

## 1. Prerequisites & Environment Setup

### Rust Toolchain & Debug Probe Utilities

Install the target Cortex-M toolchains and `probe-rs` for SWD/JTAG flashing, RTT `defmt` streaming, and debugging:

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

`tools/host.py` provides a System Shock 2 – Xerxes styled desktop/web workstation and background daemon for target synchronization (`⚡ Sync`), real-time RTT `defmt` log streaming, bidirectional embedded shell commands (`CLI` mode), sequential-storage flash filesystem management (`FS` mode), interactive `@device_test` scanning/filtering/execution (`TEST` mode, superseding the legacy bringup UI), and one-click **Target Bug** (`WORM-*`) diagnostic packaging:

```bash
# Launch interactive workstation (defaults to port 8899 in native desktop window)
python tools/host.py

# Run daemon in background / headless mode
python tools/host.py --no-browser --detach

# Forward shell commands directly to the running target over RTT
python tools/host.py cli motor speed 100
python tools/host.py cli battery status

# Inspect or manipulate the target sequential-storage flash filesystem
python tools/host.py fs ls
python tools/host.py fs cat metadata
python tools/host.py fs crash-log

# Auto-discover board_type via host_fs and run async pytest device suites
python tools/host.py fs board-type
python tools/host.py test --list
python tools/host.py test --scenario smoke
python tools/host.py test --scenario integration
python tools/host.py test -k "tof_proximity or battery"
python tools/host.py test --stress-runs 10
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

### 2. Rust Host Unit & Integration Test Debugging (`CodeLLDB`)

`[profile.test]` in `Cargo.toml` sets `opt-level = 0` and `debug = 2` (while preserving `opt-level = "s"` on `[profile.dev]` for MCU flash budgets) so local variables are never optimized out during host test stepping:

- **Inline CodeLens**: Open any `<crate>/tests/*.rs` integration test file and click **`Debug`** directly above `#[test]`.
- **Run & Debug Launch Profiles**:
  - **`Rust: Debug Tests in Current File (CodeLLDB)`** — Compiles and launches the active `<crate>/tests/<name>.rs` binary under `CodeLLDB` with `--nocapture`.
  - **`Rust: Debug Single Test by Name (CodeLLDB)`** — Prompts for a test function filter in the active `<crate>/tests/*.rs` file.
  - **`Rust: Debug Host Tool (host_cli / host_fs)`** — Builds and symbolically debugs `host_cli`, `host_fs`, or `code_gen`.
- **Terminal `cargo nextest` + `CodeLLDB`**:
  ```bash
  cargo nextest run --debugger codelldb-launch <test_name>
  ```

### 3. JavaScript ES Module Test Debugging (`node --test`)

Open any suite in `tools/tests/js/*.test.mjs` (or module under `tools/js/`), set a breakpoint, and launch **`JS: Debug Current Test File (node --test)`** to step through zero-globals workstation controllers with full call-stack and closure inspection.

### 4. Bare-Metal Target Firmware Symbolic Debugging (`probe-rs` & `Cortex-Debug`)

Because a USB SWD probe can only be claimed over raw USB by one host process at a time, stop any background `host.py` daemon first (`python tools/host.py stop` or VS Code task **`host: stop host.py daemon (release USB probe)`**):

- **`probe-rs-debug` (Integrated `defmt` + `shell>` RTT)**:
  - **`Debug Shell (probe-rs RP2040)`**, **`Debug Firmware (probe-rs RP2040)`**, and **`Debug Carrier Board Shell (probe-rs MCXN947)`** automatically run their `cargo build` `preLaunchTask`, flash the debug ELF, halt at reset/`main`, decode RTT Channel 0 (`Defmt`), and expose RTT Channel 1 (`String` `shell>`) in VS Code.
- **`Cortex-Debug` (Segger J-Link & CMSIS-DAP Pico Probe + Dual-Core Compounds)**:
  - Launch single-core profiles (**`Debug Shell Core 0 (J-Link)`**, **`Debug App Core 0 (J-Link)`**, **`Debug Carrier Board Shell (J-Link MCXN947)`**, **`Debug Shell (Cortex-Debug Pico Probe)`**) or dual-core compounds (**`Debug Shell Dual-Core (J-Link)`**, **`Debug App Dual-Core (J-Link)`**).
  - `sourceFileMap` (`"firmware": "${workspaceFolder}"`) automatically maps DWARF paths even if an ELF was compiled via `./tools/build/build_firmware.sh` or `./tools/verify.py`.
  - Under OpenOCD (`Pico Probe`), you can concurrently attach `host_cli` to the active GDB server TCP port to send CLI commands that trigger Rust firmware breakpoints:
    ```bash
    cargo run -p host_cli -- -o localhost:50000 --elf target/thumbv6m-none-eabi/debug/cat_detector_shell
    ```
