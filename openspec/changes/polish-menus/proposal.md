# Proposal

## Why

The menu bar was added to make every tab reachable on a narrow terminal, but the menus
themselves stayed minimal. Items do not show their keys, toggles show no state, View mixes
settings with actions, and several shortcuts act in the wrong context.

## What Changes

- Regroup the menus: File holds run and data actions, Options holds speed and display
  settings, View holds refresh, sorting, and expansion.
- Show a right-aligned shortcut on every menu item, separators between groups, a radio on
  the active update speed, and a check on each enabled display toggle.
- Mark items that cannot act on the current tab as muted with a short reason, and make
  activation do nothing.
- Scope the `s`, `S`, and `R` service keys to the Services tab.
- Ignore Alt menu accelerators while a dialog or the filter field has focus, and stop
  Ctrl/Alt plus a letter from triggering plain-letter commands.
- Accept `Ctrl+F` to start a filter.
- Make the Help panel scrollable and grouped so every key is reachable at 40x12.
- Name End tree in the Processes and Details hint rows.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: menu layout, item hints and state, context-scoped shortcuts, filter key,
  shortcut hints, and the help panel.

## Impact

- `src/app.rs`: menu table, item state and availability.
- `src/ui/chrome.rs`: dropdown rendering.
- `src/action.rs`, `src/input.rs`: shortcuts, disabled activation, help scrolling.
- `src/ui/dialog.rs`: help layout.
- Tests, snapshots, help text, README.
