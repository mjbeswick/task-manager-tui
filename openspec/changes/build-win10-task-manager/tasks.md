# Tasks

## 1. Skeleton and window chrome

- [x] 1.1 Create the Cargo project `task-manager-tui` with binary `taskmgr`, edition 2024, and the dependencies from design.md. Verify: `cargo build` succeeds and `taskmgr --version` prints the crate version.
- [x] 1.2 Add the CLI flags `--interval`, `--theme`, `--ascii`, `--tab`, and `--once`. Verify: `taskmgr --help` lists all five, and an unknown `--tab` value exits non-zero with a message naming the valid tabs.
- [x] 1.3 Implement terminal lifecycle: raw mode, alternate screen, hidden cursor, a panic hook, and restoration on every exit path. Verify: a test drives a failure path and asserts restore is called once, and an interactive run leaves the shell usable.
- [x] 1.4 Build `App`, the tab enum, and the event loop that waits on key events and sample messages. Verify: unit tests cover quit on `q` and `Ctrl+C`, and tab keys move the active tab.
- [x] 1.5 Implement the theme module with the light and dark palettes and the ASCII glyph set. Verify: unit tests assert a contrast ratio of at least 4.5 between text and its background in both themes, and that ASCII mode replaces every non-ASCII glyph.
- [x] 1.6 Draw the menu bar, seven-tab strip, status bar, and minimum-size message. Verify: `insta` golden buffers at 120x40, 80x24, 40x20, and the below-minimum size show the expected chrome.
- [x] 1.7 Implement the dialog model for confirmation, error, run, and help, capturing all keys while open. Verify: goldens for each dialog and a test that Esc closes without firing the action.
- [x] 1.8 Write the README with build instructions, key map, and terminal requirements. Verify: every command in the README runs as written.

## 2. Metrics and sampling

- [x] 2.1 Define `Snapshot`, `ProcRow`, `ProcKey`, `Capabilities`, and the metric `Option` types. Verify: unit tests cover `ProcKey` equality across PID reuse and merge of consecutive samples.
- [x] 2.2 Implement the sampler thread, the channel to the UI, and skip-if-overrun behaviour. Verify: a test with a controllable clock shows one sample per interval, no backlog when a sample overruns, and immediate refresh on resume.
- [x] 2.3 Implement throughput from counter deltas, including the first-sample and counter-reset cases. Verify: unit tests assert zero on the first sample, zero on a decreased counter, and the exact rate for a known delta and elapsed time.
- [x] 2.4 Normalise CPU to whole-system percent. Verify: unit tests cover one busy core on eight (12.5%), all cores busy (100%), and process sum within tolerance of the total.
- [x] 2.5 Map platform process states to Running, Suspended, Not responding, and Stopped. Verify: a table-driven test covers every `sysinfo` status variant including zombie to Stopped.
- [x] 2.6 Declare capabilities per platform for per-process network, GPU, affinity, handle counts, compressed memory, and link speed. Verify: tests assert the Linux and macOS declarations and that every `None` metric formats as `-`.
- [x] 2.7 Record a refresh budget. Verify: a test snapshots a synthetic table of 800 processes and a timed release run reports one sample under one second on this machine, with the measured figure written into the README.

## 3. Processes tab

- [x] 3.1 Implement classification into Apps, Background processes, and System processes, with per-name aggregation. Verify: fixture tests cover a macOS bundle path, a Linux desktop-entry match, root-owned processes, and processes sharing a name.
- [x] 3.2 Render the grouped table with all seven columns and the Fewer details view. Verify: goldens for both views at three widths show right-aligned numeric columns and a stable Name column.
- [x] 3.3 Implement sorting with a direction marker and selection keyed by `ProcKey`. Verify: tests cover default CPU-descending order, equal-value stability across samples, and selection following a process that reorders.
- [x] 3.4 Implement group and row expansion with remembered state. Verify: goldens for expanded, collapsed group, expanded row, and collapse-all states.
- [x] 3.5 Implement the name filter with empty state and restore. Verify: tests cover matching, no match, clear, and that selection returns when the filtered process still exists.
- [x] 3.6 Implement end task: confirmation for system and other-owned processes, start-time identity check, `SIGTERM`, force `SIGKILL`, and dialogs for permission refused and already gone. Verify: tests spawn a child process and end it, and an injected error path asserts both dialogs.
- [x] 3.7 Wire process totals into the status bar. Verify: a golden at 80x24 shows CPU, memory, disk, network, and update speed without shifting when one metric becomes a dash.

## 4. Performance tab

- [x] 4.1 Build the graph widget: half-block fill, grid, fixed and adaptive scales, value label, and unwritten left side for short history. Verify: unit tests compare cell output for known series, and goldens cover both scales.
- [x] 4.2 Implement the rail and panel layout with the narrow-terminal reflow. Verify: goldens at 120x40, 80x24, and 40x20 keep the graph at least twenty columns wide.
- [x] 4.3 Implement the CPU panel with its six stat boxes and the logical processor grid. Verify: goldens for four-core and one-core fixtures show the grid and its hidden case.
- [x] 4.4 Implement the memory panel with in use, available, total, and dashes for unreported compressed and committed. Verify: goldens for a fully reported fixture and one with unknown figures.
- [x] 4.5 Implement the disk panel with busiest-disk headline, per-disk graphs, and capacity. Verify: goldens for one and three disks, including an idle disk.
- [x] 4.6 Implement the network panel with receive, send, adapter name, and dashed link speed. Verify: goldens for traffic and for no adapter.
- [x] 4.7 Implement the GPU panel and its unavailable message. Verify: a golden shows "Not available on this system" and no graph when no adapter reports.
- [x] 4.8 Document the graph scale behaviour and the meaning of each stat box in the README. Verify: the documented labels match the goldens.

## 5. Details tab

- [x] 5.1 Render the flat table with the nine Windows 10 columns. Verify: a golden at 120x40 shows every column header in order and a dash for network.
- [x] 5.2 Implement sorting by every column and selection by `ProcKey`. Verify: tests cover PID numeric order, user-name order, direction reversal, and selection following a reordered row.
- [x] 5.3 Implement end process and end process tree with descendant collection. Verify: a test spawns a parent with two children and asserts all three are signalled, with the count shown in the dialog.
- [x] 5.4 Implement priority levels mapped to `nice` values through `setpriority`. Verify: a test changes a child process to High and back to Normal, reads the value from the system, and asserts the permission dialog when the change is refused.
- [x] 5.5 Add the affinity action as disabled with its reason. Verify: a golden shows it disabled and a test asserts activation changes nothing.
- [x] 5.6 Implement the process details panel with command line, path, parent, start time, threads, and owner. Verify: goldens for a full row, a restricted row with dashes, and the panel closing when the process exits.
- [x] 5.7 Implement the filter for this tab. Verify: tests cover match, no match, and clear with selection restore.

## 6. Users tab

- [x] 6.1 Parse `who` output into sessions. Verify: fixture tests cover a console session, two terminals for one user, and empty output.
- [x] 6.2 Aggregate per-user CPU, memory, and disk from process ownership. Verify: unit tests sum a fixture's processes by uid and skip unreadable ones.
- [x] 6.3 Render the table with Active and Disconnected status and the current-session marker. Verify: goldens for two users, one disconnected, and root.
- [x] 6.4 Implement adding and removing rows on login and logout without a restart. Verify: a test swaps fixtures and asserts the row appears, disappears, and selection moves.
- [x] 6.5 Implement the session process panel. Verify: a golden shows one user's processes and dashes for unreadable fields.

## 7. Services tab

- [x] 7.1 Build the command runner that takes a program and argument vector, captures output, and never uses a shell. Verify: tests assert a label containing spaces, quotes, and `;` arrives as exactly one argument, and a missing program returns an error rather than a panic.
- [x] 7.2 Parse `systemctl show` blocks into unit rows. Verify: fixture tests cover running, stopped, missing description, and malformed lines.
- [x] 7.3 Parse `launchctl list` into job rows. Verify: fixture tests cover a running job with PID, a stopped job with exit status, and the header line.
- [x] 7.4 Wire both backends into the tab, including the Group column for user and system scope. Verify: goldens from each platform's fixture and a `--once --tab services` smoke run.
- [x] 7.5 Implement start, stop, and restart with privilege refusal before any command runs. Verify: a recording runner asserts the exact argument vectors for user scope and asserts no command runs for system scope.
- [x] 7.6 Add the confirmation dialog naming the service and its dependants warning. Verify: a golden shows it, and cancelling leaves the table unchanged.
- [x] 7.7 Implement the name and description filter with empty state. Verify: tests cover match, no match, and clear.
- [x] 7.8 Document which manager supplies services on each platform and what the app refuses. Verify: the README section matches the capability declarations.

## 8. Startup tab

- [x] 8.1 Parse desktop-entry files and implement disable-by-override in the user autostart directory. Verify: tempdir tests round-trip an entry, assert the system file is untouched, and assert `Hidden=true` produces Disabled.
- [x] 8.2 Parse launch agent plists, `launchctl list`, and `print-disabled`. Verify: fixture tests cover an enabled agent, a disabled agent, and a plist pointing at a missing program.
- [x] 8.3 Wire both backends into the tab with real status. Verify: goldens from each fixture and a `--once --tab startup` smoke run.
- [x] 8.4 Implement enable and disable with system-scope refusal and manager-failure dialogs. Verify: a recording runner asserts `launchctl disable gui/<uid>/<label>` and the systemd equivalent, and asserts nothing runs for system scope.
- [x] 8.5 Keep the Startup impact column at "Not measured". Verify: a test asserts no fixture ever produces High, Medium, or Low.
- [x] 8.6 Implement the properties dialog with command line, owner, and source file. Verify: a golden shows a full entry and one with a missing program.

## 9. App history tab

- [x] 9.1 Accumulate CPU time, peak memory, disk totals, and last seen per application name. Verify: unit tests feed three synthetic samples and assert the accumulated CPU time equals the sampled interval product.
- [x] 9.2 Implement versioned history persistence: atomic write, periodic flush, corrupt-file recovery, and 30-day retention. Verify: tempdir tests cover write, reload, corruption moved aside, and pruning of an old record.
- [x] 9.3 Render the table with the six columns and the network dash note. Verify: goldens for populated and empty history at two widths.
- [x] 9.4 Implement delete usage history with confirmation. Verify: a test asserts stored totals are cleared, running applications remain with reset totals, and cancel changes nothing.
- [x] 9.5 Document what is accumulated and where the file lives on each platform. Verify: the README paths exist after a run and match the documented ones.

## 10. Settings and menus

- [x] 10.1 Implement settings load and save with defaults and an unreadable-file fallback. Verify: tempdir tests cover missing file, valid file, invalid TOML, and theme surviving a restart.
- [x] 10.2 Wire the Options menu: theme, update speed, and column visibility. Verify: tests assert each change takes effect immediately and is persisted.
- [x] 10.3 Wire the View menu: refresh now, columns, expand all, collapse all, and sort. Verify: tests assert each item's effect on the active tab and that disabled items do nothing.
- [x] 10.4 Implement the run-new-task dialog from the File menu with fixed-argument execution and an error dialog on failure. Verify: a test launches a harmless command and asserts a failing command reports its exit status.

## 11. Integration and release checks

- [x] 11.1 Run the full golden sweep: all seven tabs, both themes, three sizes, with dialogs overlaid. Verify: `cargo test` passes and every new golden has been reviewed visually.
- [ ] 11.2 Add the GitHub Actions matrix for Ubuntu and macOS running format, clippy, test, and release build. Verify: the workflow lists the same commands that pass locally, and the first CI run is green.
- [ ] 11.3 Add a Linux container smoke test for `taskmgr --once`. Verify: the container run exits zero and prints a frame containing the tab strip.
- [x] 11.4 Write the platform notes: what each target reports, what shows a dash, and why. Verify: each claim matches a capability declaration and a test.
- [x] 11.5 Run the release checks. Verify: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate build-win10-task-manager --strict` all pass.

## Workflow follow-up

- Archive the change after the release checks pass and the goldens are reviewed.
- Verify the archived specs contain all ten capabilities.
