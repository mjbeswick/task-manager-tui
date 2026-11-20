# Design

## Context

- `surface_alt` is hand-written per theme; Phosphor is `#121A12` on `#0F160F`, Amber is
  `#1A130C` on `#16100A`.
- `ramp` blends toward black or white by a fixed 20% midpoint and 55% top, independent of
  theme.

## Decisions

### Visible stripes

Set Phosphor to `#162216` and Amber to `#22180D`. Both clear six channel steps from their
surface while staying in the same family. A test requires `max(|surface - surface_alt|) >= 5`
for every theme.

### A per-theme gradient multiplier

Add `gradient_strength: f32` to `Theme`, 1.0 by default, 1.5 for Phosphor and Amber.
`ramp` scales its midpoint and top stops by it, clamped so the top never exceeds 90% toward
the target. Direction, gamma correction, and the no-rim rule are unchanged.

*Alternative.* Raise the global stops. Rejected: the other eight themes are already
balanced.

## Risks / Trade-offs

- [Stronger fade] Phosphor's top becomes noticeably darker → That is the request; the tip
  still follows the ramp and never becomes black or white.
