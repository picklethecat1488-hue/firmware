# 🔴 `[WORM-TRANSLUCENT-PLANARIAN-439]` Exclude tests targeting a different board type when 'All Binaries' is selected

- **UUID**: `4c1a0a02-df57-47ae-915a-bd318ddf9011`
- **ID**: `WORM-TRANSLUCENT-PLANARIAN-439`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `tools/host_daemon/provider/daemon/test_manager.py`

#### Description

In Test Mode UI, selecting 'All Binaries' currently includes @device_test suites and binary facets targeting incompatible board types (e.g. carrier_board* tests when rp2040_flash_2mb / cat_detector is active). 'All Binaries' should exclude tests and binaries that target a different board type.
