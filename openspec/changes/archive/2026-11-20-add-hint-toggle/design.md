# Design

## Context

- `chrome::hint_area(area)` reserves the last content row when the area is tall enough.
- Every tab checks that helper and draws through `draw_hint`.
- Settings already persist theme, chart style, colour, and gradient.

## Decisions

### One persisted boolean

`Settings.show_hints` defaults to true. `App` mirrors it, and `hint_area` takes the app so
it can return `None` when hidden or when there is no room.

### F4 and the View menu

F4 is free (F1 help, F2 appearance, F5 refresh, F10 menu). The View menu gains a Shortcut
hints item that dispatches the same action, so the toggle is discoverable and clickable.

*Alternative.* Put it in Options. Rejected: it is a view preference, like details and
columns.

## Risks / Trade-offs

- [Layout jump] Hiding the line gives the table one extra row → Expected; the setting is
  about reclaiming space.
