# Tasks

## 1. Flag

- [x] 1.1 Add `--chart-style` with a parser built from `ChartStyle::ALL`, and verify tests for `blocks`, a mixed-case value, and an unknown value
- [x] 1.2 Apply the override before `App::new`, and verify a unit test that the app starts with Blocks when the flag is set

## 2. Documentation and checks

- [x] 2.1 Add the flag to the README flags table, and verify it matches the CLI help
- [x] 2.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-chart-style-flag --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
