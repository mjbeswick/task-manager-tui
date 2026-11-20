# Proposal

## Why

Opening a modal runs a dim pass that replaces every cell background with the window
background. That erases the chrome and status bands, so the first and last rows change
colour the moment a dialog opens.

## What Changes

- Dim only the foregrounds, leaving every background as drawn.
- The menu, tab strip, status bar, row stripes, and selection keep their colours under a
  modal.
- The modal panel still paints its own opaque surface.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: a modal must not change the chrome or status backgrounds.

## Impact

- `src/ui/dialog.rs`: the dim pass.
- A render test with a modal open.
