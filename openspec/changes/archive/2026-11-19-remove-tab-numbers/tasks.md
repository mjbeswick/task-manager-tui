# Tasks

## 1. Remove the digits

- [x] 1.1 Drop the digit prefix in the wide and compact strips, and verify render tests that the labels carry no digits and the box follows the new width
- [x] 1.2 Keep the 1-7 bindings and update the tab tests that referenced the printed numbers

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate remove-tab-numbers --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
