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
* **Controllers**: [controller/controllers.toml](controller/controllers.toml)
* **CLIs**: [shell.toml](shell.toml)
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

## 6. Relative Paths & Privacy Hygiene Mandate
* **Relative File Paths Only**: Always use relative file paths (e.g., `controller/controllers.toml`, `shell.toml`, `docs/mcu_decoupling.md`) when creating, modifying, or linking tracked files in the workspace. Never embed absolute filesystem paths or absolute `file://` URLs in any tracked repository files.
* **Personal Information Elision**: Ensure personal information (usernames, local user home directories, private environment paths) is never committed or leaked into tracked files, documentation, or issue reports.

## 7. Worm Tracking & Issue Resolution
* When querying active issues to work on, run `python tools/dashboard.py list-worms --open`.
* By default, `list-worms --open` excludes worms in `PLANNED` status. The agent must NOT autonomously select or work on `PLANNED` worms unless explicitly requested by the user. Use `python tools/dashboard.py list-worms --planned` to view planned items.
* To register a new worm: `python tools/dashboard.py --add-worm "Description" --severity HIGH --category CONTROLLER`
* To resolve a worm: `python tools/dashboard.py --resolve-worm WORM-001 --notes "Resolution notes"`

## 8. Xerxes VCS Dashboard & Workstation Launcher
The VCS dashboard provides interactive smartlog DAG tree visualization, staged/unstaged file management, code review, and worm tracking inside a standalone application window:
* **Launch Native Desktop Workstation (Default)**:
  ```bash
  python tools/dashboard.py
  ```
  Spawns a native OS standalone window (Cocoa WKWebView on macOS / WebKitGTK on Linux via `pywebview`) with dark HUD theme styling, resize gripper, and automatic server exit on window close. Detaches from the terminal automatically.
* **Launch Eel Standalone Window (Chromium App Mode)**:
  ```bash
  python tools/dashboard.py --eel
  # or
  python tools/dashboard.py --browser eel
  ```
  Launches the dashboard inside a dedicated Chromium-based standalone app window using the Eel bridge and local user profile directory.
* **Foreground & Terminal Options**:
  ```bash
  # Run in terminal foreground without detaching:
  python tools/dashboard.py --foreground

  # Start server only without opening any desktop window:
  python tools/dashboard.py --no-browser

  # Stop any active dashboard instance running on the port:
  python tools/dashboard.py --stop
  ```

