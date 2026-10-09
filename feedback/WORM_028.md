# 🟢 `[WORM-028]` Create separate firwmware architecture docs

- **UUID**: `7b3d890e-45a1-43b8-a232-037f0a818cd3`
- **ID**: `WORM-028`
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
