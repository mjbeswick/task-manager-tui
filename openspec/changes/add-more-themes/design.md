# Design

## Context

See `proposal.md` - Why.

- `ThemeName::ALL` drives the picker, `t`, `--theme`, and the contrast tests.
- `Theme` requires a full role set; missing a role is a compile error.
- `ansi16` maps roles by background luminance, so new palettes need no branch.

## Decisions

### Append four variants after Amber

Order: Light, Dark, Nord, Gruvbox, Phosphor, Amber, Tokyo Night, Catppuccin Mocha,
Solarized Light, Rosé Pine. `next`/`previous` wrap over the ten.

*Alternative.* Interleave light and dark palettes. Rejected: cycling would lose the
current "Windows pair first, then moods" grouping.

### Palette summary

| Theme | bg | surface | chrome | text | muted | accent | on_accent |
|---|---|---|---|---|---|---|---|
| Tokyo Night | `#1A1B26` | `#1F2335` | `#16161E` | `#C0CAF5` | `#7AA2F7` | `#7AA2F7` | `#1A1B26` |
| Catppuccin Mocha | `#1E1E2E` | `#313244` | `#181825` | `#CDD6F4` | `#A6ADC8` | `#89B4FA` | `#1E1E2E` |
| Solarized Light | `#FDF6E3` | `#FFFBF0` | `#EEE8D5` | `#073642` | `#586E75` | `#1C6FA8` | `#FDF6E3` |
| Rosé Pine | `#191724` | `#1F1D2E` | `#16141F` | `#E0DEF4` | `#908CAA` | `#C4A7E7` | `#191724` |

Each also sets `surface_alt`, `status`, `border`, `selection`, `graph_fill`,
`graph_grid`, `good`, `warn`, `bad`, and a five-step heat ramp. Heat runs from the
background family into warm red, as the existing dark themes do.

Solarized Light's accent is darkened from the canonical `#268BD2` to `#1C6FA8` so the
near-white `on_accent` passes 4.5:1.

### One error message source

The `--theme` rejection builds its list from `ThemeName::ALL` keys instead of a literal,
so adding a theme cannot leave the help text stale.

## Risks / Trade-offs

- [Contrast] Pastel palettes can fail text or muted contrast → The existing suite runs
  over `ALL`; values are adjusted until it passes rather than lowering the threshold.
- [Picklist length] Ten entries make `t` slower to reach a specific theme → The Appearance
  dialog still cycles, and `--theme` takes an exact key.

## Chart colours per theme

Each palette gains `charts: [Color; 5]`, indexed CPU, Memory, Disk, Network, GPU.
`ChartColor::PerResource` indexes that array, replacing the hardcoded light/dark table.
The fixed colours and Theme keep their current behaviour.

Values reuse each palette's identifying hues:

| Theme | CPU | Memory | Disk | Network | GPU |
|---|---|---|---|---|---|
| Tokyo Night | `#7AA2F7` | `#BB9AF7` | `#9ECE6A` | `#E0AF68` | `#7DCFFF` |
| Catppuccin Mocha | `#89B4FA` | `#CBA6F7` | `#A6E3A1` | `#F9E2AF` | `#94E2D5` |
| Solarized Light | `#268BD2` | `#6C71C4` | `#859900` | `#CB4B16` | `#2AA198` |
| Rosé Pine | `#C4A7E7` | `#EBBCBA` | `#9CCFD8` | `#F6C177` | `#EB6F92` |

The existing six keep their current light/dark resource colours where they have them, and
gain palettes drawn from Nord, Gruvbox, Phosphor, and Amber roles. A test requires each
colour to differ from the chart surface by at least 2:1.
