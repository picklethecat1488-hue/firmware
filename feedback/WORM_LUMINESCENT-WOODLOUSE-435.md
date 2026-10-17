# 🟢 `[WORM-LUMINESCENT-WOODLOUSE-435]` Parse ANSI OSC 8 hyperlink escape sequences in terminal and CLI console

- **UUID**: `69aec711-3219-44b5-8810-6ab0a59d1138`
- **ID**: `WORM-LUMINESCENT-WOODLOUSE-435`
- **Status**: `RESOLVED`
- **Severity**: `LOW`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host_daemon`
- **Created**: `2026-10-17 02:52:08 UTC`
- **Resolved**: `2026-10-17 02:56:45 UTC`

#### Description

Cargo and CLI utilities emit ANSI OSC 8 hyperlink sequences (\x1b]8;;<url>\x1b\ or \x1b]8;;<url>\x07). Terminal formatters in dashboard and host_daemon do not recognize OSC 8 escapes, causing raw '8;;https://...' text to be rendered in the CLI console.

#### Resolution Notes

Parsed ANSI OSC 8 hyperlink escape sequences (\x1b]8;;url\x1b\text\x1b]8;;\x1b\ as well as BEL-terminated and orphaned 8;; sequences) into styled HTML anchor tags across terminal.py, code_review.html.j2, and host_daemon.html.j2, preventing raw '8;;' from rendering in the CLI console.
