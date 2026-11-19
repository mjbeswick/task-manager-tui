# Tasks

## 1. Style plumbing

- [x] 1.1 Add `ChartStyle::Blocks` with label, key, parser, and a four-style cycle, and verify the model test covers the new wrap
- [x] 1.2 Fix the Appearance back-cycle to use the style list length, and verify a test that it returns from Filled to Blocks

## 2. Blocks renderer

- [x] 2.1 Compute per-column levels once and pass the older neighbour to a blocks column renderer, and verify a render test that a rising edge uses `▛` and a falling edge uses `▟`
- [x] 2.2 Use `◢`/`◣` for one-step slopes and eighth blocks for flat tips, and verify each glyph appears for its case
- [x] 2.3 Fall back to the eighths renderer in ASCII mode, and verify only ASCII symbols are drawn

## 3. Documentation and checks

- [x] 3.1 Add Blocks to the README style list, and verify it matches the Appearance dialog
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-blocks-chart-style --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
