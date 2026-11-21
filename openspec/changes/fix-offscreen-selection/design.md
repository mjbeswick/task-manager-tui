# Design

`apply_snapshot` records, per list tab, whether the selection sat inside the drawn
viewport. After the syncs and the clamp it scrolls just enough to keep those selections
in view. `keep_selection_visible` takes the tab as a parameter so it can run for tabs
other than the active one.

A tab with no frame drawn yet has no viewport, so it is not followed. The first frame
starts at the top with the first row selected.

Rejected: pinning an unchosen selection to the top row. It needs a per-tab flag set from
every navigation path, and it still leaves a chosen row free to sink out of view.
