# 🟢 `[WORM-MOIST-MILLIPEDE-488]` Host flashes cat_detector_app instead of shell due to session contamination

- **UUID**: `fce239b7-1b59-46e6-931e-0a1d7d0c0bce`
- **ID**: `WORM-MOIST-MILLIPEDE-488`
- **Status**: `RESOLVED`
- **Severity**: `HIGH`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host_daemon`
- **Created**: `2026-10-22 01:38:41 UTC`
- **Resolved**: `2026-10-22 01:42:18 UTC`

#### Description

When flashing or running host daemon without explicit target arguments, cat_detector_app gets flashed instead of cat_detector_shell. Root cause: test_worm_composted_palolo_725 overwrites target/host_session.json with cat_detector_app during pytest runs, discover_target_packages defaults to first binary in Cargo.toml, and app/Cargo.toml lacks default-run.

#### Resolution Notes

Configured default-run = 'cat_detector_shell' and prioritized shell binaries in target discovery. Isolated test sessions with tmp_path and guarded save_last_session against PYTEST_CURRENT_TEST to prevent test suites from polluting the developer's live workspace host session.
