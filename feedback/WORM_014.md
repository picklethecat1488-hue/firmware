# 🟢 `[WORM-014]` Unresolved worms in VCS UI

- **UUID**: `a9d4483b-b38d-4f56-bdef-7ef9e5b1be0a`
- **ID**: `WORM-014`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-07 02:29:34 UTC`
- **Resolved**: `2026-10-07 02:38:05 UTC`

#### Description

I see these two unresolved worms in the VCS UI which seem to be an artifact of porting changes over from the hardware repo

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `screenshot` | [pasted_screenshot_1791340187710.png](attachments/WORM-014/pasted_screenshot_1791340187710.png) | Pasted screenshot |

#### Resolution Notes

Prevent phantom OPEN worm tags in VCS UI for non-existent bug IDs ported from external repositories
