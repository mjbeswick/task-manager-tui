# Tasks

## 1. Ramp

- [x] 1.1 Set the ramp midpoint to 22% and top stop to 50%, and verify an eight-row fill whose top differs from the base by a measurable step and from the surface
- [x] 1.2 Verify the gradient-off path stays flat

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate fix-gradient-visibility --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
