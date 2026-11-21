# Design

`draw_stat_row` puts every box on one row from 80 columns and half of them per row below
that. So there are never more than two rows, and `stat_height` stays correct at two or
four lines. The old branch that put two boxes per row below 40 columns made three rows
and is removed. At 38 columns, three boxes per row still leave 11 characters each.

Each box row is two lines tall. A row that does not fit is left out, so a label never
appears without its value.

`draw_cpu` uses `stat_height` in both branches instead of a fixed three. The Memory
footnote gets its own layout row, shown only when the panel has room for it.

Rejected: shrinking labels to one line. The label and value pair is the Windows 10 look.
