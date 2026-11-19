# Tasks

## 1. Braille line rendering

- [x] 1.1 Add a dot-matrix buffer that maps a plot of W by H cells to 2W by 4H dots and flushes set dots as braille characters, and verify unit tests for a corner dot, an empty cell, and the plot extents
- [x] 1.2 Draw Dots by setting one dot per sample, and verify a test that the marker lands on the expected dot row
- [x] 1.3 Draw Line by joining consecutive dots with a dot-space Bresenham segment, and verify a test that a rising sample leaves a continuous trail between the two points
- [x] 1.4 Keep the current bullet and vertical bar path when ASCII glyphs are active, and verify the ASCII test still finds only ASCII symbols

## 2. Gradient fill

- [x] 2.1 Blend each fill row from the chart colour toward the surface as it rises, keeping the edge cell strongest, and verify a test that the bottom cell and the top cell differ
- [x] 2.2 Keep the flat fill in 16-colour mode and in ASCII mode, and verify the coarse test still finds one fill colour
- [x] 2.3 Confirm sparklines inherit the gradient through `fill_column`, and verify a rail render test shows more than one fill colour

## 3. Theme chart colour

- [x] 3.1 Add a Theme chart colour that resolves to the active theme's chart fill, and verify a unit test that it changes when the theme changes
- [x] 3.2 Add it to the Appearance cycling, the CLI and settings parsers, and the help text, and verify the existing chart-colour tests cover the new variant

## 4. Documentation and integration checks

- [x] 4.1 Note the braille requirement and the ASCII fallback in the README chart section, and verify the note renders in the existing style table
- [x] 4.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-chart-resolution --strict`, and record the output

## Workflow follow-up

- Archive the change after its checks pass and the work is reviewed.
