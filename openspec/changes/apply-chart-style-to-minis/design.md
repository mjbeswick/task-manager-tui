# Design

## Context

- `graph::draw` already renders the selected style for the main and per-core charts? Only
  the main chart uses `draw`; the per-core cells call `draw_mini`, which always fills.
- The rail calls `graph::sparkline`, which always fills.
- Both mini paths use one sample per column, newest on the right.

## Decisions

### One mini plotter, style-aware

Add `plot_series(buf, area, series, maximum, style, color, bg, theme)`, which shares the
style switch with the main renderer but omits the grid, labels, and history resampling.

- Filled: `fill_column`.
- Blocks: `blocks_column` with the older column's total, the same as the main chart.
- Line and Dots: a braille `Dots` grid sized to the mini area, flushed with the ramp.
- ASCII Line and Dots: one bullet per sample.

`sparkline` becomes a thin wrapper that resolves `max` and calls `plot_series`. The
per-core `draw_mini` delegates to `sparkline` with `max = 100`.

*Alternative.* Route minis through `draw`. Rejected: it draws a grid, value label, and
footnote, none of which belong in a one- or two-row cell.

## Risks / Trade-offs

- [One-row line charts] Four braille dot rows in one cell row → The marker is coarse but
  still shows direction; Filled and Blocks remain the natural choice at that size.
- [Duplication] The main renderer keeps its own loop for resampling and labels → Shared
  plotting could be factored later without changing behaviour.
