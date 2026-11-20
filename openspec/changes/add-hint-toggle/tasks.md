# Tasks

## 1. Setting

- [x] 1.1 Add `show_hints` to settings and App, default true, and verify the default and round trip
- [x] 1.2 Add F4 and the View menu item, and verify both toggle and persist

## 2. Rendering

- [x] 2.1 Make `hint_area` return none when hidden, and verify a render test with no hint row and one more data row

## 3. Documentation and checks

- [x] 3.1 Add F4 and Shortcut hints to help and README, and verify they match
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-hint-toggle --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
