# Design

## Context

- `draw_header` draws a totals row above the labels when `area.height >= 12`, and a
  separate two-row header otherwise.
- `totals(app)` already formats CPU, memory, disk, network, and GPU for the current
  layout, with compact units for the narrow columns.
- Group headings render with empty numeric cells, so the row has space for the figures.
- `draw_item` knows the item and its global index.

## Decisions

### One header row at every height

`header_rows` becomes a constant 2: the label row and the rule. The height threshold and
the `totals` flag disappear. The table gains one row on tall terminals and keeps the same
row count on short ones.

*Alternative.* Keep the tall-header branch and only move the totals. Rejected: the extra
row exists only for the totals, so leaving it would keep the wasted space.

### Draw the totals from the Apps heading row

`draw_item` calls `draw_totals` when the item is the Apps kind heading and its index is
zero. Index zero means the heading is the first visible row, which covers both the
scrolled case and the filtered case without extra checks.

The figures are right-aligned at `x + width - 2` in each numeric column, using the row's
own base background so a striped or selected row stays uniform, and bold text.

*Alternative.* A separate overlay pass after the rows. Rejected: it would need to know the
Apps row's y and background again.
*Alternative.* Keep the totals on every kind heading. Rejected: the totals describe the
whole table, not each group.

### Nothing else moves

Column widths, sort markers, shading, scrollbar geometry, and the hint line do not change.
The row count gain is the only layout effect.

## Risks / Trade-offs

- [Totals scroll away] Scrolling past Apps hides the totals → They describe the list and
  return with the heading; the alternative is stamping them on a process row.
- [Contrast] Bold figures sit on the Apps row background, which may be a stripe → The
  existing contrast tests cover text on surface and surface_alt.
