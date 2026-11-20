# Design

## Context

- The README sample predates the raised tabs, hint line, system totals on the Apps row, and
  the Process menu.
- Installation is two lines and assumes a Rust toolchain and `/usr/local` write access.

## Decisions

### Capture the running app, not a mock

Take real screenshots from the release binary in the terminal, then crop the window chrome.
No generated mock can drift from the font and colours as easily as a real frame can.

### Keep the sample frame short

Replace the long sample with a short accurate one, since the screenshots carry the visual
detail.

## Risks / Trade-offs

- [Platform fonts] Screenshots come from macOS Terminal; Linux glyph rendering may differ →
  The README already explains the ASCII fallback.
