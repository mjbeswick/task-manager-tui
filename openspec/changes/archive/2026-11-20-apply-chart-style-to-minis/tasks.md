# Tasks

## 1. Mini plotter

- [x] 1.1 Add `plot_series` with Filled, Blocks, Line, Dots, and ASCII paths, and verify sparkline tests for each style
- [x] 1.2 Pass the style from the rail sparkline and per-core `draw_mini`, and verify a render test that changing the style changes the mini glyphs

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate apply-chart-style-to-minis --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
