# Tasks

## 1. Theme chart palettes

- [x] 1.1 Add a five-colour chart palette to every theme and verify a test that the five differ within a theme and each reaches 2:1 against the surface
- [x] 1.2 Resolve Per resource through the active theme's palette and verify a test that switching themes changes all five colours
- [x] 1.3 Verify Teal, Blue, Orange, Magenta, Mono, and Theme still override the palette

## 2. New themes

- [x] 2.1 Add Tokyo Night, Catppuccin Mocha, Solarized Light, and Rosé Pine with the full role set and append them to the theme order, and verify the parse and cycle tests cover ten names
- [x] 2.2 Build the `--theme` error list from the theme table and verify a typo names every key
- [x] 2.3 Verify the contrast suite passes for all ten themes at all three depths

## 3. Documentation and checks

- [x] 3.1 Update the README theme list and the Per resource description, and verify they match the picker
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-more-themes --strict`, and record the output

## Workflow follow-up

- Archive `add-theme-collection` and `add-chart-resolution` before this change.
- Archive this change after its checks pass and the work is reviewed.
