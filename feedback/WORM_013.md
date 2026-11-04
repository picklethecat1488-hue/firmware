# 🟢 `[WORM-013]` Feedbacks about carrier_board.md

- **UUID**: `6e2400a4-8fa5-46fd-938a-58996597f407`
- **ID**: `WORM-013`
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
