# Proposal

## Why

The help panel is missing several keys added recently, and there is no in-app way to see
the version, platform, active appearance, or where settings and history are stored.

## What Changes

- Correct the help list: add `i`, the gradient in the F2 row, and both meanings of Space.
- Add Help > About with version, platform, theme, colour depth, settings path, and data path.
- About is a read-only modal, closed like Help.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: adds the help accuracy and About panel requirement.

## Impact

- `src/ui/dialog.rs`: help rows, About variant.
- `src/app.rs`, `src/action.rs`: menu item and dialog.
- Tests and README.
