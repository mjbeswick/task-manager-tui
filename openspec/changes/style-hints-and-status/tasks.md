# Tasks

## 1. Hint colours

- [x] 1.1 Draw `[key]` in accent bold and actions in muted, and verify a render test that the key cells use the accent while the action cells use muted

## 2. Status colours

- [x] 2.1 Split each metric into a muted label and a text-coloured value, and verify a render test for both styles and an unchanged field width
- [x] 2.2 Draw the speed value in the accent, and verify it keeps its fixed field

## 3. Checks

- [x] 3.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate style-hints-and-status --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
