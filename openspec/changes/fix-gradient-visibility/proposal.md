# Proposal

## Why

The previous gradient change stopped the ramp at 28% toward the surface. The top is no
longer washed out, but the fade is now too shallow to see: base and top cols look nearly
identical.

## What Changes

- Raise the top stop to 50% and the midpoint to 22%, so the fade is visible while the top
  still carries the series hue.
- Change the requirement from "closer to the chart colour than the surface" to "visibly
  different from both the base and the surface".

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: the gradient must be perceptible, not merely non-washed-out.

## Impact

- `src/ui/graph.rs`: the ramp stops and the gradient test.
