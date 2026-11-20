# Design

## Context

- `draw_cpu` shows cores when `logical > 1 && inner.height >= 12`.
- `draw_cores` picks 4, 2, or 1 columns and falls back to a label-only cell when a frame
  does not fit.

## Decisions

### Require a real cell

Compute the same column count and row count that `draw_cores` would use, then require
`cell_height >= 4` and `cell_width >= 10`. Only then allocate the cores strip. Otherwise
the layout gives that height to the main graph.

Remove the unframed fallback: if a cell still fails the check, skip it rather than draw a
label-only box.

*Alternative.* A fixed terminal-height threshold. Rejected: width and core count both
change the cell size, so the threshold would misfire.

## Risks / Trade-offs

- [Fewer cores on screen] Machines with many cores hide the section earlier → That is the
  intent; the main graph stays readable.
