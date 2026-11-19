# Proposal

## Why

The Options menu carries a Next theme item that cycles the theme, while `t` and the
Appearance dialog already do the same and show the current theme by name. The extra item
duplicates the action in a place where the current theme is not visible.

## What Changes

- Remove Next theme from the Options menu.
- Remove the menu item variant behind it.
- Keep `t` and the Appearance dialog picker.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: theme switching is described through `t` and the Appearance dialog, not the
  Options menu.

## Impact

- `src/app.rs`: the menu table and item enum.
- `src/action.rs`: the menu dispatch arm.
- `tests/snapshots`: the Options menu frame.
