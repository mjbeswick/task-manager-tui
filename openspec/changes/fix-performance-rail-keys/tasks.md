# Tasks

## 1. Move the rail

- [x] 1.1 Handle the Performance tab in `move_selection`, and verify a test that Down moves CPU to Memory and Up returns to CPU
- [x] 1.2 Verify wrapping at both ends, so Up from CPU selects GPU and Down from GPU selects CPU

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate fix-performance-rail-keys --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
