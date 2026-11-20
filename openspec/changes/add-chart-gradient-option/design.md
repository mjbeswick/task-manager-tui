# Design

## Context

- Appearance currently has Theme, Chart style, and Chart colour rows.
- `ramp` is called from `fill_column`, `blocks_column`, and the braille flush.
- `GraphSpec` and `PlotSpec` carry the chart colour, style, and theme.

## Decisions

### One persisted boolean

`Settings.chart_gradient: bool` defaults to true. `App` mirrors it, and the Appearance
row toggles it with the existing left/right controls. The effects layer persists it like
the style and colour.

### Thread the flag through the plot specs

`GraphSpec` and `PlotSpec` gain `gradient: bool`. A `FillSpec { edge, bg, theme, gradient }`
bundles the arguments for `fill_column`, `blocks_column`, and the braille flush, keeping
the argument counts below the lint limit.

When `gradient` is false, each of those functions returns `edge` instead of calling
`ramp`; the coarse/ASCII rules are unchanged.

*Alternative.* A global or thread-local flag. Rejected: hidden state in the renderer and
hard to test.

## Risks / Trade-offs

- [Default] Gradients stay on unless the user changes them → Existing visuals are
  unchanged for anyone who never opens Appearance.
- [Appearance height] The dialog gains a row → It grows by one line and still fits the
  minimum terminal.
