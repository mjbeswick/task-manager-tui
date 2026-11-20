# Design

## Context

- `ramp(edge, bg, rise)` reaches the surface at `rise = 1`.
- `fill_column` maps body rows through `0.2 + rise * 0.65`, so the top body row is about
  85% of the way to the surface.
- Blocks uses the same shape with a `0.5` factor.

## Decisions

### End the ramp at 28% toward the surface

`ramp` uses a dim midpoint at 12% and a top stop at 28%. The fill passes `rise` directly,
so the bottom is the chart colour and the top is still recognisably the series, with a
visible fade between them.

*Alternative.* Keep a strong fade and raise the contrast by darkening the surface. Rejected:
  it changes every panel background for one chart effect.

## Risks / Trade-offs

- [Less depth] The fade is subtler than before → The tip remains the brightest cell and the
  base the strongest, so the direction still reads.
- [Sparklines] Short sparklines have fewer rows, so the ramp is compressed further → They
  already used the same function and benefit from the stronger top.
