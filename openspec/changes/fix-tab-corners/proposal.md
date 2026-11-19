# Proposal

## Why

The open active tab uses `└` on the left and `┘` on the right, which closes the shape into
a box with a missing middle. The rule should flow into the sides instead, so the corners
must turn outward.

## What Changes

- Swap the rule-row corners: `┘` on the left, `└` on the right.
- Keep the middle empty and the rule resuming after the right corner.
- No other tab geometry changes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the open tab's bottom corners face outward to meet the rule.

## Impact

- `src/ui/chrome.rs`: two glyph choices.
- `README.md`: the corner wording.
- Render snapshots.
- Coordination: archive `add-open-tab-corners` before this change.
