"""Reproduction and regression unit test for WORM-005 (BUG-005) and CR-ccf64ca.

Verifies that:
1. The carrier board 2.0 app is integrated into the app crate alongside cat_detector.
2. carrier_board is not a separate standalone workspace member in Cargo.toml.
3. The carrier board firmware design doc resides under app/carrier_board.md with all required design evaluations.
4. app/carrier_board_bringup.yaml exists as the source of truth for bringup verification.
"""

from pathlib import Path
import sys
import yaml

if sys.version_info >= (3, 11):
    import tomllib
else:
    import tomli as tomllib


WORKSPACE_ROOT = Path(__file__).resolve().parent.parent.parent


def test_carrier_board_project_structure() -> None:
    """Verify carrier_board is integrated in app crate and standalone crate is removed."""
    # 1. Root Cargo.toml workspace membership does not have carrier_board as standalone crate
    root_cargo_path = WORKSPACE_ROOT / "Cargo.toml"
    assert root_cargo_path.exists(), "Root Cargo.toml must exist"
    root_cargo = tomllib.loads(root_cargo_path.read_text(encoding="utf-8"))
    members = root_cargo.get("workspace", {}).get("members", [])
    assert "carrier_board" not in members, "carrier_board must NOT be a separate standalone member"
    assert "app" in members, "app crate must be in workspace.members"

    # 2. Standalone carrier_board directory must not exist
    cb_dir = WORKSPACE_ROOT / "carrier_board"
    assert not cb_dir.exists(), "Standalone carrier_board/ directory must not exist"

    # 3. app crate contains carrier_board modules and binary
    app_dir = WORKSPACE_ROOT / "app"
    app_cargo_path = app_dir / "Cargo.toml"
    assert app_cargo_path.exists(), "app/Cargo.toml must exist"
    app_cargo = tomllib.loads(app_cargo_path.read_text(encoding="utf-8"))

    # Assert carrier_board_shell is registered as a binary in app crate
    bins = [b.get("name") for b in app_cargo.get("bin", [])]
    assert "carrier_board_shell" in bins, "carrier_board_shell binary must be in app/Cargo.toml"

    # Assert source files exist under app/src/
    assert (app_dir / "src" / "carrier_board.rs").exists(), "app/src/carrier_board.rs must exist"
    assert (app_dir / "src" / "carrier_board_shell.rs").exists(), (
        "app/src/carrier_board_shell.rs must exist"
    )
    assert (app_dir / "tests" / "carrier_board_tests.rs").exists(), (
        "app/tests/carrier_board_tests.rs must exist"
    )


def test_carrier_board_firmware_document() -> None:
    """Verify carrier board firmware documentation exists under app/carrier_board.md with all required evaluations."""
    doc_path = WORKSPACE_ROOT / "app" / "carrier_board.md"
    assert doc_path.exists(), "app/carrier_board.md must exist"

    content = doc_path.read_text(encoding="utf-8")

    # Architecture & Key Components
    assert "Carrier Board 2.0" in content
    assert "MCX N947" in content or "MCXN947" in content
    assert "W25N01GV" in content
    assert "BQ24074" in content
    assert "MAX17048" in content
    assert "LP5009" in content
    assert "IQS7222A" in content
    assert "NINA-B312" in content

    # FlexSPI assigned to Core 1; Core 0 for real-time sensing and audio
    assert "FlexSPI Port A (Core 1)" in content or "FlexSPI NAND Flash Storage Controller" in content
    assert "Core 1" in content

    # 1 Mb/s UART support
    assert "1 Mb/s" in content or "1Mb/s" in content

    # Cost breakdown for .text and .data
    assert ".text" in content
    assert ".data" in content
    assert "Neutron NPU" in content or "eIQ" in content
    assert "DSP" in content or "PowerQuad" in content
    assert "gesture" in content.lower() or "camera" in content.lower()
    assert "location" in content.lower()

    # Application software for DSP/NPU (Burn, Candle, tract, JAX)
    assert "Burn" in content or "burn-rs" in content
    assert "JAX" in content
    assert "PyTorch" in content or "Candle" in content or "tract" in content

    # Design Hardening & MPU
    assert "ARMv8-M" in content or "MPU" in content
    assert "Execute Never" in content or "XN" in content
    assert "Block Lock" in content or "envelope encryption" in content or "Keystore" in content

    # Secure Boot & Firmware Updating
    assert "Secure Boot" in content or "Root of Trust" in content
    assert "Ed25519" in content or "signature" in content.lower()
    assert "anti-rollback" in content.lower() or "rollback" in content.lower()

    # Bidirectional BLE UART & service endpoint
    assert "service endpoint" in content.lower() or "Service Endpoint" in content
    assert "telemetry" in content.lower()

    # Option 3: Embassy Multi-Executor AMP
    assert "Option 3" in content or "Embassy Multi-Executor" in content
    assert "AMP" in content

    # SRAM constraints including RTT defmt and CLI buffers
    assert "SRAM" in content
    assert "RTT" in content
    assert "defmt" in content

    # Communication & Telemetry: RTT vs High-Speed UART Service Model
    assert "UART" in content
    assert "Perfetto" in content
    assert "Service Model" in content or "service model" in content

    # Operating Power State Matrix & Subsystem Measurements
    assert "Operating Power State Matrix" in content
    assert "Deep Standby" in content or "State 0" in content
    assert "Peak Burst" in content or "State 3" in content
    assert "#operating-power-state-matrix" in content or "Operating Power State Matrix" in content
    assert "PowerQuad" in content or "Neutron" in content
    assert "mA" in content

    # Custom Embassy HAL with I3C, I2S, High-Resolution Timer, Watchdog, RTC
    assert "embassy-mcx" in content or "Embassy" in content
    assert "I3C" in content
    assert "I2S" in content or "I^2S" in content
    assert "Timer" in content or "timer" in content
    assert "Watchdog" in content or "wwdt" in content.lower()
    assert "RTC" in content

    # On-Device Test Frameworks (defmt-test, post-bringup UART test runner)
    assert "defmt-test" in content or "embedded-test" in content

    # On-Chip Peripheral Core Hardware Verification
    assert "On-Chip" in content or "on-chip" in content
    assert "I2C" in content
    assert "I3C" in content

    # Customizable Bootloader (SSBL), XIP, and SRAM relocation for OTA
    assert "SSBL" in content or "Second-Stage Bootloader" in content
    assert "XIP" in content
    assert "flash_loader_ram" in content or ("SRAM" in content and "relocat" in content)

    # Inter-core IPC framework & cost breakdown
    assert "IPC" in content
    assert "minicbor" in content or "CBOR" in content

    # Embassy framework invariant for SSBL and all executable code
    assert "Embassy Framework Invariant" in content or ("Embassy" in content and "executable code" in content.lower())

    # Host CLI and Host FS tooling support
    assert "host_cli" in content
    assert "host_fs" in content
    assert "field servicing" in content.lower() or "production" in content.lower()

    # Resolution of RP2040 Core 1 SRAM execution limitation
    assert "RP2040" in content
    assert "XIP" in content

    # UI states, speaker chimes, boot times, and Standby power state
    assert "Standby" in content
    assert "chime" in content.lower()
    assert "BOOT_FAILED" in content or "boot failure" in content.lower()
    assert "OTA_PROGRAMMING" in content or "Magenta" in content
    assert "Boot Timing" in content or "boot time" in content.lower()

    # Modular expansion card support and Cargo features
    assert "expansion card" in content.lower()
    assert "features" in content.lower()
    assert "expansion-audio" in content or "expansion-camera" in content

    # Application controllers list (CR-7c6e41f, WORM-012)
    assert "SystemController" in content
    assert "BatteryController" in content
    assert "LedController" in content
    assert "SensorController" in content
    assert "SpeakerController" in content
    assert "controller::speaker_controller" in content
    assert "AudioController" not in content
    assert "BleController" in content
    assert "FilesystemController" in content
    assert "TelemetryController" in content
    assert "ShellController" in content
    assert "ThermalController" in content

    # Storage descriptor URI model & flash separation
    assert "dev:builtin-flash" in content
    assert "dev:ext-flash" in content

    # ARMv8-M MPU hardening & stack limits
    assert "Null Page Guard" in content
    assert "MSPLIM" in content and "PSPLIM" in content

    # Inter-core IPC: direct struct copying & CBOR
    assert "Direct Plain Old Data (POD) Struct Copying" in content or "zerocopy" in content
    assert "minicbor" in content

    # Cache architecture (I-Cache & D-Cache)
    assert "D-Cache" in content and "I-Cache" in content

    # Structured error handling & model updates in host_cli
    assert "ServiceError" in content
    assert "pub context: [u32; 4]" in content
    assert "register_snapshot" not in content
    assert "model-update" in content

    # UI and Audio state invariants
    assert "OVERTEMP_ALERT" in content
    assert "Low Battery / Overtemp Alert" in content
    assert "SSBL Failure Alert" not in content
    assert "OTA Success Fanfare" not in content
    assert "PMIC" not in content
    assert "GESTURE_DETECTED" not in content
    assert "BATTERY_CHARGING" in content
    assert "BATTERY_LOW" in content
    assert "BATTERY_CRITICAL" in content

    # Expansion board error handling and fault isolation (CR-e45b9423)
    assert "eliminates on-card identification EEPROMs" in content
    assert "Modular Expansion Board Verification & Error Handling" in content
    assert "ERR_EXPANSION_NOT_FOUND" in content or "ServiceError" in content
    assert "SRAM Usage vs. Expansion Headroom" in content
    assert "Framework Validation: Integration with Touch Sensor Gesture Processor" in content
    assert "dev:eeprom" not in content

    # Internal flash partitions renaming to 'app' and 'metadata' (CR-e45b9423)
    assert "`app`" in content
    assert "`metadata`" in content
    assert "slot_a" not in content

    # Strongly typed integer enums for ProgramMetadata (CR-e45b9423)
    assert "pub enum StorageDeviceId" in content
    assert "pub enum PartitionId" in content
    assert "device_id: StorageDeviceId" in content
    assert "partition_id: PartitionId" in content
    assert "ProgramMetadata" in content
    assert ".program_metadata" in content or "program_metadata" in content.lower()

    # Architecture risk matrix, bringup milestones, and final delivery objective (CR-e45b9423, CR-7ddac53)
    assert "Architecture Risk Identification & Mitigation Matrix" in content
    assert "Bringup & Verification Milestones & Deliverables Roadmap" in content
    assert "Final Delivery Objective" in content
    assert "AR-1" in content and "AR-6" in content
    assert "Milestone 1 (M1): Silicon Baseline, Embassy HAL Foundation" in content
    assert "Milestone 5 (M5)" in content

    # 1-Finger (1F) Capacitive Touch Gestures and Interaction Model
    assert "Capacitive Touch Gesture Recognition & Interaction Model (1F Gestures)" in content
    assert "1F Tap (Single Tap)" in content
    assert "1F Double Tap" in content
    assert "1F Swipe Forward" in content
    assert "1F Swipe Back" in content
    assert "1F Long Press" in content
    assert "1F Extra Long Press" in content
    assert "model::types::Gesture" in content or "enum Gesture" in content
    assert "TelemetryRecord::Gesture" in content
    assert "DualLongPress" in content
    assert "SingleTap" in content
    assert "DoubleTap" in content
    assert "SwipeForward" in content
    assert "SwipeBack" in content
    assert "LongPress" in content
    assert "ExtraLongPress" in content
    assert "GestureSource" in content
    assert "ActionButton" in content
    assert "Finger Down Tone" in content
    assert "Swipe Tone" in content
    assert "SW1" in content or "Action Button" in content
    assert "10–30 Hz" in content or "10-30 Hz" in content
    assert "FINGER_DOWN" in content and "FINGER_MOVE" in content and "FINGER_UP" in content
    assert "GestureDetector" in content

    # 4. Hardware FIFO buffering, on-chip high-resolution timer, and sensor fusion
    assert "FIFO" in content
    assert "CTIMER" in content or "high-resolution timer" in content.lower()
    assert "sensor fusion" in content.lower() and "FIFO" in content

    # 5. Factory provisioning structured logging & status display
    assert "status display" in content.lower() or "HUD" in content
    assert "JUnit" in content or "MES" in content

    # 6. PowerDown alignment with RP2040 SystemController / SystemStatus
    assert "SystemStatus" in content or "Active, Sleep, PowerDown" in content

    # CR-6979a20c: Factory provisioning Off / ship mode transition and USB wake without opening enclosure
    assert "Ship Mode" in content or "ship-mode" in content or "ship mode" in content.lower()
    assert "without opening the enclosure" in content.lower() or "without opening" in content.lower()


def test_carrier_board_bringup_yaml() -> None:
    """Verify app/carrier_board_bringup.yaml exists as source of truth for bringup verification."""
    yaml_path = WORKSPACE_ROOT / "app" / "carrier_board_bringup.yaml"
    assert yaml_path.exists(), "app/carrier_board_bringup.yaml must exist"

    data = yaml.safe_load(yaml_path.read_text(encoding="utf-8"))
    assert data["project_name"] == "Carrier Board 2.0"
    assert "MCXN947" in data["device_chip"]
    assert "carrier_board_shell" in data["cargo_target"]

    steps = data.get("steps", [])
    assert len(steps) >= 10, "carrier_board_bringup.yaml must define comprehensive bringup steps"

    step_names = [s.get("name", "") for s in steps]
    assert any("I3C" in name for name in step_names), "Must include I3C validation step"
    assert any("Watchdog" in name or "Reset" in name for name in step_names), "Must include Watchdog/Reset validation step"
