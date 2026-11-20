# Tasks

## 1. Help

- [x] 1.1 Add `i`, the gradient, and both Space meanings to the help rows, and verify the help render test includes them

## 2. About

- [x] 2.1 Add Help > About and a dialog with version, platform, theme, depth, and paths, and verify a render test
- [x] 2.2 Close About on Esc, Enter, and q, and verify input tests

## 3. Checks

- [x] 3.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-help-about --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
