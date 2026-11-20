# Design

## Context

- `snapshot.app_usage` comes from `History::rows`; the sampler owns persistence.
- `Effect::ClearHistory` wipes all and the sampler flushes.
- `AppUsage` has no PID; processes are matched by app name.

## Decisions

### `x` for clear, `e`/Delete for end

`x` is free and reads as remove. `e`/Delete already mean end task on process tabs; routing
it on App History is consistent. Both confirm with `Dialog::Confirm`, then dispatch a new
effect.

- `Effect::ClearUsage(String)` removes one record and flushes.
- `PendingAction::EndApp(String)` resolves matching processes at confirm time and emits one
  `Effect::Signal` per process, so the existing main-loop error handling applies.

Processes are matched by `name` and `Kind::App`, excluding processes whose parent is also
a match, so a tree is not signalled twice.

*Alternative.* A new bulk effect. Rejected: individual signals reuse all existing error and
notice paths.

### History removal

Add `History::remove(name)` and `Sampler::clear_app(name)`, mirroring `clear_history`.

## Risks / Trade-offs

- [Name collisions] Two bundles can share a display name → The confirmation shows the name
  and count; the common case is one bundle.
- [Partial failure] One refused PID stops later signals → Same rule as end tree; the dialog
  names the failing PID.
