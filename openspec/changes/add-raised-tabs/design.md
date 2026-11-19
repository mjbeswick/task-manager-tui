# Design

## Context

See `proposal.md` - Why.

- The layout is menu, labels, rule, content, status: four chrome rows.
- Tabs start at the content's left edge, are separated by one empty cell, and are
  numbered.
- The rule row is a continuous `hline` with an accent span under the active label.
- `Glyphs` already has `tl`, `tr`, `bl`, `br`, `vline`, and `hline`, with ASCII twins.

## Decisions

### One extra row for the top border

The layout becomes menu, tab top, labels, rule, content, status. `MIN_HEIGHT` stays 12,
leaving seven content rows at the minimum and six once the hint row is reserved.

*Alternative.* Draw the top border on the label row with overline characters. Rejected:
  terminal cell height is fixed and the overline style is poorly supported.
*Alternative.* Leave the tab open at the top. Rejected: it reads as an underline, not a
  tab, which is what this change fixes.

### Start tabs one column in

The strip starts at `area.x + 1` so the first tab has a cell to its left for the box's
left side. Every tab keeps its current label text and one-cell gap after it. The active
tab's box therefore occupies three extra cells outside the text: one on the left and the
two existing gaps on either side are reused rather than added.

The label's trailing space is overwritten by the right `│`; digits and letters never move
inside the strip relative to each other.

*Alternative.* Widen every tab by two cells. Rejected: it costs width on a narrow
terminal and shifts every digit.

### Box parts

For the active cell at `(start, width)`:

- Top row: `┌` at `start - 1`, `┐` at `start + width - 1`, `hline` between, all accent.
- Label row: `vline` at `start - 1` and `start + width - 1`, accent; the label keeps its
  accent text and window background.
- Rule row: `└` at `start - 1`, `┘` at `start + width - 1`, `hline` between, accent. The
  border-coloured rule was drawn first, so the box bottom replaces its span and the line
  leaves the `┘` without a gap.

ASCII mode uses the same glyphs from the ASCII table.

### Hit area covers the sides

The active tab's hit rect grows to `start - 1 .. start + width` so a click on either
side selects the tab. Inactive tabs keep their label rect.

## Risks / Trade-offs

- [Height] One more chrome row → `MIN_HEIGHT` still leaves six usable rows; the hint line
  is dropped first if the content is very short.
- [Narrow terminals] The active tab can fall off the strip → The spec says no box then;
  the rule stays continuous.
- [ASCII] The box is made of plus signs → Still clearly a box, and no Unicode leaks into
  ASCII mode.
