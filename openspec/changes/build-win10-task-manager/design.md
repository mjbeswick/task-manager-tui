# Design

## Context

Greenfield. There is no code, no existing spec, and no prior art in the repository.
See `proposal.md` for motivation and `specs/` for the behavior contract.

Constraints that shape the design:

- Two targets, Linux and macOS. Nothing may assume one of them.
- Rust, terminal only. The visual reference is the Windows 10 Task Manager, which
  is a flat, light, menu-driven application with a tab strip and dense tables.
- Process data must stay honest. A missing metric shows a dash, never zero.
- The application runs as the invoking user and never elevates itself.
- This machine is macOS, so Linux correctness has to come from fixtures and CI,
  not from hand testing.

## Goals / Non-Goals

**Goals**

- One rendering path, one state model, and one sampler shared by both platforms.
- Every tab testable without a real process table: fixtures feed the renderer.
- Sampling off the key path, bounded memory, bounded command use.
- Visual fidelity where the terminal allows it: Windows 10 palette, flat borders,
  tab underline, 60 second area graphs.

**Non-Goals**

- Windows support. Not a target, not tested.
- Per-process network attribution. Neither platform exposes it without root and
  extra tooling; the column shows a dash.
- Processor affinity. Not exposed through a supported interface on either target.
- Measuring startup impact. The column exists and reads "Not measured", which is
  also a value Windows 10 produces.
- Mouse input. A terminal mouse is not part of the Windows 10 fidelity goal.
- Replacing `btop`. This is the Task Manager shape, not a denser monitor.

## Decisions

### Sampling runs on its own thread

A sampler thread owns the `sysinfo::System` and wakes on an interval with a
`std::sync::mpsc` channel to the UI. The UI loop waits on either a key event or a new
`Snapshot` and redraws.

*Alternatives.* A single loop that samples between key reads blocks input for the
duration of a refresh, which on a busy machine is tens of milliseconds and can spike.
An async runtime (`tokio` plus `crossterm`'s event stream) buys nothing here: there are
two event sources and one timer. The channel keeps both cheap and testable.

Sampling happens at 1 s by default, adjustable to 0.5 s, 2 s, or paused. If a sample
overruns its interval, the next tick is skipped rather than queued, so work cannot
accumulate. The interval and the UI frame are separate: the UI redraws on input
immediately and otherwise at 250 ms for the clock-free parts.

### `sysinfo` for metrics, native CLIs for services

`sysinfo` covers processes, CPU, memory, disks, networks, users, and process disk I/O
on both targets. Services and startup entries have no cross-platform crate worth using,
so the application shells out to `systemctl` and `launchctl` and parses their output.

*Alternatives.* Reading `/proc` and calling `proc_pidinfo` directly gives more control
and more code for two platforms, and every field would need its own tests. DBus for
systemd means `zbus`, a runtime, and a policy story; `launchd` has no public API for
this and the private one can break. Text output plus fixtures is the cheaper, more
honest path, and it degrades to a dash when a manager changes its format.

All external commands run through one helper that takes a program and an argument
vector, never a shell string. Service labels with spaces or quotes therefore cannot
become extra arguments.

### One immutable `Snapshot`, one mutable `App`

`Snapshot` is plain owned data: system totals, per-core percentages, per-disk and
per-network rates, a vector of `ProcRow`, session rows, service rows, startup rows, and
a `Capabilities` value. It is cheap to clone into tests and contains no platform handles.

`App` holds view state only: active tab, selection key, scroll offset, sort column and
direction, collapsed keys, filter text, dialog stack, theme, update speed. Tab state
survives tab switches because it lives in `App`, not in a per-frame struct.

Crucially, selection is keyed by `ProcKey { pid, start_time }`, never by row index.
That is what keeps a selection on the same process when rows reorder, and what prevents
a reused PID from inheriting a stale selection. Every move writes the new key at once;
leaving the old key in place makes the next sample restore the previous row.

Sorting live by CPU and rebuilding the list every second still moves rows under the
cursor. So a move also starts a two second hold: samples keep refreshing values, but
the last block order is kept. A block is a kind header or a group with its children,
so reordering can never split a group. After the hold expires the next sample re-sorts.

*Alternatives.* Storing the `sysinfo::System` in the UI and querying it during render
couples rendering to the platform and makes golden tests impossible without a live
machine. A store-and-subscribe model (`ratatui` idioms with `Component`) adds
indirection for seven small tabs.

### Rendering is a pure function of `App` and `Snapshot`

Each tab exposes `fn render(area: Rect, buf: &mut Buffer, app: &App, snap: &Snapshot)`.
`Buffer` is ratatui's cell grid, so a `TestBackend` can render any tab at any size with
a fixture snapshot and compare against an `insta` golden.

*Alternatives.* Rendering through `Frame` in `Terminal::draw` works but makes tests
depend on a terminal; rendering into `Buffer` directly is the same pixels with none of
the setup.

Only visible rows are formatted. With several hundred processes, work is proportional
to terminal height, not process count.

### CPU is normalised to whole-system percent

`sysinfo` reports per-process CPU as a percentage of one core, so eight busy cores read
800%. Windows 10 reports a share of the whole machine, so both process rows and the
system total divide by the logical processor count. This satisfies the spec's "total
equals parts" scenario and keeps the status bar from showing 800%.

Memory uses resident set size, which is the closest available analogue of the Windows
10 working set; it is labelled Memory and not Private working set.

### Missing data is a first-class value

`Capabilities` declares, per platform: per-process network, per-process GPU, affinity,
handle counts, compressed memory, and link speed. Metric cells are `Option<T>` and the
formatter renders `None` as `-`. Tabs consult `Capabilities` for actions, so choosing
"disabled with a reason" happens in one place instead of seven.

*Alternative.* Zero-filling missing metrics is less code and lies about the machine.

### Graphs use half blocks over a grid

A graph is `VecDeque<f32>` of 60 samples. Columns map to the 60 sample slots against
the plot width, so a full history always spans the whole plot instead of stopping at
60 columns. Rows are drawn bottom-up with `▀`, `▄`, and `█`, giving two vertical levels
per cell. The style and colour come from settings, so Filled, Line, and Dots share one
renderer rather than three. A dim grid is written first, then the filled
area over it, so the area reads like the Windows 10 graph rather than a bar chart.
Percentage graphs fix the scale at 100; throughput graphs scale to the window maximum
with the maximum labelled.

*Alternative.* Braille gives four times the resolution but renders as dots, not the
solid area Windows 10 uses, and its cell coverage depends on font.

### Settings and history are plain files

- Settings: TOML at `directories::ProjectDirs`, holding theme, update speed, visible
  columns, and last tab.
- History: versioned JSON in the same directory, written atomically (temp file plus
  rename) every 30 s and on exit. A parse failure moves the file aside and starts
  empty, so a corrupt file never blocks startup.

Both paths resolve through `directories`, giving `~/.config` and `~/.local/share` on
Linux and `~/Library/Application Support` on macOS.

### Platform backends sit behind one interface

```
platform::Capabilities
platform::services::{list, start, stop, restart}
platform::startup::{list, set_enabled}
platform::sessions::list
```

macOS implements these with `launchctl` and plist parsing; Linux with `systemctl` and
desktop-entry parsing. Each parser takes `&str` and returns rows, so fixture text from
either platform is testable on either machine.

Service specifics:

- Linux: one `systemctl show --all --type=service --property=...` call for the list,
  then `--user` units for actions when the unit belongs to the user scope.
- macOS: `launchctl list` for label, PID, and exit status; `print-disabled` for startup
  state; `kickstart` to start, `kill SIGTERM` to stop, `kickstart -k` to restart. Jobs
  are never unloaded, so stopping one does not remove it from the list.
- System-scope actions are refused before running anything when the effective user is
  not root, matching the spec.

### Ending processes verifies identity first

Before signalling, the application checks that the PID still carries the recorded start
time. Termination is `SIGTERM`; force is `SIGKILL`. End tree collects descendants by
walking parent links, then signals leaves first. Failures map `EPERM` to the privilege
dialog and `ESRCH` to the "already gone" dialog.

### Windows 10 visual tokens

Layout: one row of menu, one row of tabs, content, one row of status.

| Token | Light | Dark |
|---|---|---|
| Background | `#F0F0F0` | `#202020` |
| Surface | `#FFFFFF` | `#272727` |
| Text | `#000000` | `#FFFFFF` |
| Muted text | `#605E5C` | `#A0A0A0` |
| Border | `#CCCCCC` | `#3A3A3A` |
| Accent | `#0078D7` | `#0078D7` |
| Selection | `#CCE8FF` | `#0A3D62` |
| Graph fill | `#2BA98E` | `#4CC2A8` |
| Graph grid | `#E0E0E0` | `#3A3A3A` |

Borders are single-line box drawing, no double lines, with the tab strip underlined in
the accent. Groups use `▾` and `▸`. A `--ascii` flag swaps box drawing, arrows, and
blocks for `-`, `+`, `v`, `>`, and `#`, for terminals with unreliable fonts.

### Key map

| Key | Action |
|---|---|
| `q`, `Ctrl+C` | Quit |
| `F10`, `Alt+F/O/V/H` | Menu bar |
| `F1` | Help |
| `F2` | Appearance |
| `F5` | Refresh now |
| `1`-`7`, `Tab`, `Shift+Tab` | Tabs |
| `Up`/`Down`, `j`/`k`, `PgUp`/`PgDn`, `Ctrl+d`/`Ctrl+u`, `Home`/`End`, `g`/`G` | Move and scroll |
| `Enter`, `Left`/`Right`, `h`/`l` | Expand, collapse, inspect |
| `<` / `>` / `r` | Sort column / reverse |
| `/` | Filter, `Esc` clears |
| `Delete`, `e` | End task |
| `E` | End process tree |
| `f` | Fewer details |
| `t` | Theme |
| `+` / `-` | Update speed |
| `Space` | Toggle startup entry, expand a process group |
| `?` | Help |
| `s` / `S` / `R` | Service start / stop / restart |

### The mouse is a shortcut layer over the same actions

Mouse capture stays on for the session. Every frame records where it drew clickable
regions into a `HitMap`, so there is no second layout to keep in sync. A click maps
to the action the keyboard would have run: tabs, menu labels and items, table rows, the
Performance rail, and dialog buttons. A second click on the same row within 400 ms
runs that row's Enter action. The wheel moves three rows. Because drawing owns the
geometry, a click can never disagree with what is on screen.

### Module map

```
src/main.rs            terminal lifecycle, event loop
src/cli.rs             arguments: --interval, --theme, --ascii, --once, --tab
src/app.rs             App state, tab state, dialog stack
src/input.rs           key events to Action
src/action.rs          Action enum and dispatch, including confirmation flow
src/model.rs           Snapshot, ProcRow, ProcKey, Capabilities
src/sample/            sampler thread, rate math, grouping, status mapping
src/store/             settings, history
src/platform/          capabilities and the three backends per OS
src/ui/                chrome, dialog, graph, format, theme, one module per tab
```

`--once` renders one frame, prints it, and exits. It exists so CI can exercise the Linux
binary in a container without a real terminal, and so the whole render path has a smoke
test on any machine.

### Testing

- Unit tests: rate math, counter reset, CPU normalisation, grouping classification,
  status mapping, sort stability, formatting, key decoding, plist and desktop-entry
  parsing, `systemctl` and `launchctl` parsers.
- Golden tests: `insta` snapshots rendered through `TestBackend` at 120x40, 80x24, and
  40x20, for all seven tabs, light and dark, plus dialog overlays.
- Fixtures: recorded `launchctl list`, `print-disabled`, `systemctl show`, `who`, and
  desktop-entry files, checked in under `tests/fixtures/`.
- CI: GitHub Actions matrix on `ubuntu-latest` and `macos-latest` running
  `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and a release build.
  The Linux job also runs `taskmgr --once --tab services` under a TTY-less container.

## Risks / Trade-offs

- **Refresh cost on machines with many processes** → refresh only the process fields
  the tables use, never command lines or environments on every tick; verify a one
  second budget with 800 processes, then raise the interval or drop to visible-only
  fields.
- **Manager output changes between versions** → parser fixtures per variant, unknown
  lines ignored, and a tab that reports "unavailable" rather than a wrong table.
- **Killing the wrong process after PID reuse** → start-time check immediately before
  signalling, and a refresh-free signal path.
- **Golden tests churn on `ratatui` upgrades** → pin the version in `Cargo.toml`,
  review snapshot diffs as visual regressions rather than accepting them blindly.
- **Block glyphs render unevenly in some fonts** → `--ascii` fallback and a documented
  terminal requirement.
- **Linux behaviour unverified on this machine** → fixtures cover parsing, CI covers
  build and tests, and a container smoke test covers startup; anything depending on a
  live systemd is called out as unverified until run there.
- **Process GPU fields exist in `sysinfo` but are driver-dependent** → treat as
  optional, dash when `None`, never as a crash path.
