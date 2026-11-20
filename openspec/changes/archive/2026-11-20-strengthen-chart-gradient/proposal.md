# Proposal

## Why

Filled charts fade almost all the way to the surface at the top, so tall fills look
washed out and lose the series colour. The gradient is doing more work than the data.

## What Changes

- Compress the ramp so the fill's top stays closer to the chart colour than to the
  surface.
- Use the full base-to-top rise instead of starting partway up the ramp.
- Sparklines and Blocks inherit the same ramp.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: the gradient keeps the series hue at the top.

## Impact

- `src/ui/graph.rs`: `ramp`, `fill_column`, and the Blocks body ramp.
- Render tests for the fill tip.
