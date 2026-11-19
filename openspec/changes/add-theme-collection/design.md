# Design

## Context

See `proposal.md` - Why.

- `ThemeName` has two variants. `Theme` is a flat struct of RGB roles, with `LIGHT` and
  `DARK` constants and `from_name`.
- Settings store the theme as a string; `cli.rs` accepts `light` or `dark`; `t` and the
  Appearance dialog toggle between the two.
- `ansi16` chooses its light or dark named variant by testing
  `theme.name == ThemeName::Light`.
- Hardcoded `Color::White` appears on accent-filled menu items and dialog rows.
- The status bar is drawn on `surface` today, and the menu and tab rows on `bg`.

## Goals / Non-Goals

**Goals:**

- One table drives the palette list, the picker, and the CLI.
- Every role in every theme passes the same contrast suite.
- No hardcoded colour survives in the chrome.

**Non-Goals:**

- User-defined themes or palette editing.
- Per-theme glyph or font choices.
- Auto-switching with the terminal's own light or dark mode.
- Changing the balance of the status bar; that is a separate change.

## Decisions

### A table-driven theme list

`ThemeName::ALL` holds the six names in a fixed order: Light, Dark, Nord, Gruvbox,
Phosphor, Amber. `label()`, `key()`, `parse()`, `next()`, and `previous()` all derive
from that list. `Theme::from_name` maps each name to its constant palette.

The order puts the two Windows themes first, so `t` from the default reaches a familiar
look after one press and the CRT themes are opt-in.

### Full palettes, including chrome and status

Each palette defines every role:

| Role | Purpose |
|---|---|
| `bg`, `surface`, `surface_alt` | window, content, striped rows |
| `chrome`, `status` | menu and tab band, bottom bar |
| `text`, `muted`, `border` | foregrounds and rules |
| `accent`, `on_accent` | filled blocks and their text |
| `selection`, `heat[5]` | selected rows, resource cells |
| `good`, `warn`, `bad` | status tones |
| `graph_fill`, `graph_grid` | charts |

`on_accent` replaces every hardcoded `White`. Light accent-filled blocks on Nord and
Amber need dark text, so a fixed white would fail the contrast suite.

### 16-colour mapping keyed by luminance

`ansi16` currently branches on the theme's name. That breaks with six themes. It will
pick the light or dark named variant from the background's relative luminance, using the
existing `contrast()` helper. The role table stays hand-written; only the choice of
column becomes computed.

### Picker, CLI, and fallback

- `t` and the Appearance dialog call `next()`/`previous()`.
- `--theme` parses through `ThemeName::parse`; the error lists all six keys.
- `settings.theme` stays a string. An unknown value falls back to Light and adds a
  notice through the existing `App::new` notices path.

*Alternative.* A theme dropdown. Rejected: the Appearance dialog already uses a cycling
row, and six entries are quick to reach with either arrow.

### Chrome bands and the window title

`draw_menu_bar` and `draw_tabs` fill with `chrome`; the filter field keeps its `surface`
inset. `draw_status_bar` fills with `status`. The `Task Manager` label moves to the
right of the chrome band for every tab, not just Performance.

The terminal title is set in `main`: once after the terminal starts, then whenever the
active tab changes. It reads `Task Manager - Processes` and uses
`crossterm::terminal::SetTitle`, written with the same `execute!` already used for the
alternate screen.

## Risks / Trade-offs

- [Palette drift] Six hand-written palettes diverge in role coverage → The struct does
  not compile until every field is set, and the contrast suite runs over `ALL`.
- [Reduced depth] Nord and Gruvbox collapse to similar greys at 16 colours → The
  luminance-based mapping keeps text and selection apart; other roles may merge, which is
  acceptable at 16 colours.
- [Hidden dependency] The terminal title is not visible in tests → Verify by running the
  binary and switching tabs, not by unit test alone.
- [Notice text] An unknown saved theme adds a notice that may crowd the status bar →
  Existing truncation applies, and the status-bar change keeps the right block fixed.
