# Workspace Rules: Pre-Commit Validation

Before finalizing any task, committing changes, or proposing modifications to the codebase, you MUST run the unified pre-commit verification script to ensure the codebase remains healthy:

```bash
./tools/verify.sh
```

## Validation Guidelines
1. **Execution**: Always run `./tools/verify.sh` from the repository root. Heavy test suites, linters, and verification runs may also be offloaded to the `anvil` cloud server via `tools/anvil verify` or `tools/anvil pytest` to conserve local compute.
2. **Outcome Verification**: Confirm the script exits successfully and outputs `Verification PASSED`.
3. **Resolution**: If any component fails (such as Cargo tests, Python tests, formatting, or firmware builds), you must address the failure and re-run `./tools/verify.sh` until it passes before concluding your work.
4. **Process Management & Rerun Hygiene**: When restarting or re-running test suites (`./tools/verify.sh`, `pytest`, `cargo test`, `cargo nextest`, `tools/anvil pytest`, `tools/anvil verify`, etc.), you MUST explicitly terminate/kill any preceding running instances of that test or task before launching a new execution. Never allow multiple overlapping runs of the same test command.
5. **GitHub CI Monitoring & Local Stack Remediation**: When commits have been pushed to a remote feature branch or when instructed to inspect CI, you may inspect GitHub Actions CI workflow results (e.g. via `gh run list --limit 5`, `gh pr checks`, or `gh run view <run-id> --log-failed`) and resolve any reported failures. However, the agent MUST NOT autonomously create pull requests (`gh pr create`) or merge pull requests (`gh pr merge`). All pull request creation, peer code reviews, and PR merges MUST be performed manually by the user.
6. **Host Daemon Firmware Synchronization**: When updating or re-flashing target firmware during iterative development, debugging, or test bringup, do NOT terminate or restart the running `host.py` daemon. Instead, execute `python tools/host.py sync` (or invoke `POST http://127.0.0.1:8899/api/sync`) to trigger incremental compilation, flashing via `probe-rs`, and RTT reconnection while preserving the running workstation and HTTP server session.

---

## Remote Cloud Server (`anvil`)

For resource-intensive workloads, parameter sweeps, long-running test suites, simulation analysis, and remote verification, cloud execution is available via Vast.ai instances orchestrated by `tools/anvil`:

```ssh-config
Host anvil
  HostName <vast-ssh-host>
  Port <vast-ssh-port>
  User root
  IdentityFile "~/.ssh/id_rsa"
  StrictHostKeyChecking accept-new
```

### Usage Guidelines & Shared Workstation Configuration
1. **Shared Configuration & Provisioning**: Cloud instances are provisioned on Vast.ai (`ubuntu:22.04`) via `tools/anvil provision` / `tools/anvil release` and shared across `firmware`, `hardware`, and `SSD` using `~/.anvil.env` in `$HOME`. Remote paths dynamically resolve to `/workspace/<repo>` (fallback `$HOME/<repo>`).
2. **Automated Remote Bootstrapping**: `tools/anvil bootstrap` automatically installs build toolchains (`build-essential`, `cmake`, `pkg-config`), Rust (`rustup` + `cargo-nextest`), and persistent Miniforge on `/workspace/miniforge3` (creating the `firmware-env` conda environment).
3. **Remote Execution & Host CLI Reverse Proxying**: Run test suites (`tools/anvil pytest`), verification (`tools/anvil verify`), or cargo checks (`tools/anvil cargo`). Use `tools/anvil cli <command...>` (or `tools/anvil host <subcmd...>`) to execute host commands while reverse proxying port 8899 to the developer's local `host.py` daemon.
4. **Preceding Run Cancellation & Artifact Sync**: Ensure preceding runs of the same command are cancelled (`tools/anvil stop`) before initiating a new remote execution. Use `tools/anvil pull <remote_path> [local_dest]` or `tools/anvil recordings` to download artifacts.
5. **Lock File Tracking & COB Auto-Release Mandate**: Track active execution in `~/.anvil.lock` in `$HOME`. Any rented Vast.ai instance claim MUST be automatically released at Close of Business (COB) or when idle if no processes are using it (`tools/anvil auto-release` or `tools/anvil release`). Never leave cloud instances running unattended or overnight. See [docs/remote_cloud_runner.md](docs/remote_cloud_runner.md) for full architecture and protocol specifications.

---

## Core Architecture & Code Invariants

### 1. Test Isolation & Regression Unit Testing
* Unit tests MUST be completely isolated from implementation code. Do NOT mix unit tests inside implementation files.
* Place all unit and integration tests in a `tests/` subdirectory at the crate root level (e.g., `model/tests/state_machine_tests.rs`) for Rust crates, or in `tools/tests/` for host Python utilities.
* New unit and integration tests should be added when code is updated, including coverage for happy and sad cases. Mock structures used for testing and validation should be placed in separate listings.
* **Regression Unit Testing Mandate**: Whenever a regression is identified, investigated, or bisected to a prior change, you MUST introduce dedicated regression unit tests (or add active regression assertions to existing test suites) that explicitly guard against the identified regression before concluding the task.
* **Functional Behavioral Testing Mandate**: All test code (Rust unit/integration tests and host Python test suites) MUST test the actual functionality and runtime behavioral invariants of the code being run (e.g. verifying state transitions, execution outcomes, and side-effects such as halting repeated ticks on driver faults). Tests must NOT verify behavior by simply inspecting source files for the existence of specific statements, strings, or AST tokens.
* **Prohibition on Redundant Wrapper Tests**: Regression tests must assert new behavioral invariants directly against the code under test in the canonical test suite of that language/subsystem (Rust unit/integration tests in `tests/` for firmware crates, Python `pytest` suites in `tools/tests/` for host tools). Do NOT create shallow wrapper tests (such as Python test files or functions that merely invoke `cargo test` or `pytest` via subprocess on existing test cases) without exercising new functionality.
* **Update Existing Tests Directly Over Duplicate Tests**: If an existing test case intended to guard an invariant does not properly check for a bug, update the original test case directly to assert the missing invariant rather than creating redundant wrapper or duplicate test listings.
* **Omit Tests When Invariant Cannot Be Unit Tested**: If it is not possible to unit test a regression fix (e.g. physical hardware timing, motor cogging, or electrical inertia that cannot be captured in a mock without false precision) and existing test suites already verify logical invariants, omit adding an artificial regression test rather than introducing a meaningless test that merely duplicates arithmetic or wraps other tests.

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

### 5. Personal Privacy & Identity Redaction
* **Zero Personal Name & Email Leaks**: The user's personal real name, local usernames, and personal email addresses MUST NEVER be leaked into git-tracked files, code comments, documentation guides, bug reports, or code review summaries. Use generic placeholders like `[Customer Name]`, `<username>`, or `User`.
* **VCS Engine & UI Sanitization**: Git interaction layers (`GitEngine`), smartlog DAG generation, commit inspect APIs, and VCS web workstation views (`code_review.html.j2`, `diff_view.html.j2`) MUST automatically sanitize commit authors and email addresses (e.g. historical user handles and personal email domains) to `User` and `user@workspace`.

### 6. Fast Search & Tooling Hygiene
* **Silver Searcher (`ag`) Mandate**: For file, text, and code symbol searches across the codebase, always use The Silver Searcher (`ag`) instead of recursive `grep -rn` or unbounded `find .`. `ag` automatically respects `.gitignore` rules, ignores `target/` and build artifacts, and executes synchronously without spawning long-running background tasks.

---

## Workflow & Issue Tracking Principles

### 1. Work Tracking & Task Management
* **Task List (`TODO.md`)**: Maintain and track planned tasks, active implementation steps, outstanding engineering checklist items, and completed work in `TODO.md` in the workspace root. Keep checklist items updated (`[ ]` -> `[x]`) as subtasks progress.

### 2. Pending Code Review Inspection
* Whenever beginning a new task, turn, or feature implementation, you MUST inspect the `feedback/` directory—including the primary aggregated report (`feedback/CR.md`), granular commit review files (`feedback/CR_<commit>.md`), or query the review database (`target/code_review.sqlite`, `python tools/dashboard.py list-reviews --open`) for pending code review feedback, active review comments, or requested revisions. Any unaddressed feedback (particularly `MUST_FIX` blockers and architecture `PROPOSAL` items) must be prioritized and resolved before progressing to new development tasks.

### 3. Worm Tracker & Historical Context Inspection
* Whenever working on tasks, investigating issues, or modifying existing subsystems, you MUST inspect the worm tracking records (`feedback/WORM_<id>.md`, `target/worms.sqlite`, `python tools/dashboard.py list-worms --open`) for past context, historical failure modes, reproduction steps, and resolved invariants. Leveraging past context prevents re-introducing known regressions. All worm report attachments (`attachments/`, `target/attachments/`, `feedback/attachments/`) are tracked in **GitHub LFS** and considered **non-confidential** and public to the repository; never attach sensitive credentials, secret tokens, private keys, or proprietary secrets.
* **Planned Worm Exemption**: Issues marked with status `PLANNED` represent deferred work or backlog roadmaps. `python tools/dashboard.py list-worms --open` excludes `PLANNED` worms by default. The assistant MUST NOT autonomously pick up or work on `PLANNED` worms unless specifically and explicitly requested by the user.

### 4. Single-Bug Focus & Atomic Issue Remediation
* To prevent context pollution and attention degradation during extended problem-solving sessions, you MUST investigate, diagnose, and resolve only ONE bug/worm or defect at a time:
  1. **Registry & Context**: Query the worm tracker or register the new defect with reproduction steps and classification.
  2. **Isolated Reproduction**: Construct an isolated reproduction script or minimal failing unit test asserting the flawed invariant *before* editing production code.
  3. **Targeted Fix**: Implement the minimal necessary change strictly scoped to the defect.
  4. **Dedicated Regression Test**: Codify the reproduction into an active unit test asserting the correct invariant.
  5. **Verification**: Run pre-commit checks (`./tools/verify.sh`) to verify 100% pass rate.
  6. **Review Tasks & Functional Checklist**: When changing status to `RESOLVED`, populate the worm's review tasks and functional checklist (`review_tasks`) with concise, actionable verification steps and executable demonstration commands that verify the change from a user-experience perspective (such as a host CLI command e.g. `python tools/host.py cli ...`, interactive diagnostic shell, or UI inspection). Do NOT use `pytest` commands as review verification tasks, as automated test suites are already run during pre-commit checks and CI; verification commands should demonstrate the actual behavioral outcome or visual fix in the user experience.
  7. **Resolution, Linkage & Commit**: Mark the worm resolved (`python tools/dashboard.py resolve-worm --id <ID> --notes "<Summary>"`), attach the fix commit hash (`python tools/dashboard.py attach-commit --id <ID> --commit <hash>`), and commit atomically before picking up the next task.

### 5. User-Managed Code Review & Autonomous PR Prohibition
* The assistant is strictly PROHIBITED from autonomously creating pull requests (`gh pr create`) or merging pull requests (`gh pr merge`). All pull request creation, peer code reviews, and PR merges MUST be performed manually by the user.

### 6. Bidirectional Dashboard & Script Cross-Patching Mandate
* **Automatic Cross-Repository Synchronization**: Any bug fix, feature enhancement, styling update, or refactoring applied to developer scripts, tooling, or dashboard components in one repository (`firmware` or `hardware`) that is applicable to the other repository MUST be automatically cross-patched and mirrored into that repository, and vice versa.
* **Synchronized Subsystems**: This includes, but is not limited to:
  - Remote cloud runner and lifecycle orchestration (`tools/anvil`, `tools/vast_manager.py`, `~/.anvil.env`, `~/.anvil.lock`).
  - VCS web workstation and dashboard backends (`tools/dashboard.py`, `provider/vcs/git_engine.py`, `provider/worm_report/`, `provider/bug_report/`).
  - Shared workstation templates (`diff_view.html.j2`, `diff_component.html.j2`, `code_review.html.j2`, `worm_report.html.j2`, `bug_report.html.j2`).
  - Smartlog DAG tree rendering, commit selection, node highlighting, and author identity sanitization.
* **Zero Disparity**: Never allow dashboard bug fixes or tooling improvements (such as node highlighting, reverse-proxying, detached branch persistence, or SSH port handling) to remain isolated in one repository when the other repository shares the same architecture or template.

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
   - Bidirectional dashboard and script cross-patching mandate.

7. [Remote Cloud Runner & Vast.ai Architecture](docs/remote_cloud_runner.md)
   - Vast.ai container provisioning, lifecycle hooks, and shared `~/.anvil.env` configuration.
   - Dynamic `/workspace` persistent storage detection and fallback paths.
   - Remote dependency bootstrapping (`build-essential`, Rust, Miniforge, CUDA).
   - Activity lock tracking (`~/.anvil.lock`), port forwarding, and COB auto-release protocols.

