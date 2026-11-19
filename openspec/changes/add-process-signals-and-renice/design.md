# Design

## Context

The interface never signals a process itself. `action.rs` turns input into `Effect`
values, and `main.rs` performs the system call. End task and end tree already follow
that path through `PendingAction` and `Dialog::Confirm`. Priority uses the same split via
`Effect::Priority` and `platform::set_priority`.

`platform::signal(pid, force)` currently chooses SIGTERM or SIGKILL from a boolean. Both
target platforms use the POSIX signal numbers.

## Goals / Non-Goals

**Goals:**

- One signal path for both fixed actions and the new dialog.
- Keep the effect layer pure and testable; no system calls in `action.rs`.
- Keep the Windows 10 priority labels while allowing any nice value.

**Non-Goals:**

- Signals beyond the six listed in the spec.
- Remembering the last signal or tree choice between invocations.
- Priority changes for services, or per-process affinity.
- A number-entry parser beyond an optional leading minus and ASCII digits.

## Decisions

### A `Signal` enum in the model

`Signal` carries a label, a short name, and the POSIX number: HUP 1, INT 2, KILL 9, TERM
15, CONT 18, STOP 19. It lives beside `Priority` in `model.rs`, so the dialog, the effect,
and the tests share one list. The platform layer passes the number to `libc::kill`.

*Alternative.* Pass raw integers from the UI. Rejected: the dialog would hard-code
numbers and confirmation rules twice.

### `Effect::Signal` carries the signal, not a force flag

`Effect::Signal { key, signal, tree }` replaces `{ key, force, tree }`. `EndTask` and
`EndTree` map to TERM and KILL, so the existing bindings keep their behaviour and the
main loop has one branch.

### Confirmation is decided in `action.rs`

The dialog chooses a signal. Before sending, `action.rs` asks for confirmation when the
signal is KILL or STOP, or when the process is a system process or owned by another
user. That mirrors `confirm_end_task` and keeps ownership checks in one layer.

A confirmed choice becomes `PendingAction::Signal(key, signal, tree)` and travels through
the existing `Dialog::Confirm`. A choice that needs no confirmation emits the effect
directly.

### The signal dialog is the primary confirmation

The panel names the process, its PID, and the chosen signal. It is not a second question
after a first one; it replaces the old implicit TERM. `Enter` sends, `Esc` cancels.

*Alternative.* A submenu under End task. Rejected: it hides five of six signals behind
one binding, and the dialog already has the user's attention.

### Numeric priority as a single text field

`Dialog::Priority { key, name, input, error }` holds the digits. It is seeded from the
process's current nice value.

- `Up`/`Down` adjust the parsed value by one; `PgUp`/`PgDn` by five. Each rewrites the
  field, so the value shown and the value sent never diverge.
- Digits and a leading `-` append to the field; `Backspace` removes the last character.
- `Enter` parses. An empty or invalid field sets `error` and leaves the dialog open.
- A parsed value is clamped to -20..19 before `Effect::Priority` is emitted.
- The Windows 10 label comes from `Priority { nice }.label()`, already written and tested.

*Alternative.* Keep five selectable rows. Rejected: it cannot express `renice -n 7`.
*Alternative.* Arrow keys only. Rejected: reaching -20 takes twenty presses.

### Tree sends repeat the same signal

`descendants()` already walks parent links from the current sample. The main loop sends
the chosen signal to the selected PID first, then each descendant, stopping at the first
error and reporting the PID that failed. This matches the existing end-tree behaviour.

## Risks / Trade-offs

- [Stale tree] A child forked after the last sample is missed → Same limitation as end
  tree today; the next sample shows it, and the user can signal it directly.
- [Signal numbers] Literal numbers are POSIX, not portable to Windows → The crate targets
  Linux and macOS only, as `platform/mod.rs` already assumes.
- [Clamping hides intent] Typing 40 quietly becomes 19 → The dialog shows the clamped
  value before the user confirms; no silent send.
- [Destructive default] `Enter` in the dialog sends without a second question for TERM →
  The dialog is an explicit choice, and KILL and STOP still confirm.
