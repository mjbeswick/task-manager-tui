# Tasks

- [x] Restore the strip in `src/ui/chrome.rs` and drop the button and dropdown.
- [x] Add full, short, and mini labels to `Tab` and pick a tier from the width.
- [x] Remove `TabMenuState`, `app.tab_menu`, `Action::TabMenuOpen`, and their dispatch.
- [x] Remove the `Alt+T` binding, its help row, and the tab-menu hit targets.
- [x] Rewrite the tab-menu tests as strip tests; assert all seven labels at 40, 60, and 120 columns.
- [x] Regenerate snapshots; update README prose and help.
- [ ] Regenerate screenshots. Not run: both PNGs render at 120 columns, where the full tier is unchanged, and the PNG step needs a browser capture.
- [x] Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate restore-tab-strip --strict`.
- [ ] Deploy to this Mac, hackberry, and rock-5b; push `main`.
