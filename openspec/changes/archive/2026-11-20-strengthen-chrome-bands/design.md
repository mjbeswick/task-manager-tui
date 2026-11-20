# Design

## Context

- `chrome` and `status` are hand-written per palette.
- Dark themes used values a few steps from `bg`; Nord, Tokyo, Catppuccin, and Rosé Pine set
  `status` equal to `bg`.
- Existing tests check text contrast on the bands, not the bands against `bg`.

## Decisions

### A minimum channel step

Every band is at least 6 steps away from `bg` on its largest channel, while staying close
enough to read as the same family. New values:

| Theme | bg | chrome | status |
|---|---|---|---|
| Light | `#F0F0F0` | `#E2E2E2` | `#E8E8E8` |
| Dark | `#202020` | `#141414` | `#181818` |
| Nord | `#2E3440` | `#242A34` | `#262C36` |
| Gruvbox | `#282828` | `#1C1C1C` | `#202020` |
| Phosphor | `#0A0F0A` | `#020502` | `#020E02` |
| Amber | `#0F0B06` | `#060402` | `#181006` |
| Tokyo Night | `#1A1B26` | `#10111A` | `#14151E` |
| Catppuccin | `#1E1E2E` | `#12121D` | `#181826` |
| Solarized | `#FDF6E3` | `#E4DECB` | `#F2ECD9` |
| Rosé Pine | `#191724` | `#100E19` | `#13111C` |

### Test the separation

For each theme, assert `max(|chrome - bg|) >= 6` and the same for `status`, per channel.

*Alternative.* Raise the contrast requirement. Rejected: bands are decoration, not text;
a channel step is the right measure.

## Risks / Trade-offs

- [Heavier bands] A visible band costs a little window brightness → It is the point; the
  content keeps its own surface.
- [Light themes] Status is only 8 steps from bg → Still perceptible and keeps the light
  window airy.
