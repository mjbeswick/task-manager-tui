# Proposal

## Why

The wheel currently scrolls whichever tab is active, no matter where the pointer is.
On the Performance tab it can scroll a list the user is not looking at, and there is no
visible scroll control anywhere. In a process list of several hundred rows the only
position cue is the `12/365` counter, and there is no way to jump a long distance without
holding a key.

## What Changes

- Wheel events are routed by what is under the pointer: a table, the Performance rail,
  an open menu, or nothing.
- A one-column scrollbar appears at the right edge of any list longer than its viewport,
  with a proportional thumb.
- The thumb can be dragged, and the track above or below it pages the view.
- Each list gains an explicit viewport offset, separate from the selection, so pointer
  scrolling does not move the highlight.
- On terminals narrower than 60 columns the scrollbar is dropped and the existing
  position counter remains.
- No new dependency: rows, thumb, and track are drawn and hit-tested by the same frame.

## Capabilities

### New Capabilities

- `list-scrolling`: pointer-routed wheel events, the scrollbar, drag and page gestures,
  the viewport model, and the narrow-terminal fallback.

### Modified Capabilities

None. Existing wheel behavior is restated under the new capability; the old requirement
will be removed when the first change is archived, before this one.

## Impact

- `src/ui/hits.rs`: a wheel zone and scrollbar regions in the hit map.
- `src/app.rs`: a viewport offset per list tab and clamping on every sample.
- `src/ui/*.rs`: scrollbar drawing and viewport-based row windows.
- `src/main.rs`: wheel dispatch by target instead of always moving the selection.
- `openspec/specs/`: a new `list-scrolling` capability after archive.
