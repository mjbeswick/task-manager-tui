# Tasks

## 1. Tab strip

- [x] 1.1 Replace the vertical separator with a single background-coloured space, and verify a render test that the tab row contains no `│`
- [x] 1.2 Draw one continuous rule across the strip and repaint the active tab's span in the accent, and verify a test that the rule covers the full width and the active span changes colour
- [x] 1.3 Verify the compact strip at 60 columns keeps the continuous rule and the active span

## 2. Documentation and checks

- [x] 2.1 Update the README tab-line wording, and verify it matches the rendered strip
- [x] 2.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate simplify-tab-strip --strict`, and record the output

## Workflow follow-up

- Archive `refine-menu-and-tab-chrome` before this change.
- Archive this change after its checks pass and the work is reviewed.
