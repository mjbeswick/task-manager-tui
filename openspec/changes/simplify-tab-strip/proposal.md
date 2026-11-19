# Proposal

## Why

The tab strip still draws vertical bars between labels, which read as column dividers.
The underline is a set of short segments with gaps, so the strip has three competing
horizontal shapes instead of one clean rule.

## What Changes

- The vertical separators between tabs are removed; a single space keeps the labels
  apart.
- The row under the tabs is one continuous horizontal line across the whole strip.
- The part of the line under the active tab uses the accent colour; the rest uses the
  border colour.
- Label text, digits, hit areas, and the window title do not change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the tab strip drops its separators and replaces the centred segments with
  one continuous rule whose active span is accented.

## Impact

- `src/ui/chrome.rs`: separator drawing and the rule loop.
- `README.md`: the tab-line wording.
- Render snapshots for every tab.
- Coordination: archive `refine-menu-and-tab-chrome` before this change.
