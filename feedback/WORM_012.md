# 🟢 `[WORM-012]` Rename audio controller to speaker controller

- **UUID**: `6dab9263-f0c5-499d-bc7b-aa0db863a03d`
- **ID**: `WORM-012`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `CONTROLLER`
- **Component**: `carrier_board`
- **Created**: `2026-10-04 16:43:43 UTC`
- **Resolved**: `2026-10-04 16:54:31 UTC`

#### Description

Rename the AudioController definition in carrier_board.md to SpeakerController

#### Resolution Notes

Renamed AudioController and controller::audio_controller to SpeakerController and controller::speaker_controller in app/carrier_board.md and updated regression test in test_carrier_board_project.py.
