# Worm Report Tracker: Hardware Engineering Bug Tracker

> Automated bug tracking, triage, and issue registry generated via Firmware Bug Report Engine.

## Tracker Overview

| Metric | Details |
| :--- | :--- |
| **Report Date** | `2026-10-15 20:59:31 UTC` |
| **Total Issues** | `34` |
| **Open Issues** | `2` |
| **Resolved / Closed** | `32 (94%)` |

## Issues by Severity

| Severity | Count | Meaning |
| :--- | :---: | :--- |
| **`[CRITICAL]`** | 0 | System crashes, build failures, blockages, or electrical shorts. |
| **`[HIGH]`** | 4 | Major functional defects, broken routing, DRC violations, or unphysical behavior. |
| **`[MEDIUM]`** | 27 | Silkscreen collisions, layout sub-optimality, or visual clipping. |
| **`[LOW]`** | 3 | Minor aesthetic imperfections or documentation notes. |

## Issues by Category

| Category | Count | Description |
| :--- | :---: | :--- |
| **`FIRMWARE`** | 6 | Core firmware logic, state machines, async tasks. |
| **`CONTROLLER`** | 1 | Domain controllers, PID loops, event dispatch. |
| **`DRIVER`** | 0 | Hardware peripheral drivers, embedded-hal. |
| **`PLATFORM`** | 1 | Chip support, PAC, HAL, clock/power management. |
| **`BOARD`** | 1 | Board support packages (BSP), pinmux, boards. |
| **`MODEL`** | 0 | Domain data models, configurations, state definitions. |
| **`SHELL`** | 0 | CLI interface, shell commands, debug console. |
| **`INFRASTRUCTURE`** | 23 | Build tooling, compilers, test runners, headless tools. |
| **`UI`** | 0 | Web dashboards, CLI viewers, review interfaces. |
| **`GENERAL`** | 2 | Unclassified or cross-cutting firmware issues. |

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
- [x] **`[HIGH]`** [#WORM-017](#worm-017): Working tree actions doesn't show up unless I click an older commit `[dashboard]` (`RESOLVED`)
- [x] **`[LOW]`** [#WORM-018](#worm-018): Populate component list (`RESOLVED`)
- [x] **`[LOW]`** [#WORM-019](#worm-019): Port dashboard to Eel `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-020](#worm-020): Reduce dashboard font sizes `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-021](#worm-021): Detach dashboard from terminal after opening `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-022](#worm-022): Dashboard App icons show up as Chrome icons `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-023](#worm-023): Add Planned status to worms `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-024](#worm-024): UI icons appear corrupted `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-025](#worm-025): Close the dashboard when closing the app `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-026](#worm-026): It doesn’t look like the dashboard is closing automatically :-( `[Das]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-027](#worm-027): Apply dashboard theme to Window UI `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-028](#worm-028): Create separate firwmware architecture docs `[cat_fountain, carrier_board]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-029](#worm-029): Replace integer ID’s with Docker-pattern ID's `[dashboard]` (`RESOLVED`)
- [x] **`[MEDIUM]`** [#WORM-UNDULATING-REDWORM-56](#worm-undulating-redworm-56): README.md is broken (`RESOLVED`)
- [ ] **`[MEDIUM]`** [#WORM-GLIDING-BRISTLE-252](#worm-gliding-bristle-252): New Worm Report (`OPEN`)
- [ ] **`[MEDIUM]`** [#WORM-EARTHEN-NYMPH-383](#worm-earthen-nymph-383): SensorController: Periodic read failed. (`OPEN`)
- [x] **`[MEDIUM]`** [#WORM-LOAMY-BRISTLE-455](#worm-loamy-bristle-455): Elide physical address and personal email in privacy sanitizer `[sanitizer]` (`RESOLVED`)
- [x] **`[HIGH]`** [#WORM-TRANSLUCENT-TAPEWORM-953](#worm-translucent-tapeworm-953): Dashboard stuck at 20% on launch `[dashboard]` (`RESOLVED`)

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

### <a id="worm-017"></a> 🟢 `[WORM-017]` Working tree actions doesn't show up unless I click an older commit

- **UUID**: `064fec38-c4a1-436a-ae12-26ea26844579`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-07 23:26:10 UTC`
- **Resolved**: `2026-10-08 02:42:55 UTC`

#### Description

Working tree actions doesn't show up unless I click an older commit, then click on the working tree changes and staging area

#### Resolution Notes

Updated diff_view.html.j2 template to dynamically render workingTreeCommitArea with display: flex and filesHeaderTitle as 'Working Tree Files' when active_commit is 'working', and added DOMContentLoaded and loadFilesForActiveCommit display state synchronization.

---

### <a id="worm-018"></a> 🟢 `[WORM-018]` Populate component list

- **UUID**: `53d3ceb2-8a2f-4117-8fa7-f1c9c2160daa`
- **Status**: `RESOLVED`
- **Severity**: `LOW`
- **Category**: `INFRASTRUCTURE`
- **Created**: `2026-10-07 23:27:07 UTC`
- **Resolved**: `2026-10-08 02:49:15 UTC`

#### Description

Populate the component textbox with an autocomplete list based off prior worm history

#### Resolution Notes

Added <datalist id="worm-component-list"> to worm_report.html.j2 connected to worm-component input, dynamically populated and kept up to date via updateComponentDatalist() from prior worm history, with server-side extraction fallback. Added regression unit test test_regression_worm_018_component_autocomplete_datalist.

---

### <a id="worm-019"></a> 🟢 `[WORM-019]` Port dashboard to Eel

- **UUID**: `12787545-0734-473d-9ff2-6c2f38ee7191`
- **Status**: `RESOLVED`
- **Severity**: `LOW`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-08 19:50:38 UTC`
- **Resolved**: `2026-10-08 20:01:42 UTC`

#### Description

Port the dashboard to Eel as a standalone app. Maintain the general UI, but open the dashboard in a standalone window, instead of a web browser.

#### Resolution Notes

Integrated Eel standalone desktop window support with browser discovery, /eel.js route, and --eel/--app CLI options

---

### <a id="worm-020"></a> 🟢 `[WORM-020]` Reduce dashboard font sizes

- **UUID**: `680c7663-029b-47a5-9f21-9ccfa0d5c73a`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-08 21:05:29 UTC`
- **Resolved**: `2026-10-08 21:13:25 UTC`

#### Description

Please reduce the dashboard font sizes to about 80% of what they are now. They are currently too large.

---

### <a id="worm-021"></a> 🟢 `[WORM-021]` Detach dashboard from terminal after opening

- **UUID**: `61e94a99-68fb-4ea0-9c7b-f4f4d09bd59f`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-08 21:06:01 UTC`
- **Resolved**: `2026-10-08 21:26:46 UTC`

#### Description

Add a check to dashboard.py to ensure that only one dashboard can run at a time, and detach the dashboard from the current terminal after loading completes.

#### Resolution Notes

Added single-instance check preventing duplicate dashboard processes using lock file and port check, and implemented terminal detachment after loading completes with --no-detach/--foreground and --stop CLI controls.

---

### <a id="worm-022"></a> 🟢 `[WORM-022]` Dashboard App icons show up as Chrome icons

- **UUID**: `8fb06a58-7b19-4768-8ee9-bbf0b9b0187a`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-08 21:06:54 UTC`
- **Resolved**: `2026-10-08 21:33:24 UTC`

#### Description

The dashboard icons show up as ugly Chrome icons in the taskbar. Please ensure the correct themed PWA app icon shows up in the taskbar.

#### Resolution Notes

Added pregenerated app.icns, updated manifest.json display_override, and implemented ensure_macos_app_bundle in launcher.py to launch standalone window via signed macOS .app shim displaying themed icon in Dock and dark title bar.

---

### <a id="worm-023"></a> 🟢 `[WORM-023]` Add Planned status to worms

- **UUID**: `be9303ee-9ed8-4d56-a20c-0974ab03587c`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-08 21:08:10 UTC`
- **Resolved**: `2026-10-08 21:18:20 UTC`

#### Description

Add a "Planned" status to Worms, and update skills and dashboard CLI so that the agent will not work on Planned Worms by default. Keep the default status in the Worm console as Open.

#### Resolution Notes

Added PLANNED status to WormStatus enum, updated CLI to exclude PLANNED from --open and added --planned filter, updated worm report template to include PLANNED tab with default OPEN status, and documented agent planned worm exemption in GEMINI.md and skill.

---

### <a id="worm-024"></a> 🟢 `[WORM-024]` UI icons appear corrupted

- **UUID**: `c5a08be1-3c15-4624-8731-c5d5874bf55e`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-09 00:28:30 UTC`
- **Resolved**: `2026-10-09 00:36:21 UTC`

#### Description

Attached below, both dashboard UI icons appear corrupted

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [image.png](attachments/WORM-024/image.png) | Uploaded screenshot image.png |
| `screenshot` | [image.png](attachments/WORM-024/image.png) | Uploaded screenshot image.png |

#### Resolution Notes

Rasterize SVG icons at full resolution with alpha transparency, proper centered coordinates, and generate high-res multi-resolution app.icns up to 1024x1024 for Retina macOS Dock and taskbar

---

### <a id="worm-025"></a> 🟢 `[WORM-025]` Close the dashboard when closing the app

- **UUID**: `04a45bbe-385d-4912-99b4-3363835f8632`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-09 00:32:10 UTC`
- **Resolved**: `2026-10-09 00:48:41 UTC`

#### Description

Can we close the dashboard server when closing the app window? No point in keeping it running after the app closes.

#### Behavior Comparison

- **Expected**: $ python tools/dashboard.py
- **Actual**: $ python tools/dashboard.py

#### Resolution Notes

Register on_close callback in webview and eel launchers to cleanly terminate background dashboard server and unlink lock file upon window closure

---

### <a id="worm-026"></a> 🟢 `[WORM-026]` It doesn’t look like the dashboard is closing automatically :-(

- **UUID**: `f863343e-01e3-4a38-a010-6dbb55320a05`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `Das`
- **Created**: `2026-10-09 02:49:22 UTC`
- **Resolved**: `2026-10-09 03:13:46 UTC`

#### Description

See traceback below

#### Execution / Console Logs

```text
<username> at THE-HEARTH-NODE in ~/gh/firmware (main●●) 
$ python tools/dashboard.py 
Dashboard workstation is already running at http://127.0.0.1:8877 (PID 99042).
(firmware-env) 
<username> at THE-HEARTH-NODE in ~/gh/firmware (main●●) 
$ python tools/dashboard.py
Dashboard workstation is already running at http://127.0.0.1:8877 (PID 99042).
(firmware-env) 
<username> at THE-HEARTH-NODE in ~/gh/firmware (main●●) 
$ python tools/dashboard.py
Dashboard workstation is already running at http://127.0.0.1:8877 (PID 99042).
(firmware-env)
```

#### Resolution Notes

Hook window closed event to terminate pywebview process cleanly on window exit, ensure server shutdown unlinks lock file and exits, and automatically detect and recover orphaned background server instances.

---

### <a id="worm-027"></a> 🟢 `[WORM-027]` Apply dashboard theme to Window UI

- **UUID**: `5965947b-7974-4334-933d-7f00c32c6ac7`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-09 02:50:05 UTC`
- **Resolved**: `2026-10-09 03:26:33 UTC`

#### Description

Can you please apply the dashboard theme to the window UI, and add a gripper to the bottom right or make the resize borders of the window more visible?

#### Resolution Notes

Apply dashboard theme to native window UI with dark appearance and transparent title bar, add high-contrast themed border to window body, and add retro-styled resize gripper to the bottom right corner.

---

### <a id="worm-028"></a> 🟢 `[WORM-028]` Create separate firwmware architecture docs

- **UUID**: `7b3d890e-45a1-43b8-a232-037f0a818cd3`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `PLATFORM`
- **Component**: `cat_fountain, carrier_board`
- **Created**: `2026-10-09 20:18:18 UTC`
- **Resolved**: `2026-10-09 20:25:32 UTC`

#### Description

Hello, can you please create separate architecture docs for the rp2040-based cat fountain firmware architecture, and NXP MCX 947-based carrier board firmware arcitecture, with a top level firmware architecture doc. Then update the architectural definitions in README.md, cat_fountain.md, and carrier_board.md to refer to these separate architecture docs, simplifying all three existing documents in the process.

#### Resolution Notes

Created separate architecture docs docs/firmware_architecture.md, docs/cat_fountain_architecture.md, and docs/carrier_board_architecture.md. Updated and simplified README.md, app/cat_detector.md, app/cat_fountain.md, and app/carrier_board.md with links to dedicated architecture guides.

---

### <a id="worm-029"></a> 🟢 `[WORM-029]` Replace integer ID’s with Docker-pattern ID's

- **UUID**: `b52165bd-d3e5-409a-bb0c-2f3ff9284a27`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `dashboard`
- **Created**: `2026-10-09 20:51:32 UTC`
- **Resolved**: `2026-10-09 21:02:49 UTC`

#### Description

Replace the integer-based issue ID’s with Docker-pattern ID’s which are generated at random using a CRNG (E.g. openssl’s rand function). The issue format should be:

   • Format: [ADJECTIVE]-[ANIMAL/NOUN]-[3 digits]
 ┃   • Examples:
 ┃       • WORM-SWIFT-FOX-42
 ┃       • WORM-BOLD-LYNX-809
 ┃       • WORM-IRON-CRANE-17

Please compress the adjective and noun lists to save code space.

#### Resolution Notes

Replaced integer-based issue IDs with Docker-pattern IDs generated via CRNG with compressed word lists.

---

### <a id="worm-undulating-redworm-56"></a> 🟢 `[WORM-UNDULATING-REDWORM-56]` README.md is broken

- **UUID**: `dc740603-beaa-4db2-b995-bcaa130a0ba2`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `GENERAL`
- **Created**: `2026-10-15 03:35:22 UTC`
- **Resolved**: `2026-10-15 03:38:54 UTC`

#### Description

See the error below

#### Execution / Console Logs

```text
Unable to render rich display

Parse error on line 32:
...s tasks| Controller Layer ShellBin -
-----------------------^
Expecting 'SEMI', 'NEWLINE', 'EOF', 'AMP', 'START_LINK', 'LINK', 'LINK_ID', got 'NODE_STRING'

For more information, see https://docs.github.com/get-started/writing-on-github/working-with-advanced-formatting/creating-diagrams#creating-mermaid-diagrams
```

#### Resolution Notes

Fixed syntax error in README.md Mermaid diagram by replacing unquoted space-separated subgraph identifiers with valid quoted titles and identifiers, updated edge targets, and added regression tests in tools/tests/test_worm_undulating_redworm_56_mermaid_syntax.py.

---

### <a id="worm-gliding-bristle-252"></a> 🔴 `[WORM-GLIDING-BRISTLE-252]` New Worm Report

- **UUID**: `7e4bd5e7-82e8-44b4-bce7-6b05b7cfbc56`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-10-15 20:53:40 UTC`

---

### <a id="worm-earthen-nymph-383"></a> 🔴 `[WORM-EARTHEN-NYMPH-383]` SensorController: Periodic read failed.

- **UUID**: `a177e2b9-941f-4bd8-85ad-bc66c2789e13`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-10-15 20:53:09 UTC`

#### Description

I see a warning message from SEnsorController when I run `GpioStatus`

#### Reproduction Steps

1. 1. flash shell app
2. 2. run `gpio status`

#### Execution / Console Logs

```text
gpio status
g372.454013 INFO [controller::shell_controller::log_command] received command "Gpio { subcommand: Some(Status), pin: None }"
372.454165 INFO [controller::shell_controller::log_success] command execution succeeded
GPIO Pin Status (0..29):
Pin   Level
-----------
GP0   Low  (0)
GP1   Low  (0)
GP2   Low  (0)
GP3   Low  (0)
GP4   Hih (1)
GP5   High (1)
GP6   High (1)
GP7   Low  (0)
GP8   Low  (0)
GP9   High (1)
GP10  High (1)
GP11  Low  (0)
GP12  Low  (0)
GP13  High (1)
GP14  High (1)
GP15  High (1)
GP16  High (1)
GP17  Low  (0)
GP18  Low  (0)
GP19  Low  (0)
GP20  High (1)
GP21  Low  (0)
GP22  Low  (0)
GP23  Low  (0)
GP24  Low  (0)
GP25  Low  (0)
GP26  High (1)
GP27  Low  (0)
GP28  High (1)
GP29  High (1)

shell> 372.458954 WARN [controller::sensor_controller::{impl#11}::run::{async_fn#0}] SensorController: Periodic read failed.
372.509120 WARN [controller::sensor_controller::{impl#11}::run::{async_fn#0}] SensorController: Periodic read failed.
```

---

### <a id="worm-loamy-bristle-455"></a> 🟢 `[WORM-LOAMY-BRISTLE-455]` Elide physical address and personal email in privacy sanitizer

- **UUID**: `09c3136e-35c6-442e-9708-46ac711fa0ed`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `sanitizer`
- **Created**: `2026-10-15 00:37:10 UTC`
- **Resolved**: `2026-10-15 00:37:11 UTC`

#### Description

Extend the zero-dependency personal privacy and identity sanitizer (`tools/dashboard/provider/sanitizer.py`) to systematically redact physical addresses (street names, numbers, suite/apt/unit designators, P.O. Boxes, and postal/ZIP codes) and personal email addresses (user handles, consumer email domains like `@gmail.com`, `@me.com`, etc.) across developer feedback, bug reports, and logs. Additionally, update VCS engine commit author and email mapping to redact user author identities to `User` and `user@workspace`.

#### Resolution Notes

1. Added physical street address, unit/suite, and P.O. Box regex redaction to `<address>` and `<zip>` in `elide_personal_info()`, using pure regex matching without requiring an engineering whitelist or supplier address preservation.
2. Dynamically discovered user emails, handles, and system usernames to map all personal identifiers and consumer email domains to `<email>`.
3. Updated `elide_commit_author()` and `elide_commit_email()` in `tools/dashboard/provider/sanitizer.py` to map user author identities and commit emails to `User` and `user@workspace` via dynamic discovery.
4. Updated VCS workstation frontend `sanitizeAuthor()` in `diff_view.html.j2` and `code_review.html.j2` to read the author blacklist dynamically from the backend session.
5. Added dedicated regression unit tests in `tools/tests/test_privacy_and_relative_paths.py`.

---

### <a id="worm-translucent-tapeworm-953"></a> 🟢 `[WORM-TRANSLUCENT-TAPEWORM-953]` Dashboard stuck at 20% on launch

- **UUID**: `3e17a613-50d8-4c90-9e62-24e63b8f13fe`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-15 00:32:11 UTC`
- **Resolved**: `2026-10-15 00:32:17 UTC`

#### Description

Upon launching or restarting the dashboard workstation, the UI was stuck displaying the feedback database synchronization modal at 20% progress. Inspection revealed a JavaScript `SyntaxError: Unexpected token ')'` in `diff_view.html.j2:1705` within the `sanitizeAuthor()` function introduced during privacy sanitization hardening. Because the script failed to parse, `initDiffView()` and `performInitialSync()` never ran, leaving the modal stuck at its initial 20% HTML state.

#### Resolution Notes

1. Pruned dangling closing parenthesis in `sanitizeAuthor()` condition in `tools/dashboard/templates/diff_view.html.j2`.
2. Hardened workstation initialization lifecycle so `initDiffView()` fires immediately if `document.readyState !== 'loading'`, eliminating dependency on potentially pre-fired `DOMContentLoaded` events.
3. Added comprehensive regression test `test_regression_bug_translucent_tapeworm_953_dashboard_js_syntax_and_sync_screen` in `tools/tests/test_dashboard.py` verifying balanced syntax and executing `node -c` across extracted `<script>` blocks in all dashboard templates (`diff_view.html.j2`, `code_review.html.j2`, `worm_report.html.j2`).

---
