# Tasks

## 1. Fixed-width fields

- [x] 1.1 Format CPU, Memory, Disk, Network, GPU, and the speed word into fixed-width fields, and verify a unit test that each formatter pads to its field for small, mid, and maximum values
- [x] 1.2 Format the position counter as four-digit numerator and denominator, and verify a unit test that `1/9` and `123/4567` both occupy the same width
- [x] 1.3 Verify with two render frames at different values that the metric block starts at the same column

## 2. Stable zones

- [x] 2.1 Draw the metric block first, then the left action and position, then truncate the notice into the remaining middle zone, and verify a render test with a notice that the metrics and action do not move
- [x] 2.2 Choose dropped metrics from the fixed-width summary, and verify a narrow render test keeps the same metric set across two frames with different values
- [x] 2.3 Verify a notice longer than the middle zone is truncated instead of overlapping either side

## 3. Integration checks

- [x] 3.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate stabilize-status-bar --strict`, and record the output

## Workflow follow-up

- Archive the change after its checks pass and the work is reviewed.
