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

- `tui-shell`: the Mouse support scenario for the wheel now scrolls the viewport under
  the pointer instead of moving the selection.

## Impact

- `src/ui/hits.rs`: a wheel zone and scrollbar regions in the hit map.
- `src/app.rs`: a viewport offset per list tab and clamping on every sample.
- `src/ui/*.rs`: scrollbar drawing and viewport-based row windows.
- `src/main.rs`: wheel dispatch by target instead of always moving the selection.
- `openspec/specs/`: a new `list-scrolling` capability, and the updated `tui-shell`
  mouse scenario after archive.
- Coordination: `polish-visual-design` also edits `tui-shell` and the list headers.
  That change archives first; this one follows, and the scrollbar uses whatever rows
  area the new two-row header leaves.
