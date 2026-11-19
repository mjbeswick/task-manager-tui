# Proposal

## Why

The strip is flat: numbered labels over a continuous rule. It does not read as a set of
tabs, and the active tab is only a differently coloured label and span.

## What Changes

- Add one row above the tab labels for the active tab's top border.
- Draw the active tab as a box with `┌ ┐ │ └ ┘ ─`: top border on the new row, vertical
  sides on the label row, and a bottom border that replaces the accent span of the
  continuous rule.
- Labels shift one column right to leave room for the left side, keeping every digit in
  place relative to its label.
- Inactive tabs keep their labels and the continuous rule, with no separators.
- Narrow layouts that drop the active tab simply draw no box.
- ASCII mode draws the same shape with `+`, `-`, and `|`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the active tab becomes a raised box drawn with box characters, sitting on
  the continuous rule.

## Impact

- `src/ui/chrome.rs`: six-band layout, box drawing, tab start offset.
- `README.md`: the tab shape and the extra row.
- Render snapshots for every tab.
- Coordination: archive `simplify-tab-strip` before this change.
