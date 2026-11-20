# Tasks

## 1. Stronger ramp

- [x] 1.1 Set the ramp midpoint to 12% and top stop to 28% toward the surface, and verify a unit test for the stops
- [x] 1.2 Pass the full rise from `fill_column` and `blocks_column`, and verify an eight-row fill whose row below the tip is closer to the chart colour than the surface

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate strengthen-chart-gradient --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
