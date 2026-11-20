# Proposal

## Why

On a fresh Processes tab the selection is the Apps heading, not a process row. Pressing
`K` there only leaves a status notice, so the key appears broken until the user selects a
group or child.

## What Changes

- Pressing `K` on a kind heading opens the signal dialog for the first process listed under
  that heading.
- The dialog names the process and its PID, so the target is explicit.
- Confirmation rules are unchanged: KILL, STOP, system processes, and other owners still
  ask first.
- If the heading has no process, the existing notice remains.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `process-list`: the signal dialog can open from a kind heading.

## Impact

- `src/action.rs`: heading resolution.
- Tests for opening from the default heading.
