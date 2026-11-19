# Proposal

## Why

Filled, Line, and Dots cover vertical precision and smooth lines, but a filled chart cannot
show a diagonal edge: the top of each column steps by eighths. Quadrant blocks and
half-cell triangles can express that edge, and eighth blocks keep the vertical detail.

## What Changes

- Add a fourth chart style, Blocks, after Dots in the Appearance cycle.
- Blocks fills with full cells, uses eighth blocks for the vertical tip, half-cell
  triangles for one-step slopes, and quadrant blocks for steeper joins with the older
  neighbouring column.
- ASCII mode and 16-colour mode keep their existing glyph rules.
- Existing Filled, Line, and Dots rendering is untouched.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: the style list gains Blocks, with its own scenario.

## Impact

- `src/model.rs`: the style variant, label, key, parser, cycle.
- `src/ui/graph.rs`: a blocks column renderer.
- `src/action.rs`: the Appearance style back-cycle length.
- `README.md`: the style list.
