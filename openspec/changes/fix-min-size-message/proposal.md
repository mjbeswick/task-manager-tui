# Proposal

## Why

Below the minimum size the warning is cut off. At 39x12 it reads "taskmgr needs at least
40 columns and 1", which hides the row count and the current size.

## What Changes

- Wrap the warning at word boundaries and centre each line.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the too-small message is shown in full.

## Impact

- `src/ui/chrome.rs`: the too-small branch of `draw`.
