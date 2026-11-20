# Proposal

## Why

The Appearance dialog uses Left and Right to change its values, but the priority dialog
uses only Up, Down, and Page keys. Left and Right are unhandled there and are the obvious
decrease and increase controls.

## What Changes

- Left decreases the nice value by one; Right increases it by one.
- Up, Down, PgUp, and PgDn keep their current steps.
- The dialog hint names the new keys.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `details`: the priority step scenario includes Left and Right.

## Impact

- `src/input.rs`: two key mappings.
- `src/ui/dialog.rs`: the hint line.
- Tests for input and stepping.
