# Tasks

## 1. Viewport state and clamping

- [ ] 1.1 Add `viewport: usize` to the Processes, Details, Services, Startup, Users, and App history states. Verify: unit tests read the viewport after a sample and after a tab switch.
- [ ] 1.2 Add `clamp_viewport(len, visible)` and call it on sample, filter, sort, expand, and tab switch. Verify: tests shrink a 300 row list to 5 while the viewport is at 200 and assert the viewport lands at 0.
- [ ] 1.3 Draw rows from the viewport instead of the selected index in every list tab. Verify: existing goldens are unchanged with the selection in view.

## 2. Scrollbar

- [ ] 2.1 Implement the pure geometry function for thumb height and top. Verify: unit tests cover 300 rows in 20 visible, one overflowing row, and a list that fits.
- [ ] 2.2 Draw the one-column track and thumb, reserving the column only when the bar is shown. Verify: goldens show a full-width table when it fits and a one-column-narrower table when it does not.
- [ ] 2.3 Keep the bar out of layouts below 60 columns. Verify: a 50 column golden shows no bar and the `12/300` counter.

## 3. Wheel routing

- [ ] 3.1 Publish `ScrollZone` regions for each list and the Performance rail from the frame. Verify: `tests/mouse.rs` finds a scroll zone over the rows, the rail, and a menu.
- [ ] 3.2 Route wheel events through the hit map: three rows per event for tables, one step for the rail, one item for an open menu, nothing elsewhere. Verify: tests dispatch a wheel at each target and assert only that region changed.
- [ ] 3.3 Accumulate bursts without dropping events. Verify: a test sends five events and asserts a fifteen row move.

## 4. Drag and page

- [ ] 4.1 Add drag state to the main loop and map pointer travel to viewport movement. Verify: a test drags the thumb across the track and asserts the viewport reaches the computed fraction and that selection is unchanged.
- [ ] 4.2 End the drag on release and ignore later motion. Verify: a test releases, moves, and asserts the viewport no longer changes.
- [ ] 4.3 Page on track clicks above and below the thumb by `visible - 1`, clamped. Verify: tests at the top, middle, and bottom pages.
- [ ] 4.4 Publish thumb and track targets separately so a press on the thumb starts a drag and a press on the track pages. Verify: hit tests distinguish the two rects.

## 5. Keyboard and samples

- [ ] 5.1 Keep keyboard movement selection-first and always bring the selection into view. Verify: tests move the selection after a pointer scroll and assert it appears at the nearest edge.
- [ ] 5.2 Clamp the viewport when a sample removes rows. Verify: a test removes the tail of the list while scrolled to the end and asserts rows remain visible.

## 6. Documentation and checks

- [ ] 6.1 Document wheel routing, drag, page clicks, and the narrow fallback in the README and the help dialog. Verify: every documented gesture has a test named after it.
- [ ] 6.2 Run the release checks. Verify: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-mouse-scrolling --strict` pass.
- [ ] 6.3 Verify the scrollbar in a PTY with a real drag. Verify: an SGR drag moves the view and release stops it.

## Workflow follow-up

- Archive `build-win10-task-manager` first, then this change, so the `list-scrolling`
  capability lands on a clean spec tree.
- Remove the old wheel scenario from `tui-shell` when the first change is archived.
