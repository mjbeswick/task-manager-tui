# Proposal

## Why

Wide tables are hard to track across seven columns, and every row currently reads the
same weight. A process pinning the machine looks like any other row, and a stopped
service looks like a running one. Subtle row stripes and a small colour vocabulary fix
both without tinting whole rows.

## What Changes

- Every list table gains alternating row stripes: Processes, Details, Services, Startup,
  Users, and App history.
- The Processes heat ramp becomes hotter at its top end and its thresholds are retuned,
  so the highest step reads as a warning rather than another amber.
- Process status words gain colour roles: Not responding warns, Suspended and Stopped
  warn, and the default states stay plain.
- Service status words gain colour roles: Running good, Stopped muted.
- Selection still overrides both the stripe and the status colour.
- Truecolor and 256-colour terminals show the stripes; 16-colour terminals drop them
  rather than fake a shade.
- The tab strip gains its Windows 10 numbers, and the active tab becomes a filled
  accent block instead of an underline.

## Capabilities

### New Capabilities

- `table-readability`: alternating row stripes, resource heat escalation, status colour
  roles, and the contrast guarantees that keep every combination readable.

### Modified Capabilities

- `tui-shell`: the tab strip requirement changes from an accent underline to numbered
  labels with a filled active tab.

## Impact

- `src/theme.rs`: a new surface role, retuned heat steps, hotter top colours, and
  contrast tests.
- `src/ui/processes.rs`, `details.rs`, `services.rs`, `startup.rs`, `users.rs`,
  `app_history.rs`: stripe every body row and colour status words.
- `src/ui/chrome.rs`: numbered tabs and the filled active tab.
- `src/model.rs`: a mapping from status to a theme role.
- `README.md`: a short note on what the stripes, colours, and tab numbers mean.
