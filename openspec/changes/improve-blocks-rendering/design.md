# Design

## Context

- `blocks_column` compares the top cell's fill with the older column's fill in the same
  row.
- `▛` (`UL+UR+LL`) has an empty bottom-right, so a bottom-anchored fill loses a quadrant
  at the boundary. `▙` (`UL+LL+LR`) is the horizontal mirror of `▟` and keeps the floor.
- `◢`/`◣` paint a full-height edge, so they overstate a one-eighth step by seven eighths.
- `sample_column` treats `span <= 1.0` as interpolation; at native width the midpoint is
  between two samples, so it averages them.

## Decisions

### A pure glyph table

`slope_glyph(own: u8, other: u8, glyphs: &Glyphs) -> char` with `own` 1..=8 and `other`
0..=8:

- `own == 8` -> `block_full`.
- `other == 0` or `|d| <= 2` -> `eighths[own-1]`.
- `|d| >= 6` and `d > 0` -> `slope_rise`; `d < 0` -> `slope_fall`.
- `d > 0` and `own >= 5` -> `block_rise`.
- `d < 0` and `other >= 5` and `own <= 5` -> `block_fall`.
- otherwise `eighths[own-1]`.

where `d = own - other`. This keeps every return within 3/8 of the true edge on both
sides, and the fallbacks are exact.

### Glyphs carry the angular characters

`Glyphs` gains `block_rise`, `block_fall`, `slope_rise`, `slope_fall`, all `Option<char>`.
Unicode sets them; ASCII sets `None`, so the guard is structural rather than an ad-hoc
`theme.ascii` check.

### Native width is exact

`span <= 1.0` becomes `span < 1.0`. At width 60 the peak branch then reads the single
sample in its column. Widths 59 and below already used the peak branch; 61 and above
already interpolated.

### Blocks uses a tighter ramp

The body within a Blocks column ends at `0.2 + rise * 0.5` toward the background instead
of `0.65`, so the slope glyph is not the only visible part of the silhouette. Filled keeps
its existing ramp.

## Risks / Trade-offs

- [Fewer slopes] Some columns that used to draw a triangle now draw an eighth → The
  eighth is the accurate shape for a small step.
- [Spike asymmetry] A spike still gets a slope on one side and a cliff on the other → No
  Block Elements glyph has a middle hole; the alternative is a lie.
- [Fonts] Angular glyphs remain less universal than braille → They are opt-in through
  Blocks, and ASCII mode disables them.
