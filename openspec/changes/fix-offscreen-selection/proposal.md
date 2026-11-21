# Proposal

## Why

A sample re-sorts the list and the selection follows its row, but the viewport stays put.
On a fresh start the Details tab selected launchd, which then sank off screen. Priority
and End task acted on a row the user could not see.

## What Changes

- If the selection was in view before a sample, it stays in view after it.
- If the user scrolled the selection out of view with the pointer, a sample still leaves
  the viewport alone.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `list-scrolling`: a visible selection stays visible across samples.

## Impact

- `src/app.rs`: `apply_snapshot` and `keep_selection_visible`.
