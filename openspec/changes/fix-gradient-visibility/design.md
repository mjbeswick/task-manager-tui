# Design

## Context

- `ramp` stops at 12% and 28% toward the surface.
- `fill_column` starts at 5% of the ramp, so the top body row is about 27% faded.
- The 28% top is numerically a gradient but below what the eye reads as one.

## Decisions

### A mid-range top

Set the midpoint to 22% and the top stop to 50%. The base stays at the chart colour, the
top is half way to the surface: clearly lighter, still recognisably the series.

*Alternative.* Return to 85%. Rejected: that is the washed-out look this work started
from.
*Alternative.* Add a setting for gradient strength. Rejected: another preference for a
single visual, when On/Off already exists.

## Risks / Trade-offs

- [Linear-light distance] A 50% linear blend is brighter in sRGB than a naive 50% looks →
  The test measures a minimum channel distance rather than asserting a ratio.
