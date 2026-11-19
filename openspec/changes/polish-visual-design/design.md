# Design

## Context

See proposal.md for the why. The current state that shapes the approach:

- `Theme` holds every colour as `Color::Rgb`. Nothing checks colour depth.
- `graph::draw` paints Filled charts with full and upper half blocks only.
- `Glyphs` has a Unicode set and an ASCII twin, guarded by
  `ascii_glyphs_are_all_ascii`.
- `HitMap` records click targets while drawing. Headers record nothing.
- Filter text already lives per tab in `App`, with `filter_editing` for input.
  Processes, Details, Services and Startup use it. App history and Users do not.
- The menu bar draws the literal title `Task Manager` at the right.
- Chart colour is one setting, default `teal`, applied to every chart.

## Goals / Non-Goals

**Goals:**

- Closer Windows 10 fidelity is the route to a better look. Every visual change
  here has a Windows 10 counterpart.
- One colour mapping point, so themes stay written once in 24-bit.
- No new crates.

**Non-Goals:**

- Rounded corners, gradients or other styling with no Windows 10 counterpart.
- Braille graphs. Fonts on Linux consoles often lack them.
- Heat shading on Details, App history or Users. Windows 10 shades only the
  Processes tab.
- Theme editing or user palettes.

## Decisions

### Colour depth is mapped once, after drawing

Themes stay 24-bit. After each frame is drawn into the ratatui buffer, one pass
maps every cell's foreground and background to the active depth before flush.

- Truecolor: no change.
- 256: nearest xterm 256 entry by squared distance in RGB. Cache by input colour.
- 16: a fixed table per theme that maps each theme field to a named ANSI colour.
  Nearest-match collapses the greys, so selection and background would merge.
  The table keeps roles apart instead.

Detection order: `--color` flag, then `COLORTERM` of `truecolor` or `24bit`,
then `TERM` of `linux` or starting `vt` for 16, else 256. SSH usually drops
`COLORTERM`, so a kitty or alacritty session on a remote Linux box still gets
256 rather than 16. macOS Terminal.app sets no `COLORTERM` and lands on 256,
which is correct.

*Alternative.* Three hand-written palettes per theme. Rejected: six palettes
drift apart, and every new colour needs six entries. The pass is a single
loop over at most a few thousand cells per frame.

### Heat ramp

Five steps by share of the system total. Thresholds: 1, 5, 15, 30 and 50
percent. CPU uses share of total CPU. Memory uses share of physical memory.
Disk and Network use share of the current system rate, which is the figure the
header total shows.

Light ramp starts from the Windows 10 pale yellow and ends at its orange. The
starting values are `#FFF4C4`, `#FFE9A8`, `#FDD87F`, `#F9C25B`, `#F5A43F`. Dark
has no Windows 10 reference, so it uses a muted amber ramp from `#3A3322` to
`#7A4E14` with light text. Final values come from a reference screenshot during
implementation. The contrast test is the gate, not these numbers.

At 16 colours the ramp collapses to two steps: shares of 15 percent or more get
yellow, the rest stay plain.

*Alternative.* Shade by the row's absolute value. Rejected: Disk and Network have
no fixed ceiling, so a share is the only scale that means the same thing on
every column.

### Smooth Filled graphs

`Glyphs` gains an eighths array, `▁▂▃▄▅▆▇█`. The ASCII twin repeats the current
`_`, `"` and `#` steps. Each column's top cell uses the eighth matching its
remainder, drawn in an edge colour. Cells below use a fill colour mixed 55
percent toward the chart background. 16 colour mode keeps half blocks in graphs
and sparklines, because Linux console fonts often lack the eighths.

*Alternative.* Braille dots for four times the resolution. Rejected: weaker
fill, uneven font support, and Windows 10 draws an area, not a dot cloud.

### Per resource colour

`Per resource` is a new chart colour value. The graph colour resolves per series:
CPU `#117DBB`, Memory `#8B12AE`, Disk `#4DA60C`, Network `#A74F01`, GPU
`#2BA98E`. Dark theme uses lighter variants that pass the existing chart
visibility check. The F2 dialog lists it first.

This resolves the clash with the existing F2 setting. Per resource is one more
choice, not a replacement. A single colour choice still forces every chart to
that colour.

The default changes from `teal` to `per-resource` for missing settings only. A
settings file that already says `teal` keeps teal.

### Rail sparklines

The rail reuses `graph::draw` at one row high with the eighths glyphs. Each entry
takes three rows: name, sparkline, value. Below fifteen rows of rail, entries
fall back to one row with name and value. The rail width stays as it is.

### Header totals

The Processes header becomes two rows: totals above labels. The table body
starts one row lower. Below twelve content rows the totals row is dropped.

### Filter field in the menu bar

The field replaces the `Task Manager` title on list tabs. Performance keeps the
title. It is drawn as `/ Filter` when empty and `/ text ×` when set. The `/` is
both the icon and the key hint. A `⌕` icon was dropped because Linux console
fonts often lack it. The `×` is a click target that clears. The ASCII twin uses
`x` and a `|` cursor.

The existing per-tab filter state and `filter_editing` flag are reused. App
history and Users gain a filter string in their tab state, and their row lists
apply it. Match highlighting is a span split on the first case-insensitive match
in the name cell.

*Alternative.* A filter row above each table. Rejected: costs a row on every tab,
and Windows 11 places search in the title bar, which is the same spot.

### Click to sort

Each table header records one `Target::Header(column)` per column while drawing.
A click calls the same sort action as `<` and `>`, and flips when the column is
already sorted.

## Risks / Trade-offs

- [Two-row header shifts every row by one] → Row hit targets come from drawing,
  so they move with it. `tests/mouse.rs` gets a case that clicks the first row
  under the new header. Page size reads the table body height, not a constant.
- [Every golden frame changes] → Regenerate in one pass and review the diff
  tab by tab before committing.
- [Heat ramp hurts readability on some monitors] → Contrast test on every step
  in both themes, plus the 16 colour fallback.
- [Wrong colour depth detected] → `--color` overrides it, and the README names
  the flag in a troubleshooting line.
- [Eighth blocks missing from a user's font] → `--ascii` already exists. The
  README mentions it next to `--color`.
- [Filter field crowds the menu bar at 40 columns] → It shrinks to `/` first.
  Menus are never truncated.

## Migration Plan

No data migration. Settings gain one accepted chart colour value. An unknown value
already falls back to the default, so an older build reading a newer file shows
the default colour.
