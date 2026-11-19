# Design

## Context

- `draw_tabs` prefixes each label with `index + 1` and a space.
- Wide labels use `label()`, compact labels use the first four characters of
  `short_label()`.
- Hit rectangles and the box geometry derive from the rendered text width.

## Decisions

### Drop the prefix, keep the key map

The text becomes `"{label} "` or `"{short} "`. Everything downstream uses the new width,
so the box, corners, and hit rects follow automatically. The digit keys are handled in
`input.rs` and do not depend on the strip.

*Alternative.* Keep the digits only when wide. Rejected: the strip would change meaning
when it resizes.

## Risks / Trade-offs

- [Discoverability] New users lose the visual cue for 1-7 → The help dialog still lists
  the keys, and every key also works without a pointer.
