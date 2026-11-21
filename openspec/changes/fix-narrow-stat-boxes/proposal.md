# Proposal

## Why

Narrow Performance panels show stat labels with no values. The CPU panel without core
charts gives its stat boxes three rows, but two rows of boxes need four. Memory loses its
second row to the footnote. Below 40 columns the boxes wrap to three rows, which
`stat_height` does not count.

## What Changes

- Size the stat area from the same per-row count that lays the boxes out.
- Use that height on every panel, including CPU without core charts.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: every stat box shows its value at every supported size.

## Impact

- `src/ui/performance.rs`: `stat_height`, `draw_stat_row`, and `draw_cpu`.
