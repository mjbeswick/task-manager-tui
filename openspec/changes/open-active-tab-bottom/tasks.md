# Tasks

## 1. Open the tab

- [x] 1.1 Replace the active tab's bottom border with an empty span and continue its sides through the rule row, and verify a test that the span is blank and the sides are accent `│`
- [x] 1.2 Verify the rule stops at the left side and resumes after the right side
- [x] 1.3 Verify compact and ASCII strips keep the same open shape

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate open-active-tab-bottom --strict`, and record the output

## Workflow follow-up

- Archive `add-raised-tabs` before this change.
- Archive this change after its checks pass and the work is reviewed.
