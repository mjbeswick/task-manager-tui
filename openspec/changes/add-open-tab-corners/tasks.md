# Tasks

## 1. Corners

- [x] 1.1 Replace the active tab's rule-row `│` ends with `└` and `┘` in the accent, and verify the corners, the blank span, and the rule resuming after the right corner
- [x] 1.2 Verify the narrow strip and ASCII mode use the same corners from the glyph table

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-open-tab-corners --strict`, and record the output

## Workflow follow-up

- Archive `open-active-tab-bottom` before this change.
- Archive this change after its checks pass and the work is reviewed.
