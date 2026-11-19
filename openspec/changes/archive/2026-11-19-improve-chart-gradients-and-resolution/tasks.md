# Tasks

## 1. Colour ramp

- [x] 1.1 Add `theme::mix_linear` using the existing sRGB transfer function, and verify a test that a half-way blend of black and white is close to mid-grey in linear light rather than the darker sRGB average
- [x] 1.2 Add `graph::ramp(edge, bg, rise)` with a dim midpoint stop, and verify a test that rise 0 is the chart colour, rise 1 is the surface, and the midpoint keeps the hue
- [x] 1.3 Use the ramp in `fill_column` and verify a test that an eight-row fill has at least eight distinct colours, while 16-colour mode keeps one flat colour

## 2. Line colour

- [x] 2.1 Store one peak fraction per braille cell and record it from `set` and `line`, and verify a test that two cells holding different heights end with different peaks
- [x] 2.2 Flush braille cells through the ramp, and verify a test that a line's top cell is brighter than its base cell and that coarse mode keeps one colour

## 3. Resampling

- [x] 3.1 Add `sample_column(values, column, width)` with interpolation for upscaling and peak for downscaling, and verify unit tests for both directions and for a column beyond the history
- [x] 3.2 Use the sampler in `draw` for Filled, Line, and Dots, and verify a test that a narrow plot keeps a one-sample spike and a wide plot produces intermediate values instead of repeats
- [x] 3.3 Verify the empty-history behaviour still leaves the left side blank and that ASCII mode draws only ASCII symbols

## 4. Documentation and checks

- [x] 4.1 Note the gamma-correct ramp, the value-coloured line, and the peak-preserving resampling in the README performance section, and verify the note fits the existing prose
- [x] 4.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate improve-chart-gradients-and-resolution --strict`, and record the output

## Workflow follow-up

- Archive `add-chart-resolution` before this change, so its braille and gradient requirements land first.
- Archive this change after its checks pass and the work is reviewed.
