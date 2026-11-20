# Proposal

## Why

The tab strip ends with a "Task Manager" label that duplicates the terminal window title
and crowds the tabs on narrower terminals. Nothing depends on it.

## What Changes

- Remove the in-app label from the right of the tab strip.
- Keep the terminal window title, which already reads `Task Manager - <tab>`.
- No layout row changes; the strip simply ends with the last tab.

## Capabilities

### New Capabilities

None.

## Impact

- `src/ui/chrome.rs`: delete the label block.
- One tab test and the render snapshots.
- No spec changes: the terminal title requirement stays and the label was never specified.
