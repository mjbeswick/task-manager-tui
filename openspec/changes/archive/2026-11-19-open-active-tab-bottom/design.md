# Design

## Context

- `draw_tabs` draws the active box after the rule: `tl`/`hline`/`tr` on the top row,
  `vline` sides on the label row, and `bl`/`hline`/`br` on the rule row.
- The rule is drawn first across the whole strip, so the box bottom currently replaces it.

## Decisions

### Clear the span, then continue the sides

On the rule row, overwrite `left..=right` with spaces on the window background, then draw
`vline` at `left` and `right` in the accent. The tab therefore has two sides reaching the
content edge and an open middle. Because the border rule was drawn first, the cleared
span is the only gap in it.

*Alternative.* Leave the rule under the tab and drop only the corners. Rejected: the line
would still close the tab visually.
*Alternative.* Keep the bottom border and rely on colour. Rejected: the shape is the
problem, not the colour.

### Nothing else changes

The top border, digits, label widths, gap, hit rect, and compact threshold stay the same.
ASCII mode uses `|` for the sides and the same empty span.

## Risks / Trade-offs

- [Visual ambiguity] With no bottom border, a box could read as a header. The rule
  stopping at both sides is the cue that it opens into the content.
- [Content colour] The open span shows the window background, not the table surface → It
  spans one row only and matches the strip, which is the intent.
