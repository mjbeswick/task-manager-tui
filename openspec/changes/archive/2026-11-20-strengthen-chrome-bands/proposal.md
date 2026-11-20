# Proposal

## Why

The chrome and status backgrounds sit within a few RGB of the window background in most
themes. On the dark palettes the difference is 0-5 per channel, so the first row (menu) and
last row (status) read as the same surface as the window.

## What Changes

- Retune `chrome` and `status` for all ten themes with a perceptible step from `bg`.
- Keep chrome and status distinct from each other and from `surface`.
- Add a test that every theme's bands differ from the background by a minimum channel step.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: chrome and status bands must be visibly separated from the background.

## Impact

- `src/theme.rs`: ten palettes and one test.
