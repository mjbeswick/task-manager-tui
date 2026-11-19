# Proposal

## Why

The Users tab shows one row per terminal session, but its totals are summed per
user. A user with four terminals gets the same totals on four rows. On a Linux
server that read as 7.6 GB of use on a 15 GB machine, from 1.9 GB of real use.

## What Changes

- The Users tab shows one row per user, as Windows 10 does.
- CPU, Memory, and Disk appear once per user, summed over that user's processes.
- The Session column names the user's first session and counts the rest, for
  example `pts/0 +3`.
- Status reads Active when any of the user's sessions is active.
- The current marker stays on the user running this application.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `users`: rows are per user rather than per session.

## Impact

- `src/sample/mod.rs` merges sessions by user ID.
- `src/model.rs` gains a count of extra sessions on `SessionRow`.
- `src/ui/users.rs` formats the Session column.
- Golden frames for the Users tab change only where a user has several sessions.
- No change to settings, history, or other tabs.
