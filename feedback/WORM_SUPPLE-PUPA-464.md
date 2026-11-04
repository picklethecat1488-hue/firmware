# 🟢 `[WORM-SUPPLE-PUPA-464]` Create host_daemon tool

- **UUID**: `3024e952-f8f5-4cdf-83e8-7d9889343c52`
- **ID**: `WORM-SUPPLE-PUPA-464`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `host_daemon`
- **Created**: `2026-10-17 00:32:33 UTC`
- **Resolved**: `2026-10-17 01:00:46 UTC`

#### Description

Can you please create a host daemon tool that wraps host_cli and host_fs tools based on the UI framework the dashboard uses? The idea is that host daemon will compile host tools and flash the target on startup, detach from the terminal, and supports any host_cli or host_fs command through a native CLI similar to the code review CLI. There should be a dropdown box to select the active UI mode-CLI or FS, as well as separate logging and CLI output view panels

#### Reproduction Steps

1. PROBE_RS_SPEED=4000 python tools/host_daemon.py --target thumbv6m-none-eabi --package cat_detector --bin cat_detector_shell --release

#### Behavior Comparison

- **Expected**: 1. Attach to target
- **Actual**: _Not specified_
