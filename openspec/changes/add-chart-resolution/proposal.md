# Proposal

## Why

Filled charts already resolve eight vertical steps per cell, but Line and Dots draw one
marker per cell row, so a 20-row plot has only 20 vertical positions and staircase edges.
Filled charts also use one flat fill colour, which hides the shape behind the fill.

## What Changes

- Line and Dots render through a braille dot matrix: 2 x 4 dots per cell, joined with a
  dot-space line for smooth curves.
- Filled charts gain a vertical gradient, blending the chart colour toward the surface as
  the fill rises.
- Sparklines use the same gradient because they share the column renderer.
- A Theme chart colour follows the active theme's chart fill, so a themed window does
  not clash with a fixed teal.
- ASCII mode keeps the current line markers and flat fill.
- 16-colour mode keeps flat fills but still gains braille lines.
- No new dependency and no new glyph set.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: the chart appearance requirement describes braille lines and a gradient
  fill, and a new requirement covers sub-cell resolution and the fallbacks.

## Impact

- `src/ui/graph.rs`: a braille dot buffer for Line and Dots, a gradient walk in
  `fill_column`, and the ASCII fallback.
- `src/theme.rs`: gradient helper if `mix` needs a reusable ramp; no new role.
- `src/ui/performance.rs`: no layout change, but the tests read symbols from the plot.
- `openspec/specs/performance/spec.md` and `README.md` chart notes.
