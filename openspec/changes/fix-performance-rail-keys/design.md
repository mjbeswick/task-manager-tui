# Design

## Context

- `move_selection` matches the list tabs and returns false for Performance.
- `PerformanceTab.resource` has `next()` and `previous()`, used by mouse clicks? No, by
  the wheel path only through `MoveDown`/`MoveUp`, which also hit the no-op.
- `Action::MoveDown`/`MoveUp` are the only keyboard path.

## Decisions

### Handle Performance in `move_selection`

For the Performance tab, call `resource.next()` when `delta > 0` and `resource.previous()`
when `delta < 0`, then return true. PageUp/PageDown and Top/Bottom therefore also step
once, which is harmless on a five-item rail.

*Alternative.* Handle it in `dispatch` before `move_selection`. Rejected: two places would
then own selection movement, and the wheel path also goes through `dispatch`.

## Risks / Trade-offs

- [Large deltas] `PageDown` steps one resource instead of ten → The rail has five entries;
  one step is the sensible interpretation.
