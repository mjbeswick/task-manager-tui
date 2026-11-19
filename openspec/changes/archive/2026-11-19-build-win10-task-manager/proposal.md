# Proposal

## Why

There is no task manager on Linux or macOS that looks and reads like the Windows 10
Task Manager. Existing monitors such as `btop` and `htop` optimise for density and
keyboard speed, but not for the familiar seven-tab shape, grouped process table, or
the labelled performance graphs. Recreating that shell gives a terminal user a
known interface on two platforms that never had it, and it makes the Windows 10
information architecture available where the underlying data actually exists.

Nothing like this exists today, and the data now exists: `sysinfo` reports
per-process CPU, memory, and disk I/O on both Linux and macOS.

## What Changes

- New Rust binary `task-manager-tui`, a terminal application for Linux and macOS.
- Seven tabs in the Windows 10 order and shape: Processes, Performance, App history,
  Startup, Users, Details, Services.
- Windows 10 chrome: menu bar, tab strip, grouped process table, performance graphs
  with a 60 second history, bottom status bar, and the "Fewer details" view.
- Windows 10 light theme by default, with the Windows 10 dark theme available.
- Real system data on both platforms: processes, per-core CPU, memory, disk, network,
  sessions, services, and startup entries.
- Keyboard-driven equivalent of every Windows 10 click, plus a menu bar reachable
  with F10.
- Honest gaps: data a platform does not expose shows `-` rather than a fabricated
  number. Per-process network, GPU when no driver reports it, and processor affinity
  fall into this group.
- Persisted app history so CPU time survives restarts, plus user settings.

## Capabilities

### New Capabilities

- `tui-shell`: window chrome, menu bar, tab strip, dialog model, theme, colour, and
  keyboard navigation shared by every tab.
- `system-metrics`: the sampling layer, rate arithmetic, CPU normalisation, per-core
  and whole-system series, update speed, and process status mapping.
- `process-list`: the Processes tab, including grouping by app, background, and
  system process, columns, sorting, expansion, and end task.
- `performance`: the Performance tab panels for CPU, memory, disk, network, and GPU,
  with graphs, stat boxes, and the 60 second history.
- `app-history`: the App history tab and the on-disk usage record it reads.
- `startup`: the Startup tab, its entries, and enabling or disabling them.
- `users`: the Users tab and the sessions it reports.
- `details`: the Details tab, its flat process table, per-process actions, and
  priority changes.
- `services`: the Services tab, its units or agents, and start, stop, and restart.
- `platform-support`: what each platform provides, how privilege failures surface,
  and the build and test requirements for both targets.

### Modified Capabilities

None. This is a new project with no existing specs.

## Impact

- New repository at `~/Projects/task-manager-tui`, Rust 2024 edition.
- Dependencies: `ratatui`, `crossterm`, `sysinfo`, `serde`, `serde_json`, `toml`,
  `directories`, `libc`, `clap`, and `insta` for snapshot tests.
- External commands: `launchctl` on macOS, `systemctl` on Linux, `who` on both.
- Platform backends for services and startup entries, one per target.
- No existing code, API, or user is affected. The application does not run on Windows.
