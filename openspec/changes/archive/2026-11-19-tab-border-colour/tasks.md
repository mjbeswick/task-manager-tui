# Tasks

## 1. Match the outline to the rule

- [x] 1.1 Draw the active tab's top, sides, and corners with the border style and verify the outline and the rule share a foreground
- [x] 1.2 Verify the active label stays accent-coloured

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate tab-border-colour --strict`, and record the output

## Workflow follow-up

- Archive `fix-tab-corners` before this change.
- Archive this change after its checks pass and the work is reviewed.
