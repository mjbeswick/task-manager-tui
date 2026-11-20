# Proposal

## Why

On narrow terminals the tab strip drops labels, and a dropped tab is only reachable by
cycling or a digit. A top-level Tabs menu keeps every tab visible and selectable at any
width.

## What Changes

- Add a Tabs menu between View and Process, listing all seven tabs.
- Selecting an item switches to that tab.
- Alt+T opens it; Alt+P and Alt+H move to indices 4 and 5.
- At 40 columns the six menu labels are drawn with compact spacing so every menu still
  fits without truncation.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the menu bar gains a Tabs menu and compact spacing on narrow terminals.

## Impact

- `src/app.rs`: menu table and item variant.
- `src/action.rs`: item mapping.
- `src/input.rs`: Alt+T and shifted indices.
- `src/ui/chrome.rs`: shared menu offsets and compact padding.
- Tests, help, README.
