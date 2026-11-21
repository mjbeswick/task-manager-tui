# Proposal

## Why

The Tabs menu duplicates the tab strip at every width. At 100 columns it repeats labels
that are already visible and clickable, and the menu bar carries a permanent entry for a
control the strip already provides.

## What Changes

- Remove the Tabs menu from the menu bar, leaving File, Options, View, Process, and Help.
- Show all seven full labels when they fit, instead of shortening them below 90 columns.
- Replace the whole strip with one `Tabs: <active> ▾` button when the labels do not fit.
- Put the tab list in a dropdown under that button, with the active tab marked and the
  `1`-`7` keys shown.
- Keep `Alt+T` as the keyboard path to the tab list.
- Keep `Tab`, `Shift+Tab`, and `1`-`7` unchanged.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the menu bar loses Tabs, the strip becomes adaptive, and the tab list opens
  from its button or `Alt+T`.

## Impact

- `src/app.rs`: menu table and tab-menu state.
- `src/ui/chrome.rs`: strip fit check, button, and dropdown.
- `src/action.rs`, `src/input.rs`: tab-menu actions and accelerators.
- `src/ui/hits.rs`: button and item targets.
- Tests, snapshots, help, README.
