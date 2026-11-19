# Tasks

## 1. Glyph selection

- [x] 1.1 Add optional angular glyphs to `Glyphs` and set them to `None` in ASCII, and verify the ASCII glyph test covers the new fields
- [x] 1.2 Implement `slope_glyph` with the reviewed table, and verify the 8 x 9 matrix test
- [x] 1.3 Add invariants: every returned glyph keeps both bottom quadrants, and mirrors swap `▟`/`▙` and `◢`/`◣`

## 2. Column and sampling

- [x] 2.1 Use `slope_glyph` in `blocks_column` and apply the tighter Blocks ramp, and verify an end-to-end draw that the newest column is rightmost and its falling join is `▙`
- [x] 2.2 Make native-width sampling exact, and verify a spike at width 60 is not averaged

## 3. Checks

- [x] 3.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate improve-blocks-rendering --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
