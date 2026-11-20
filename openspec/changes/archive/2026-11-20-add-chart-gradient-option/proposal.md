# Proposal

## Why

The gradient is always on, with no way to get the flat Windows-style fill or to avoid
the ramp on terminals where it reads poorly. Every other chart appearance choice is
adjustable and persisted.

## What Changes

- Add a Gradient On/Off row to Appearance, persisted with the other chart settings.
- When on, charts keep the current gamma-correct ramp.
- When off, every fill, line cell, and sparkline uses the flat chart colour.
- No new key binding; the row uses the existing left/right controls.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: gradients become optional, and the existing gradient requirements apply
  only when the option is on.

## Impact

- `src/store/settings.rs`, `src/app.rs`, `src/action.rs`, `src/ui/dialog.rs`: the setting.
- `src/ui/graph.rs`: a gradient flag through graph and mini specs.
- `src/ui/performance.rs`: pass the flag.
- `README.md`: note the new Appearance row.
