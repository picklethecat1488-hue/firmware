# CLI Tools, Flashing, Host Daemon & Verification Workflows

This guide covers toolchain prerequisites, host-based unit testing, embedded target flashing, the **Xerxes Host Workstation & Daemon** (`tools/host.py`), **Forge** VCS/review/worm tracking (`tools/forge.py`), and pre-commit verification (`./tools/verify.sh`).

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

`tools/host.py` provides a System Shock 2 – Xerxes styled desktop/web workstation and background daemon for target synchronization (`⚡ Sync`), real-time RTT `defmt` log streaming, bidirectional embedded shell commands (`shell>`), sequential-storage flash filesystem management (`fs>`), automated YAML hardware bringup checklists (`bringup>`), and one-click **Target Bug** (`WORM-*`) diagnostic packaging:

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

# Parse and run YAML hardware bringup verification checklists
python tools/host.py bringup --list
python tools/host.py bringup --script app/cat_detector_bringup.yaml --run
python tools/host.py bringup --script app/carrier_board_bringup.yaml --step

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

Always run `./tools/verify.sh` before committing changes to execute parallel Rust formatting, host & target MCU Clippy checks, custom AST architectural validators (tracing hierarchy, `#[controller_context]`, multicore safety, `Debug` derives, constant addresses), Oxlint zero-globals JavaScript linting, parallel `node --test` ES module suites, Python `pytest` suites, and target/host debug binary builds:

```bash
# Complete local pre-commit verification
./tools/verify.sh

# Individual Python & JS test runners
node --test --test-concurrency=4 tools/tests/js/*.test.mjs
pytest tools/tests

# Offload verification, cargo, or pytest to the remote anvil cloud server
tools/anvil verify
tools/anvil cargo nextest run
tools/anvil pytest
```

See [Remote Cloud Runner (`anvil` / Vast.ai)](remote_cloud_runner.md) for remote provisioning and reverse-proxied hardware CLI execution (`tools/anvil cli ...`).
