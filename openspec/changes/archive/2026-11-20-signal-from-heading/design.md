# Design

## Context

- `open_signal_dialog` uses `selected_process`, which resolves process and group rows but
  returns `None` for a kind heading.
- A kind heading knows its `Kind`, so the first matching process is well defined.

## Decisions

### Resolve a heading to its first process

Add a local helper that tries `selected_process` first, then, for `ProcSel::Kind(kind)`,
returns the first snapshot process with that kind. Signals use it. The dialog already
shows the name and PID, so no hidden target.

*Alternative.* Change the default selection to a group row. Rejected: it changes what the
whole tab opens on and every test around the first row.

## Risks / Trade-offs

- [Implicit target] The first process under a heading may not be the one the user meant →
  The dialog names it before any signal is sent, and destructive signals still confirm.
