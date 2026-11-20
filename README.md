# taskmgr

A Windows 10 Task Manager clone for Linux and macOS terminals. Seven tabs, the
Windows 10 and modern terminal themes, grouped process table, and 60 second graphs,
driven from the keyboard or the mouse.

```
 File  Options  View  Help                         / Filter
Proc  Perf  History  Startup  Users  Details  Services
                                                       15%      62%    5.3MB/s
Name                                         Status    CPU ▼ Memory       Disk
────────────────────────────────────────────────────────────────────────────────
▾ Apps (2)
  ▸ Safari (1)                                        6.4%   1.2 GB  15.0 KB/s
  ▸ Ghostty (1)                                       2.1%   380 MB  15.0 KB/s
▾ Background processes (2)
  ▸ node (1)                                         12.5%   512 MB  15.0 KB/s
  ▸ postgres (1)                                      1.1%  96.0 MB  15.0 KB/s
▾ System processes (2)
  ▸ WindowServer (1)                                  4.8%   1.2 GB  15.0 KB/s
  ▸ launchd (1)                                       0.2%  12.0 MB  15.0 KB/s
 Fewer details  1/9                   CPU 15.2%   Memory 62.5%   Disk 5.3 MB/s
```

## Install

```sh
cargo build --release
install -m755 target/release/taskmgr /usr/local/bin/taskmgr
```

Requires Rust 2024 edition (1.85 or newer). Linux needs systemd for the Services
and Startup tabs; macOS uses launchd.

## Run

```sh
taskmgr
```

| Flag | Meaning |
|---|---|
| `--tab <name>` | Open `processes`, `performance`, `app-history`, `startup`, `users`, `details`, or `services` |
| `--theme <name>` | `light`, `dark`, `nord`, `gruvbox`, `phosphor`, `amber`, `tokyo-night`, `catppuccin`, `solarized-light`, or `rose-pine` for this run |
| `--chart-style <name>` | `filled`, `line`, `dots`, or `blocks` for this run |
| `--interval <speed>` | `high` (0.5 s), `normal` (1 s), `low` (2 s), `paused` |
| `--ascii` | ASCII glyphs instead of box drawing and blocks |
| `--color <depth>` | `auto`, `truecolor`, `256`, or `16`. Auto reads `COLORTERM` and `TERM` |
| `--once` | Render one frame, print it, exit. Useful without a terminal |

### Colours look wrong

Auto detection picks truecolor when `COLORTERM` says so, 16 colours on the
Linux console, and 256 colours everywhere else. Over SSH `COLORTERM` is often
dropped, so force it with `--color truecolor` if your terminal supports it.
If graph blocks show as boxes or question marks, your font lacks them. Use
`--ascii`.

## Keys

| Key | Action |
|---|---|
| `q`, `Ctrl+C` | Quit |
| `F10`, `Alt+F`/`Alt+O`/`Alt+V`/`Alt+H` | Menu bar, arrows navigate, Enter activates |
| `F1` | Keyboard help |
| `F2` | Appearance: theme, chart style, chart colour, chart gradient |
| `F5` | Refresh now |
| `1`-`7`, `Tab`, `Shift+Tab` | Switch tabs |
| `Up`/`Down` or `j`/`k`, `PgUp`/`PgDn`, `Ctrl+d`/`Ctrl+u` | Move and scroll |
| `g` / `G`, `Home` / `End` | Top and bottom |
| `Enter` | Open process details, or expand a group |
| `Left`/`Right`, `h`/`l` | Collapse, expand, inspect |
| `<` `>` `r` | Sort column, reverse |
| `/` | Filter the list. `Enter` keeps the filter, `Esc` clears it |
| `Delete`, `e` | End task |
| `E` | End process tree |
| `K` | Send a signal: TERM, KILL, INT, HUP, STOP, CONT, optionally to the tree |
| `f` | Fewer details |
| `t` | Next colour theme |
| `+` `-` | Update speed |
| `p` | Set process priority as a nice value from -20 to 19 |
| `a` | Set affinity, unavailable on both platforms |
| `?` | Keyboard help |
| `Space` | Enable or disable a startup entry, expand a process group |
| `s` `S` `R` | Start, stop, restart a service |

## Filter

Every list tab has a filter field at the right of the menu bar. Press `/` or
click it, type, and press `Enter`. Matches are case-insensitive and the matching
part of each name is highlighted. Each tab keeps its own filter.

## Mouse

Mouse capture is on. Click a tab to switch, a menu label to open it, a menu item to
activate it, a table row to select it, or a resource in the Performance rail to show
it. Click a row twice to expand or inspect it. Dialog buttons respond to clicks.
Click a column header to sort by it, and click it again to reverse. Click the filter
field to type a filter, or its `×` to clear it. Every action still has a key.

Clicking outside an open menu closes it, and that click is consumed, so it cannot
select a row, switch a tab, or start a filter. Clicking the dropdown's border or empty
space leaves the menu open.

The wheel acts on what is under the pointer. Over a table it moves the viewport three
rows without moving the selection; over the Performance rail it moves the selected
resource; over an open menu it moves the highlight. Long lists show a one-column
scrollbar at the right edge. Drag its thumb to move through the list, or click the
track above or below the thumb to page by one screen. Below 60 columns the bar is
dropped and the position counter in the status bar takes over.

## Tabs

- **Processes** - CPU, Memory, Disk, and Network cells shade from pale yellow
  to red by their share of the system total. The Apps heading row carries the
  system totals in the numeric columns, so the header stays one row. Rows group
  under Apps, Background processes, and
  System processes. App rows collapse by default; expanding one lists its processes. While you move
  the selection the row order holds for two seconds, so the list does not re-sort
  under the cursor; values keep updating. End task asks before touching a system
  process or another user's process.
- **Performance** - CPU, Memory, Disk, Network, and GPU panels with a bordered
  60 second graph, stat boxes, and one bordered graph per logical processor.
  A full history always spans the whole plot, whatever the terminal width.
  Percentage graphs keep a fixed 0-100 scale, so a quiet graph looks quiet. Disk
  and network graphs scale to the largest value in the window and label that
  maximum. Filled charts use a gamma-correct colour ramp, line and dot charts take
  the colour of the highest sample in each braille cell, and resampling interpolates
  when the plot is wide and keeps peaks when it is narrow. View > Appearance, or F2,
  switches between Filled, Line, Dots, and Blocks rendering, chart colours Teal, Blue,
  Orange, Magenta, Mono, or Theme, and turns chart gradients on or off. Blocks fills with quadrants, eighth blocks, and
  half-cell triangles to draw a sloped area edge. Per resource takes five distinct colours from the
  active theme, one per resource.
- **App history** - CPU time, peak memory, and disk totals per application,
  accumulated across restarts.
- **Startup** - login agents from launchd or XDG autostart, with their real
  enabled state.
- **Users** - sessions from `who`, with each user's CPU, memory, and disk totals.
- **Details** - every process with PID, status, owner, and priority.
- **Services** - launchd jobs or systemd units, start, stop, and restart.

## Table readability

The theme collection holds Windows Light, Windows Dark, Nord, Gruvbox Dark, Phosphor,
Amber, Tokyo Night, Catppuccin Mocha, Solarized Light, and Rosé Pine. `t`, the Appearance
dialog, and `--theme` cycle through them in that order.

List tables alternate a slightly lighter or darker row background so a wide row is
easier to follow across its columns. Processes escalate cell shading through 1, 3, 8,
20, and 40 percent of the system total, ending in red. Status words carry meaning
beyond their text: Not responding is red, Suspended and Stopped are amber, service
Running is green, and service Stopped is muted. A selected row always uses the
selection colour, so the stripe and the status tone never compete with it. Every tab
names its own keys in a line at the bottom of its content. One continuous line runs
beneath the tabs. The active tab
is drawn as an open box with box characters: a top border, vertical sides, and outward
`┘` `└` corners that join the line, with an empty bottom. The 1-7 keys still jump to a
tab even though the digits are not printed. Its label and box use the accent colour. The
box costs one extra chrome row.

## What each platform reports

Anything a platform does not expose shows `-`. It is never a zero.

| Figure | Linux | macOS |
|---|---|---|
| Per-process CPU, memory, disk I/O | yes | yes |
| Per-process GPU | no | no |
| Per-process network | no | no |
| Processor affinity | no | no |
| Handle count | open file handles from `/proc/sys/fs/file-nr` | no |
| Compressed memory | no | no |
| Link speed | no | no |
| Disk active time | no | no |
| Startup impact | Not measured | Not measured |

Why: neither kernel attributes network or GPU work to a process through an
interface this application uses, and neither offers process affinity portably.
Startup impact needs a login-time measurement that this application does not do,
so it reports the same "Not measured" value Windows 10 can show.

## Where data lives

Both paths come from the platform's standard directories.

| File | Linux | macOS |
|---|---|---|
| Settings | `~/.config/task-manager-tui/settings.toml` | `~/Library/Application Support/task-manager-tui/settings.toml` |
| App history | `~/.local/share/task-manager-tui/history.json` | `~/Library/Application Support/task-manager-tui/history.json` |

History is versioned JSON, written to a temporary file and renamed, so an
interrupted write leaves the previous copy readable. A corrupt file is moved
aside and the application starts with an empty history. Records unseen for 30
days are dropped. Clear it with View > Clear app history.

## Services and startup backends

- **macOS**: `launchctl list` for jobs, `launchctl print-disabled` for startup
  state, `kickstart` to start, `kill SIGTERM` to stop, `kickstart -k` to restart,
  `enable`/`disable` for startup entries. Jobs are never unloaded.
- **Linux**: one `systemctl show --all --type=service` call for the unit list,
  `systemctl --user` for user units, and desktop files in `~/.config/autostart`
  and `/etc/xdg/autostart` with a user override for disabling.

Every command is executed as a program plus an argument vector. Nothing goes
through a shell, so a unit name with spaces or quotes cannot become a second
command. Actions that need root are refused before anything runs, with a message
saying so.

## Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Golden frames live in `tests/snapshots/` with `insta`. After an intentional
visual change, regenerate with `INSTA_UPDATE=always cargo test --test render`,
then read the diff before committing.

Sampling budget, measured on an M1 with 608 processes in release mode: worst
refresh 65 ms, well inside the one second interval. The sampler thread skips a
tick rather than queueing one when a sample overruns.

## Known gaps

- Windows is not a target.
- No per-process network or GPU numbers, and no affinity control.
- Startup impact stays "Not measured".
