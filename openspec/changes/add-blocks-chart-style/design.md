# Design

## Context

- `fill_column` paints one column from the bottom with eighths.
- `draw` already computes each column's value and level; a Blocks renderer needs the
  older neighbour's level to choose a slope glyph.
- `Glyphs` has `block_full` and `eighths`; the angular glyphs are constants.

## Decisions

### Precompute levels, then render each column with its older neighbour

`draw` collects `level[column]` in eighths. The Blocks loop receives the column's own total
and the total of `column + 1`, which is the older column to the left on screen.

For the top cell:

- `diff = own_units - neighbour_units` at that row.
- `diff >= 2`: `▟`, filled more on the right, so the edge falls to the left.
- `diff <= -2`: `▛`, filled more on the left, so the edge rises to the left.
- `|diff| == 1`: `◢` or `◣` for a one-step diagonal.
- otherwise: the eighth block for the fraction.

Cells below the top stay `█`, coloured with the existing ramp. This is one glyph per cell,
so no two families ever fight inside a cell.

*Alternative.* Quadrants for every cell. Rejected: a single wide column has no horizontal
detail, so the fill would look lopsided.
*Alternative.* Triangles for all slopes. Rejected: a two-eighth step needs a half-cell
edge, not a quarter-cell triangle.

### ASCII and coarse fallbacks

ASCII mode has no angular glyphs, so Blocks falls back to the eighths renderer. Colour
depth does not change glyph choice; 16-colour mode keeps the same shapes with flat fills,
as Filled does.

### The cycle grows to four

`ChartStyle::ALL` gains Blocks. The Appearance back-cycle currently hardcodes a length of
three; it switches to `ALL.len()`.

## Risks / Trade-offs

- [Noisy edges] Rapid alternating values could alternate `◢`/`◣` every column → The glyph
  still follows the data; the gradient colour keeps the shape readable.
- [Font support] Quadrants and triangles are less universal than braille → They are opt-in
  through the Blocks style, and ASCII mode avoids them entirely.
