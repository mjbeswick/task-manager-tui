# Proposal

## Why

The gradient blends toward `theme.surface`. In themes where that surface is close to the
series colour, the fade has almost no distance to travel, so the fill looks flat. Other
themes are fine because their surface is far from the series colour.

## What Changes

- Choose the gradient's far end by contrast against the series colour: darken a bright
  series, lighten a dark one, whichever has more contrast.
- Keep the same midpoint and top proportions, gamma-correct blending, and no rim.
- `ramp` no longer needs the background colour.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: the gradient must be visible for every theme, not just those whose surface
  is far from the series colour.

## Impact

- `src/ui/graph.rs`: the ramp target and call sites.
- Tests across all themes.
