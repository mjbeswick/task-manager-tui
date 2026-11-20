# Tasks

## 1. Remove the rim

- [x] 1.1 Colour the tip with the ramp when gradients are on, in Filled and Blocks, and verify the tip is not the raw colour and is no brighter than the row below
- [x] 1.2 Verify eight rows still show eight distinct steps and the gradient-off fill stays flat

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate smooth-chart-gradient --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
