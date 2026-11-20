# Design

## Context

- `ramp(edge, bg, rise)` mixes toward `bg` up to 68%.
- On Phosphor the surface is near-black, so a bright green fades well.
- On Solarized Light the surface is near-white and the teal series fades toward a colour
  close to its own brightness, so the step is small.

## Decisions

### Target the contrasting end of the series colour

`ramp(edge, rise)` computes `target = if contrast(edge, Black) >= contrast(edge, White) { Black } else { White }`
and mixes toward it: midpoint 20%, top 55%. The background is no longer an input.

This guarantees a measurable luminance change for any series colour, independent of the
theme. Bright green darkens, near-black Mono on a light theme lightens.

*Alternative.* Mix toward the theme's `text` colour. Rejected: text can be close to the
series in luminance too.
*Alternative.* A per-theme gradient strength. Rejected: ten knobs for one effect.

## Risks / Trade-offs

- [Direction varies] Some series lighten, others darken → The gradient is relative to the
  series, which is what keeps it visible.
- [Dark themes] A very dark series lightens instead of darkening → Still visible and still
  reads as a fade.
