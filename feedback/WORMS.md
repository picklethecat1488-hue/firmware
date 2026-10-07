# Worm Report Tracker: Hardware Engineering Bug Tracker

> Automated bug tracking, triage, and issue registry generated via Firmware Bug Report Engine.

## Tracker Overview

| Metric | Details |
| :--- | :--- |
| **Report Date** | `2026-10-07 23:29:11 UTC` |
| **Total Issues** | `18` |
| **Open Issues** | `2` |
| **Resolved / Closed** | `16 (88%)` |

## Issues by Severity

| Severity | Count | Meaning |
| :--- | :---: | :--- |
| **`[CRITICAL]`** | 0 | System crashes, build failures, blockages, or electrical shorts. |
| **`[HIGH]`** | 3 | Major functional defects, broken routing, DRC violations, or unphysical behavior. |
| **`[MEDIUM]`** | 13 | Silkscreen collisions, layout sub-optimality, or visual clipping. |
| **`[LOW]`** | 2 | Minor aesthetic imperfections or documentation notes. |

## Issues by Category

| Category | Count | Description |
| :--- | :---: | :--- |
| **`FIRMWARE`** | 3 | Core firmware logic, state machines, async tasks. |
| **`CONTROLLER`** | 1 | Domain controllers, PID loops, event dispatch. |
| **`DRIVER`** | 0 | Hardware peripheral drivers, embedded-hal. |
| **`PLATFORM`** | 0 | Chip support, PAC, HAL, clock/power management. |
| **`BOARD`** | 1 | Board support packages (BSP), pinmux, boards. |
| **`MODEL`** | 0 | Domain data models, configurations, state definitions. |
| **`SHELL`** | 0 | CLI interface, shell commands, debug console. |
| **`INFRASTRUCTURE`** | 12 | Build tooling, compilers, test runners, headless tools. |
| **`UI`** | 0 | Web dashboards, CLI viewers, review interfaces. |
| **`GENERAL`** | 1 | Unclassified or cross-cutting firmware issues. |

## Issue Checklist

- [x] **`[MEDIUM]`** [#WORM-001](#worm-001): Remove hardware-specific categories from the dashboard `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-002](#worm-002): Update BUG to WORM `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-003](#worm-003): GEMINI.md should be core agent skills only `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-004](#worm-004): Change Code Review Icon to a wrench `[dashboard]` (`RESOLVED`)
- [x] **`[LOW]`** [#WORM-005](#worm-005): carrier board 2.0-Create trivial "hello world" shell project `[carrier_board]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-006](#worm-006): Forgot to update the VCS UI (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-007](#worm-007): Word wrap doesn't seem to be working `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-008](#worm-008): Update Contributing.md with dashboard docs (`RESOLVED`)
- [x] **`[HIGH]`** [#WORM-009](#worm-009): Still seeing issues with multiple Diff Viewer windows being open after exiting from the Code Review and Bug Report windows (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-010](#worm-010): "Open in VSCode" didn't work (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-011](#worm-011): Traceback in dashboard `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-012](#worm-012): Rename audio controller to speaker controller `[carrier_board]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-013](#worm-013): Feedbacks about carrier_board.md `[carrier_board]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-014](#worm-014): Unresolved worms in VCS UI `[dashboard]` (`RESOLVED`)
- [x] **`[HIGH]`** [#WORM-015](#worm-015): Unable to open code review or worm report pages from VCS UI `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-016](#worm-016): Remove my username from tracked files (`RESOLVED`)
- [ ] **`[HIGH]`** [#WORM-017](#worm-017): Working tree actions doesn't show up unless I click an older commit `[dashboard]` (`OPEN`)
- [ ] **`[LOW]`** [#WORM-018](#worm-018): Populate component list (`OPEN`)

## Detailed Issue Log

### <a id="worm-001"></a> 🟢 `[WORM-001]` Remove hardware-specific categories from the dashboard

- **UUID**: `0afd22f6-361f-424d-a361-21ec90d4c818`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-03 19:26:02 UTC`
- **Resolved**: `2026-10-03 19:43:19 UTC`

#### Description

Remove hardware-specific categories from the dashboard. We will use the hardware dashboard to track hardware issues.

#### Resolution Notes

Removed hardware categories (PCB, CAD, SIMULATION) and replaced with firmware categories in WormCategory, models, exporters, and UI templates

---

### <a id="worm-002"></a> 🟢 `[WORM-002]` Update BUG to WORM

- **UUID**: `d8280544-0a15-4a1b-8e9b-cf6be47ee55b`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-03 19:26:41 UTC`
- **Resolved**: `2026-10-03 20:30:00 UTC`

#### Description

Update the name used for tracking issues from BUG to WORM in all code, UI and docs. Update the icon used to indicate an issue from a bug icon to a worm icon.

#### Resolution Notes

Updated issue tracking nomenclature from BUG to WORM across all data models, SQLite storage, CLI subcommands/flags, UI templates, and documentation. Replaced bug icon with worm (🪱).

---

### <a id="worm-003"></a> 🟢 `[WORM-003]` GEMINI.md should be core agent skills only

- **UUID**: `45a673a6-ff51-4901-b831-230917b4e348`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-03 19:27:20 UTC`
- **Resolved**: `2026-10-03 20:50:19 UTC`

#### Description

We made a change in the hardware repo so that GEMINI.md would be for core agent skills only, and subsystem-specific skills would be moved to different files under a docs/ directory to reduce token burn. Can you locate this change in the hardware repo, and apply the same mandate here? We also need to add a back-combat mandate to firmware projects- basically, unless duly noted any firmware code should apply to all firmware targets in the repo.

#### Resolution Notes

Streamlined GEMINI.md to core agent skills, modularized subsystem guides into docs/, and added universal firmware target back-compatibility mandate.

---

### <a id="worm-004"></a> 🟢 `[WORM-004]` Change Code Review Icon to a wrench

- **UUID**: `5aad836e-d894-4ae3-8d9d-83f7923c8d3a`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-03 19:28:39 UTC`
- **Resolved**: `2026-10-03 20:52:41 UTC`

#### Description

Change the Code Review icon used in UI from a magnifying glass icon to a wrench icon

#### Resolution Notes

Changed Code Review icon in UI from magnifying glass to wrench across diff_view and code_review templates, verified with dedicated test.

---

### <a id="worm-005"></a> 🟢 `[WORM-005]` carrier board 2.0-Create trivial "hello world" shell project

- **UUID**: `8bbf6a0f-de44-4b50-aa68-dd5b1452fcdb`
- **Status**: `RESOLVED`
- **Severity**: `LOW`
- **Category**: `BOARD`
- **Component**: `carrier_board`
- **Created**: `2026-10-03 19:29:54 UTC`
- **Resolved**: `2026-10-03 21:33:17 UTC`

#### Description

Create a trivial "hello world" project we can use to start implementing the firmware needed for the carrier board 2.0 architecture. Create a carrier board firmware document

#### Resolution Notes

Created carrier board 2.0 shell crate with hello-world binary, tests, hardware constants, and comprehensive docs/carrier_board_firmware.md guide.

---

### <a id="worm-006"></a> 🟢 `[WORM-006]` Forgot to update the VCS UI

- **UUID**: `e06664ae-ef13-4658-b71f-1db435ab8e9e`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Created**: `2026-10-03 22:27:07 UTC`
- **Resolved**: `2026-10-03 22:55:03 UTC`

#### Description

Affects commit: 330e1abbddeeedac588a5fe59eb434f28b75171b

Screenshot attached. Also, none of the WORMs are resolved

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [pasted_screenshot_1791066480358.png](attachments/WORM-006/pasted_screenshot_1791066480358.png) | Pasted screenshot |

#### Resolution Notes

Canonicalize commit worm tag IDs to WORM-xxx and query worms.sqlite without being overwritten by stale bugs.sqlite

---

### <a id="worm-007"></a> 🟢 `[WORM-007]` Word wrap doesn't seem to be working

- **UUID**: `becde191-82f5-4ff3-bb35-02e41e4d971e`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-03 22:36:51 UTC`
- **Resolved**: `2026-10-03 22:59:19 UTC`

#### Description

When I turn on word wrap in the dashboard tool, text still seems to be unwrapped

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [pasted_screenshot_1791067074636.png](attachments/WORM-007/pasted_screenshot_1791067074636.png) | Pasted screenshot |

#### Resolution Notes

Fix word wrap CSS rules for unified and side-by-side diff views in code review and diff component

---

### <a id="worm-008"></a> 🟢 `[WORM-008]` Update Contributing.md with dashboard docs

- **UUID**: `e187e841-c48f-4e9f-9c07-7eac80415538`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-10-03 23:51:07 UTC`
- **Resolved**: `2026-10-03 23:56:31 UTC`

#### Resolution Notes

Added comprehensive documentation to CONTRIBUTING.md covering the System Shock 2 - Xerxes dashboard, CLI subcommands, web workstation views, SQLite and markdown synchronization, Git LFS attachments, and pre-commit invariants.

---

### <a id="worm-009"></a> 🟢 `[WORM-009]` Still seeing issues with multiple Diff Viewer windows being open after exiting from the Code Review and Bug Report windows

- **UUID**: `454cbbfa-94ce-4f56-8fa8-99fca7fc20de`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `INFRASTRUCTURE`
- **Created**: `2026-10-04 00:20:04 UTC`
- **Resolved**: `2026-10-04 00:31:49 UTC`

#### Resolution Notes

Enforced window.name reuse on Diff Viewer, Code Review, and Worm Tracker; updated returnToDashboard and setVerdict to focus opener or existing named window and close instead of spawning duplicate windows

---

### <a id="worm-010"></a> 🟢 `[WORM-010]` "Open in VSCode" didn't work

- **UUID**: `8e571b93-ae93-4737-b018-7a2c824c4aca`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-10-04 00:40:02 UTC`
- **Resolved**: `2026-10-04 01:31:02 UTC`

#### Description

Clicked on "Open in VSCode" next to app/carrier_board.md, and nothing seemed to happen. I was viewing the website in the VSCode simple browser window

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `document` | [pasted_document_1791074450389.txt](attachments/WORM-010/pasted_document_1791074450389.txt) | Pasted text document |

#### Resolution Notes

Add server /api/open-vscode endpoint and editor launcher with CLI and URL scheme fallback for embedded VS Code Simple Browser

---

### <a id="worm-011"></a> 🟢 `[WORM-011]` Traceback in dashboard

- **UUID**: `e11627c9-9cf6-4ddf-9ca0-57959b0fedbe`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-04 16:43:17 UTC`
- **Resolved**: `2026-10-04 16:57:36 UTC`

#### Description

Traceback after leaving dashboard running overnight

#### Execution / Console Logs

```text
File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64052)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64053)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64055)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64056)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64057)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64058)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64059)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64060)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64061)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64062)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64063)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64064)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64065)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64066)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64067)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64068)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64069)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64070)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64071)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64072)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64073)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64074)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64075)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64076)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64077)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64078)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64079)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64080)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64081)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64082)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64083)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64084)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64085)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64086)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64087)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64088)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64089)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64090)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64092)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64093)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64094)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64095)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64096)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64097)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64098)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64099)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64100)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64101)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64102)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64103)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64105)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64107)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64108)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64109)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64110)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64111)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64112)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64113)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64114)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64115)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64116)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64117)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64118)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64119)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64120)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64121)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64122)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64123)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64124)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64125)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64126)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64127)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64128)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64129)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64130)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64131)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64132)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64133)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64134)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64135)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64136)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64137)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64138)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64139)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64140)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64141)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64142)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64143)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64144)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64145)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64146)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64147)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64148)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64149)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64150)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64151)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64152)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64153)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64154)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64155)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64156)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64157)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64158)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64159)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64160)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64161)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64162)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64163)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64164)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64165)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64166)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64167)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64168)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64169)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64170)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64171)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64172)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64173)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64174)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64175)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64176)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64177)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64178)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64179)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64180)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64181)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64182)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64183)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64184)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64185)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64186)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64187)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64188)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64189)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64190)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64191)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64192)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64193)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64194)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64195)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64196)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64197)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64198)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64199)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64200)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64201)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64202)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64203)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64204)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64205)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64206)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64207)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64208)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64209)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64210)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64211)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64212)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64213)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64214)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64215)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64216)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64217)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64219)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64220)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64221)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64222)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64223)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64224)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64225)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64226)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64227)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64228)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64229)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64230)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64231)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64232)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64233)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64234)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64235)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64236)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64237)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64238)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64239)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64240)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64241)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64242)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64243)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 167, in do_GET
    files = self.server.git_engine.get_changed_files(commit, include_feedback=include_feedback)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 1412, in get_changed_files
    status_out = run_git_command(cmd_status, cwd=self.repo_root)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64244)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 167, in do_GET
    files = self.server.git_engine.get_changed_files(commit, include_feedback=include_feedback)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 1412, in get_changed_files
    status_out = run_git_command(cmd_status, cwd=self.repo_root)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64245)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 167, in do_GET
    files = self.server.git_engine.get_changed_files(commit, include_feedback=include_feedback)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 1412, in get_changed_files
    status_out = run_git_command(cmd_status, cwd=self.repo_root)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64246)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 167, in do_GET
    files = self.server.git_engine.get_changed_files(commit, include_feedback=include_feedback)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 1412, in get_changed_files
    status_out = run_git_command(cmd_status, cwd=self.repo_root)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64247)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64248)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64249)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64250)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64251)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64252)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64253)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64254)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 49926)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 423, in do_POST
    self._handle_update_verdict(data)
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 722, in _handle_update_verdict
    out_path = self.server.review_server.save_and_sync()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/code_review/server.py", line 575, in save_and_sync
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/code_review/sqlite_store.py", line 254, in save_session
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/contextlib.py", line 141, in __enter__
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/code_review/sqlite_store.py", line 43, in _get_connection
sqlite3.OperationalError: unable to open database file
----------------------------------------
^C
Terminating neural link via interrupt signal...
Xerxes workstation offline. Terminal released.
```

#### Resolution Notes

Resolved unclosed SQLite connections in build_session() and ensured immediate connection closure in DashboardRequestHandler to prevent open file descriptor exhaustion.

---

### <a id="worm-012"></a> 🟢 `[WORM-012]` Rename audio controller to speaker controller

- **UUID**: `6dab9263-f0c5-499d-bc7b-aa0db863a03d`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `CONTROLLER`
- **Component**: `carrier_board`
- **Created**: `2026-10-04 16:43:43 UTC`
- **Resolved**: `2026-10-04 16:54:31 UTC`

#### Description

Rename the AudioController definition in carrier_board.md to SpeakerController

#### Resolution Notes

Renamed AudioController and controller::audio_controller to SpeakerController and controller::speaker_controller in app/carrier_board.md and updated regression test in test_carrier_board_project.py.

---

### <a id="worm-013"></a> 🟢 `[WORM-013]` Feedbacks about carrier_board.md

- **UUID**: `6e2400a4-8fa5-46fd-938a-58996597f407`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `GENERAL`
- **Component**: `carrier_board`
- **Created**: `2026-10-04 16:50:42 UTC`
- **Resolved**: `2026-10-04 17:11:10 UTC`

#### Description

Some more general feedbacks about carrier_board.md:
-host_fs should support EEPROM programming in the same way it can program other flash devices
-eeprom should be sequential storage based in a sequential format similar to USB descriptors
-the ProgramMetadata section should contain all the partition and descriptor info for a given firmware target
-we want to use FIFO for this design-cap touch, and otherwise. can we use an on chip high resolution timer to integrate and timestamp buffered samples, where needed? we also need to elaborate on how FIFO buffering will work with sensor fusion
-Production Manufacturing & Factory Provisioning- going with this approach, I think we also need some kind of structure logging and status display on the host side
-under section 6-the definition of PowerDown is diff than the one rp2040 uses; rp2040 uses these states: Active/Sleep/PowerDown. They will use the same system controller. Can we resolve this?

#### Resolution Notes

Updated app/carrier_board.md with all 6 architecture requirements: dev:eeprom host_fs support, USB-descriptor sequential-storage TLV format, ProgramMetadata partition table section, hardware FIFO and CTIMER microsecond timestamping for sensor fusion, host_cli structured logging and terminal HUD for factory provisioning, and power state harmonization with RP2040 SystemController SystemStatus (Active, Sleep, PowerDown). Added regression assertions in test_carrier_board_project.py and verified with ./tools/verify.sh.

---

### <a id="worm-014"></a> 🟢 `[WORM-014]` Unresolved worms in VCS UI

- **UUID**: `a9d4483b-b38d-4f56-bdef-7ef9e5b1be0a`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-07 02:29:34 UTC`
- **Resolved**: `2026-10-07 02:38:05 UTC`

#### Description

I see these two unresolved worms in the VCS UI which seem to be an artifact of porting changes over from the hardware repo

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [pasted_screenshot_1791340187710.png](attachments/WORM-014/pasted_screenshot_1791340187710.png) | Pasted screenshot |

#### Resolution Notes

Prevent phantom OPEN worm tags in VCS UI for non-existent bug IDs ported from external repositories

---

### <a id="worm-015"></a> 🟢 `[WORM-015]` Unable to open code review or worm report pages from VCS UI

- **UUID**: `5d358f8e-a5fa-4bfc-ba86-0231449f60e8`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-07 03:39:52 UTC`
- **Resolved**: `2026-10-07 03:41:04 UTC`

#### Description

workstationModal in diff_view.html.j2 was mistakenly nested inside modalBranchPicker due to a missing closing div tag on modalBranchPicker introduced in f4fc95e2. Because modalBranchPicker has display: none, workstationModal was never visible when opened.

#### Resolution Notes

Closed modalBranchPicker div properly so workstationModal is mounted directly under body rather than being hidden inside modalBranchPicker.

---

### <a id="worm-016"></a> 🟢 `[WORM-016]` Remove my username from tracked files

- **UUID**: `732d3b51-3220-448f-95a3-491046668c64`
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

---

### <a id="worm-017"></a> 🔴 `[WORM-017]` Working tree actions doesn't show up unless I click an older commit

- **UUID**: `064fec38-c4a1-436a-ae12-26ea26844579`
- **Status**: `OPEN`
- **Severity**: `HIGH`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-07 23:26:10 UTC`

#### Description

Working tree actions doesn't show up unless I click an older commit, then click on the working tree changes and staging area

---

### <a id="worm-018"></a> 🔴 `[WORM-018]` Populate component list

- **UUID**: `53d3ceb2-8a2f-4117-8fa7-f1c9c2160daa`
- **Status**: `OPEN`
- **Severity**: `LOW`
- **Category**: `INFRASTRUCTURE`
- **Created**: `2026-10-07 23:27:07 UTC`

#### Description

Populate the component textbox with an autocomplete list based off prior worm history

---
