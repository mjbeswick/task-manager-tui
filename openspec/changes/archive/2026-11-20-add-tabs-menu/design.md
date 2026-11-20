# Design

## Context

- `MENUS` is a fixed array; `draw_menu_bar` lays out `" label "` cells and records hits.
- `draw_menu_dropdown` recomputes the menu x by adding `label.len() + 2`.
- Six padded labels total 42 cells, but the minimum terminal is 40.

## Decisions

### A shared menu-offset function

`menu_offsets(area)` returns whether compact spacing is active and each menu's x and width.
It measures the padded layout once; if it does not fit, each label is drawn as `"label "`
with no leading space. Both the bar and the dropdown use the same offsets, so clicks and
rendering cannot disagree.

### Tabs at index 3

Order: File, Options, View, Tabs, Process, Help. Alt+T opens it; Process and Help move to
Alt+P and Alt+H. Items map to the existing `Action::GoTab`.

## Risks / Trade-offs

- [Tighter bar] At 40 columns labels sit one space apart → Still clickable and complete;
  the filter hint is dropped at that width, as before.
- [Menu duplication] Tabs exist in both the strip and the menu → The menu is the reliable
  path on narrow screens; the strip stays for quick switching.
