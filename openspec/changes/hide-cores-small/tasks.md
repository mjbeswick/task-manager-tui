# Tasks

## 1. Size gate

- [x] 1.1 Compute the cell size before allocating the cores strip and hide it below four rows or ten columns, and verify renders at 120x40 and 80x24
- [x] 1.2 Remove the unframed fallback and verify no label-only CPU cells are drawn

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate hide-cores-small --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
