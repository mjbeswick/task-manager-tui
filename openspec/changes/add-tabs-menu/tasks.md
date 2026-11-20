# Tasks

## 1. Menu

- [x] 1.1 Add the Tabs menu with seven items and the `Tab(index)` item variant, and verify a test that each label maps to its tab
- [x] 1.2 Move Alt+P and Alt+H to indices 4 and 5 and add Alt+T at 3, and verify input tests

## 2. Narrow layout

- [x] 2.1 Share menu offsets between the bar and dropdown and use compact spacing when the padded layout does not fit, and verify 40-column render and click tests

## 3. Documentation and checks

- [x] 3.1 Update help and README for the Tabs menu and Alt+T, and verify they match
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-tabs-menu --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
