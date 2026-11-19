# Proposal

## Why

The hint line and the status bar are drawn in one muted colour from end to end, so keys,
labels, and values all read with the same weight. The eye has to parse the line to find
the key or the figure.

## What Changes

- Hint keys and their brackets use the accent colour and bold; the actions stay muted.
- Status-bar labels stay muted; values use the brighter text colour.
- The update-speed value uses the accent colour.
- No wording, width, or layout changes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: hint keys are accented, and status-bar labels and values are visually
  distinct.

## Impact

- `src/ui/chrome.rs`: hint drawing and status metric drawing.
- Render tests for both.
