# Design

## Context

See `proposal.md` for motivation and `specs/list-scrolling/spec.md` for behavior.

Today each table computes its scroll window during drawing from the selected row.
There is no stored viewport, so the selection is the scroll position. Frames already
record clickable regions in `ui::hits::HitMap`, and `main` already translates clicks
into actions. Wheel events bypass that map and always move the selection.

## Goals / Non-Goals

**Goals**

- One scrolling model shared by all six list tabs.
- Pointer scrolling that never fights the keyboard.
- A visible, draggable position control.
- No new dependency and no platform-specific mouse handling.

**Non-Goals**

- Horizontal scrolling. Columns already adapt or drop.
- Inertial or smooth scrolling. Terminals deliver discrete events.
- Scrollbars on the Performance graphs or in dialogs.
- Mouse-driven text selection.

## Decisions

### A stored viewport, not selection-derived scroll

Each list tab gains `viewport: usize`, the index of its first visible row. Drawing
windows rows from `viewport`, not from `selected`. Keyboard movement still moves
`selected` and then clamps `viewport` so the selection is visible, which keeps the
existing feel. Wheel, drag, and page gestures change `viewport` only.

*Alternatives.* Keeping scroll derived from selection is less state, but then dragging
a scrollbar would drag the highlight through hundreds of rows, and a click on the track
would select a row the user did not aim at. The ratatui `Scrollbar` widget draws well
but owns neither hit-testing nor drag state, so it would still need this viewport.

### Scrollbar geometry is a pure function

```
track_height = rows_area.height
thumb_height = max(1, track_height * visible / len)
thumb_top    = round((track_height - thumb_height) * viewport / (len - visible))
```

The thumb uses `█` and the track a dim `│`, both from the theme, so both palettes stay
readable. A list that fits draws nothing and keeps the column for data.

One column is reserved only when the bar is drawn. The other columns are laid out from
`area.width - 1` in that case, so nothing shifts by a fraction of a column.

### The frame publishes its scroll regions

`HitMap` gains three targets: `ScrollZone(Zone)`, `ScrollbarThumb(Zone)`, and
`ScrollbarTrack(Zone)`, where `Zone` is a tab or `Rail`. Wheel dispatch asks the map
what is under the pointer; no target means no scroll. This is the same mechanism that
already makes clicks land on tabs and rows, so the wheel cannot disagree with the
drawing.

The menu wheel is handled without a zone: while a menu is open, any wheel over the
dropdown moves its highlight, and the dropdown is already registered as menu items.

### Drag lives in the main loop

`main` keeps `drag: Option<Drag { zone, start_row, start_scroll }>`. A press on a thumb
records the pointer row and current viewport; mouse movement maps row deltas to row
deltas by `delta * (len - visible) / usable_track`; release clears it. Because the drag
target survives redraws, the thumb keeps following the pointer even as the list
re-renders.

### Page clicks keep one row of context

A track click above or below the thumb moves `visible - 1` rows, the same convention as
PageUp and PageDown. It clamps at zero and at `len - visible`.

### Clamping is central

Every mutation path - sample, filter, sort, expand, tab switch - ends in
`clamp_viewport(len, visible)`. Without a single clamp the stored viewport outlives the
rows it described, which is the classic blank-screen bug.

### Narrow fallback

Below 60 columns the scrollbar is skipped and the existing `12/300` counter is the only
position cue. It costs a column, which is the wrong trade at that width.

## Risks / Trade-offs

- **Wheel zones that do not cover the row area** → register the zone from the same rect
  used to draw rows, and test that a wheel anywhere in it scrolls.
- **Thumb math at the boundary** → `len - visible` is zero when everything fits; the
  early return covers it, and tests include exactly one overflowing row.
- **Viewport drift after a filter** → one clamp function called from every mutator, with
  tests that shrink the list under the viewport.
- **A drag that starts on the thumb but ends off the bar** → the drag state, not the
  pointer position, decides where movement applies.
- **Terminals that report one wheel event per notch** → three rows per event, matching
  the existing behavior, so slow wheels stay usable.

## Migration Plan

1. Add the viewport fields and clamping, with no behavior change in drawing.
2. Switch drawing to the viewport and add a `--` hidden scrollbar behind the same
   geometry tests.
3. Publish zones and route the wheel.
4. Add drag and track clicks.
5. Draw the bar and reserve the column.
6. Update goldens, README, and the help table.

Rollback is one revert: the scrollbar and viewport are additive and no persisted data
changes.

## Open Questions

None blocking. Thumb color intensity can be tuned after seeing it on a real terminal.
