# 🔴 `[WORM-SILKEN-CASTING-127]` Diff viewer missing green highlight for line additions in side-by-side and unified views

- **UUID**: `b2ab3811-057b-4884-b0cf-df9a0f781f3e`
- **ID**: `WORM-SILKEN-CASTING-127`
- **Status**: `OPEN`
- **Severity**: `LOW`
- **Category**: `UI`
- **Component**: `dashboard`

#### Description

In the VCS diff view (diff_component.html.j2), line additions and inserted lines do not have the green background highlight. Unified diff lines use row-add / add / insert classes which are not targeted by the CSS rules (which only targeted row-insert), and the background color used teal instead of standard green.
