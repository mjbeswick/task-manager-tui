# Design

## Layout

The window loses two rows and gains one rule:

```
 row 0  menu bar + filter field
 row 1  unbroken horizontal rule
 row 2+ active tab's content
 last   status bar
```

The rule stays because the menu bar needs a bottom edge and the content panels
should not float against the chrome. It is drawn across the full width in the
border colour, with no gaps.

## Tab switching

- `1`-`7`, `Tab`, and `Shift+Tab` are unchanged.
- `Alt+T` opens the existing dropdown, anchored at the left edge one row under
  the rule. The panel lists all seven tabs, marks the active one, and shows each
  digit.
- No hit target remains for a tab label or the old `Tabs:` button, so the mouse
  can no longer switch tabs. The keyboard is the only path until the View menu
  carries the list.

## Constraints

- The View menu is untouched; listing tabs there is a separate change.
- `MIN_WIDTH` and `MIN_HEIGHT` stay at 40x12.
- Content layout inside each tab is unchanged; the two freed rows go to the tab's
  own content area.

## Left to the builder

- Whether to delete or keep the now-unused glyph helpers in `chrome.rs`.
- How much of `draw_tab_menu` to rewrite; only the anchor needs to move.
