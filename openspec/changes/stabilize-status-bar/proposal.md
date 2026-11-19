# Proposal

## Why

The status bar joins metrics at their natural widths and right-aligns the result. When CPU
goes from `9.0%` to `10.0%`, or disk goes from `999 B/s` to `1.0 KB/s`, every part of the
bar shifts sideways. Notices can also push the metrics, so the bar jitters on almost every
sample.

## What Changes

- Each metric occupies a fixed-width field, so the summary's width stays constant.
- The position counter pads its numerator to the width of the largest row count.
- The bar is split into fixed left, middle, and right zones. A notice is truncated inside
  the middle zone and never moves the metrics.
- Metrics are still dropped on narrow terminals, but the choice is made from the widest
  possible summary rather than the current values.
- No metric is added, removed, or sampled differently.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: adds a requirement that the status bar keeps a stable layout as values
  change.

## Impact

- `src/ui/chrome.rs`: metric formatting, zone layout, and the position counter.
- `src/ui/format.rs`: fixed-width helpers if the formatting does not stay local.
- `openspec/specs/tui-shell/spec.md`.
