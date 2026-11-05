# 🔴 `[WORM-CREEPING-TAPEWORM-215]` Only first commit in smartlog tree gets highlighted

- **UUID**: `2912e28f-e587-4b0f-8cc2-5642a1042e48`
- **ID**: `WORM-CREEPING-TAPEWORM-215`
- **Status**: `OPEN`
- **Severity**: `MEDIUM`
- **Category**: `UI`
- **Component**: `dashboard`
- **Created**: `2026-11-05 13:33:41 UTC`

#### Description

In diff_view.html.j2, selectCommit attempts to find active commit elements via document.getElementById('node-' + commitHash.slice(0, 7)). When git abbreviations exceed 7 characters (e.g. 8-char short SHAs), element lookup returns null, causing commit selection highlights to fail on click.
