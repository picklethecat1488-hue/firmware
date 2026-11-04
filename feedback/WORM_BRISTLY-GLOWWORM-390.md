# 🟢 `[WORM-BRISTLY-GLOWWORM-390]` Please add the “Attachments & Screenshots” UI to the host bug reporter page

- **UUID**: `953d25f6-47e6-4f48-a63b-40ee57ceccde`
- **ID**: `WORM-BRISTLY-GLOWWORM-390`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `Host`
- **Created**: `2026-10-17 23:33:41 UTC`
- **Resolved**: `2026-10-18 00:09:31 UTC`

#### Description

Please add the “Attachments & Screenshots” section to the host tool target bug page, that way I can include additional context when opening bugs.

#### Resolution Notes

Added the Attachments & Screenshots UI section to the host tool target bug page (host_daemon.html.j2), including drag-and-drop dropzone, file input, attachments grid, clipboard pasting for screenshots/text, Git LFS notice, and /api/target_bug backend support for saving manual attachments to disk and worm reports.
