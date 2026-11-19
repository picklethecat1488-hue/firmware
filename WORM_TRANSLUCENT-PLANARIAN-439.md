# 🟢 `[WORM-TRANSLUCENT-PLANARIAN-439]` Exclude tests targeting a different board type when 'All Binaries' is selected

- **UUID**: `4c1a0a02-df57-47ae-915a-bd318ddf9011`
- **ID**: `WORM-TRANSLUCENT-PLANARIAN-439`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `tools/host_daemon/provider/daemon/test_manager.py`
- **Created**: `2026-11-19 03:49:45 UTC`
- **Resolved**: `2026-11-19 04:02:36 UTC`

#### Description

In Test Mode UI, selecting 'All Binaries' currently includes @device_test suites and binary facets targeting incompatible board types (e.g. carrier_board* tests when rp2040_flash_2mb / cat_detector is active). 'All Binaries' should exclude tests and binaries that target a different board type.

#### Attached Commits

- `c00249442c159c1887ed4db7a99e7f69fb07b29f`

#### Review Tasks & Functional Checklist

- [ ] Open host.py Test Mode with RP2040 attached and confirm 'All Binaries' lists only rp2040* tests and cat_detector_* binaries | `python tools/host.py test --list-devices` <!-- id:7c2b3c8a -->

#### Resolution Notes

Resolved board_type per binary from app/Cargo.toml + board/board.toml and updated DeviceTestRunnerManager.scan_tests() so 'All Binaries' excludes tests and binary facets targeting incompatible board types.
