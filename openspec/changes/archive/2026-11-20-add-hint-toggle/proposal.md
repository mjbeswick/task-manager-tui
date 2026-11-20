# Proposal

## Why

The contextual hint line costs a row on every tab. Users who know the keys may prefer the
extra table row, and there is currently no way to reclaim it.

## What Changes

- F4 toggles the hint line.
- View > Shortcut hints mirrors the toggle for the mouse.
- The choice persists with the other settings.
- When hidden, no row is reserved and the table or graph gains it back.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the hint line can be hidden and the choice persists.

## Impact

- `src/store/settings.rs`, `src/app.rs`: setting and state.
- `src/input.rs`, `src/action.rs`, `src/app.rs`: key, menu item, toggle.
- `src/ui/chrome.rs`: hint reservation checks the setting.
- Help, README, tests.
