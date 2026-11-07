# 🟢 `[WORM-FLESHY-WOODLOUSE-266]` Not abel to commit updated feedbacks through VCS UI

- **UUID**: `f49f45d9-fb73-4558-a7dc-915a6288625a`
- **ID**: `WORM-FLESHY-WOODLOUSE-266`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-11-06 23:58:59 UTC`
- **Resolved**: `2026-11-07 00:11:59 UTC`

#### Description

I was not able to commit updated feedbacks through the VCS UI, instead I had to git on the shell with the ‘-f’ flag to add them to the repo.

#### Execution / Console Logs

```text
$ git add feedback/WORM_ORGANIC-FIREWORM-553.md
The following paths are ignored by one of your .gitignore files:
feedback
hint: Use -f if you really want to add them.
hint: Disable this message with "git config set advice.addIgnoredFile false"
(firmware-env) 
$ git add -f feedback/WORM_ORGANIC-FIREWORM-553.md
(firmware-env)
```

#### Attached Commits

- `02cf15ed07efd9c105ad04ae238719bc01a4dbfa`
- `2e4061fbdfb35122ea541e21b068202d0be9f935`

#### Review Tasks & Functional Checklist

- [ ] Verify git status displays untracked/modified feedback files without being blocked by gitignore | `git status feedback/` <!-- id:d2fc9bac -->
- [ ] Verify GitEngine stages and commits feedback files without requiring git add -f | `python -c 'from tools.dashboard.provider.vcs.git_engine import GitEngine; from pathlib import Path; GitEngine(Path(".")).stage_file("feedback/WORM_FLESHY-WOODLOUSE-266.md")'` <!-- id:814a31c1 -->
- [ ] Inspect VCS web workstation working tree view to verify feedback files appear under Agent Feedback | `python tools/dashboard.py --list` <!-- id:57b289a0 -->

#### Resolution Notes

Removed feedback/ directory from root .gitignore across firmware and hardware repositories to allow git tracking and VCS UI working tree discovery of code review and worm markdown reports, while preserving ignore rules for binary attachments (attachments/ and feedback/attachments/). Added defensive force-add fallback in GitEngine.stage_file and GitEngine.commit_files to transparently honor explicit user staging and commit actions even if a file pattern matches .gitignore.
