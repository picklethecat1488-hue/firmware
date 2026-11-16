# Embedded Rust Firmware & Host Bringup Workstation

Bare-metal `#![no_std]` Rust firmware powered by the **Embassy** asynchronous executor, portable host-testable domain controllers & `embedded-hal` BSPs, and interactive System Shock 2 – Xerxes engineering workstations (**Host Daemon** & **Forge**).

---

## Featured Demo: Interactive Firmware Tools Walkthrough

<div align="center">
  <a href="https://www.youtube.com/watch?v=-B0owhFyFh4">
    <img src="https://img.youtube.com/vi/-B0owhFyFh4/maxresdefault.jpg" alt="Watch the Firmware Tools Demo on YouTube" width="840" />
  </a>
  <p>
    <a href="https://www.youtube.com/watch?v=-B0owhFyFh4"><strong>▶ Watch the Firmware Tools Demo on YouTube</strong></a>
  </p>
</div>

The walkthrough showcases the end-to-end embedded firmware & hardware-in-the-loop bringup workflow:
- **Xerxes Host Workstation & Daemon (`tools/host.py`)**: One-click target compilation & SWD/JTAG flashing (`⚡ Sync`), real-time Segger RTT `defmt` telemetry streaming, interactive embedded CLI console (`shell>`), sequential-storage flash filesystem & symbolicated crash-dump inspection (`fs>`), automated YAML hardware bringup checklists (`bringup>`), and one-click **Target Bug** (`WORM-*`) diagnostic capture.
- **Forge Workstation (`tools/forge.py`)**: Unified **Smartlog DAG & Diff Viewer**, inline **Code Review**, and **Worm Defect Matrix** backed by dual-layer SQLite + detached-worktree Markdown synchronization and Git LFS attachments.

---

## Quick Start

```bash
conda env create -f environment.yml && conda activate firmware-env

python tools/host.py           # Launch interactive Xerxes Host Bringup, RTT & Flash Workstation
python tools/forge.py          # Launch VCS Diff / Smartlog / Code Review / Worm Matrix Workstation
./tools/verify.sh              # Run parallel Rust, AST, Oxlint JS, Node & Pytest verification
```

For toolchain setup, flashing commands, `cargo nextest`, and full CLI subcommands, see **[CLI Tools, Flashing, Host Daemon & Verification Workflows](docs/cli_and_workflows.md)**.

---

## Supported Hardware Targets

| Target Platform | MCU & Execution Model | Documentation |
| :--- | :--- | :--- |
| **[Cat Fountain / Cat Detector](docs/cat_fountain_architecture.md)** | **Raspberry Pi Pico (RP2040)** — Dual-core Cortex-M0+ proximity sensing, dual-ToF gesture detection, current-protected pump control, and flash telemetry. | [Architecture](docs/cat_fountain_architecture.md) · [Bringup Guide](app/cat_detector.md) ([`cat_fountain.md`](app/cat_fountain.md)) |
| **[Carrier Board 2.0](docs/carrier_board_architecture.md)** | **NXP MCX N947** — Dual-core Cortex-M33 AMP (PowerQuad DSP + eIQ Neutron NPU), capacitive touch gesture fusion, FlexSPI NAND filesystem, and BLE GATT. | [Architecture](docs/carrier_board_architecture.md) · [Bringup Guide](app/carrier_board.md) |

---

## Documentation Index

- **[CLI Tools, Flashing, Host Daemon & Verification Workflows](docs/cli_and_workflows.md)** — Rust/`probe-rs`/Conda setup, `cargo nextest`, target flashing, `tools/host.py`, `tools/forge.py`, and `./tools/verify.sh`.
- **[Unified Firmware Architecture Guide](docs/firmware_architecture.md)** — Layered workspace crate demarcation (`model`, `peripheral`, `controller`, `platform`, `board`, `app`) and runtime task topology.
- **[Microcontroller Decoupling & BSPs](docs/mcu_decoupling.md)** — `Board::init` hardware encapsulation, target-agnostic `embedded-hal` drivers, and host mock parity.
- **[Peripheral Sharing & Concurrency Patterns](docs/peripheral_sharing.md)** — Zero-allocation Embassy actor channels vs diagnostic shell interior mutability.
- **[Domain Controller Design, Task Runners & Codegen](docs/controller_design.md)** — `#[controller_context]` isolation and Rinja build-time code generation (`controllers.toml` / `shell.toml`).
- **[Embedded Logging, Tracing & Host Tools](docs/logging_tracing.md)** — Structured `defmt` telemetry, zero-alloc `#[tracing::instrument]` Perfetto spans, `host_cli`, and `host_fs`.
- **[JavaScript Style, Zero-Globals & Workstation UI Architecture](docs/javascript_style_architecture.md)** — Modular `tools/js/` ES modules for `host.py` & `forge.py`, strict `runtime_context.js` isolation, and parallel `node --test` suites.
- **[VCS Workstation, Code Review & Xerxes HUD Templates](docs/vcs_code_review.md)** — `tools/forge.py` architecture, Smartlog DAG, dual-layer SQLite/Markdown sync, and Git LFS worm attachments.
- **[Remote Cloud Runner (`anvil` / Vast.ai)](docs/remote_cloud_runner.md)** — Remote Rust/Python verification, cloud builds, and reverse-proxied local hardware CLI execution.
- **[Hardware-Firmware Co-Design](docs/hardware_firmware_codesign.md)** & **[Component Downselection Report](docs/downselection_report.md)** — Bare-metal Rust component qualification, register maps, and power/sensor trade studies.
- **[Contributing Guide](CONTRIBUTING.md)** — Coding conventions, `Cortex-Debug` / `probe-rs` debugging, compile-time RAM/flash profiling, and `board.toml` / `app.toml` schemas.
