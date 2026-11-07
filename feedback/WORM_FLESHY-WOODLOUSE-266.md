# 🟢 `[WORM-FLESHY-WOODLOUSE-266]` Not abel to commit updated feedbacks through VCS UI

- **UUID**: `f49f45d9-fb73-4558-a7dc-915a6288625a`
- **ID**: `WORM-FLESHY-WOODLOUSE-266`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-11-06 23:58:59 UTC`
- **Resolved**: `2026-11-07 00:22:38 UTC`

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
- `032ab77c020b1a4bda7c34175d9cdcebf9f66816`

#### Review Tasks & Functional Checklist

- [ ] Verify git status on code branches is clean and does not show untracked feedback directory | `git status` <!-- id:d2fc9bac -->
- [ ] Verify .gitignore decouples feedback/ and attachments/ | `git check-ignore feedback/ attachments/` <!-- id:814a31c1 -->
- [ ] Verify GitEngine stages and commits feedback files using force-add fallback without raising gitignore errors | `python -c 'from tools.dashboard.provider.vcs.git_engine import GitEngine; from pathlib import Path; GitEngine(Path(".")).stage_file("feedback/WORM_FLESHY-WOODLOUSE-266.md")'` <!-- id:57b289a0 -->

#### Resolution Notes

Restore feedback/ in .gitignore across firmware and hardware to decouple to detached branches; preserve GitEngine force-add fallback for UI staging and commits
