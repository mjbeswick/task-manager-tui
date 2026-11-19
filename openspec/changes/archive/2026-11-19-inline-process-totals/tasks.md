# Tasks

## 1. Collapse the header

- [x] 1.1 Make the column header a single label row plus rule at every height, and verify render tests at 120x36 and 80x14 that the header is one row and the table starts one row higher

## 2. Inline totals

- [x] 2.1 Draw the totals on the Apps heading row when it is the first visible row, using the row background and bold figures, and verify a render test that the figures sit under their columns and the heading text is intact
- [x] 2.2 Omit the totals when the viewport is scrolled past Apps or the filter hides it, and verify both cases
- [x] 2.3 Verify only the columns present in the layout receive a total at 80 columns

## 3. Documentation and checks

- [x] 3.1 Update the README Processes description, and verify it still matches the rendered header
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate inline-process-totals --strict`, and record the output

## Workflow follow-up

- Archive after `add-tab-shortcut-hints`, which changed the content height.
- Archive this change after its checks pass and the work is reviewed.
