# Proposal

## Why

The tab menu duplicates the strip. `Alt+T` and the `Tabs: <active> ▾` button both open
a list of the seven tabs that are already on screen, and the button deletes the strip
at exactly the widths where the labels are still useful. The previous change removed
the strip instead of the menu, which left no tabs on screen at all.

## What Changes

- Restore the tab strip, its active-tab box, and the rule beneath it.
- Remove the tab dropdown: `Alt+T`, its state, its actions, its hit targets, and its help row.
- Remove the `Tabs: <active> ▾` button.
- Keep all seven labels visible at every width by shortening them: full labels when
  they fit, then short labels, then three-letter labels.
- Keep `1`-`7`, `Tab`, `Shift+Tab`, and clicking a label.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the strip is the only tab list, labels shorten instead of collapsing
  into a button, and the keyboard no longer has an `Alt+T` list.

## Impact

- `src/app.rs`: tab labels by width; tab-menu state removed.
- `src/ui/chrome.rs`: strip fit tiers; dropdown removed.
- `src/action.rs`, `src/input.rs`, `src/ui/hits.rs`: tab-menu action and targets removed.
- `src/ui/dialog.rs`: help row removed.
- Tests, snapshots, README, screenshots.
