# Tasks

## 1. Setting and dialog

- [x] 1.1 Add `chart_gradient` to settings and `App`, default true, and verify settings tests
- [x] 1.2 Add the Gradient row to Appearance with left/right and persistence, and verify action tests for both states

## 2. Rendering

- [x] 2.1 Thread the flag through `GraphSpec`, `PlotSpec`, and the fill helpers, and verify with gradients off every chart cell equals the flat chart colour
- [x] 2.2 Verify gradients on still produce the eight-row ramp

## 3. Documentation and checks

- [x] 3.1 Note the Gradient row in the README Appearance section, and verify it matches the dialog
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-chart-gradient-option --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
