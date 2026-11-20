# Proposal

## Why

On an 80x24 terminal the per-core section is split into eight cramped cells of about three
rows, each with a frame and a waveform. It adds noise and steals height from the main graph
without being readable.

## What Changes

- Show per-processor charts only when each cell gets at least four rows and ten columns.
- Otherwise omit the section and give its height to the main graph and stat boxes.
- No label-only fallback cells; the section is either readable or absent.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: per-processor charts are hidden when the cells are too small, instead of
  drawn without frames.

## Impact

- `src/ui/performance.rs`: the size check and the core grid.
- Render tests and snapshots at 80x24.
