# Proposal

## Why

Only the Services tab tells the user which keys act on the selected row. Every other
tab has the same kind of actions but leaves the bindings to the help panel, which the
user has to open and read.

## What Changes

- Every tab reserves its last content row for a hint line, the way Services already does.
- The line lists the tab's own keys in `[key] action` form, in priority order, and clips
  at the right edge.
- Hints follow the current state: Fewer or More details, Show or Hide panel, Enable or
  Disable, and the service scope note.
- Processes: end task, signal, priority, details toggle.
- Details: panel toggle, end task, signal, priority.
- Startup: enable or disable, then the selected entry's command.
- Users: show or hide the detail panel.
- App history: properties.
- Performance: resource selection and Appearance.
- Services keeps its existing start, stop, and restart hints through the shared line.
- No new key bindings; the line only exposes what already works.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: adds a requirement that each tab shows its contextual shortcuts in one
  bottom hint line, with the line following state and never covering data.

## Impact

- `src/ui/chrome.rs`: a shared hint builder and one-line renderer.
- `src/ui/processes.rs`, `details.rs`, `services.rs`, `startup.rs`, `users.rs`,
  `app_history.rs`, `performance.rs`: reserve the hint row and drop their own hints.
- `README.md`: note that every tab names its keys in the bottom line.
- Scrollbars and row counts shrink by one where the hint row is new.
