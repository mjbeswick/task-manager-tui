# Proposal

## Why

End task, End tree, Signal, Priority, and Properties are keyboard-only or hidden behind `i`
and the Enter modal. Every other major action has a menu path; process actions do not.

## What Changes

- Add a Process menu between View and Help.
- Items: End task, End tree, Signal, Priority, Properties, each mapping to the existing
  action.
- Alt+P opens it; Alt+H still opens Help, now at index 4.
- On tabs without a selected process, the existing notices explain why nothing happens.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the menu bar gains a Process menu.

## Impact

- `src/app.rs`: menu table and item variants.
- `src/action.rs`: item mapping.
- `src/input.rs`: Alt+P.
- Tests, help text, README.
