# Design

## Context

- `MENUS` has six entries; the Tabs entry is drawn in the bar and duplicates the strip.
- `draw_tabs` shortens labels to four characters below 90 columns and then drops the
  labels that do not fit.
- The menu dropdown renderer, navigation, and activation are all table-driven.

## Decisions

### Five bar menus, one hidden tab list

Remove Tabs from `MENUS` and from `MenuItem`. The tab list becomes its own small state
`TabMenuState { item }`, rendered under the tab row, so the bar keeps the five menus the
window uses. `Alt+T` opens it; `Tab`, `Shift+Tab`, and `1`-`7` are unchanged.

### Measure full labels only

Drop the four-character abbreviation. Measure the full labels plus their separators; if
they all fit, draw the strip exactly as today. They fit at 72 columns and above. Below
that, the strip becomes one button. This avoids the abbreviated `Perf`/`Hist` row and
makes the threshold follow the labels instead of a hardcoded 90.

### The button is the active tab

`Tabs: <active> ▾` is drawn in the same raised box as an active tab, so the row keeps one
control and one visual language. Its hit area opens the tab list. The list marks the
active tab and shows the digit keys on the right, using the same panel and selection
styles as a menu dropdown.

*Alternative.* Keep the Tabs menu and hide it when the strip fits. Rejected: the bar would
change shape with width and the duplication would remain exactly when space is tightest.

## Risks / Trade-offs

- [Short terminals] At 40 columns the button takes about 18 cells, leaving room for the
  filter hint → The filter already shrinks at that width.
- [Mouse habits] Clicking a tab label no longer works in button mode → The button opens a
  list with every tab, and keys still switch directly.
- [Alt+T] No longer opens a bar menu → It is documented in the help list and opens the
  same list from the button.
