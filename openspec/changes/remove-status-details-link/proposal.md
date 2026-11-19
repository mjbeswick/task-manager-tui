# Proposal

## Why

The Processes tab now lists `[f] Fewer details` in its bottom hint line. The status bar
also draws a clickable `Fewer details` link at the bottom left, so the same action appears
twice and the link competes with the position counter for space.

## What Changes

- Remove the clickable Fewer or More details label from the status bar.
- Remove its hit target; the status bar left side shows only the position counter.
- Keep the `f` key, the View menu item, and the hint line.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the status bar summary no longer includes the Fewer details hint.

## Impact

- `src/ui/chrome.rs`: status bar layout and hit recording.
- `src/ui/hits.rs`, `src/action.rs`: remove the unused target and its click arm.
- `tests/mouse.rs`, `tests/snapshots`: update expectations.
