# 🟢 `[WORM-SILKEN-CASTING-127]` Diff viewer missing green highlight for line additions in side-by-side and unified views

- **UUID**: `b2ab3811-057b-4884-b0cf-df9a0f781f3e`
- **ID**: `WORM-SILKEN-CASTING-127`
- **Status**: `RESOLVED`
- **Severity**: `LOW`
- **Category**: `UI`
- **Component**: `dashboard`
- **Created**: `2026-11-05 14:30:52 UTC`
- **Resolved**: `2026-11-05 14:30:53 UTC`

#### Description

In the VCS diff view (diff_component.html.j2), line additions and inserted lines do not have the green background highlight. Unified diff lines use row-add / add / insert classes which are not targeted by the CSS rules (which only targeted row-insert), and the background color used teal instead of standard green.

#### Review Tasks & Functional Checklist

- [ ] Verify green highlight styling on diff line additions in dashboard UI | `python tools/dashboard.py` <!-- id:2f3193c0 -->

#### Resolution Notes

Updated diff_component.html.j2 and code_review.html.j2 to use green highlight rgba(34, 197, 94, 0.22) and #22c55e for line additions in side-by-side and unified views. Added comprehensive support for .row-add, .add, .insert, and .row-insert classes. Cross-patched matching selectors into hardware repository.
