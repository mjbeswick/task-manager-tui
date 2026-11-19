# Proposal

## Why

A review of the Blocks renderer found three defects: the falling quadrant is the wrong
mirror and punches a hole in the fill; triangles fire on one-eighth steps and paint a
full-cell edge; and native-width sampling averages a spike away at exactly 60 columns.

## What Changes

- Falling slopes use `▙`, the horizontal mirror of `▟`, never a top-empty quadrant.
- One- and two-eighth steps keep the exact eighth block. Triangles are reserved for six or
  more eighths, and no slope glyph is drawn when the older column is empty.
- The top glyph is chosen by one pure function with a full 8 x 9 table test and
  bottom-anchored/mirror invariants.
- `sample_column` is exact when the plot width equals the history length.
- Slope glyphs move into `Glyphs`, with `None` in ASCII mode.
- Blocks uses a slightly tighter ramp so the tip does not sit on an almost invisible body.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: Blocks glyph selection is specified, and native-width sampling is exact.

## Impact

- `src/ui/graph.rs`: glyph selection, the column loop, and the sampler.
- `src/theme.rs`: optional slope glyph fields.
- Tests for the glyph table, invariants, sampling, and an end-to-end draw.
