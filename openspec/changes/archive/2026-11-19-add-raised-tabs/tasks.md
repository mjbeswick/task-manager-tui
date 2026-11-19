# Tasks

## 1. Raised tab

- [x] 1.1 Add a tab-top row to the layout and start the strip one column in, and verify a render test that the labels keep their digits and the content still draws at the minimum height
- [x] 1.2 Draw the active tab's `┌─┐`, `│` sides, and `└─┘` bottom in the accent, and verify the corners and the rule's continuity at both sides
- [x] 1.3 Grow the active tab's hit rect to include both sides, and verify a click on either side selects it

## 2. Fallbacks

- [x] 2.1 Verify the compact strip at 60 columns draws the box around the shortened active label
- [x] 2.2 Verify that when the active tab is dropped for width, no box is drawn and the rule stays continuous
- [x] 2.3 Verify ASCII mode uses only ASCII box characters

## 3. Documentation and checks

- [x] 3.1 Update the README tab wording and note the extra chrome row, and verify it matches the rendered strip
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-raised-tabs --strict`, and record the output

## Workflow follow-up

- Archive `simplify-tab-strip` before this change.
- Archive this change after its checks pass and the work is reviewed.
