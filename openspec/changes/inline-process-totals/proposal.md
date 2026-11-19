# Proposal

## Why

The Processes header spends a whole row on system totals above the column labels, which
reads as a second header and costs a row of table space. The totals belong beside the
first group, where they describe the list below them.

## What Changes

- Remove the separate totals row; the column header becomes a single row at every height.
- Draw the system totals in the numeric columns of the Apps heading row, right-aligned
  under the same columns they summarise.
- Omit the totals when the Apps heading is not visible, so they never appear on a process
  row.
- Keep the per-column unit formatting and the bold style.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `process-list`: adds a requirement that system totals share the Apps heading row.

## Impact

- `src/ui/processes.rs`: header height, totals drawing, row rendering.
- `tests/snapshots`: Processes frames at every width.
- The table gains one row on tall terminals.
