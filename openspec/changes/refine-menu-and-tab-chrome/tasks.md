# Tasks

## 1. Chrome layout

- [x] 1.1 Add a one-row tab rule between the tab strip and the content, keep `MIN_HEIGHT` at 12, and verify a render test that all five bands are present and the minimum frame still draws content
- [x] 1.2 Keep the menu row on the chrome background with the filter field on a surface inset, and verify a render test that menu and tab backgrounds differ
- [x] 1.3 Drop the filled active-tab block and draw the active label in the accent colour, and verify a render test that the active cell background is the window background
- [x] 1.4 Draw a centred underline segment under every tab, accent for the active tab and border for the rest, and verify render tests at 120 and 60 columns that segments are separated and centred

## 2. Menu dismissal

- [x] 2.1 Publish a menu-panel target over the dropdown border and empty area, and verify a mouse test that the panel and its items are distinguishable from the area outside
- [x] 2.2 Close an open menu on any click outside the label, panel, and items, consume that click, and verify tests that a row, tab, header, and filter click leave their underlying state unchanged
- [x] 2.3 Keep the menu open for a click on the panel border or empty space and verify the menu index is unchanged
- [x] 2.4 Close the menu on a click with no target and verify no action runs

## 3. Documentation and checks

- [x] 3.1 Note the click-away dismissal and the tab underline in the README mouse and tabs sections and in the help dialog, and verify the help panel still fits at 120 columns
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate refine-menu-and-tab-chrome --strict`, and record the output

## Workflow follow-up

- Archive `add-theme-collection` and `add-mouse-scrolling` before this change, so the tab-strip and mouse wording lands in order.
- Archive this change after its checks pass and the work is reviewed.
