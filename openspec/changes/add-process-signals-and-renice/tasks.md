# Tasks

## 1. Signal data and platform plumbing

- [x] 1.1 Add a `Signal` enum to `src/model.rs` with TERM, KILL, INT, HUP, STOP, and CONT, each with a short name, a label, and its POSIX number, and verify a unit test asserts the six numbers and unique names
- [x] 1.2 Change `platform::signal` to take a signal number instead of a force flag, and verify its tests still deliver SIGTERM and SIGKILL to a child and report a missing PID
- [x] 1.3 Replace `Effect::Signal { force }` with `{ signal, tree }`, map `Action::EndTask` to TERM and `Action::EndTree` to KILL, and verify `cargo test` still covers both existing end-task paths
- [x] 1.4 Update `src/main.rs` to send the chosen signal and name it in success and failure notices, and verify a refused signal still opens the permission dialog

## 2. Signal dialog and dispatch

- [x] 2.1 Add `Dialog::Signal` and bind `K` on the Processes and Details tabs, listing the six signals plus a tree toggle, and verify input tests cover the key and both tabs
- [x] 2.2 Add signal submission that sends directly for TERM, INT, HUP, and CONT on an owned process, and routes KILL, STOP, a system process, and another user's process through `Dialog::Confirm`, and verify action tests cover each branch
- [x] 2.3 Carry the signal and tree choice through `PendingAction::Signal` so confirmation emits the same effect, and verify a confirmed KILL reaches `Effect::Signal`
- [x] 2.4 Send the chosen signal to every descendant when the tree toggle is on, stopping at the first error, and verify a test uses the fixture's parent and child PIDs
- [x] 2.5 Show the existing "select a process" notice for a kind heading, and verify an action test leaves the dialog closed and emits no effect

## 3. Numeric priority

- [x] 3.1 Rework `Dialog::Priority` to hold a text field seeded from the current nice value, with Up/Down stepping by one, PgUp/PgDn by five, digits and a leading minus appended, and Backspace editing, and verify action tests for each
- [x] 3.2 Parse on Enter, clamp to -20..19, refuse an empty or invalid field with a visible error, emit `Effect::Priority`, and verify tests cover a normal value, both clamps, and the empty case
- [x] 3.3 Render the field with the matching Windows 10 label and the range hint, and verify a render test shows the value and its label
- [x] 3.4 Remove the stale "select a process on the Details tab" notice and verify a test opens the dialog from the Processes tab
- [x] 3.5 Add `K` and the numeric `p` description to the in-app help and the README key table, and verify the help panel fits its longest row at 120 columns

## 4. Integration checks

- [x] 4.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-process-signals-and-renice --strict`, and record the output

## Workflow follow-up

- Archive the change after its checks pass and the work is reviewed.
