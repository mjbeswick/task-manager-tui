# Design

## Context

- `draw_hint` writes the whole hint string with one muted style.
- The status bar builds one joined string of `label value` fields and draws it muted.
- The fixed-width field layout must not change.

## Decisions

### Style hint text character by character

Walk the hint string once. While inside `[` and `]`, use `accent` with bold; outside, use
`muted`. The characters and widths are unchanged, so clipping still happens before this
pass.

*Alternative.* Parse groups into two runs and draw each run. Rejected: the brackets and
inner key need separate styling anyway, so the character walk is simpler and handles
truncation naturally.

### Split each status metric into label and value

A metric becomes a small struct: label, value, field width, and whether it is the speed.
Drawing puts `label ` muted and the right-aligned value in `text`, or `accent` for the
speed. The summary width and drop order use the same struct, so the stable layout test
keeps passing.

*Alternative.* Keep one string and recolour ranges afterwards. Rejected: it needs a second
parse of the rendered line.
