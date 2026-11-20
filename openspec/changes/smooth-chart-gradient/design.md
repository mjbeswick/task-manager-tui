# Design

## Context

- `fill_column` and `blocks_column` colour the topmost occupied cell with raw `edge` when
  gradient is on and `is_top` is true.
- Rows below use `ramp`, which is most faded at the top.
- The jump from faded body to raw tip creates the highlight line.

## Decisions

### Let the ramp own the tip

Remove `is_top` from the colour decision when a gradient is active:

```text
colour = if coarse || !gradient { edge } else { ramp(edge, bg, 0.05 + rise * 0.95) }
```

The tip keeps its partial-height glyph, so the silhouette stays visible without a colour
rim. The gradient is now monotonic: base bright, top dim.

*Alternative.* Blend the tip halfway between the top stop and raw edge. Rejected: it
still produces a local maximum, just a smaller one.

## Risks / Trade-offs

- [Edge visibility] The tip no longer pops in colour → The cell shape and the top stop's
  contrast against the surface carry the edge.
