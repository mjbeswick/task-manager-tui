# Proposal

## Why

The specs say every tab renders on Linux and every test passes there. Nothing has
checked that yet. The CI workflow exists but has never run, because the repo had
no remote. All work so far ran on macOS only.

## What Changes

- Host the repository as a private GitHub repo so the existing CI workflow runs.
- Keep the Ubuntu and macOS matrix for format, clippy, test, and release build.
- Run the smoke render inside a plain Linux container as well as on the runner.
  The container has no systemd and no login sessions, so it checks that Services,
  Startup, and Users degrade to a message instead of failing.
- Build and test on a real Linux server on the local network.
- Fix whatever the first Linux runs expose.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `platform-support`: every push to main is verified on both platforms, and the
  application renders in a Linux container with no service manager.

## Impact

- New private GitHub remote. Pushes from here on run CI.
- `.github/workflows/ci.yml` gains a container smoke job.
- Linux-only code paths in `src/platform/linux.rs` may need fixes.
- No change to behaviour on macOS.
