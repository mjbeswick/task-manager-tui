# Proposal

## Why

The App history tab can only inspect an entry or wipe all history. Users expect to clear a
single app's totals or end the app's running processes from the row they selected.

## What Changes

- `x` clears the selected entry after confirmation; other entries stay.
- `e` or Delete ends every running process for the selected app after a confirmation that
  names the app and the process count.
- The hint line names both actions.
- The menu keeps Clear app history as the wipe-everything action.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `app-history`: adds per-entry clear and end-app actions.

## Impact

- `src/action.rs`, `src/app.rs`, `src/input.rs`: actions, confirmations, effects.
- `src/store/history.rs` and `src/sample/mod.rs`: remove one record.
- `src/main.rs`: handle the new effect.
- Tests and README.
