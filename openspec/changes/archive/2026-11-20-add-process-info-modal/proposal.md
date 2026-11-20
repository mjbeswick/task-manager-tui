# Proposal

## Why

On the Processes tab, Enter on a process row does nothing and the only detailed view is a
static Properties dialog behind `i`. Users expect Enter on a selected process to show its
details, and the numbers should stay current while the dialog is open.

## What Changes

- Enter on a process row opens a modal with live details; Enter on a group or kind heading
  keeps expanding or collapsing it.
- The modal shows identity, ownership, status, priority, threads, uptime, CPU, memory,
  disk, network, GPU, command line, and executable.
- Values update with every sample while it is open.
- Esc, Enter, or q closes it; the selection does not change.
- `i` opens the same modal on a process row.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `process-list`: adds a process details modal and its open/close rules.

## Impact

- `src/app.rs`: a dialog that stores the process key.
- `src/action.rs`: open and toggle routing.
- `src/input.rs`: dialog keys.
- `src/ui/dialog.rs`: live content rendering.
- Tests and help text.
