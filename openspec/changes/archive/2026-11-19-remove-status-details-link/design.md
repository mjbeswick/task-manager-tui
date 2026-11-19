# Design

## Context

- `draw_status_bar` builds a `Fewer details` or `More details` label, draws it at the
  left, and records `Target::FewerDetails`.
- `left_width` reserves space for that label before the notice and the fixed metric block.
- The keyboard binding `f`, the View menu item, and the new hint line all remain.

## Decisions

### Delete the label, its width, and its target

Remove the action string, the `show_action` flag, the action parameter from
`left_width`, the drawing branch, and the `hits.add`. Remove `Target::FewerDetails`, its
`App::click` arm, and the crate-internal hit test that used it.

The status bar left side is now the position counter alone. The notice middle zone has
slightly more room, and the metric block stays anchored on the right.

*Alternative.* Keep the label but stop recording a hit. Rejected: a label that looks
clickable but is not is worse than no label.

## Risks / Trade-offs

- [Discoverability] The toggle is no longer in the status bar → The hint line on the
  Processes tab names it with its key, and the View menu keeps the item.
