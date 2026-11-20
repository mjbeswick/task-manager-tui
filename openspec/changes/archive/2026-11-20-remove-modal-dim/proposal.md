# Proposal

## Why

Dimming changed colours behind the dialog and caused two rounds of fixes: first the bands,
then the borders. The modal panel already has a border and a surface, so the dim pass adds
friction without much focus benefit.

## What Changes

- Remove the dim pass and its helper.
- The window behind a modal keeps every foreground and background exactly as drawn.
- The modal panel remains the only opaque surface on top.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: opening a modal no longer changes the window behind it.

## Impact

- `src/ui/dialog.rs`: remove the dim call and function.
- Update the two modal colour tests.
