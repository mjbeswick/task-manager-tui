# Proposal

## Why

System services and processes owned by another user need administrator rights. The app
currently refuses or fails with a permission message, so the user has to leave it and run
the command by hand.

## What Changes

- Per-action elevation: only the requested service command or signal runs elevated.
- The whole TUI never runs as root.
- macOS uses the system administrator prompt through `osascript`.
- Linux uses `pkexec`, then `sudo -n`, then a masked in-app password field feeding
  `sudo -S`.
- Signals become an elevated `kill -s NAME pid...` when the target belongs to another user
  or root.
- Signalling a process tree includes its descendants in the elevated command.
- A cancelled or failed elevation changes nothing and reports the reason.

## Capabilities

### New Capabilities

- `elevation`: per-action administrator execution, platform helpers, fallbacks, and the
  no-storage rule.

### Modified Capabilities

- `services`: a system-scope action offers elevation instead of refusing.

## Impact

- `src/platform/mod.rs`: elevated command construction and runners.
- `src/app.rs`, `src/action.rs`, `src/input.rs`, `src/ui/dialog.rs`: effect, confirmation,
  password modal.
- `src/main.rs`: elevation handling.
- Tests, help, README.
