# Design

## Context

- `toggle_expand` on a child row currently does nothing; on a group it toggles expansion.
- `Dialog::Properties` holds static lines, so it cannot update while samples arrive.
- `show_properties` already builds identity lines from `ProcRow`.

## Decisions

### A key-based dialog

Add `Dialog::ProcessInfo { key: ProcKey, name: String }`. The draw pass resolves the key
against the current snapshot every frame, so values are live and a vanished process is
reported rather than frozen.

*Alternative.* Store formatted lines once, like Properties. Rejected: the numbers go stale
immediately.

### Enter routes by row kind

In `toggle_expand`, a `ProcSel::Proc` opens the modal; `Kind` and `Group` keep the existing
expand/collapse path. `i` calls the same opener on the Processes tab. Other tabs keep their
current `i` behaviour.

*Alternative.* A new key. Rejected: Enter is the obvious action on a selected row, and it
was a no-op there.

### Live line builder shared with Properties

`process_lines` gains metrics and stays available for static Properties. The modal builds
its lines with a richer function that reads the row each frame, including formatted CPU,
memory, disk read/write, network, and GPU.

### Closing

`translate_dialog` treats `ProcessInfo` like `Properties`: Enter, Esc, q, and F1 close it.
No action is dispatched.

## Risks / Trade-offs

- [Height] A long command line can push the modal past 40 rows → The dialog caps at the
  terminal height and the command line is truncated.
- [Enter semantics] Enter no longer does nothing on a child row → It now does the useful
  thing, and expansion remains on Right and Space.
