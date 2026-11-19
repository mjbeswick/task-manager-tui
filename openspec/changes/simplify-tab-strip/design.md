# Design

## Context

- `draw_tabs` writes each label, then a separator: `│ ` in the wide layout and a space
  in the compact one.
- The rule row draws one segment per tab, centred on the label with one cell of padding
  on each side.
- `hits` records the label rects, which this change does not move.

## Decisions

### A single space replaces the separator

After each tab, advance one column and leave it on the window background. The label text
already ends with a trailing space, so the visual gap is two cells; no glyph is drawn.
The last tab does not need a trailing gap, but the extra cell is harmless and keeps the
loop simple.

*Alternative.* Keep a one-cell vertical bar but colour it like the background. Rejected:
  a glyph that must never be seen is a trap for a later theme change.

### One continuous line with an accented span

The rule row fills the full strip width with the border-coloured `hline`. The active tab's
cell, recorded while drawing labels, is then repainted with the accent `hline` across its
label width. The line therefore has no gaps, and the accent begins and ends exactly at
the active label.

*Alternative.* Keep centred segments and fill the gaps with border. Rejected: it needs
  two loops over the same geometry and produces the same pixels as one continuous line.

### Nothing else moves

Digits, labels, the window title, the compact threshold, and every hit rectangle stay as
they are. The change is confined to the separator branch and the rule loop.

## Risks / Trade-offs

- [Active tab off screen] At very narrow widths a tab can be dropped; then no accent span
  is drawn → The rule stays continuous and the labels that fit are still readable.
- [Gap width] Removing the bar widens the useful strip by one cell per tab → Labels fit
  slightly earlier; no layout constant changes.
