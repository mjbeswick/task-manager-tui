# Proposal

## Why

The application works, but it looks flat. Rows carry no sense of load. Graphs
step in whole cells, so a smooth CPU curve reads as a staircase. The Performance
rail is a bare list of words. Filters exist on four tabs but hide behind `/`,
and two list tabs have none. Column headers do not respond to clicks.

The Windows 10 Task Manager already solves most of this. Its heat-shaded cells,
header totals, rail sparklines and per-resource colours carry the look. Bringing
them over makes the application beautiful and more faithful at the same time.

Colour is the other gap. Every colour is 24-bit. Terminals without truecolor,
such as the Linux virtual console, older xterms, tmux without `Tc`, and macOS
Terminal.app, show wrong or garish colours today.

## What Changes

- Processes tab shades CPU, Memory, Disk and Network cells by load, in the
  Windows 10 yellow to orange ramp. Text stays readable on every step.
- The Processes header shows the system total above each resource label,
  for example `27%` over `CPU`.
- Clicking a column header sorts by it. Clicking the sorted column reverses it.
- Filled graphs use eighth-block glyphs for a smooth top edge, with a stronger
  edge colour above a lighter fill.
- Each Performance rail entry shows a mini sparkline and its current value.
- A new `Per resource` chart colour gives CPU, Memory, Disk, Network and GPU
  their Windows 10 colours. It becomes the default for new settings.
- A filter field sits at the right of the menu bar on every list tab. It shows
  a hint when empty, takes focus on `/` or a click, and highlights matches.
- App history and Users gain filtering.
- Colour depth is detected at start. Truecolor, 256 and 16 colour terminals
  each get a palette mapped for them. A `--color` flag overrides detection.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: colour depth detection and fallback, the shared filter field,
  and header clicks in mouse support.
- `process-list`: heat-shaded cells, header totals, click to sort, and match
  highlighting in the filter.
- `performance`: smooth filled graphs, rail sparklines, and the per-resource
  chart colour.
- `services`: the filter shows in the shared filter field.
- `app-history`: filter by name.
- `users`: filter by user name.

## Impact

- Code: `src/theme.rs`, `src/ui/chrome.rs`, `src/ui/graph.rs`,
  `src/ui/performance.rs`, `src/ui/processes.rs`, `src/ui/services.rs`,
  `src/ui/app_history.rs`, `src/ui/users.rs`, `src/ui/hits.rs`, `src/app.rs`,
  `src/action.rs`, `src/input.rs`, `src/cli.rs`, `src/store/settings.rs`.
- Tests: every insta golden frame changes. `tests/mouse.rs` gains header and
  filter clicks. Theme contrast tests cover the heat ramp.
- No new dependencies. Ratatui and crossterm already cover everything needed.
- Settings: one new chart colour value. Saved files keep working, and an old
  saved colour is kept as is.
- Docs: README keys, flags and Known gaps. The Known gaps line about mouse
  input is stale and is removed here.
