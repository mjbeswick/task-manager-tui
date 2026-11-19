# Tasks

## 1. Shared hint line

- [x] 1.1 Add `chrome::tab_hints(app)` returning the line for each tab, and verify unit tests for all seven tabs including the Fewer/More and Show/Hide variants
- [x] 1.2 Add `chrome::hint_area` and `chrome::draw_hint` with clipping, and verify tests for a wide line, a clipped line, and a content area shorter than four rows
- [x] 1.3 Verify the line is muted on the surface and matches the spacing of the existing Services hint

## 2. List tabs

- [x] 2.1 Reserve the hint row in Processes and verify the body, scrollbar geometry, and visible count use the shrunken height
- [x] 2.2 Do the same in Details, Users, and App history, and verify each render test finds its hint on the last content row
- [x] 2.3 Replace the private hint drawing in Services and Startup with the shared line, and verify the scope note and enable/disable wording still follow the selected row
- [x] 2.4 Verify no table row is drawn under the hint line on any list tab

## 3. Performance tab

- [x] 3.1 Reserve the hint row before splitting the rail and panel, and verify the graph still fits its minimum height and the hint reads on the last content row

## 4. Documentation and checks

- [x] 4.1 Note in the README that every tab names its keys in the bottom line and verify the existing tab descriptions still read correctly
- [x] 4.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-tab-shortcut-hints --strict`, and record the output

## Workflow follow-up

- Archive after the chart changes, so the render snapshots settle once.
- Archive this change after its checks pass and the work is reviewed.
