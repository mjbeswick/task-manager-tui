# Design

## Context

- `ramp` stops at 22% and 50%; the fill maps rows across 5% to 100% of that range.
- A tall chart still looks nearly flat in the Phosphor theme.

## Decisions

### A deeper top stop

Mid 32%, top 68%. The top remains a recognisable tint of the series against the surface,
while the base-to-top difference roughly doubles.

*Alternative.* Add a gradient-strength setting. Rejected: On/Off is enough, and a third
preference for one visual is clutter.

## Risks / Trade-offs

- [Light themes] A 68% blend toward a near-white surface is pale → It still differs from
  the surface, which the test enforces, and the gradient is the point.
