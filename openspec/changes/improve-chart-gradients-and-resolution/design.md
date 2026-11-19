# Design

## Context

See `proposal.md` - Why.

- `graph::mix` blends sRGB components directly and returns non-RGB colours unchanged.
- `fill_column` chooses one mixed colour per row and uses the raw chart colour on the
  top edge cell.
- `Dots` stores a braille mask per cell and flushes every cell with one style.
- `draw` maps each column to a history slot with integer arithmetic, so a narrow plot
  picks one sample and a wide plot repeats samples.
- 16-colour and ASCII modes already divert through `coarse`.

## Goals / Non-Goals

**Goals:**

- Noticeably smoother fills without new glyphs or dependencies.
- A line whose colour carries the same height information as its position.
- Resampling that never loses a spike or invents a value beyond the history.

**Non-Goals:**

- Dithering, gradients across the time axis, or animated transitions.
- Sextants or any glyph with worse font coverage than braille.
- Per-channel alpha, which terminal cells do not have.
- Changing the palette or the chart-colour choices.

## Decisions

### Linear-light blending

Add `theme::mix_linear(a, b, t)`: convert each sRGB channel to linear light with the
existing transfer function, interpolate, convert back, and round. Non-RGB colours return
`a`, matching `mix`.

*Alternative.* Keep the sRGB `mix`. Rejected: its midpoint is darker than both ends,
which is what makes the current ramp muddy.

### A three-stop ramp

`graph::ramp(edge, bg, rise)` returns the colour for a fill height `rise` in 0..1:

- 0 to 0.5: `mix_linear(edge, mid, rise * 2)`
- 0.5 to 1: `mix_linear(mid, bg, (rise - 0.5) * 2)`

`mid = mix_linear(edge, bg, 0.45)`. The cell heights are already eighths, so a tall plot
gets one colour per row and at least eight distinct values over eight rows. Coarse mode
returns `edge` for every row, as today.

*Alternative.* A two-stop edge-to-surface ramp. Rejected: the middle of a tall fill then
loses the hue that identifies the series.

### Braille cells carry their peak

`Dots` gains a `peak: Vec<f32>` beside the mask, one value per cell. `set` records the
fraction of the dot it plots and keeps the maximum; `line` records the fraction of the
segment's endpoint sample across every dot it sets. Flushing maps `peak` through the same
`ramp`, so a cell at the top of a line is brighter than a cell near the axis. Coarse mode
uses the flat chart colour.

*Alternative.* One colour per plotted sample. Rejected: several samples share a cell, and
the cell has one foreground, so the choice has to collapse to one value anyway.

### Resampling is a pure function of the history

`sample_column(values: &[f32], column: usize, width: usize) -> Option<f32>` treats the
values as newest-last and maps `column` from the right:

- `start = column * len / width`, `end = (column + 1) * len / width`.
- If `start >= len`, return `None` so the left side stays empty.
- When `end - start < 1`, interpolate between the two neighbouring values at the column
  midpoint.
- Otherwise return the maximum value in the covered range.

The plot calls this once per column. Upscaling therefore draws a smooth edge, and
downscaling keeps a one-sample spike.

*Alternative.* Maximum for both directions. Rejected: a wide plot would repeat samples
and keep the visible steps this change removes.
*Alternative.* Mean for downscaling. Rejected: a 100 percent spike between quiet samples
would disappear.

## Risks / Trade-offs

- [Colour count] Some themes have close chart and surface colours, so eight steps may
  collapse after rounding → Tests use each theme's own colours and require the count only
  when contrast allows; otherwise the ramp still never runs backwards.
- [Memory] One float per braille cell → The grid is plot-sized, a few thousand cells at
  worst.
- [Braille colour] A cell holding a descending segment takes the higher endpoint → The
  line still reads as a gradient, and the cell has only one foreground.
- [Interpolation] A single isolated sample upscales to a narrow tent shape → Correct for a
  line chart and hidden under a filled one.
