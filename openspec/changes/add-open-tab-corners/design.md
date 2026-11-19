# Design

## Context

- The rule row under the active tab is cleared to the window background.
- The left and right ends of that cleared span currently carry a `│` in the accent.

## Decisions

### Put `└` and `┘` at the span ends

Swapping the two `│` cells for `bl` and `br` joins each side to the rule direction while
leaving the middle blank. Nothing else moves: the top border, label, span width, rule,
and hit rect are unchanged.

*Alternative.* Keep the straight sides and add the corners outside the span. Rejected: it
would widen the box by two cells and shift every following label.

### ASCII follows the glyph table

`bl` and `br` are `+` in ASCII mode, so the same code draws an ASCII-correct box without a
branch.

## Risks / Trade-offs

- [Corner ambiguity] `└` and `┘` point into an empty span → That is the open-tab shape;
  the rule resuming outside the right corner makes the reading unambiguous.
