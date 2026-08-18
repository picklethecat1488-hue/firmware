---
name: cat-fountain-development
description: >-
  Use this skill to run code generation, build/flash firmware, execute target bringup,
  stream logs, and interact with the sequential-storage flash filesystem.
---

# Cat Fountain Development & Diagnostic Runbook

Use this guide to run codebase tasks, flashing procedures, interactive bringup steps, and offline flash log decoders.

## 1. Code Generation
If you need to view or regenerate controllers, channels, or CLI command routers:
* **Controllers**: [controller/controllers.toml](file:///Users/daparker/gh/firmware/controller/controllers.toml)
* **CLIs**: [shell.toml](file:///Users/daparker/gh/firmware/shell.toml)
* Run the host generator utility to list controllers/CLIs or generate skeletons:
  ```bash
  cargo run -p code_gen -- list-controllers
  cargo run -p code_gen -- list-clis
  cargo run -p code_gen -- cli-sample Motor
  ```

## 2. Host Verification & Flashing
Before flashing, ensure the host environment builds successfully and checks pass.
* Run verification:
  ```bash
  ./tools/verify.sh
  ```
* Flashing the diagnostic shell:
  ```bash
  probe-rs download target/thumbv6m-none-eabi/debug/cat_detector_shell --chip RP2040
  ```
* Flashing the production app:
  ```bash
  cargo run --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_app
  ```

## 3. Interactive Hardware Bringup
Run the automated bringup guide under the Conda environment:
```bash
conda run -n firmware-env python tools/helpers/bringup.py --config projects/cat_detector_bringup.yaml
```

## 4. Log Streaming & Interactive Console (`host_cli`)
To read RTT log streams or interact with the serial CLI:
* Standard run (autodetects shell target):
  ```bash
  cargo run -p host_cli -- --elf target/thumbv6m-none-eabi/debug/cat_detector_shell
  ```
* Attaching to an active GDB/OpenOCD VS Code debug session:
  ```bash
  cargo run -p host_cli -- -o localhost:50000 --elf target/thumbv6m-none-eabi/debug/cat_detector_shell
  ```

## 5. Sequential Storage Flash Queries (`host_fs`)
To query files or decode logs directly from the device's flash:
* List files:
  ```bash
  cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/debug/cat_detector_app ls
  ```
* Extract telemetry to CSV:
  ```bash
  cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/release/cat_detector_app export-telemetry telemetry.csv
  ```
* Decode crash logs:
  ```bash
  cargo run -p host_fs -- --elf target/thumbv6m-none-eabi/release/cat_detector_app crash-log
  ```
