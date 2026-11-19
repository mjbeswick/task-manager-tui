# Proposal

## Why

The palette list covers Windows, Nord, Gruvbox, and two CRT moods. Users who live in
Tokyo Night, Catppuccin, Solarized, or Rosé Pine have no matching option, so the app is
the only terminal window that does not follow the rest of their desktop.

## What Changes

- Add four themes: Tokyo Night, Catppuccin Mocha, Solarized Light, and Rosé Pine.
- Keep the existing six and the fixed order, with the new four after Amber.
- Every theme supplies five per-resource chart colours, so Per resource follows the active
  palette instead of a hardcoded light/dark table.
- Extend the picker, `--theme`, and the README list; the cycling UI does not change.
- Every new palette carries the full role set and must pass the existing contrast suite
  in both truecolor and reduced depths.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the theme list grows from six to ten names.
- `performance`: Per resource resolves through the active theme's five chart colours.

## Impact

- `src/theme.rs`: four palettes, four enum variants, labels, keys, and a five-colour
  chart palette on every theme.
- `src/model.rs`: Per resource resolves through the theme palette.
- `src/cli.rs`: the error message lists the new keys.
- `README.md`: the theme list and `--theme` row.
- No renderer or layout change.
- Coordination: archive `add-theme-collection` before this change.
