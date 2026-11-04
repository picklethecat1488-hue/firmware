# 🟢 `[WORM-022]` Dashboard App icons show up as Chrome icons

- **UUID**: `8fb06a58-7b19-4768-8ee9-bbf0b9b0187a`
- **ID**: `WORM-022`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-08 21:06:54 UTC`
- **Resolved**: `2026-10-08 21:33:24 UTC`

#### Description

The dashboard icons show up as ugly Chrome icons in the taskbar. Please ensure the correct themed PWA app icon shows up in the taskbar.

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [pasted_screenshot_1791493684556.png](attachments/WORM-022/pasted_screenshot_1791493684556.png) | Pasted screenshot |

#### Resolution Notes

Added pregenerated app.icns, updated manifest.json display_override, and implemented ensure_macos_app_bundle in launcher.py to launch standalone window via signed macOS .app shim displaying themed icon in Dock and dark title bar.
