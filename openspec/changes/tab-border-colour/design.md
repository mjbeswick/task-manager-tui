# Design

## Context

- The active box and its corners are drawn with one `accent` style.
- The rule is drawn with a `border` style.

## Decisions

### Use the border colour for the frame

Introduce a `frame` style with `fg(theme.border)` on the window background and use it for
the top border, sides, and bottom corners. The label keeps the existing accent style.

*Alternative.* Recolour the rule to the accent. Rejected: the rule runs the whole width
and would become the loudest element.

## Risks / Trade-offs

- [Less emphasis] A border-coloured outline is quieter → The accent label still marks the
  active tab, and the open shape plus corners carry the geometry.
