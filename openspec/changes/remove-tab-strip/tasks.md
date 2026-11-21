# Tasks

- [x] Remove the strip, active-tab box, and fit check from `src/ui/chrome.rs`; re-layout the window as menu, rule, content, status.
- [x] Keep the rule row under the menu bar and skip the strip's two rows.
- [x] Re-anchor the `Alt+T` dropdown to the left edge under the rule.
- [x] Drop `Target::Tab` and `Target::TabButton` from hits, action dispatch, and mouse tests.
- [x] Retarget the strip unit tests to the new layout; add one asserting no tab labels are drawn.
- [x] Regenerate golden frames and update README prose.
- [x] Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate remove-tab-strip --strict`.
