# 🔴 `[WORM-003]` GEMINI.md should be core agent skills only

- **UUID**: `45a673a6-ff51-4901-b831-230917b4e348`
- **ID**: `WORM-003`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-03 19:27:20 UTC`

#### Description

We made a change in the hardware repo so that GEMINI.md would be for core agent skills only, and subsystem-specific skills would be moved to different files under a docs/ directory to reduce token burn. Can you locate this change in the hardware repo, and apply the same mandate here? We also need to add a back-combat mandate to firmware projects- basically, unless duly noted any firmware code should apply to all firmware targets in the repo.
