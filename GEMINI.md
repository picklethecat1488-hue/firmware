# Workspace Rules: Pre-Commit Validation

Before finalizing any task, committing changes, or proposing modifications to the codebase, you MUST run the unified pre-commit verification script to ensure the codebase remains healthy:

```bash
./tools/verify.sh
```

## Validation Guidelines
1. **Execution**: Always run `./tools/verify.sh` from the repository root.
2. **Outcome Verification**: Confirm the script exits successfully and outputs `Verification PASSED`.
3. **Resolution**: If any component fails (such as Cargo tests, Python tests, formatting, or firmware builds), you must address the failure and re-run `./tools/verify.sh` until it passes before concluding your work.
4. **Process Management & Rerun Hygiene**: When restarting or re-running test suites (`./tools/verify.sh`, `pytest`, `cargo test`, `cargo nextest`, etc.), you MUST explicitly terminate/kill any preceding running instances of that test or task before launching a new execution. Never allow multiple overlapping runs of the same test command.
5. **GitHub CI Monitoring & Local Stack Remediation**: When commits have been pushed to a remote feature branch or when instructed to inspect CI, you may inspect GitHub Actions CI workflow results (e.g. via `gh run list --limit 5`, `gh pr checks`, or `gh run view <run-id> --log-failed`) and resolve any reported failures. However, the agent MUST NOT autonomously create pull requests (`gh pr create`) or merge pull requests (`gh pr merge`). All pull request creation, peer code reviews, and PR merges MUST be performed manually by the user.

---

## Software Architecture & Code Guidelines

### 1. Test Isolation
* Unit tests MUST be completely isolated from implementation code. Do NOT mix unit tests inside implementation files. 
* Place all unit and integration tests in a `tests/` subdirectory at the crate root level (e.g., `model/tests/state_machine_tests.rs`) for Rust crates, or in `tools/tests/` / `tools/validation/tests/` for host Python utilities.
* New unit and integration tests should be added when code is updated, including coverage for happy and sad cases.
* Mock structures used for testing and validation should be placed in separate listings. It's okay to put host and target code in the same listing.
* **Regression Unit Testing Mandate**: Whenever a regression is identified, investigated, or bisected to a prior change, you MUST introduce dedicated regression unit tests (or add active regression assertions to existing test suites) that explicitly guard against the identified regression before concluding the task.

### 2. Microcontroller Decoupling & BSPs
* Do NOT perform conditional driver setup or extract GPIO pins inside main application files (`main.rs`, `shell.rs`).
* Encapsulate all initialization, pin configuration, and dynamic address setup inside the `Board::init` constructor in the Board Support Package (BSP) target/host implementations (e.g., `bsp_target.rs` / `bsp_host.rs`).
* Do NOT prefix files or structs with MCU model numbers (e.g., do not write `rp2040_sensor.rs`). Keep driver wrappers target-independent.
* Vendor-specific code should only go in the board or app crate. It may go into the platform crate if placed inside a vendor's platform support module (e.g., [platform/src/rp2040/lib.rs](file:///Users/daparker/gh/firmware/platform/src/rp2040/lib.rs)).
* Do NOT use fixed CPU cycle delays (e.g. `cortex_m::asm::delay`) in code. Instead, use frequency-independent time-based delays (e.g., `embassy_time::Timer` or `embassy_time::Delay`).

### 3. Peripheral Sharing
* To share a peripheral driver between multiple controllers:
  * **System Integration**: Use the Actor/Message-Passing pattern. Run the peripheral inside its own isolated task and communicate via async channels (e.g., `embassy_sync::channel::Channel`).
  * **Bringup/Shell**: Use Interior Mutability & Shared References (`Rc` + `RefCell` or `Mutex`/`Arc`).
  * **Forbidden**: Never pass raw mutable references across tasks.
* Static mutable variables (statics) MUST NOT be used outside of core monitor tasks, interrupt callbacks, panic handling, and hardware/communication buffers requiring static lifetimes.
* When initializing global statics (such as `OnceLock` instances), always verify the result of `.set(...)` (e.g., via `.expect(...)` or `.unwrap()`) to catch double-initialization bugs immediately. Do NOT discard the result with `let _ =`.

### 4. Controller Design & Constraints
* Decorate every domain controller context struct inside the `controller` crate with `#[crate::tracing::controller_context]`.
* Controllers MUST NOT instantiate other controllers (e.g. do not call `FilesystemController::new()` inside `MotorController`).
* CLI shell commands must use platform-level direct operations (e.g., `platform::flash::read_file_direct`) instead of instantiating controller tasks directly.
* Boilerplate controller setups, task runners, and CLI command groups are automatically generated using Rinja. When modifying controllers or CLI handlers, edit `controllers.toml`, `shell.toml`, or the template files in `controller/templates/` instead of writing boilerplate directly.
* When adding new code gen, the code should be generated by the `tools/code_gen` host tool and follow the same design patterns as the existing codegen.

### 5. Logging & Tracing Standards
* Instrument all async tasks, controller loops, and main entry points by default using `defmt` logging macros for startup, tick, and command changes.
* Use the consolidated tracing facade module `use crate::tracing;` (which re-exports `platform::tracing`). Do NOT import the standard `tracing` crate directly, as it requires `alloc` and is incompatible with the target `no_std` environment.
* When using `#[tracing::instrument]`, do NOT list `self` in the `skip(...)` attribute.

### 6. Host Diagnostic & Debugging Tools
* Host-based filesystem debugging must be performed using the host utility in `tools/host_fs`.
* Host-based CLI interface interaction, RTT log streaming, and tracing must be performed using the host utility in `tools/host_cli`.
* Host-based VCS Smartlog DAG inspection, commit split/combine, merge conflict resolution, interactive code review, and bug tracking are performed using `tools/dashboard.py` (serving the System Shock 2 - Xerxes workstation).

### 7. Documentation Standards & Code Style
* Code documentation MUST be comprehensive. Always write clear docstrings (using `///` in Rust and PEP-257 docstrings in Python) for all newly defined or modified structs, enums, public methods, public functions, and struct fields.
* Docstrings should concisely describe:
  * The purpose of the item.
  * Safety invariants or error conditions.
* Constant values in Python production code must be assigned to module-level or class-level `ALL_CAPS` named constant variables rather than being embedded as inline magic literals.
* Prefer Python `match / case` pattern matching syntax when comparing the same subject field or expression against multiple comparands, enum variants, or constant branches rather than chained `if / elif / else` ladders.
* **Parameter & Signature Hygiene**: Whenever modifying, refactoring, or simplifying functions, subroutines, or methods, any parameters that become unused MUST be immediately pruned from both the function signature and all caller invocations. Do NOT leave unused parameters in signatures, accept dummy parameters, or pass dead constant arguments.
* **No Dead Code**: Unused code and dead parameters that do not affect execution must be removed.
* **No Backward Compatibility Shims**: Do NOT introduce, retain, or propose backward compatibility shims, aliases, legacy wrappers, deprecated fallbacks, or obsolete re-exports. Update callers directly to canonical symbols.

### 8. Include Directives Placement
* The `include!` macro directives loading generated topology or code structures (e.g. `include!(concat!(env!("OUT_DIR"), "/generated_app.rs"));` or `include!(concat!(env!("OUT_DIR"), "/generated_board.rs"));`) MUST be placed at the very beginning of the source file listing (i.e. the first line of code in a listing, after module-level doc comments), if they are used.

### 9. Task Tracking, Code Review & Bug Reporting
* **Task List (`TODO.md`)**: Maintain and track planned work in a `TODO.md` file at the repository root for non-trivial or multi-step tasks. Keep `TODO.md` updated as tasks progress, marking completed items and noting any new sub-tasks discovered during implementation.
* **Pending Code Review Inspection (`feedback/CR.md`, `feedback/CR_<commit>.md`, `target/code_review.sqlite`, `python tools/dashboard.py`)**: Whenever beginning a new task, turn, or feature implementation, inspect the `feedback/` directory or query the review database for pending code review feedback, active review comments, or requested revisions:
  1. **Markdown Reports (`feedback/CR.md` & `feedback/CR_<commit>.md`)**: Human-readable overview containing verdict badges, line-by-line file diff links, formatted code snippets, reviewer notes, and an action items checklist (`- [ ]`).
  2. **SQLite Backing Store (`target/code_review.sqlite`)**: Robust ACID SQLite store (`SQLiteReviewStore`).
  3. **Dashboard & Code Review CLI (`python tools/dashboard.py`)**: Query review status in the console (`python tools/dashboard.py --reviews`), filter unresolved comments (`python tools/dashboard.py --reviews --open`), resolve items (`python tools/dashboard.py --resolve-comment <id>`), or launch the interactive System Shock 2 - Xerxes dashboard (`python tools/dashboard.py`).
  Any unaddressed feedback in `feedback/CR.md` (particularly `MUST_FIX` blockers and architecture `PROPOSAL` items) must be prioritized and resolved before progressing to new development tasks.
* **Worm Tracker & Historical Context Inspection (`feedback/WORMS.md`, `feedback/WORM_<id>.md`, `target/worms.sqlite`, `python tools/dashboard.py`)**: Inspect worm records for historical failure modes, reproduction steps, and resolved invariants:
  1. **Markdown Tracker (`feedback/WORMS.md` & `feedback/WORM_<id>.md`)**: Human-readable registry with severity/category breakdowns, reproduction step walkthroughs, log/screenshot attachment tables, and resolution notes. All worm report attachments (`feedback/attachments/`) are tracked in **GitHub LFS** and considered **non-confidential** and public to the repository; never attach sensitive credentials, secret tokens, private keys, or proprietary secrets.
  2. **SQLite Backing Store (`target/worms.sqlite`)**: ACID SQLite store (`SQLiteWormStore`).
  3. **Worm Report CLI (`python tools/dashboard.py`)**: List active worms (`python tools/dashboard.py --worms`), list open worms only (`python tools/dashboard.py list-worms --open`), register worms (`python tools/dashboard.py add-worm "<title>" --severity <SEV> --category <CAT>`), or mark issues resolved (`python tools/dashboard.py resolve-worm <WORM-ID> --notes "<notes>"`).
* **Single-Bug Focus & Atomic Issue Remediation**: Investigate, diagnose, and resolve only ONE bug/worm or defect at a time. Do NOT attempt to batch or concurrently remediate multiple unrelated issues in a single turn, PR, or commit stack.
  For each defect:
  1. *Registry & Context*: Query or register the worm with reproduction steps.
  2. *Isolated Reproduction*: Construct an isolated reproduction script or minimal failing unit test asserting the flawed invariant *before* editing production code.
  3. *Targeted Fix*: Implement the minimal necessary change strictly scoped to the defect.
  4. *Dedicated Regression Test*: Codify the reproduction into an active unit test asserting the correct invariant.
  5. *Verification*: Run pre-commit checks (`./tools/verify.sh`).
  6. *Resolution & Commit*: Mark the worm resolved in `tools/dashboard.py` and commit the fix atomically.
* **Unresolved Worm Persistence & Documentation Mandate**: If a worm cannot be fully resolved in the current turn or session, update the worm entry in SQLite, `feedback/WORMS.md`, and `feedback/WORM_<id>.md` (via `python tools/dashboard.py`) with all investigation notes, reproduction steps, and blocking details, and leave its status as `OPEN`. NEVER mark an unresolved worm as resolved, closed, or silently drop it from tracking.

### 10. Hardware-Firmware Co-Design & Downselection Architecture
* **Target Firmware Environment**: Bare-metal Rust (`no_std`, Embassy asynchronous executor, `embedded-hal` driver abstractions, `defmt` logging, stack-based zero-allocation concurrency, and static memory analysis) as detailed in [CONTRIBUTING.md](file:///Users/daparker/gh/firmware/CONTRIBUTING.md).
* **Reference Hardware Architecture**: Hardware component selection and pinmux topology are governed by the Test Board Revision 2.0 Downselection Process documented in [docs/downselection_report.md](file:///Users/daparker/gh/firmware/docs/downselection_report.md):
  - **MCU Subsystem**: NXP MCX N947 / N946 dual ARM Cortex-M33 @ 150MHz with eIQ Neutron NPU (42 GMACs), PowerQuad DSP, 2MB dual-bank Flash, 512KB SRAM.
  - **Telemetry & Calibration Storage**: Winbond W25N01GV 1Gb Serial SLC NAND Flash over high-speed Dual-Channel FlexSPI (100 MHz SDR OD mode).
  - **Power & Battery Management**: TI BQ24074 dynamic Power-Path Li-Ion charger (`/PGOOD` wake interrupt, `/CHG` state monitoring) and ADI MAX17048 precision fuel gauge (I2C0).
  - **Diagnostic & Console Bridge**: FTDI FT232RNQ USB-UART bridge IC connected to USB-C (UART0 @ 1-3 Mbaud, CBUS reset/boot controls) and 10-pin ARM SWD Cortex debug header.
  - **HMI & Sensors**: TI LP5009 I2C constant-current RGB LED driver (I2C0 0x14) and Cypress CY8CMBR3116 capacitive touch IC (I2C1 + CAP_INT).
* **Programmable Component Qualification**: Any active electronic component, sensor IC, PMIC, or microcontroller integrated into designs that requires software control MUST meet:
  - Permissive open-source driver code (Rust crate implementing `embedded-hal` traits or portable C library).
  - Complete, non-confidential public datasheets and register maps.
  - Zero proprietary closed-source binary firmware blobs or NDA-encumbered stacks.

### 11. Markdown Preview Asset Location
* All markdown preview galleries, rendered figures, and diagnostic snapshots intended for visual evaluation MUST be placed inside the workspace under `recordings/previews/` using relative image paths to ensure compatibility with VS Code Markdown Preview security sandbox restrictions.
