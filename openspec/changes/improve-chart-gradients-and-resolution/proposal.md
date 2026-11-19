# Proposal

## Why

Filled charts blend one hue with the surface using a simple sRGB mix, which turns muddy
in the middle and leaves the top of a tall fill looking like the bottom of a short one.
Line charts draw every braille dot in the same colour, so a low line and a peak look
identical. Resampling picks one sample per column when the plot is narrow, dropping
spikes, and repeats samples when it is wide, leaving visible steps.

## What Changes

- Fills and sparklines use a gamma-correct, three-stop ramp: surface at the top, a dim
  tint through the middle, and the chart colour at the base.
- Braille line and dot cells take a colour from the highest sample they contain, so the
  line carries a vertical gradient from the base colour to a bright tint.
- Columns interpolate linearly between samples when the plot is wider than the history.
- Columns keep the peak of the samples they cover when the plot is narrower than the
  history, so a one-sample spike survives.
- 16-colour mode keeps flat fills; ASCII mode keeps its current renderer.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: adds requirements for smooth multi-stop gradients, value-coloured line
  cells, and peak-preserving resampling.

## Impact

- `src/ui/graph.rs`: the colour ramp, the braille colour map, and the column sampler.
- `src/theme.rs`: a reusable linear-light mix helper beside the existing `mix`.
- `README.md`: a line on the chart gradient and resampling.
- Coordination: `add-chart-resolution` is applied but not archived. This change archives
  after it and builds on its braille and gradient work.
