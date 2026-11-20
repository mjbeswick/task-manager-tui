# Design

## Context

- `draw_tabs` draws the label right-aligned when there is free space.
- The terminal title is set in `main` and follows the active tab.

## Decisions

### Delete the block

Remove the `put_right` call and its guard. The tab strip ends after the last tab.

*Alternative.* Move it elsewhere. Rejected: the terminal title already covers it.

## Risks / Trade-offs

- [Identification] The window no longer shows the name in-app → The terminal title and the
  tui-shell specification still name the application.
