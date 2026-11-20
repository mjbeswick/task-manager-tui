# Tasks

## 1. Stripes and gradient

- [x] 1.1 Raise `surface_alt` for Phosphor and Amber, and verify a test that every theme's stripe differs from its surface by at least five channel steps
- [x] 1.2 Add `gradient_strength` and scale the ramp stops, and verify Phosphor and Amber produce a larger base-to-top step than before while others are unchanged

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate boost-phosphor-amber --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
