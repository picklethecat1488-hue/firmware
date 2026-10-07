# 🟢 `[WORM-016]` Remove my username from tracked files

- **UUID**: `732d3b51-3220-448f-95a3-491046668c64`
- **ID**: `WORM-016`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-10-07 22:12:15 UTC`
- **Resolved**: `2026-10-07 22:29:44 UTC`

#### Description

See below, remove my username from tracked files and update your skills to always use relative file paths when updating tracked files. For bugs and worms, and other files which contain tracebacks, make a code update to automatically elide personal information when saving bug and code report feedback.

#### Execution / Console Logs

```text
.agents/skills/cat-fountain-development/SKILL.md:14
* **Controllers**: [controller/controllers.toml](file:///Users/<username>/gh/firmware/controller/controllers.toml)
.agents/skills/cat-fountain-development/SKILL.md:15
* **CLIs**: [shell.toml](file:///Users/<username>/gh/firmware/shell.toml)
.cargo/config.toml:7
runner = "/Users/<username>/gh/firmware/tools/runner.sh"
.cargo/config.toml:12
"--remap-path-prefix", "firmware=/Users/<username>/gh/firmware",
CONTRIBUTING.md:188
- To add a new controller or modify module parameters, edit the configuration in [controller/controllers.toml](file:///Users/<username>/gh/firmware/controller/controllers.toml).
CONTRIBUTING.md:189
- To add or modify interactive CLI subcommands, arguments, or resolver fields, edit [shell.toml](file:///Users/<username>/gh/firmware/shell.toml).
CONTRIBUTING.md:190
- The template files are defined in `controller/templates/` (e.g. [generated_controllers.rs.jinja](file:///Users/<username>/gh/firmware/controller/templates/generated_controllers.rs.jinja) and [sample_cli.rs.jinja](file:///Users/<username>/gh/firmware/controller/templates/sample_cli.rs.jinja)).
CONTRIBUTING.md:290
4.  Link the new project in the root [Cargo.toml](file:///Users/<username>/gh/firmware/Cargo.toml) workspace members list.
GEMINI.md:74
1. [Microcontroller Decoupling & BSPs](file:///Users/<username>/gh/firmware/docs/mcu_decoupling.md)
GEMINI.md:79
2. [Peripheral Sharing & Concurrency Patterns](file:///Users/<username>/gh/firmware/docs/peripheral_sharing.md)
GEMINI.md:85
3. [Domain Controller Design, Task Runners & Codegen](file:///Users/<username>/gh/firmware/docs/controller_design.md)
GEMINI.md:91
4. [Embedded Logging, Tracing & Host Tools](file:///Users/<username>/gh/firmware/docs/logging_tracing.md)
GEMINI.md:96
5. [Hardware-Firmware Co-Design & Downselection Architecture](file:///Users/<username>/gh/firmware/docs/hardware_firmware_codesign.md)
GEMINI.md:101
6. [VCS Workstation, Code Review & Xerxes HUD Templates](file:///Users/<username>/gh/firmware/docs/vcs_code_review.md)
app/carrier_board.md:5
The source of truth for bringup verification steps is [`app/carrier_board_bringup.yaml`](file:///Users/<username>/gh/firmware/app/carrier_board_bringup.yaml).
app/carrier_board.md:857
- Execute all automated checks codified in [`app/carrier_board_bringup.yaml`](file:///Users/<username>/gh/firmware/app/carrier_board_bringup.yaml). Verification passes only when 100% of checks succeed.
app/carrier_board.md:1124
The authoritative source of truth for bringup verification is [`app/carrier_board_bringup.yaml`](file:///Users/<username>/gh/firmware/app/carrier_board_bringup.yaml). All hardware verification procedures must follow the step definitions established in that configuration.
app/carrier_board.md:1164
- Executes steps 1–4 of [`app/carrier_board_bringup.yaml`](file:///Users/<username>/gh/firmware/app/carrier_board_bringup.yaml).
app/carrier_board.md:1177
- Executes steps 5–7 of [`app/carrier_board_bringup.yaml`](file:///Users/<username>/gh/firmware/app/carrier_board_bringup.yaml).
app/carrier_board.md:1191
- Executes steps 8–10 of [`app/carrier_board_bringup.yaml`](file:///Users/<username>/gh/firmware/app/carrier_board_bringup.yaml).
app/carrier_board.md:1200
- Executes steps 11–14 of [`app/carrier_board_bringup.yaml`](file:///Users/<username>/gh/firmware/app/carrier_board_bringup.yaml).
app/carrier_board.md:1220
- Executes complete bringup suite (steps 1–16 of [`app/carrier_board_bringup.yaml`](file:///Users/<username>/gh/firmware/app/carrier_board_bringup.yaml)).
app/cat_detector.md:79
The official, executable source of truth for these bringup steps is defined in [projects/cat_detector_bringup.yaml](file:///Users/<username>/gh/firmware/projects/cat_detector_bringup.yaml). You should run the interactive bringup helper script [bringup.py](file:///Users/<username>/gh/firmware/tools/helpers/bringup.py) to guide you through this checklist, compile/flash the correct target binaries automatically, and generate a markdown verification report:
app/cat_detector.md:97
Below is the sequence of bringup steps defined in [projects/cat_detector_bringup.yaml](file:///Users/<username>/gh/firmware/projects/cat_detector_bringup.yaml). The bringup script compiles and downloads the required firmware automatically as indicated by `flash_before` directives.
docs/hardware_firmware_codesign.md:11
* **Bare-Metal Rust Specification**: The target runtime environment is bare-metal Rust (`no_std`, Embassy asynchronous executor, `embedded-hal` and `embedded-hal-async` driver abstractions, `defmt` structured logging, stack-based zero-allocation concurrency, and static memory analysis) as detailed in [CONTRIBUTING.md](file:///Users/<username>/gh/firmware/CONTRIBUTING.md).
docs/hardware_firmware_codesign.md:14
Component selection and pinmux topologies are governed by the Test Board Revision 2.0 Downselection Process documented in [docs/downselection_report.md](file:///Users/<username>/gh/firmware/docs/downselection_report.md):
```

#### Resolution Notes

Purged username from all git-tracked files across firmware and hardware repositories, converted absolute links to relative paths, updated SKILL.md with relative paths and privacy mandate, implemented automatic personal info elision for bug/worm and code report feedback in both repos, and codified dedicated regression test suites.
