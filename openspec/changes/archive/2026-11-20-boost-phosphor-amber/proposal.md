# Proposal

## Why

Phosphor and Amber use the smallest surface-to-stripe step of any theme (3-4 channel
steps), so the row stripes disappear. Their saturated chart colours also make the fixed
gradient proportion read weaker than on the other palettes.

## What Changes

- Increase `surface_alt` for Phosphor and Amber to a clearly visible step.
- Add a per-theme gradient strength, 1.5 for Phosphor and Amber and 1.0 elsewhere, which
  deepens the midpoint and top stops without changing the direction.
- Add a minimum stripe-step test for every theme.

## Capabilities

### New Capabilities

None.

## Impact

- `src/theme.rs`: two stripe colours and a new strength field on all palettes.
- `src/ui/graph.rs`: ramp takes the strength.
