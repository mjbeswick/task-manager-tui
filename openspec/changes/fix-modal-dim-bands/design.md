# Design

## Context

- `dim` runs before the dialog panel and rewrites each cell to `fg(old fg).bg(theme.bg)`.
- That deletes chrome, status, stripes, heat, and selection backgrounds.

## Decisions

### Keep the background, dim the text

Read each cell's current background, then set its style to `fg(theme.muted).bg(current)`.
The panel covers the centre, so the dimming still reads, and every background survives.

*Alternative.* Blend every background halfway to the window background. Rejected: chrome
and status are already close to it, so they would still shift and the bands would lose
their separation.

## Risks / Trade-offs

- [Less dimming] Bright backgrounds such as a selected row stay bright → The muted
  foreground and the opaque panel still separate the dialog from the window.
