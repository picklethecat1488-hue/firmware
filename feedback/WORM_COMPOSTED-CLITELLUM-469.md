# 🟢 `[WORM-COMPOSTED-CLITELLUM-469]` Corrupted CLI output

- **UUID**: `99ddf751-bef5-4af7-934c-d451cfe78363`
- **ID**: `WORM-COMPOSTED-CLITELLUM-469`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard, host_daemon`
- **Created**: `2026-10-17 02:52:14 UTC`
- **Resolved**: `2026-10-17 03:01:00 UTC`

#### Description

See attached screenshot below

#### Reproduction Steps

1. $ PROBE_RS_SPEED=4000 python tools/host_daemon.py  --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_app --release

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [screenshot_paste_1792205540950_hs7gc.png](attachments/WORM-COMPOSTED-CLITELLUM-469/screenshot_paste_1792205540950_hs7gc.png) | Pasted screenshot screenshot_paste_1792205540950_hs7gc.png |

#### Resolution Notes

Resolved CLI output corruption by fixing two root causes: (1) Added ANSI OSC 8 hyperlink escape sequence tokenization and orphaned 8;; URL normalization to terminal.py, host_daemon.html.j2, and code_review.html.j2 so hyperlink escapes are rendered as HTML anchors rather than leaking raw '8;;' strings; (2) Corrected #cliView and #fsView container display property from 'flex' to 'block' and added 'flex-direction: column' to .panel-content so log lines stack vertically rather than being rendered side-by-side in horizontal flex columns.
