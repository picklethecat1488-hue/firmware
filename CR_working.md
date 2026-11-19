# Code Review Report: Code Review: firmware

> Automated code review log and findings generated via Quake Code Review Engine.

## Review Overview

| Metric | Details |
| :--- | :--- |
| **Review Date** | `2026-11-19 02:42:55 UTC` |
| **Revisions** | `working` |
| **Overall Verdict** | **`IN_REVIEW`** |
| **Review Progress** | `0/1 files reviewed (0%)` |
| **Total Comments** | `15 findings` |

## Findings by Severity

| Severity | Count | Meaning |
| :--- | :---: | :--- |
| **[MUST FIX]** | 15 | Must be resolved before merge; bugs, defects, or safety regressions. |
| **[PROPOSAL]** | 0 | Architecture ideas, design proposals, or optional enhancements. |
| **[NIT]** | 0 | Minor formatting, naming, or cosmetic cleanups. |

## File-by-File Review Findings

### [`.cargo/config.toml`](file:///Users/<username>/gh/firmware/.cargo/config.toml) — ⏳ `PENDING`

#### **[MUST FIX]** [.cargo/config.toml:L12](file:///Users/<username>/gh/firmware/.cargo/config.toml#L12)
<!-- comment-uuid: 4bac1bf1-44df-4c9f-bf4c-d041a0130890 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```
"-C", "link-arg=-Tprogram_metadata.x",
```

> **Reviewer (Reviewer)**: oh we have to update this file for the carrier board too, don’t we?

### [`app/src/carrier_board_shell.rs`](file:///Users/<username>/gh/firmware/app/src/carrier_board_shell.rs) — ⏳ `PENDING`

#### **[MUST FIX]** [app/src/carrier_board_shell.rs:L26](file:///Users/<username>/gh/firmware/app/src/carrier_board_shell.rs#L26)
<!-- comment-uuid: 675de4ba-5c7f-43b0-808e-daec16979fb6 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```
metadata_start: 0x001E_0000,
```

> **Reviewer (Reviewer)**: can we structure this binary more like the cat detector shell and update our docs about creating binaries

### [`docs/cli_and_workflows.md`](file:///Users/<username>/gh/firmware/docs/cli_and_workflows.md) — ⏳ `PENDING`

#### **[MUST FIX]** [docs/cli_and_workflows.md:L56](file:///Users/<username>/gh/firmware/docs/cli_and_workflows.md#L56)
<!-- comment-uuid: f349b9c6-3f43-46b4-82d7-f578afdc369c -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```markdown
python tools/host.py bringup --script tools/device_tests/suites/test_cat_detector_device.py --run
```

> **Reviewer (Reviewer)**: can we get rid of this bringup usage and just pass an optional script to host.py test, also can we update the —script command to infer the scenario from the test script being invoked

### [`platform/host/src/lib.rs`](file:///Users/<username>/gh/firmware/platform/host/src/lib.rs) — ⏳ `PENDING`

#### **[MUST FIX]** [platform/host/src/lib.rs:L9-L14](file:///Users/<username>/gh/firmware/platform/host/src/lib.rs#L9-L14)
<!-- comment-uuid: f04b9dcd-caaf-43e5-bf87-094696b685ac -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```
pub use metadata::{
    autodetect_project_info, decode_project_metadata_bytes, find_symbol_address, ProjectInfo,
    DEFAULT_METADATA_PARTITION_SIZE, MCXN947_METADATA_PARTITION_ADDR,
    MCXN947_PROGRAM_METADATA_ADDR, PROGRAM_METADATA_MAGIC, PROGRAM_METADATA_READ_LEN,
    RP2040_METADATA_PARTITION_ADDR, RP2040_PROGRAM_METADATA_ADDR,
};
```

> **Reviewer (Reviewer)**: let’s revert this change after fixing [`@platform/host/src/metadata.rs`](file:///Users/<username>/gh/firmware/platform/host/src/metadata.rs)

### [`platform/host/src/metadata.rs`](file:///Users/<username>/gh/firmware/platform/host/src/metadata.rs) — ⏳ `PENDING`

#### **[MUST FIX]** [platform/host/src/metadata.rs:L6-L22](file:///Users/<username>/gh/firmware/platform/host/src/metadata.rs#L6-L22)
<!-- comment-uuid: d7728b8e-b8c8-44e1-a70a-623c650e5ed5 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```
/// Fixed `.program_metadata` flash address on RP2040 targets (`memory.x` `PROGRAM_METADATA`).
pub const RP2040_PROGRAM_METADATA_ADDR: u32 = 0x101A_F000;

/// Fixed `metadata` (`sequential_storage::map`) partition address on RP2040 targets.
pub const RP2040_METADATA_PARTITION_ADDR: u32 = 0x101B_0000;

/// Fixed `.program_metadata` flash address on NXP MCX N947 Carrier Board targets.
pub const MCXN947_PROGRAM_METADATA_ADDR: u32 = 0x001D_F000;

/// Fixed `metadata` (`sequential_storage::map`) partition address on MCXN947 targets.
pub const MCXN947_METADATA_PARTITION_ADDR: u32 = 0x001E_0000;

/// Standard size of the non-volatile `metadata` partition in bytes (64 KB).
pub const DEFAULT_METADATA_PARTITION_SIZE: usize = 64 * 1024;

/// Number of bytes required to read and decode `.program_metadata` in O(1).
pub const PROGRAM_METADATA_READ_LEN: usize = 256;
```

> **Reviewer (Reviewer)**: Can we move the metadata partitiion addr and size to the .elf (same, or different section as the .program_metadata section) that way we don’t need to hardcode it in this file

#### **[MUST FIX]** [platform/host/src/metadata.rs:L4](file:///Users/<username>/gh/firmware/platform/host/src/metadata.rs#L4)
<!-- comment-uuid: d9b2f329-2c45-47ce-b491-3b0262a67b0a -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```
pub const PROGRAM_METADATA_MAGIC: &[u8; 4] = b"PROG";
```

> **Reviewer (Reviewer)**: can we replace the metadata magic with a 16-bit checksum and validate the metadata matches the target specification on host.py instead??

### [`platform/src/system/types.rs`](file:///Users/<username>/gh/firmware/platform/src/system/types.rs) — ⏳ `PENDING`

#### **[MUST FIX]** [platform/src/system/types.rs:L461-L465](file:///Users/<username>/gh/firmware/platform/src/system/types.rs#L461-L465)
<!-- comment-uuid: d599c012-2fb1-4ff8-9b55-5900afc3f4a4 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```
/// Four-byte magic prefix stored at the start of `.program_metadata` descriptors.
pub const PROGRAM_METADATA_MAGIC: [u8; 4] = *b"PROG";

/// Project metadata struct embedded in the `.program_metadata` ELF/flash section
/// to allow autodetecting chip, board type, and partition layout without scanning flash.
```

> **Reviewer (Reviewer)**: let’s get rid of this after making the change in [`@platform/host/src/metadata.rs`](file:///Users/<username>/gh/firmware/platform/host/src/metadata.rs)

### [`tools/device_tests/suites/test_carrier_board_device.py`](file:///Users/<username>/gh/firmware/tools/device_tests/suites/test_carrier_board_device.py) — ⏳ `PENDING`

#### **[MUST FIX]** [tools/device_tests/suites/test_carrier_board_device.py:L34-L55](file:///Users/<username>/gh/firmware/tools/device_tests/suites/test_carrier_board_device.py#L34-L55)
<!-- comment-uuid: 2c29e179-fff6-49ad-9646-a24a16657cb1 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```python
procedure="Apply 5.0V current-limited bench power (500 mA limit) to USB_VBUS and probe TP1-TP4 with a DMM.",
    expected="All rails settle within +/-3% tolerance with quiescent draw < 45 mA and zero thermal hotspots.",
    test_points={
        "TP1 (VBUS_5V)": "5.00 V +/- 0.25 V",
        "TP2 (VSYS_3V3)": "3.30 V +/- 0.05 V",
        "TP3 (VDD_CORE)": "1.10 V +/- 0.03 V",
        "TP4 (VBAT_SIM)": "3.70 V - 4.20 V",
    },
    control_pins={
        "USB_VBUS": "J1 Pin A4/B9 (5V Input)",
        "BQ24074_EN": "EN1=LOW, EN2=HIGH (500 mA USB mode)",
    },
    description=(
        "Pre-flash benchtop electrical rail check confirming TI BQ24074 power-path regulation, "
        "3.3V system buck output, and 1.1V MCX N947 core supply."
    ),
)
async def test_01_power_rails_and_protection(host: AsyncHostSessionClient) -> None:
    """Prompt operator to verify TP1-TP4 power rails before MCU flashing."""
    meta = get_device_test_metadata(test_01_power_rails_and_protection)
    assert meta is not None
    await host.verify_operator_step(meta)
```

> **Reviewer (Reviewer)**: I think we need to make an update here, this isn’t really how normal pytest code implements test assertions. in particular, verify_operator_step shouldn’t be done on the host, it should be done from within the pytest process using standard pytest manual test functionality, so we need to port all of the bringup verification out of host.py into the device_tests module. thes manual tests should also periodically send the current procedure and user prompt to the host so the user has context on what is expected next. the test_points, control_pins, descriptions should be moved into the unit test body as pytest assertion, calls to host APIs, or removed entirely.

#### **[MUST FIX]** [tools/device_tests/suites/test_carrier_board_device.py:L28](file:///Users/<username>/gh/firmware/tools/device_tests/suites/test_carrier_board_device.py#L28)
<!-- comment-uuid: 0c632ad3-1cbf-459f-9e30-a5a700389c43 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```python
step_id="1",
```

> **Reviewer (Reviewer)**: can we remove step_id from the device_test metadata? I’d rather infer the test order from the order tests are placed in the listing

### [`tools/device_tests/suites/test_cat_detector_device.py`](file:///Users/<username>/gh/firmware/tools/device_tests/suites/test_cat_detector_device.py) — ⏳ `PENDING`

#### **[MUST FIX]** [tools/device_tests/suites/test_cat_detector_device.py:L46](file:///Users/<username>/gh/firmware/tools/device_tests/suites/test_cat_detector_device.py#L46)
<!-- comment-uuid: 4b6ccc5a-0776-43e4-8f3e-f5b5c5345888 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```python
await host.cli("fs format", expected_regex=r"Formatting|Formatted", reboot_after=True)
```

> **Reviewer (Reviewer)**: I think the host is doing too much heavy lifting here. in particular, the host_cli API should just return a string that the pytest code can test against, using a standard regex library. we should port the expected_regex and reboot_after functionality out of host.py and into the device_tests module and update all the test code to have more canonically pytest behavior. do we also need to add a separate reboot API to the host.py?

### [`tools/host_daemon/model/bringup.py`](file:///Users/<username>/gh/firmware/tools/host_daemon/model/bringup.py) — ⏳ `PENDING`

#### **[MUST FIX]** [tools/host_daemon/model/bringup.py:L25-L26](file:///Users/<username>/gh/firmware/tools/host_daemon/model/bringup.py#L25-L26)
<!-- comment-uuid: fd26d63e-c050-4ceb-827b-c91092bfbaae -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```python
CANONICAL_CAT_DETECTOR_SUITE_REL: str = "tools/device_tests/suites/test_cat_detector_device.py"
CANONICAL_CARRIER_BOARD_SUITE_REL: str = "tools/device_tests/suites/test_carrier_board_device.py"
```

> **Reviewer (Reviewer)**: can we replace this with a —project switch to host.py that infers valid bringup tests by checking the specified project against test attributes? That way we deconstrain our bringup script into multiple listings

### [`tools/host_fs/src/commands/board_type.rs`](file:///Users/<username>/gh/firmware/tools/host_fs/src/commands/board_type.rs) — ⏳ `PENDING`

#### **[MUST FIX]** [tools/host_fs/src/commands/board_type.rs:L18-L28](file:///Users/<username>/gh/firmware/tools/host_fs/src/commands/board_type.rs#L18-L28)
<!-- comment-uuid: a879e2aa-85c9-462f-9edc-1fd4d405e066 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```
/// Fixed RP2040 `.program_metadata` file offset within a full flash dump (`0x001A_F000`).
const RP2040_DUMP_PROGRAM_METADATA_OFFSET: usize = 0x001A_F000;

/// Fixed RP2040 `metadata` partition offset within a full flash dump (`0x001B_0000`).
const RP2040_DUMP_METADATA_PARTITION_OFFSET: usize = 0x001B_0000;

/// Fixed MCXN947 `.program_metadata` file offset within a full flash dump (`0x001D_F000`).
const MCXN947_DUMP_PROGRAM_METADATA_OFFSET: usize = 0x001D_F000;

/// Fixed MCXN947 `metadata` partition offset within a full flash dump (`0x001E_0000`).
const MCXN947_DUMP_METADATA_PARTITION_OFFSET: usize = 0x001E_0000;
```

> **Reviewer (Reviewer)**: commented on this stuff already. we need to read if off the elf and remove it from the host code!

#### **[MUST FIX]** [tools/host_fs/src/commands/board_type.rs:L1](file:///Users/<username>/gh/firmware/tools/host_fs/src/commands/board_type.rs#L1)
<!-- comment-uuid: 4c19cefe-baa9-4154-809f-14f825125d35 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```
//! Query canonical board identity (`board_type`) from on-device `.program_metadata`
```

> **Reviewer (Reviewer)**: if we’re doing all this heavy lifting, why not change this command into a “query_metadata” command that dumps the whole program metadata section to stdout?

### [`tools/verify.py`](file:///Users/<username>/gh/firmware/tools/verify.py) — ⏳ `PENDING`

#### **[MUST FIX]** [tools/verify.py:L26-L33](file:///Users/<username>/gh/firmware/tools/verify.py#L26-L33)
<!-- comment-uuid: 61481120-f81a-4d34-b893-eb95d98ff068 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```python
CONDA_ENV_BIN_CANDIDATES: tuple[str, ...] = (
    "/workspace/miniforge3/envs/firmware-env/bin",
    "/root/miniforge3/envs/firmware-env/bin",
    "{home}/miniforge3/envs/firmware-env/bin",
    "{home}/mambaforge/envs/firmware-env/bin",
    "{home}/miniconda3/envs/firmware-env/bin",
    "/opt/homebrew/Caskroom/miniforge/base/envs/firmware-env/bin",
)
```

> **Reviewer (Reviewer)**: seems a bit unexpected that we’d need this instead of just having conda in our environment always

### [`tools/verify.sh`](file:///Users/<username>/gh/firmware/tools/verify.sh) — ⏳ `PENDING`

#### **[MUST FIX]** [tools/verify.sh:L6-L41](file:///Users/<username>/gh/firmware/tools/verify.sh#L6-L41)
<!-- comment-uuid: d40ac44a-0537-4d08-a9d1-fa6b5059e218 -->
<!-- comment-commit: working -->
<!-- comment-status: RESOLVED -->

```bash
SKIP_DEVICE_TESTS=0
DEVICE_SCENARIO="default"
DEVICE_BOARD="auto"
DEVICE_INTERACTIVE_FLAG=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --skip-device-tests|--host-only)
            SKIP_DEVICE_TESTS=1
            shift
            ;;
        --bringup|--device-all)
            DEVICE_SCENARIO="smoke or integration or bringup"
            shift
            ;;
        --scenario)
            DEVICE_SCENARIO="$2"
            shift 2
            ;;
        --board|--device)
            DEVICE_BOARD="$2"
            shift 2
            ;;
        --interactive)
            DEVICE_INTERACTIVE_FLAG="--interactive"
            shift
            ;;
        *)
            echo "Unknown option for tools/verify.sh: $1" >&2
            echo "Supported flags: --bringup, --scenario <expr>, --board <glob>, --interactive, --skip-device-tests" >&2
            exit 1
            ;;
    esac
done

# Raise open file descriptor limit if possible
```

> **Reviewer (Reviewer)**: we should probably port this script and anvil over to python, huh? (and make the same change in the hardware repo as well)

## Action Items Checklist

- [x] **[MUST FIX]** [`.cargo/config.toml:L12`](file:///Users/<username>/gh/firmware/.cargo/config.toml#L12): oh we have to update this file for the carrier board too, don’t we? <!-- uuid:4bac1bf1-44df-4c9f-bf4c-d041a0130890 -->
- [x] **[MUST FIX]** [`app/src/carrier_board_shell.rs:L26`](file:///Users/<username>/gh/firmware/app/src/carrier_board_shell.rs#L26): can we structure this binary more like the cat detector shell and update our docs about creating binaries <!-- uuid:675de4ba-5c7f-43b0-808e-daec16979fb6 -->
- [x] **[MUST FIX]** [`docs/cli_and_workflows.md:L56`](file:///Users/<username>/gh/firmware/docs/cli_and_workflows.md#L56): can we get rid of this bringup usage and just pass an optional script to host.py test, also can we update the —script command to infer the scenario from the test script being invoked <!-- uuid:f349b9c6-3f43-46b4-82d7-f578afdc369c -->
- [x] **[MUST FIX]** [`platform/host/src/metadata.rs:L6-L22`](file:///Users/<username>/gh/firmware/platform/host/src/metadata.rs#L6-L22): Can we move the metadata partitiion addr and size to the .elf (same, or different section as the .program_metadata section) that way we don’t need to hardcode it in this file <!-- uuid:d7728b8e-b8c8-44e1-a70a-623c650e5ed5 -->
- [x] **[MUST FIX]** [`platform/host/src/lib.rs:L9-L14`](file:///Users/<username>/gh/firmware/platform/host/src/lib.rs#L9-L14): let’s revert this change after fixing [`@platform/host/src/metadata.rs`](file:///Users/<username>/gh/firmware/platform/host/src/metadata.rs) <!-- uuid:f04b9dcd-caaf-43e5-bf87-094696b685ac -->
- [x] **[MUST FIX]** [`platform/host/src/metadata.rs:L4`](file:///Users/<username>/gh/firmware/platform/host/src/metadata.rs#L4): can we replace the metadata magic with a 16-bit checksum and validate the metadata matches the target specification on host.py instead?? <!-- uuid:d9b2f329-2c45-47ce-b491-3b0262a67b0a -->
- [x] **[MUST FIX]** [`platform/src/system/types.rs:L461-L465`](file:///Users/<username>/gh/firmware/platform/src/system/types.rs#L461-L465): let’s get rid of this after making the change in [`@platform/host/src/metadata.rs`](file:///Users/<username>/gh/firmware/platform/host/src/metadata.rs) <!-- uuid:d599c012-2fb1-4ff8-9b55-5900afc3f4a4 -->
- [x] **[MUST FIX]** [`tools/host_daemon/model/bringup.py:L25-L26`](file:///Users/<username>/gh/firmware/tools/host_daemon/model/bringup.py#L25-L26): can we replace this with a —project switch to host.py that infers valid bringup tests by checking the specified project against test attributes? That way we deconstrain our bringup script into multiple listings <!-- uuid:fd26d63e-c050-4ceb-827b-c91092bfbaae -->
- [x] **[MUST FIX]** [`tools/verify.sh:L6-L41`](file:///Users/<username>/gh/firmware/tools/verify.sh#L6-L41): we should probably port this script and anvil over to python, huh? (and make the same change in the hardware repo as well) <!-- uuid:d40ac44a-0537-4d08-a9d1-fa6b5059e218 -->
- [x] **[MUST FIX]** [`tools/device_tests/suites/test_carrier_board_device.py:L34-L55`](file:///Users/<username>/gh/firmware/tools/device_tests/suites/test_carrier_board_device.py#L34-L55): I think we need to make an update here, this isn’t really how normal pytest code implements test assertions. in particular, verify_operator_step shouldn’t be done on the host, it should be done from within the pytest process using standard pytest manual test functionality, so we need to port all of the bringup verification out of host.py into the device_tests module. thes manual tests should also periodically send the current procedure and user prompt to the host so the user has context on what is expected next. the test_points, control_pins, descriptions should be moved into the unit test body as pytest assertion, calls to host APIs, or removed entirely. <!-- uuid:2c29e179-fff6-49ad-9646-a24a16657cb1 -->
- [x] **[MUST FIX]** [`tools/device_tests/suites/test_carrier_board_device.py:L28`](file:///Users/<username>/gh/firmware/tools/device_tests/suites/test_carrier_board_device.py#L28): can we remove step_id from the device_test metadata? I’d rather infer the test order from the order tests are placed in the listing <!-- uuid:0c632ad3-1cbf-459f-9e30-a5a700389c43 -->
- [x] **[MUST FIX]** [`tools/device_tests/suites/test_cat_detector_device.py:L46`](file:///Users/<username>/gh/firmware/tools/device_tests/suites/test_cat_detector_device.py#L46): I think the host is doing too much heavy lifting here. in particular, the host_cli API should just return a string that the pytest code can test against, using a standard regex library. we should port the expected_regex and reboot_after functionality out of host.py and into the device_tests module and update all the test code to have more canonically pytest behavior. do we also need to add a separate reboot API to the host.py? <!-- uuid:4b6ccc5a-0776-43e4-8f3e-f5b5c5345888 -->
- [x] **[MUST FIX]** [`tools/host_fs/src/commands/board_type.rs:L18-L28`](file:///Users/<username>/gh/firmware/tools/host_fs/src/commands/board_type.rs#L18-L28): commented on this stuff already. we need to read if off the elf and remove it from the host code! <!-- uuid:a879e2aa-85c9-462f-9edc-1fd4d405e066 -->
- [x] **[MUST FIX]** [`tools/host_fs/src/commands/board_type.rs:L1`](file:///Users/<username>/gh/firmware/tools/host_fs/src/commands/board_type.rs#L1): if we’re doing all this heavy lifting, why not change this command into a “query_metadata” command that dumps the whole program metadata section to stdout? <!-- uuid:4c19cefe-baa9-4154-809f-14f825125d35 -->
- [x] **[MUST FIX]** [`tools/verify.py:L26-L33`](file:///Users/<username>/gh/firmware/tools/verify.py#L26-L33): seems a bit unexpected that we’d need this instead of just having conda in our environment always <!-- uuid:61481120-f81a-4d34-b893-eb95d98ff068 -->
