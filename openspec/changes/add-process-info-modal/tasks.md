# Tasks

## 1. Dialog

- [x] 1.1 Add `Dialog::ProcessInfo { key, name }` and a live line builder with identity, state, metrics, command, and executable, and verify a unit test for a background process
- [x] 1.2 Render the modal with truncation, a close footer, and the exited state, and verify a render test

## 2. Routing

- [x] 2.1 Open the modal from Enter on a process row and from `i`, leaving group and kind rows on the expand path, and verify action tests for both
- [x] 2.2 Close on Enter, Esc, and q without moving the selection, and verify input and action tests

## 3. Documentation and checks

- [x] 3.1 Update the README and help text for Enter on a process row, and verify they match the behaviour
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-process-info-modal --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
