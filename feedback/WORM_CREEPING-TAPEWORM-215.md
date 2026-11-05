# 🟢 `[WORM-CREEPING-TAPEWORM-215]` Only first commit in smartlog tree gets highlighted

- **UUID**: `2912e28f-e587-4b0f-8cc2-5642a1042e48`
- **ID**: `WORM-CREEPING-TAPEWORM-215`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `UI`
- **Component**: `dashboard`
- **Created**: `2026-11-05 13:32:56 UTC`
- **Resolved**: `2026-11-05 13:34:38 UTC`

#### Description

In diff_view.html.j2, selectCommit attempts to find active commit elements via document.getElementById('node-' + commitHash.slice(0, 7)). When git abbreviations exceed 7 characters (e.g. 8-char short SHAs), element lookup returns null, causing commit selection highlights to fail on click.

#### Resolution Notes

Fixed selectCommit in diff_view.html.j2 across firmware and hardware repositories. Replaced fragile slice(0, 7) element lookups with event.currentTarget and data-hash attribute matching (exact and prefix), supporting arbitrary-length git abbreviated hashes (e.g. 8-char SHAs). Added data-hash='working' to working tree node and updated dynamic renderSmartlogList to preserve active class across selection transitions. Added dedicated regression unit tests in tools/tests/test_worm_creeping_tapeworm_215_smartlog_highlight.py and hardware tests.
