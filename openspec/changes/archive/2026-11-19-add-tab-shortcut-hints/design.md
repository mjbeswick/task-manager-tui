# Design

## Context

See `proposal.md` - Why.

- Services and Startup each draw their own hint on the last content row. The other five
  tabs use every content row for data.
- Each list computes `body_height` from the content area and passes it to the scrollbar
  geometry, so adding a reserved row changes both the visible count and the bar.
- The status bar already shows a single `Fewer details` action for Processes.

## Goals / Non-Goals

**Goals:**

- One hint implementation, one style, one place to add a tab.
- Hints derived from state, not stored, so they cannot go stale.
- No new keys and no change to what any key does.

**Non-Goals:**

- Clickable hints. The keys and the existing mouse targets remain separate.
- Multi-line hints or a help overlay.
- Replacing the global help dialog.
- Hiding hints behind a setting.

## Decisions

### A shared, pure hint builder

`chrome::tab_hints(app) -> String` returns the line for the active tab. It reads only
`App` state and is unit-tested per tab. `chrome::hint_area(area, app) -> Option<Rect>`
returns the last content row when `area.height >= 4` and the string is non-empty, and
draws nothing itself.

Each tab calls `hint_area` first, shrinks its body by one when it is `Some`, and draws
the line last through `chrome::draw_hint`.

*Alternative.* A footer inside each tab. Rejected: five copies of the same formatting and
clipping logic, which is exactly the duplication Services and Startup already show.

### Priority order, then clip

Each tab lists its most-used action first. Processes: end, signal, priority, details.
Details: panel, end, signal, priority. Services: start, stop, restart, scope. Startup:
toggle, command. Users: panel. App history: properties. Performance: resource, appearance.

The line is clipped with the existing `truncate`, so a narrow terminal loses the least
important tail rather than wrapping or overlapping a row.

*Alternative.* Drop individual hint items to fit. Rejected: it needs width measurement
per item and still ends up truncating on very narrow terminals.

### Keys in brackets, actions in muted text

The style matches the Services line today: `muted` on `surface`, one space after the
closing bracket, three spaces between items. Keeping one style means the line reads as
status, not as a second toolbar.

### State is read on every frame

The builder checks `more_details`, the Details and Users panel flags, the selected
startup row, and the selected service scope. Nothing is cached, so a toggle shows on the
next frame.

### The reserved row is part of the body calculation

Every tab subtracts the hint row before computing `visible`, the scrollbar geometry, and
the panel split. Services and Startup delete their private hint code and use the shared
line. Performance reserves the row before splitting the rail and panel, so the graphs
lose one row but nothing overlaps.

## Risks / Trade-offs

- [One less data row] Every tab gives up a row → The hint is the point of the change, and
  the minimum height still leaves a usable table.
- [Long commands] Startup's command can consume the line → The key hint is first, so the
  command truncates instead of the shortcut.
- [Test churn] Body heights and scrollbar counts shift by one → Existing render snapshots
  and row-count tests are updated with the change, not weakened.
