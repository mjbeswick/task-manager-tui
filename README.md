# taskmgr

A Windows 10 Task Manager clone for Linux and macOS terminals. Seven tabs, the
Windows 10 light and dark themes, grouped process table, and 60 second graphs,
driven entirely from the keyboard.

```
 File  Options  View  Help                                         Task Manager
Processes │ Performance │ App history │ Startup │ Users │ Details │ Services
Name                                                    Status   CPU ▼ Memory
────────────────────────────────────────────────────────────────────────────
▾ Apps (2)
  ▸ Safari (1)                                          Running   6.4%  1.2 GB
  ▸ Ghostty (1)                                         Running   2.1%  380 MB
▾ Background processes (2)
  ▸ node (1)                                            Running  12.5%  512 MB
▾ System processes (2)
  ▸ WindowServer (1)                                    Running   4.8%  1.2 GB
 Fewer details                CPU 27.0%  Memory 62.5%  Disk 1.2 MB/s  Normal
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
| `--theme <name>` | `light` or `dark` for this run |
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
| `F2` | Appearance: theme, chart style, chart colour |
| `F5` | Refresh now |
| `1`-`7`, `Tab`, `Shift+Tab` | Switch tabs |
| `Up`/`Down` or `j`/`k`, `PgUp`/`PgDn`, `Ctrl+d`/`Ctrl+u` | Move and scroll |
| `g` / `G`, `Home` / `End` | Top and bottom |
| `Enter`, `Left`/`Right`, `h`/`l` | Expand, collapse, inspect |
| `<` `>` `r` | Sort column, reverse |
| `/` | Filter, `Esc` clears |
| `Delete`, `e` | End task |
| `E` | End process tree |
| `f` | Fewer details |
| `t` | Light or dark theme |
| `+` `-` | Update speed |
| `p` | Set process priority |
| `a` | Set affinity, unavailable on both platforms |
| `?` | Keyboard help |
| `Space` | Enable or disable a startup entry, expand a process group |
| `s` `S` `R` | Start, stop, restart a service |

## Mouse

Mouse capture is on. Click a tab to switch, a menu label to open it, a menu item to
activate it, a table row to select it, or a resource in the Performance rail to show
it. Click a row twice to expand or inspect it. The wheel moves the selection three
rows. Dialog buttons respond to clicks. Every action still has a key.

## Tabs

- **Processes** - groups under Apps, Background processes, and System processes.
  App rows collapse by default; expanding one lists its processes. While you move
  the selection the row order holds for two seconds, so the list does not re-sort
  under the cursor; values keep updating. End task asks before touching a system
  process or another user's process.
- **Performance** - CPU, Memory, Disk, Network, and GPU panels with a bordered
  60 second graph, stat boxes, and one bordered graph per logical processor.
  A full history always spans the whole plot, whatever the terminal width.
  Percentage graphs keep a fixed 0-100 scale, so a quiet graph looks quiet. Disk
  and network graphs scale to the largest value in the window and label that
  maximum. View > Appearance, or F2, switches between Filled, Line, and Dots
  rendering and five chart colours: Teal, Blue, Orange, Magenta, or Mono.
- **App history** - CPU time, peak memory, and disk totals per application,
  accumulated across restarts.
- **Startup** - login agents from launchd or XDG autostart, with their real
  enabled state.
- **Users** - sessions from `who`, with each user's CPU, memory, and disk totals.
- **Details** - every process with PID, status, owner, and priority.
- **Services** - launchd jobs or systemd units, start, stop, and restart.

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
- Mouse input is not supported.
- Startup impact stays "Not measured".
