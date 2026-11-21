# Tasks

## 1. Menu bar

- [x] 1.1 Remove the Tabs menu and `MenuItem::Tab`, renumber the Alt accelerators, and verify the five menu labels and their shortcuts

## 2. Tab strip

- [x] 2.1 Measure the full labels, draw the strip when they fit, and remove the four-character abbreviation, and verify renders at 120, 80, and 40 columns
- [x] 2.2 Draw the `Tabs: <active> ▾` button in the raised box when the labels do not fit, and verify its hit target
- [x] 2.3 Add the tab-list state and dropdown with the active mark and digit hints, and verify keyboard, Alt+T, click, and outside-click behaviour

## 3. Documentation and checks

- [x] 3.1 Update help and README for the five menus, the adaptive strip, and Alt+T, and verify the wording
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate adaptive-tabs --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
