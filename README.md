# Embedded Rust Firmware & Host Bringup Workstation

Bare-metal `#![no_std]` Rust firmware powered by the **Embassy** asynchronous executor, portable host-testable domain controllers & `embedded-hal` BSPs, and interactive System Shock 2 – Xerxes engineering workstations (**Host Daemon** & **Forge**).

---

## Featured Demos

<div align="center">
  <table>
    <tr>
      <td align="center" width="50%">
        <a href="https://www.youtube.com/watch?v=P2hhQMJvClM">
          <img src="https://img.youtube.com/vi/P2hhQMJvClM/maxresdefault.jpg" alt="Watch the Live RP2040 Bench, IDE Single-Stepping & 30fps Tracing Demo on YouTube" width="410" />
        </a>
        <br />
        <a href="https://www.youtube.com/watch?v=P2hhQMJvClM"><strong>▶ Live RP2040 Bench: IDE Single-Stepping &amp; 30fps Tracing</strong></a>
      </td>
      <td align="center" width="50%">
        <a href="https://www.youtube.com/watch?v=-B0owhFyFh4">
          <img src="https://img.youtube.com/vi/-B0owhFyFh4/maxresdefault.jpg" alt="Watch the Firmware &amp; VCS Workstations Walkthrough on YouTube" width="410" />
        </a>
        <br />
        <a href="https://www.youtube.com/watch?v=-B0owhFyFh4"><strong>▶ Firmware &amp; Forge VCS Workstations Walkthrough</strong></a>
      </td>
    </tr>
  </table>
</div>

The walkthroughs showcase the end-to-end embedded firmware, live-silicon debugging, and hardware-in-the-loop bringup workflow:
- **Live Dual-Core GDB Single-Stepping & Unified Probe (`tools/host_debug`)**: Source-level VS Code `Cortex-Debug` single-stepping, hardware breakpoints, live local/register inspection, and RP2040 `TIMER.DBGPAUSE` synchronization on a connected target while `host.py` maintains an active `:50000` Segger RTT console over a single USB debug probe.
- **30fps Real-Time `PERF` & `SENSOR` Tracing (`tools/host.py`)**: `requestAnimationFrame` multi-track swimlanes with a hovering playback/zoom/filter trackbar, continuous integrated `Core 0` & `Core 1` CPU usage curves (`PERF` → **Perfetto**), live ToF proximity & thermal curves (`SENSOR` → **Rerun**), and an **11-byte** `.program_metadata` flash header with deferred `.elf` string & partition loading.
- **Xerxes Host & Forge Workstations (`tools/host.py`, `tools/forge.py`)**: One-click target compilation & SWD/JTAG flashing (`⚡ Sync`), stacked `@device_test` hardware test runner, sequential-storage flash filesystem & symbolicated crash-dump inspection (`fs>`), **Target Bug** (`WORM-*`) capture, and **Smartlog DAG / Diff / Code Review / Worm Matrix** views.

---

## Quick Start

```bash
conda env create -f environment.yml && conda activate firmware-env

python tools/host.py           # Launch interactive Xerxes Host Bringup, RTT & Flash Workstation
python tools/forge.py          # Launch VCS Diff / Smartlog / Code Review / Worm Matrix Workstation
./tools/verify.py              # Run parallel Rust, AST, Oxlint JS, Node & Pytest verification
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

- **[CLI Tools, Flashing, Host Daemon & Verification Workflows](docs/cli_and_workflows.md)** — Rust/`probe-rs`/Conda setup, `cargo nextest`, target flashing, `tools/host.py`, `tools/forge.py`, `./tools/verify.py`, and VS Code interactive symbolic test & target debugging.
- **[Unified Firmware Architecture Guide](docs/firmware_architecture.md)** — Layered workspace crate demarcation (`model`, `peripheral`, `controller`, `platform`, `board`, `app`) and runtime task topology.
- **[Microcontroller Decoupling & BSPs](docs/mcu_decoupling.md)** — `Board::init` hardware encapsulation, target-agnostic `embedded-hal` drivers, and host mock parity.
- **[Peripheral Sharing & Concurrency Patterns](docs/peripheral_sharing.md)** — Zero-allocation Embassy actor channels vs diagnostic shell interior mutability.
- **[Domain Controller Design, Task Runners & Codegen](docs/controller_design.md)** — `#[controller_context]` isolation and Rinja build-time code generation (`controllers.toml` / `shell.toml`).
- **[Embedded Logging, Tracing & Host Tools](docs/logging_tracing.md)** — Structured `defmt` telemetry, `PERF` (Perfetto) & `SENSOR` (Rerun) tracing modes, `host_debug`, `host_cli`, and `host_fs`.
- **[JavaScript Style, Zero-Globals & Workstation UI Architecture](docs/javascript_style_architecture.md)** — Modular `tools/js/` ES modules for `host.py` & `forge.py`, strict `runtime_context.js` isolation, and parallel `node --test` suites.
- **[VCS Workstation, Code Review & Xerxes HUD Templates](docs/vcs_code_review.md)** — `tools/forge.py` architecture, Smartlog DAG, dual-layer SQLite/Markdown sync, and Git LFS worm attachments.
- **[Remote Cloud Runner (`anvil` / Vast.ai)](docs/remote_cloud_runner.md)** — Remote Rust/Python verification, cloud builds, and reverse-proxied local hardware CLI execution.
- **[Hardware-Firmware Co-Design](docs/hardware_firmware_codesign.md)** & **[Component Downselection Report](docs/downselection_report.md)** — Bare-metal Rust component qualification, register maps, and power/sensor trade studies.
- **[Contributing Guide](CONTRIBUTING.md)** — Coding conventions, `Cortex-Debug` / `probe-rs` debugging, compile-time RAM/flash profiling, and `board.toml` / `app.toml` schemas.
