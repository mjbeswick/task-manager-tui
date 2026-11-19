# Design

## Context

- The rule runs left to right along the row under the labels.
- The active tab's sides end on that row.
- The current glyphs are `bl` on the left and `br` on the right.

## Decisions

### Use the outward junction at each side

A corner joins one horizontal and one vertical direction:

- Left side + rule coming from the left: horizontal-left with vertical-up is `┘` (`br`).
- Right side + rule continuing right: horizontal-right with vertical-up is `└` (`bl`).

So the code uses `br` at the left end and `bl` at the right end. The middle stays blank,
and the rule outside the box is unchanged.

*Alternative.* Keep `bl`/`br` and accept the closed look. Rejected: it reads as a box
with a hole, which is what the user sees as wrong.

## Risks / Trade-offs

- [Glyph names] `bl`/`br` are named for a closed box, so the outward use looks inverted
  in code → A comment states the junction rule beside the swap.
