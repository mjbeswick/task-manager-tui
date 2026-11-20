# Design

## Context

- The README sample predates the raised tabs, hint line, system totals on the Apps row, and
  the Process menu.
- Installation is two lines and assumes a Rust toolchain and `/usr/local` write access.
- A live capture of the release binary exposed the author's real process names.

## Decisions

### Render the demo data, not the live machine

`examples/screenshot.rs` draws `Snapshot::fixture()` plus a few generic processes
through the real ratatui renderer, so the frame matches the shipped UI. It writes the
buffer as HTML, and Chrome captures it as a PNG.

*Alternative.* Capture the running app. Rejected: the process table is machine-specific,
so the README would publish personal application names and PIDs.

### Keep the sample frame short

Replace the long sample with a short accurate one, since the screenshots carry the visual
detail.

## Risks / Trade-offs

- [Platform fonts] Screenshots are rendered with Menlo on macOS; Linux glyph rendering may
  differ → The README already explains the ASCII fallback.
- [Fixture drift] The demo data lives in one example → It calls the same `ui::draw` path as
  the app, so chrome changes show up when the images are regenerated.
