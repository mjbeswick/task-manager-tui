# Proposal

## Why

Choosing Line, Dots, or Blocks changes the main resource graph but not the rail
sparklines or the per-core mini charts, which stay filled. The Appearance setting says
it applies to every chart.

## What Changes

- Rail sparklines render with the selected style.
- Per-core mini charts render with the selected style, including their own resampling.
- Filled and Blocks keep their existing shapes; Line and Dots use braille in minis too.
- ASCII minis keep ASCII markers.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: adds that sparklines and mini charts follow the selected style.

## Impact

- `src/ui/graph.rs`: a shared mini-series plotter, used by `sparkline`.
- `src/ui/performance.rs`: the rail and per-core calls pass the style.
