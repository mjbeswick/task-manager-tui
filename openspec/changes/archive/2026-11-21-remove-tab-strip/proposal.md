# Proposal

## Why

The tab strip is the weakest part of the window. It spends two of the terminal's
rows on a raised box that hugs the active label with no padding, punches a nine-cell
gap in the content rule, and survives only above 72 columns. Below that it disappears
into a `Tabs: <active> ▾` button, so most terminal sizes never see the labels at all.
The keyboard already covers switching through `1`-`7`, `Tab`, and `Shift+Tab`, and the
tab list is on `Alt+T`.

## What Changes

- Remove the tab strip: labels, active-tab box, and the rule row under it.
- Remove the `Tabs: <active> ▾` button that replaces the strip when labels do not fit.
- Draw one rule row directly under the menu bar as the top of the content area.
- Keep `1`-`7`, `Tab`, `Shift+Tab`, and the `Alt+T` dropdown, anchored at the left
  edge below the rule.
- Leave the View menu unchanged. Listing the seven tabs there is a later change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the window layout drops the strip and its rule, and the tab list is
  reached by keyboard only until the View menu carries the tabs.

## Impact

- `src/ui/chrome.rs`: window layout, strip drawing, dropdown anchor.
- `src/ui/hits.rs`, `src/action.rs`: tab and tab-button targets removed.
- Unit tests, mouse tests, and every golden frame move up two rows.
- README prose that describes the strip.
