# Tasks

## 1. Colour depth

- [x] 1.1 Add a `ColorDepth` value with detection from `COLORTERM` and `TERM`, and a `--color auto|truecolor|256|16` flag in `src/cli.rs`. Verify: unit tests cover `COLORTERM=truecolor`, `TERM=linux`, `TERM=vt100`, `TERM=xterm-kitty` without `COLORTERM` giving 256, and the flag override.
- [x] 1.2 Add the post-draw pass that maps every buffer cell to the active depth, with a cached nearest xterm 256 lookup and a per-theme 16 colour role table. Verify: a test renders a frame at each depth and asserts no `Color::Rgb` remains at 256 or 16.
- [x] 1.3 Extend `text_is_readable_in_both_themes` to check text, muted, and selection contrast after mapping at all three depths. Verify: `cargo test theme` passes.
- [x] 1.4 Document `--color` and the `--ascii` font fallback in the README flags table and a short troubleshooting line. Verify: README lists both.

## 2. Smooth graphs and per resource colour

- [x] 2.1 Add the eighths glyph array to `Glyphs` with an ASCII twin, and cover it in `ascii_glyphs_are_all_ascii`. Verify: `cargo test theme` passes.
- [x] 2.2 Draw Filled graphs with an eighth-resolution top cell in the edge colour and a mixed fill colour below, keeping half blocks at 16 colours. Verify: graph tests assert a three eighths sample draws `▃` in truecolor and a half block at 16 colours, and existing graph tests pass.
- [x] 2.3 Add `ChartColor::PerResource` with the five Windows 10 colours and dark variants, list it first in the Appearance dialog, and make it the default for a missing setting. Verify: settings tests assert the new default, an old `teal` file still loads as Teal, and the chart visibility test covers every resource colour in both themes.

## 3. Performance rail

- [x] 3.1 Draw each rail entry as name, one row sparkline, and value line, using the series colour. Verify: a render test at 120x36 shows a sparkline and `GHz` under CPU in the rail.
- [x] 3.2 Fall back to one row per entry with name and value when the rail is shorter than fifteen rows. Verify: a render test at 80x14 shows every resource name with its value and no sparkline.
- [x] 3.3 Keep rail click targets on the whole entry height. Verify: a `tests/mouse.rs` case clicks a sparkline row and selects that resource.

## 4. Processes table

- [x] 4.1 Add the light and dark heat ramps to `Theme` and a step function using the 1, 5, 15, 30 and 50 percent thresholds, with two steps at 16 colours. Verify: unit tests cover each threshold edge and the contrast test checks text on every step.
- [x] 4.2 Shade CPU, Memory, Disk, and Network cells by share of system total, leaving dash cells, group rows, and the selected row unshaded. Verify: render tests check cell backgrounds for a busy row, an idle row, a dash cell, and a selected row.
- [x] 4.3 Add the totals row above the resource labels and drop it below twelve content rows. Verify: render tests at 120x36 show `27%` above `CPU` and at 80x14 show a one row header.
- [x] 4.4 Make page size and scrolling read the table body height. Verify: a test pages down at both header heights and the selection stays visible.

## 5. Click to sort

- [ ] 5.1 Record a header hit target per column on every sortable table and route clicks to the existing sort actions, flipping on the sorted column. Verify: `tests/mouse.rs` cases click Memory then Memory again on Processes and Details and assert order and marker.
- [ ] 5.2 Add a `tests/mouse.rs` case that clicks the first process row under the two row header. Verify: the clicked row is selected.

## 6. Filter field

- [ ] 6.1 Draw the filter field at the right of the menu bar on list tabs, with the hint, the clear mark, the shrunk narrow form, and the title kept on Performance. Verify: render tests at 120 and 40 columns, and a check that menus are never truncated.
- [ ] 6.2 Add filter state and row filtering to App history and Users. Verify: unit tests filter each tab, assert the empty state, and assert stored history is unchanged.
- [ ] 6.3 Highlight the first case-insensitive match in each visible name cell on every list tab. Verify: a render test asserts the accent style on the matched characters only.
- [ ] 6.4 Route clicks on the field to focus and on the clear mark to clear, and make Enter leave the field with the filter kept. Verify: `tests/mouse.rs` and input tests cover click focus, clear, Enter, and Esc.
- [ ] 6.5 Update the README keys and mouse sections for the field and header clicks, and remove the stale mouse line from Known gaps. Verify: README matches the behaviour.

## 7. Golden frames and checks

- [ ] 7.1 Regenerate insta snapshots with `INSTA_UPDATE=always cargo test --test render` and review every changed frame tab by tab. Verify: the diff only shows the intended visual changes.
- [ ] 7.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`. Verify: all three pass.
- [ ] 7.3 Run the release binary in a truecolor terminal, in macOS Terminal.app, and with `TERM=linux --color 16`, both themes. Verify: all three look correct with no broken glyphs.
