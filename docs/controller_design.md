# Domain Controller Design, Task Runners & Codegen

## Architectural Purpose
To establish clean domain boundaries, lifecycle supervision, and automated boilerplate generation for asynchronous controller tasks in the `controller` crate.

---

## Core Invariants

### 1. Controller Isolation & Context Decoration
* **Context Tracing Attribute**: Decorate every domain controller context struct inside the `controller` crate with `#[crate::tracing::controller_context]`.
* **No Controller Nesting**: Controllers MUST NOT instantiate other controllers (e.g., do NOT call `FilesystemController::new()` inside `MotorController`). Cross-controller communication must flow through the central message bus, channel hubs, or high-level coordinator tasks.

### 2. Interactive CLI Shell Command Dispatch
* **Direct Platform Operations**: CLI shell commands must use platform-level direct operations (e.g., `platform::flash::read_file_direct`) instead of instantiating controller tasks or background loops directly.
* **Non-Blocking Shell Execution**: Shell command handlers must execute quickly or yield periodically to ensure console responsiveness.

### 3. Automated Code Generation & Rinja Templates
* **Configuration Files as Single Source of Truth**: Boilerplate controller setups, task runners, priority mappings, and CLI command groups are automatically generated using Rinja templates.
* **Declarative Manifest Edits**: When modifying controllers, task runners, or CLI handlers, edit `controllers.toml`, `shell.toml`, or the template files in `controller/templates/` rather than manually editing generated files.
* **Unified Codegen Tool**: When adding new code generators, execute them through the `tools/code_gen` host utility and adhere strictly to existing code generation design patterns.

### 4. Include Directives Placement
* **Top-of-File Placement**: The `include!` macro directives that load generated topology or code structures (e.g., `include!(concat!(env!("OUT_DIR"), "/generated_app.rs"));` or `include!(concat!(env!("OUT_DIR"), "/generated_board.rs"));`) MUST be placed at the very beginning of the source file listing (i.e. the first line of code in a listing, immediately after module-level doc comments).

### 5. Target-Independent Controller Limits & Declarative Parameter Injection
* **No Target Hardware Constants in Domain Controllers**: Domain controllers in the `controller` crate must remain completely hardware- and target-independent. Do NOT hardcode target-specific or driver-specific electrical/safety limits (such as IC driver peak current ratings) as constants inside domain controllers.
* **Declarative Parameter Injection via `app.toml`**: Target- and application-specific operating parameters must be declared in `app.toml` under the corresponding controller entry in `[[apps.<app>.controllers]]` (e.g. `max_current_ma = 2000`), exposed via generated constants in `generated_app.rs`, and injected into the controller constructor at application startup.
* **Preserve Canonical Defaults**: Maintain canonical, generic defaults in default controller constructors (`new()`) and provide dedicated parameterized constructors (e.g. `new_with_max_current()`) for application configuration injection.
