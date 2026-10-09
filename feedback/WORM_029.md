# 🟢 `[WORM-029]` Replace integer ID’s with Docker-pattern ID's

- **UUID**: `b52165bd-d3e5-409a-bb0c-2f3ff9284a27`
- **ID**: `WORM-029`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Component**: `dashboard`
- **Created**: `2026-10-09 20:51:32 UTC`
- **Resolved**: `2026-10-09 21:02:49 UTC`

#### Description

Replace the integer-based issue ID’s with Docker-pattern ID’s which are generated at random using a CRNG (E.g. openssl’s rand function). The issue format should be:

   • Format: [ADJECTIVE]-[ANIMAL/NOUN]-[3 digits]
 ┃   • Examples:
 ┃       • WORM-SWIFT-FOX-42
 ┃       • WORM-BOLD-LYNX-809
 ┃       • WORM-IRON-CRANE-17

Please compress the adjective and noun lists to save code space.

#### Resolution Notes

Replaced integer-based issue IDs with Docker-pattern IDs generated via CRNG with compressed word lists.
