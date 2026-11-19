# Design

## Context

See `proposal.md` - Why.

- `draw_status_bar` builds `Vec<String>` parts, joins them with three spaces, and prints
  the result right-aligned. Width therefore tracks the current values.
- The position counter is `selected + 1 / len` with no padding.
- A notice is drawn at the cursor after the left items when the remaining space exceeds
  20 columns; it is not given a fixed zone.
- Metrics are dropped from the end until the bar fits.

## Goals / Non-Goals

**Goals:**

- Stable geometry for a given terminal width.
- No change to what the metrics mean or how often they update.
- No new setting.

**Non-Goals:**

- Slower refresh or value smoothing.
- Sparklines or graphs in the status bar.
- Reordering the metrics.

## Decisions

### Fixed-width metric fields

Each metric is formatted as `label`, a space, and a right-aligned value in a fixed field:

| Metric | Value field |
|---|---|
| CPU | 6 |
| Memory | 6 |
| Disk | 11 |
| Network | 11 |
| GPU | 6 |

The widths cover the largest value each formatter produces (`100.0%`, `1023.9 MB/s`,
`999.9 Gbps`). Metrics are joined with three spaces, so the summary has one constant
width. The right edge is unchanged; the left edge of the block no longer moves.

*Alternative.* Keep natural widths and reserve the maximum. Rejected: the block would
contain dead space that changes with the unit anyway.

### Fixed position field

The counter is `format!("{:>4}/{:>4}", selected + 1, len)`. Four digits cover any process
list; past that the field grows once. The action label keeps its column.

### Three fixed zones

`left` is the action label plus the position counter. `right` is the metric block, placed
first so nothing can push it. `middle` is whatever remains, and the notice is truncated
into it. If `middle` is narrower than a word, the notice is dropped rather than overlapping.

### Drop order uses the fixed widths

The existing loop that pops metrics from the end now measures the fixed-width strings.
Because those widths no longer depend on the values, a given terminal width always keeps
the same set of metrics.

## Risks / Trade-offs

- [Dead space] Fixed fields leave gaps when values are small → Right alignment puts the
  gap beside the label, which reads as a column rather than a hole.
- [Very large values] A value beyond its field widens the bar → The formatters cap at the
  next unit, so this needs a unit overflow that does not occur in practice.
- [Notice overlap] A long notice is truncated more aggressively → The alternative is a
  moving metric block, which is the problem being fixed.
