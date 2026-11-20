# Design

## Context

- `dialog::draw` calls `dim` over the whole frame before painting the panel.
- The panel already fills its rect with `surface` and draws a border.

## Decisions

### Delete the pass

Remove the `dim(buf, area, theme)` call and the function. Nothing else changes.

*Alternative.* Keep a weaker dim. Rejected: the user asked to drop it, and the next
request would likely be about another colour changing.

## Risks / Trade-offs

- [Focus] The background is as bright as the dialog → The panel border and surface still
  separate it, and modals capture input.
