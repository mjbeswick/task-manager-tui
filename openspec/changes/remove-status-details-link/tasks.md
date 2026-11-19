# Tasks

## 1. Remove the link

- [x] 1.1 Drop the Fewer/More details label and its width from the status bar, and verify a render test that the Processes status bar starts with the position counter
- [x] 1.2 Remove `Target::FewerDetails` and its click arm, and verify the hit and mouse tests no longer expect it

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate remove-status-details-link --strict`, and record the output

## Workflow follow-up

- Archive after `add-tab-shortcut-hints`, which put the toggle in the hint line.
- Archive this change after its checks pass and the work is reviewed.
