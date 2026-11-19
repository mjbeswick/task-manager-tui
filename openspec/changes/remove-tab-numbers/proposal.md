# Proposal

## Why

The tab labels carry digits that duplicate the keyboard shortcut and add visual noise to
every strip. The keys still work whether or not the digits are printed.

## What Changes

- Remove the digit prefix from every tab label, in both the wide and compact strips.
- Keep the 1-7 key bindings, the help text, and the tab hit areas.
- The active tab box shrinks to the label width.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the tab strip shows labels only.

## Impact

- `src/ui/chrome.rs`: tab text and the tests that read it.
- `README.md`: the tab wording.
- Render snapshots.
