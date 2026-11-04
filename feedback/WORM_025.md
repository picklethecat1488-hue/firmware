# 🟢 `[WORM-025]` Close the dashboard when closing the app

- **UUID**: `04a45bbe-385d-4912-99b4-3363835f8632`
- **ID**: `WORM-025`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-09 00:32:10 UTC`
- **Resolved**: `2026-10-09 00:48:41 UTC`

#### Description

Can we close the dashboard server when closing the app window? No point in keeping it running after the app closes.

#### Behavior Comparison

- **Expected**: $ python tools/dashboard.py
- **Actual**: $ python tools/dashboard.py

#### Resolution Notes

Register on_close callback in webview and eel launchers to cleanly terminate background dashboard server and unlink lock file upon window closure
