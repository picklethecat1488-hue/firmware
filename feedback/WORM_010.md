# 🟢 `[WORM-010]` "Open in VSCode" didn't work

- **UUID**: `8e571b93-ae93-4737-b018-7a2c824c4aca`
- **ID**: `WORM-010`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `FIRMWARE`
- **Created**: `2026-10-04 00:40:02 UTC`
- **Resolved**: `2026-10-04 01:31:02 UTC`

#### Description

Clicked on "Open in VSCode" next to app/carrier_board.md, and nothing seemed to happen. I was viewing the website in the VSCode simple browser window

#### Attachments & References

| Type | Filename | Description |
| :--- | :--- | :--- |
| `document` | [pasted_document_1791074450389.txt](attachments/WORM-010/pasted_document_1791074450389.txt) | Pasted text document |

#### Resolution Notes

Add server /api/open-vscode endpoint and editor launcher with CLI and URL scheme fallback for embedded VS Code Simple Browser
