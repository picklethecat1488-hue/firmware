# VCS Workstation, Code Review & Xerxes HUD Templates

## Architectural Purpose
To document the design patterns, template hierarchies, and persistence protocols powering the System Shock 2 - Xerxes workstation (`tools/dashboard.py`).

---

## Core Invariants

### 1. Template Structure & Error Guardrails
* **Jinja2 Rendering**: All dashboard web interfaces (diff viewer, code review, worm matrix) are templated using Jinja2 in `tools/dashboard/templates/` (`diff_view.html.j2`, `diff_component.html.j2`, `code_review.html.j2`, `worm_report.html.j2`).
* **Tag Balancing & Overflow Containment**: Unified diff viewers must balance syntax highlighting tags across line breaks, pin diff text columns with `min-width: 0`, and enforce `white-space: nowrap !important` on top bar action buttons with `overflow-x: auto` on headers to prevent layout breakage.
* **High-Contrast Scrollbars**: Scrollbars across all views must provide high-contrast styling with standard copper tracks (`#1e130b`), thumb background (`#965a25`), and hover states (`#ffb800`).

### 2. Dual-Layer Persistence & R+M+W Markdown Synchronization
* **SQLite Store as Primary Single Source of Truth**: Interactive mutations (review comments, file review statuses, worm creation, attachments, resolution notes) persist immediately to ACID SQLite databases (`target/code_review.sqlite`, `target/worms.sqlite`).
* **Bidirectional Markdown Sync (Read-Modify-Write)**: The server continuously synchronizes with `feedback/` (`feedback/CR.md`, `feedback/CR_<commit>.md`, `feedback/WORMS.md`, `feedback/WORM_<id>.md`). When on-disk files are edited externally, changes are detected via file modification time and safely merged without clobbering resolved statuses or existing notes.

### 3. CLI Parity & Subcommand Structure
* **Action-Topic Subcommands**: All workstation operations support corresponding CLI subcommands with consistent flags:
  - `list-reviews [--open] [--all-users] [--user <name>]`
  - `add-comment --file <path> --line <num> --body "<text>"`
  - `resolve-comment <id>`
  - `list-worms [--open] [--planned] [--severity <sev>] [--category <cat>]`
  - `add-worm "<title>" [--severity <sev>] [--category <cat>]`
  - `resolve-worm <id> --notes "<notes>"`
  - `sync`, `list-commits`, `commit [-m "<msg>"]`
* **Commit Author & Privacy Redaction**: Personal usernames, personal email addresses, and author identities (such as user nicknames and personal email domains) MUST be automatically elided and sanitized in the VCS engine (`GitEngine`), smartlog DAG models, commit info summaries, API responses, and web workstation UI templates (`code_review.html.j2`, `diff_view.html.j2`) into generic labels (`User`, `user@workspace`). Never leak the user's real or personal names in commit logs, smartlog displays, code review sessions, or git-tracked documentation.

### 4. Bidirectional Dashboard & Script Cross-Patching Mandate
* **Automatic Cross-Repository Synchronization**: Any bug fix, feature enhancement, styling update, or refactoring applied to developer scripts, tooling, or dashboard components in one repository (`firmware` or `hardware`) that is applicable to the other repository MUST be automatically cross-patched and mirrored into that repository, and vice versa.
* **Synchronized Subsystems**: This includes, but is not limited to:
  - Remote cloud runner and lifecycle orchestration (`bin/anvil`, `tools/vast_manager.py`, `~/.anvil.env`, `~/.anvil.lock`).
  - VCS web workstation and dashboard backends (`tools/dashboard.py`, `provider/vcs/git_engine.py`, `provider/worm_report/`, `provider/bug_report/`).
  - Shared workstation templates (`diff_view.html.j2`, `diff_component.html.j2`, `code_review.html.j2`, `worm_report.html.j2`, `bug_report.html.j2`).
  - Smartlog DAG tree rendering, commit selection, node highlighting, and author identity sanitization.
* **Zero Disparity**: Never allow dashboard bug fixes or tooling improvements (such as node highlighting, reverse-proxying, detached branch persistence, or SSH port handling) to remain isolated in one repository when the other repository shares the same architecture or template.

