# Proposal

## Why

The active tab currently has a bottom border on the rule row, which closes the shape into
a button floating above the content. A real tab merges with the content below: its bottom
is open and the rule stops at its sides.

## What Changes

- Remove the active tab's `└─┘` bottom border.
- Clear the rule under the active tab so the tab opens into the content.
- Keep the tab's `│` sides, and extend them through the rule row so the opening is framed.
- Inactive tabs and the rest of the rule are unchanged.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the active tab box loses its bottom border and the continuous rule is
  interrupted under the active tab.

## Impact

- `src/ui/chrome.rs`: the rule/box drawing.
- `tests/snapshots`: every tab frame.
- Coordination: archive `add-raised-tabs` before this change.
