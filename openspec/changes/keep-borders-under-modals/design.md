# Design

## Context

- `dim` sets each cell to `fg(muted).bg(current bg)`.
- Box-drawing characters are used for tab rules, table rules, chart frames, and panel
  borders.

## Decisions

### Preserve box-drawing foregrounds

For each cell, read its symbol. If it is in the box-drawing block (`U+2500..U+257F`) or is
an ASCII `-`, `|`, or `+`, keep the existing foreground; otherwise use `muted`. Backgrounds
are never changed.

*Alternative.* Skip dimming entirely. Rejected: the modal would lose its focus cue.
*Alternative.* Dim the whole frame including borders. Rejected: that is the reported bug.

## Risks / Trade-offs

- [Chart glyphs] Blocks and braille still dim → They are data, not structure; the panel
  covers most of them anyway.
