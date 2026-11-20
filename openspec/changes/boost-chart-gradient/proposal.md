# Proposal

## Why

The gradient now runs from the base to 50% toward the surface, but on a tall fill it still
reads as one colour. The requirement already asks for a visible gradient, so this is a
strength tuning, not a new behaviour.

## What Changes

- Raise the midpoint to 32% and the top stop to 68% toward the surface.
- Keep linear-light blending, the base at the chart colour, and no bright tip rim.
- Strengthen the test's minimum step from the base.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. `performance` already requires a visible gradient with a distinct top; this change
meets it more clearly.
