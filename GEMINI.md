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

## Core Architecture & Code Invariants

### 1. Test Isolation & Regression Unit Testing
* Unit tests MUST be completely isolated from implementation code. Do NOT mix unit tests inside implementation files.
* Place all unit and integration tests in a `tests/` subdirectory at the crate root level (e.g., `model/tests/state_machine_tests.rs`) for Rust crates, or in `tools/tests/` for host Python utilities.
* New unit and integration tests should be added when code is updated, including coverage for happy and sad cases. Mock structures used for testing and validation should be placed in separate listings.
* **Regression Unit Testing Mandate**: Whenever a regression is identified, investigated, or bisected to a prior change, you MUST introduce dedicated regression unit tests (or add active regression assertions to existing test suites) that explicitly guard against the identified regression before concluding the task.

### 2. Firmware Target Compatibility Mandate
* **Universal Target Compatibility**: Unless duly noted, any firmware code should apply to all firmware targets in the repository.
* **Decoupled Architecture**: Avoid target-specific lock-in in core libraries, drivers, data models, state machines, and domain controllers. Peripheral and hardware specifics must be decoupled through `embedded-hal` trait interfaces and Board Support Package (BSP) abstractions so code is universally portable across all supported boards.

### 3. Strict Code Cleanliness & Hygiene
* **No Dead Code**: Unused code, dangling functions, dead branches, and parameters that do not affect execution must be removed from the repository.
* **No Backward Compatibility Shims**: Do NOT introduce, retain, or propose backward compatibility shims, aliases, legacy wrappers, deprecated fallbacks, or obsolete re-exports. Update callers, imports, and tests directly to canonical current names and purge obsolete identifiers completely.
* **Parameter & Signature Hygiene**: When modifying, refactoring, or simplifying functions, subroutines, or methods, any parameters that become unused MUST be immediately pruned from both the function signature and all caller invocations with each change.
* **Error Handling Guardrails**: Use explicit bounds checking and validation rather than generic catch-all patterns. In Python, do NOT use `try/except` structures in core computation or logic paths except to guard I/O operations (filesystem, network, database); never silently ignore errors with `try/except/pass` blocks. In Rust, propagate errors via `Result` and handle failure variants explicitly.

### 4. Documentation & Lint Style
* Code documentation MUST be comprehensive. Always write clear docstrings (using `///` in Rust and PEP-257 docstrings in Python) for all newly defined or modified structs, enums, public methods, public functions, and struct fields.
* **Strongly Typed Enums & Newtypes**: Prefer defining strongly typed Rust enums (`#[derive(Copy, Clone, Eq, PartialEq, defmt::Format)] enum State { ... }`) or newtypes over passing raw primitive literals directly for state identifiers, command opcodes, lifecycle statuses, or category modes. In host Python utilities, subclass `str` and `Enum`.
* **Named Constant Formatting**: Constant values in production code must be assigned to module-level or associated `ALL_CAPS` named constants (e.g., `pub const MAX_BUFFER_SIZE: usize = 256;`) rather than being embedded as inline magic literals.
* **Idiomatic Iteration & Pattern Matching**: Prefer looping over iterators directly (`for item in items.iter()`, `.enumerate()`) or iterator adapters (`.map()`, `.filter()`) rather than manual integer indexing by range bounds. Prefer exhaustive Rust `match` expression pattern matching (or `if let Some(...) = ...`) over chained `if/else` ladders when comparing against variants or enum branches.
* **Markdown Preview Asset Location**: All markdown preview galleries, rendered frame previews, inspection figures, and simulation snapshots intended for visual evaluation MUST be placed inside the workspace under `recordings/previews/` using relative image paths to ensure compatibility with VS Code Markdown Preview security sandbox restrictions.

---

## Workflow & Issue Tracking Principles

### 1. Work Tracking & Task Management
* **Task List (`TODO.md`)**: Maintain and track planned tasks, active implementation steps, outstanding engineering checklist items, and completed work in `TODO.md` in the workspace root. Keep checklist items updated (`[ ]` -> `[x]`) as subtasks progress.

### 2. Pending Code Review Inspection
* Whenever beginning a new task, turn, or feature implementation, you MUST inspect the `feedback/` directory—including the primary aggregated report (`feedback/CR.md`), granular commit review files (`feedback/CR_<commit>.md`), or query the review database (`target/code_review.sqlite`, `python tools/dashboard.py list-reviews --open`) for pending code review feedback, active review comments, or requested revisions. Any unaddressed feedback (particularly `MUST_FIX` blockers and architecture `PROPOSAL` items) must be prioritized and resolved before progressing to new development tasks.

### 3. Worm Tracker & Historical Context Inspection
* Whenever working on tasks, investigating issues, or modifying existing subsystems, you MUST inspect the worm tracking records (`feedback/WORMS.md`, `feedback/WORM_<id>.md`, `target/worms.sqlite`, `python tools/dashboard.py list-worms --open`) for past context, historical failure modes, reproduction steps, and resolved invariants. Leveraging past context prevents re-introducing known regressions. All worm report attachments (`attachments/`, `target/attachments/`, `feedback/attachments/`) are tracked in **GitHub LFS** and considered **non-confidential** and public to the repository; never attach sensitive credentials, secret tokens, private keys, or proprietary secrets.
* **Planned Worm Exemption**: Issues marked with status `PLANNED` represent deferred work or backlog roadmaps. `python tools/dashboard.py list-worms --open` excludes `PLANNED` worms by default. The assistant MUST NOT autonomously pick up or work on `PLANNED` worms unless specifically and explicitly requested by the user.

### 4. Single-Bug Focus & Atomic Issue Remediation
* To prevent context pollution and attention degradation during extended problem-solving sessions, you MUST investigate, diagnose, and resolve only ONE bug/worm or defect at a time:
  1. **Registry & Context**: Query the worm tracker or register the new defect with reproduction steps and classification.
  2. **Isolated Reproduction**: Construct an isolated reproduction script or minimal failing unit test asserting the flawed invariant *before* editing production code.
  3. **Targeted Fix**: Implement the minimal necessary change strictly scoped to the defect.
  4. **Dedicated Regression Test**: Codify the reproduction into an active unit test asserting the correct invariant.
  5. **Verification**: Run pre-commit checks (`./tools/verify.sh`) to verify 100% pass rate.
  6. **Resolution & Commit**: Mark the worm resolved in `tools/dashboard.py` and commit the fix atomically before picking up the next task.

### 5. User-Managed Code Review & Autonomous PR Prohibition
* The assistant is strictly PROHIBITED from autonomously creating pull requests (`gh pr create`) or merging pull requests (`gh pr merge`). All pull request creation, peer code reviews, and PR merges MUST be performed manually by the user.

---

## Modular Subsystem & Domain Architecture Guides

To minimize global context overhead and prevent unnecessary token burn, domain- and subsystem-specific architectural mandates are maintained in dedicated reference documents under `docs/`:

1. [Microcontroller Decoupling & BSPs](docs/mcu_decoupling.md)
   - Microcontroller decoupling and Board Support Package (`Board::init`) patterns.
   - Target-independent driver wrappers and vendor code isolation.
   - Frequency-independent time-based delays (`embassy_time::Timer`) instead of cycle loops.

2. [Peripheral Sharing & Concurrency Patterns](docs/peripheral_sharing.md)
   - Actor / Message-Passing pattern with async channels for system integration.
   - Interior mutability (`Rc` + `RefCell` / `Mutex`) for bringup and diagnostic shells.
   - Strict prohibition on passing raw mutable references across tasks.
   - Global static memory restrictions and `OnceLock` initialization validation.

3. [Domain Controller Design, Task Runners & Codegen](docs/controller_design.md)
   - Controller isolation with `#[crate::tracing::controller_context]`.
   - Direct platform operations for non-blocking CLI shell commands.
   - Rinja template boilerplate generation via `controllers.toml`, `shell.toml`, and `tools/code_gen`.
   - `include!` macro directives placement at top of file.

4. [Embedded Logging, Tracing & Host Tools](docs/logging_tracing.md)
   - `defmt` structured logging macros with zero format-string overhead.
   - Consolidated platform tracing facade module (`crate::tracing`).
   - Host diagnostic and debugging utilities: `tools/host_fs` and `tools/host_cli`.

5. [Hardware-Firmware Co-Design & Downselection Architecture](docs/hardware_firmware_codesign.md)
   - Target runtime environment: bare-metal Rust (`no_std`, Embassy, `embedded-hal`, `defmt`).
   - Revision 2.0 reference hardware architecture (NXP MCX N947/N946, Winbond SLC Flash, TI BQ24074, FTDI bridge, LP5009 LED, CY8CMBR3116 touch).
   - Programmable component qualification criteria (open-source driver code, public datasheets, register maps, zero binary blobs).

6. [VCS Workstation, Code Review & Xerxes HUD Templates](docs/vcs_code_review.md)
   - Jinja2 code generation, template structures, tag balancing, and error guardrails.
   - Dual-layer persistence: SQLite backing store and Read-Modify-Write markdown synchronization.
   - Structured worm tracking, Git LFS attachments, and CLI subcommand parity.
