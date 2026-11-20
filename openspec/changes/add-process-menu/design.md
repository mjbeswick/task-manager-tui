# Design

## Context

- `MENUS` is a fixed array of four menus; indices are used for Alt shortcuts and menu state.
- `MenuItem` maps to actions in `menu_item`.
- Process actions already exist as `Action::EndTask`, `EndTree`, `OpenSignal`, `SetPriority`,
  and `ShowProperties`.

## Decisions

### Insert the menu at index 3

Order: File, Options, View, Process, Help. Alt+P maps to 3; Alt+H moves to 4. The menu table
is the single source; the tests that hardcode an index are updated.

### Reuse actions unchanged

Each item returns the existing action through `self.dispatch`. No new action variants. The
existing notices handle tabs without a process.

## Risks / Trade-offs

- [Width] Five menu labels use 36 columns → At the 40-column minimum the filter field
  already shrinks to `/`; it still fits.
- [Unavailable actions] Menu items stay visible on tabs where they do nothing → The notice
  explains it; a proper disabled state would need per-item enablement, a larger change.
