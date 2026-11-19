# Proposal

## Why

The Processes tab can only send SIGTERM or SIGKILL to a whole tree, and priority is
limited to five preset levels. Users who need SIGINT, SIGHUP, SIGSTOP, or a precise nice
value have to leave the application and run `kill` and `renice` by hand.

## What Changes

- A signal dialog on the Processes and Details tabs, opened with `K`, offering TERM,
  KILL, INT, HUP, STOP, and CONT, with an optional process-tree toggle.
- TERM, INT, HUP, and CONT send immediately for a process the user owns. KILL, STOP, a
  system process, and another user's process pass through the existing confirmation
  panel first.
- The priority dialog becomes a numeric renice over -20 to 19, seeded with the current
  value. It steps by one, steps by five, or accepts a typed value, and still shows the
  matching Windows 10 label.
- The stale "select a process on the Details tab" notice is removed, since both tabs
  resolve a selected process.
- The in-app help and the README key table gain `K` and describe `p` as a numeric value.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `process-list`: adds a signal requirement alongside end task, with confirmation rules
  per signal and per ownership.
- `details`: the priority requirement accepts any value from -20 to 19 and keeps the
  Windows 10 labels as annotations rather than the only choices.

## Impact

- `src/action.rs`: signal dialog dispatch, confirmation routing, numeric priority.
- `src/app.rs`: `Dialog::Signal`, `PendingAction::Signal`, `Effect::Signal` shape.
- `src/input.rs`: `K` binding and dialog key handling.
- `src/ui/dialog.rs`: signal list rendering, numeric priority field, help rows.
- `src/platform/mod.rs`: `signal` takes a signal number.
- `src/main.rs`: notices and errors name the signal that was sent.
- `README.md` and `specs/process-list/spec.md`, `specs/details/spec.md`.
