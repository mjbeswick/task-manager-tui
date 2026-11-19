# Proposal

## Why

The open active tab currently ends in two straight `│` cells on the rule row. The sides
run into the rule without a join, so the tab reads as a pair of loose bars rather than an
open box.

## What Changes

- Replace the two `│` cells at the bottom of the active tab with `└` and `┘` corners.
- Keep the middle of the bottom empty, so the tab still opens into the content.
- Keep the rule resuming after the right corner.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the open active tab closes its sides with box corners on the rule row.

## Impact

- `src/ui/chrome.rs`: two characters in the rule/box drawing.
- `README.md`: the tab-shape wording.
- Render snapshots.
