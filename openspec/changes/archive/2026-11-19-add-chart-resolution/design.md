# Design

## Context

See `proposal.md` - Why.

- `fill_column` paints one column from the bottom, resolving eighths into the top cell.
  The body uses `mix(edge, bg, 0.55)` and the edge uses the raw chart colour.
- `draw` maps each sample to a level in half-cell steps, then draws a bullet and vertical
  joins for Line and Dots.
- `sparkline` calls `fill_column` directly, so it inherits any fill change.
- ASCII mode and 16-colour mode already divert through `coarse`.
- `mix` handles RGB only; other colour forms pass through.

## Goals / Non-Goals

**Goals:**

- Sub-cell curves with no new dependency and no new glyph table.
- A gradient that needs no per-cell truecolor to look like one.
- Keep every existing style name and key.

**Non-Goals:**

- Quadrant or sextant blocks.
- Horizontal sub-cell resolution for Filled.
- Per-pixel anti-aliasing or dithering.
- New chart styles; the only new colour is Theme, which reads the active palette.

## Decisions

### Braille for Line and Dots

A plot of W columns by H rows becomes a dot grid of 2W by 4H. Each braille cell holds a
`u8` mask. For every sample, map its value to a dot row, set that dot for Dots, and walk a
Bresenham segment from the previous dot for Line. Cells are flushed as
`char::from_u32(0x2800 + mask)` with the chart colour.

The dot grid is allocated once per `draw` call, sized to the plot, so cost stays linear
in plot area. Empty cells are never written, leaving the grid and surface visible.

*Alternative.* Quadrant blocks. Rejected: 2 x 2 per cell is half the vertical resolution
and font coverage is worse.
*Alternative.* Sextants. Rejected: poor font coverage.
*Alternative.* Keep whole-row stepping. Rejected: this is the problem being solved.

### ASCII keeps the current renderer

Braille has no ASCII twin. When `theme.ascii` is set, Line and Dots take the existing
bullet and vertical bar path unchanged. The spec names this fallback so a console does
not get silent boxes.

### Theme as a chart colour

`ChartColor::Theme` resolves to `theme.graph_fill`, so charts follow the window palette.
The explicit colours stay, because some users want a fixed hue across themes. This is one
variant, one parse key, and one match arm; no renderer change.

### Gradient by row, not by cell

`fill_column` already walks rows from the bottom. For each row it computes
`t = row_from_top / height` and uses `mix(edge, bg, 0.15 + 0.6 * t)`, clamped. The bottom
stays near the chart colour, the top fades toward the surface. The top edge cell keeps
the stronger edge colour, preserving the existing edge highlight.

The gradient uses one colour per cell row, so truecolor shows the full ramp, 256 shows
banding, and 16-colour skips blending entirely and uses the flat fill it draws today.

*Alternative.* A gradient across the time axis. Rejected: it would encode recency a
second time and fight the right-edge value label.

## Risks / Trade-offs

- [Font coverage] Some console fonts lack braille → `--ascii` and 16-colour consoles keep
  the old renderer; the README notes the requirement.
- [Performance] A dot grid per chart per frame → The grid is plot-sized, and charts are a
  few hundred cells; the existing per-frame depth pass is heavier.
- [Banding] 256-colour terminals show visible steps → Acceptable; the ramp is one colour
  per row, which is already the resolution of the terminal.
- [Tests] Existing tests assert bullet and vertical bar symbols → They move to the ASCII
  theme, and new tests assert braille masks.
