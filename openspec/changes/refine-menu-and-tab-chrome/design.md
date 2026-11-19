# Design

## Context

See `proposal.md` - Why.

- `chrome::draw` lays out four bands: menu, tabs, content, status. `MIN_HEIGHT` is 12.
- Menu and tab rows currently both fill with `theme.chrome`. The active tab fills its
  label with `accent` and uses `on_accent` text.
- `Target::Menu` and `Target::MenuItem` are published while drawing. The dropdown's
  border and empty area publish nothing.
- `handle_mouse` runs the normal click path for every target; a click with no target
  returns without closing anything.

## Goals / Non-Goals

**Goals:**

- One predictable dismissal rule for every click while a menu is open.
- Tabs read as a cheap strip, not a second toolbar.
- No new theme role and no extra dependency.

**Non-Goals:**

- Right-click menus, context menus, or menu keyboard changes.
- Hover highlighting for tabs.
- A second header row for the title or window controls.
- Replacing the Windows 10 flat borders.

## Decisions

### One extra row for the tab rule

The layout becomes menu 1, tabs 1, rule 1, content, status 1. `MIN_HEIGHT` stays 12,
so a minimum terminal keeps seven content rows. The rule row is drawn by the same
chrome pass and is never hit-tested.

*Alternative.* Draw the underline on the tab row itself. Rejected: a terminal cell has
one foreground, so the text and its underline cannot occupy the same cell row.
*Alternative.* Use a Unicode combining low line. Rejected: font coverage is uneven and
the result depends on the terminal's cell composition.

### Menu keeps chrome, tabs return to the window background

`draw_menu_bar` keeps `theme.chrome` and the filter field keeps its `surface` inset.
`draw_tabs` fills with `theme.bg`, drops the filled active block, and draws the active
label in `accent` with bold. The window title on the tab row uses `theme.bg`.

No new role is added: `chrome` already means the menu band after this change, and the
tab strip needs no background at all.

### Centred segments, one per tab

For each tab the strip already knows `x` and `width`. On the rule row it draws a
segment of `width.saturating_sub(2).max(1)` cells centred at `x + width / 2`. The two
cells of padding leave a visible gap between neighbours. Active uses `accent`, inactive
uses `border`. In compact mode the same rule applies to the shortened labels.

If two visible tabs are adjacent with no gap, the segments merge; the compact strip
keeps a one-space separator, so this does not occur in practice.

### Click-away dismissal happens before the normal click path

`handle_mouse` gains a guard: when `app.menu.is_some()`, only `Target::Menu`,
`Target::MenuItem`, and the new `Target::MenuPanel` keep the menu open or activate it.
Every other target closes the menu and returns without dispatching. A click with no
target also closes the menu.

`draw_menu_dropdown` publishes `Target::MenuPanel` over the whole panel before its item
rects, so a click on the border or the empty area is distinguishable from a click
outside. Items are added after, so they still win.

*Alternative.* Close the menu in `App::click` for every non-menu target. Rejected: the
click would still run its action after closing, which is the problem being fixed.
*Alternative.* Close only on clicks with no target. Rejected: clicking a row behind the
menu would then both close the menu and select the row.

## Risks / Trade-offs

- [Content height] One more chrome row costs the table a row → `MIN_HEIGHT` still leaves
  seven rows, and the status bar and counter are unchanged.
- [Segment alignment] A tab shorter than three cells gets a one-cell segment → Still
  reads as an underline and never overlaps its neighbour's gap.
- [Menu at screen edge] The dropdown panel target is clipped to the terminal → It uses
  the same clipped rect as the drawing, so the two cannot disagree.
- [Click on the active menu label] It re-selects that menu rather than toggling it shut →
  Consistent with the keyboard's F10 behaviour and not part of this change.
