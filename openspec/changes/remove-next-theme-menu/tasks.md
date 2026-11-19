# Tasks

## 1. Remove the entry

- [x] 1.1 Drop Next theme from the Options menu and remove `MenuItem::Theme` with its dispatch arm, and verify the options menu renders only update speeds
- [x] 1.2 Verify `t` and the Appearance dialog still change the theme

## 2. Checks

- [x] 2.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate remove-next-theme-menu --strict`, and record the output

## Workflow follow-up

- Archive `add-more-themes` before this change.
- Archive this change after its checks pass and the work is reviewed.
