# Tasks

## 1. Swap the corners

- [x] 1.1 Draw `┘` on the left and `└` on the right of the open tab, and verify the corners, the blank middle, and the rule joining each side
- [x] 1.2 Verify the compact strip and ASCII mode use the swapped corners from the glyph table

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate fix-tab-corners --strict`, and record the output

## Workflow follow-up

- Archive `add-open-tab-corners` before this change.
- Archive this change after its checks pass and the work is reviewed.
