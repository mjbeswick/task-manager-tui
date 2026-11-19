# Proposal

## Why

The application ships two themes, both tied to the Windows 10 palette. Users who live in
a terminal all day expect a small curated set of palettes that suit their terminal, and
the current chrome has no visual separation between the menu bar, the tab strip, the
content, and the status bar.

## What Changes

- Six themes: Windows Light, Windows Dark, Nord, Gruvbox Dark, Phosphor, and Amber.
  Light and Dark stay the default pair; the other four are opt-in.
- Every theme carries the full role set, including row stripes, chrome, status, and a
  readable foreground for filled accent blocks.
- The menu row and tab strip share a `chrome` background; the status bar uses `status`.
  No new row is added.
- The Appearance dialog, `t`, `--theme`, and the settings file all cycle through the six
  themes. Unknown saved keys fall back to Windows Light.
- The terminal window title is set to `Task Manager - <tab>` and follows tab switches.
- The in-app help and README list the theme keys.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the Light and dark themes requirement becomes a six-theme collection with
  a picker, persistence, and contrast rules; a new requirement covers the chrome and
  status bands.

## Impact

- `src/theme.rs`: palettes, `ThemeName::ALL`, `on_accent`, `chrome`, `status`, and
  luminance-based 16-colour mapping.
- `src/ui/chrome.rs`: chrome and status backgrounds, filter inset, title.
- `src/app.rs`, `src/store/settings.rs`, `src/cli.rs`: picker and persistence.
- `src/main.rs`: terminal title on start and tab switch.
- `README.md`, `src/ui/dialog.rs` help rows.
