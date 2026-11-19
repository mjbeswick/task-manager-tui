# Proposal

## Why

Up and Down do nothing on the Performance tab. The specification already says they move
the resource rail, and the mouse wheel over the rail dispatches the same actions, so the
rail is unreachable from the keyboard.

## What Changes

- Up and Down move the Performance rail to the previous or next resource, wrapping at the
  ends.
- The change is a bug fix: the rail requirement already describes this behaviour, so no
  spec changes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. The existing `performance` rail requirement already covers this behaviour; the
implementation did not match it.

## Impact

- `src/app.rs`: `move_selection` handles the Performance tab.
- `src/action.rs` tests: cover the rail keys.
