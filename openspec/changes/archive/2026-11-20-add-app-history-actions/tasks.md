# Tasks

## 1. Clear one

- [x] 1.1 Add `Action::ClearUsageEntry`, the confirmation, `Effect::ClearUsage`, history removal, and sampler plumbing, and verify tests for remove-one, cancel, and persistence
## 2. End app

- [x] 2.1 Route `e`/Delete on App history to a confirmed end-app action, and verify signals are emitted for each matching root
- [x] 2.2 Verify the no-process notice and the partial-failure error path

## 3. Documentation and checks

- [x] 3.1 Update the hint line and README for `x` and `e`, and verify they match
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-app-history-actions --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
