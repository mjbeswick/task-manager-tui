# Tasks

## 1. Ramp target

- [x] 1.1 Drop `bg` from `ramp` and choose black or white by contrast from the series colour, and verify the midpoint and top stops
- [x] 1.2 Verify an eight-row fill differs by the minimum step in every theme
- [x] 1.3 Verify no rim, eight distinct steps, and the disabled flat path

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate theme-independent-gradient --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
