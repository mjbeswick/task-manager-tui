# Proposal

## Why

Opening a modal dims every foreground to `muted`, including the table rule, chart frames,
and panel borders. The structure visibly changes colour behind the dialog.

## What Changes

- Dim text only: letters, digits, and ordinary characters.
- Keep the existing foreground for box-drawing lines and their ASCII twins.
- Backgrounds stay as they are, as before.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: modal dimming must leave structural lines in their colours.

## Impact

- `src/ui/dialog.rs`: the dim pass.
- A render test comparing a rule before and after opening a dialog.
