# Proposal

## Why

The top edge of a filled chart is painted with the raw chart colour while the row below is
faded. The result is a bright highlight line, a darker band just below it, then the fill
brightening toward the base: a non-monotonic gradient that reads as a flat block with a rim.

## What Changes

- When gradients are on, the tip cell uses the top of the ramp instead of the raw colour.
- The fill then darkens monotonically from base to top with no highlight rim.
- Gradients off still draw one flat colour.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: the gradient has no bright rim and runs in one direction.

## Impact

- `src/ui/graph.rs`: the tip colour in Filled and Blocks.
- Render tests for the top two rows.
